use std::time::Duration;

use log::{debug, info, warn};
use reqwest::{Client, Response};
use crate::client::stackExchangeClientErrors::FetchError;
use serde::Deserialize;
use futures::{Stream, channel::mpsc, task::waker, Sink, SinkExt, TryFutureExt};
use tokio::{task::JoinHandle, time::sleep};

use super::stackExchangeClientErrors::StreamError;


#[derive(Clone)]
pub struct StackExchangeSimpleClient{
    client:Client,
    tags:Vec<SOFTag>
}

#[derive(Deserialize,Debug,Clone)]
pub struct SOFTag{
    name:String,
    count:i32,
    answers:Option<i32>
}
#[derive(Deserialize,Debug)]
struct TagResponse{
    items:Vec<SOFTag>
}


impl StackExchangeSimpleClient{
    pub fn new()->Self{
        StackExchangeSimpleClient{
            client:Client::new(),
            tags:Vec::new()
        }
    } 
    async fn fetch_tag_page(&self,page_number:i32)->Result<Vec<SOFTag>,FetchError>{
        let url :&str=&format!("https://api.stackexchange.com/2.3/tags?site=stackoverflow&order=desc&sort=popular&page={}",page_number).to_string();
        let response: Response=self.client.get(url).send().await?;
        response.json::<TagResponse>().await.map_err(FetchError::from).map(|tagResponse|tagResponse.items)

    }

    pub async fn fetch_all_tags(&self)->Result<Vec<SOFTag>,FetchError>{
        let mut tags:Vec<SOFTag>=vec![];
        for p in 1..26{
            match self.fetch_tag_page(p).await{
                Ok(mut tagVec)=>{
                    tags.append(&mut tagVec)
                },
                Err(e)=>{warn!("p.{} not fetched",p)}
            };
        }
        debug!("Total number of tags :{}",tags.len());
        if tags.len()==0{
            return  Err(FetchError::new("No tags detected"));
        }
        debug!("{:?}",tags);
        Ok(tags)
    }

    async fn pull_into_sink<S:Sink<Result<Vec<SOFTag>,StreamError>>+Unpin>(&mut self,mut sink:S,sleep_time:Duration,timeout:Option<Duration>)->Result<(),S::Error>{
        loop {
            debug!("Fetching latest tag popularity");
            let tags:Result<Vec<SOFTag>,StreamError>=match timeout{
                Some(timeout_duration)=>{
                    let timeout_result:Result<Result<Vec<SOFTag>, StreamError>, tokio::time::error::Elapsed>=tokio::time::timeout(
                        timeout_duration, 
                        self.fetch_all_tags().map_err(StreamError::FetchError)
                    ).await;
                    match timeout_result{
                        Ok(fetch_result)=>fetch_result,
                        Err(timeout_error)=>Err(StreamError::TimeoutError(timeout_error))
                    }

                },
                None=>self.fetch_all_tags().await.map_err(StreamError::FetchError)
            };
            
            sink.send(tags).await;
            sleep(sleep_time).await;
            
        }
    }

    pub fn stream(&self,sleep_time:Duration,timeout:Option<Duration>)->(impl Stream<Item =Result<Vec<SOFTag>,StreamError>>,JoinHandle<Result<(),mpsc::SendError>>){
        let (sender,stream)=mpsc::unbounded();
        let mut owned_client=self.clone();
        let join_handle =tokio::task::spawn(
            async move{
                owned_client.pull_into_sink(sender,sleep_time,timeout).await
            }
        );
        (stream,join_handle)
    }
    
}

