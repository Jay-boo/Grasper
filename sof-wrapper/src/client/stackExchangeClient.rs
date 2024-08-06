use log::{debug, info, warn};
use reqwest::{Client, Response};
use crate::client::stackExchangeClientErrors::FetchError;
use serde::Deserialize;

pub struct StackExchangeSimpleClient{
    client:Client,
    tags:Vec<SOFTag>
}

#[derive(Deserialize,Debug)]
pub struct SOFTag{
    name:String
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

    pub async fn fetch_all_tags(&mut self)->Result<(),FetchError>{
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
        self.tags=tags;
        Ok(())
    }
    
    pub fn fetch_tag_popularity(&self,tag:&str)-> Result<(),FetchError>{
        // let url= 
        // self.client.get(url)
        Ok(())


    }
}

