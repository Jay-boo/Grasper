use std::io;
use std::str::FromStr;

use log::log;
use reqwest::header::{HeaderMap, AUTHORIZATION, USER_AGENT,HeaderValue};
use reqwest::{Response, Client};
use serde::Serialize;
use crate::subreddit::subreddit::{Subreddit};
use crate::subreddit::feedoptions::{FeedFilter,FeedSort};
use crate::url::buildUrl;

use crate::config::{Config };
pub struct Me{
    config:Config,
    client:reqwest::Client
}

impl Me{
    pub fn new(config:&Config,client:&reqwest::Client)->Me{
        Me{
            config:config.to_owned(),
            client:client.to_owned()
        }
    }

    async fn get(&self,dest:&str)->Result<Response,io::Error>{
        let built_url:String=buildUrl(&dest.to_string());
        log::debug!("Aimed Url for GET: {}",built_url);
        let response:Response= match self.client.get(&built_url).send().await{
            Ok(response)=>response,
            Err(_e)=> return Err(io::Error::new(io::ErrorKind::NotConnected,"Not valid  url request "))
        };


        if response.status()==200 {
            log::debug!("Success");
        }
        else if response.status()==403 {
            log::debug!("Header format problem met");
            let mut headers=HeaderMap::new();
            headers.insert(USER_AGENT, HeaderValue::from_str(&self.config.user_agent).unwrap());
            let resp:Response=match Client::builder().default_headers(headers).build().unwrap().get(&built_url).send().await{
                Ok(response)=>response,
                Err(_e)=> return Err(io::Error::new(io::ErrorKind::NotConnected,"Not valid  url request "))
            };
            if resp.status()==200{
                return Ok(resp);
            }else{
                return Err(io::Error::new(io::ErrorKind::NotConnected,format!("Authentication request failed ! Watching  response status : {}",resp.status())));
            }
        }
        else{
            return Err(io::Error::new(io::ErrorKind::NotConnected,format!("Authentication request failed ! Watching  response status : {}",response.status())));
        }
        Ok(response)
    }



    async fn post<T:Serialize>(&self,dest:&str,form:&T)->Result<Response,io::Error>{
        let built_url:String=buildUrl(&dest.to_string());
        log::debug!("Aimed Url for POST: {}",built_url);
        let response:Response= match self.client.post(&built_url)
            .form(form)
            .send().await{
            Ok(response)=>response,
            Err(_e)=> return Err(io::Error::new(io::ErrorKind::NotConnected,"Not valid  url request "))
        };
        if response.status()!=200 {
            return Err(io::Error::new(io::ErrorKind::NotConnected,format!("Authentication request failed ! Watching  response status : {}",response.status())));
        }
        Ok(response)
    }


    pub async fn get_subreddit(&self,subreddit_name:&str,feed_limit:Option<i64>,sortMethod:FeedFilter)-> Subreddit{
        let mut headers=HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_str(&self.config.user_agent).unwrap());
        let subreddit_client:Client=Client::builder().default_headers(headers).build().unwrap();
        let mut subreddit:Subreddit=Subreddit::new(subreddit_name,Some(&subreddit_client));
        subreddit.get_about().await;
        subreddit
    }
}

impl Default for Me{
    fn default() -> Self {
        let mut headers=HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_str("useragentdefault").unwrap());

        Me { 
            config: Config::default(),
            client: reqwest::Client::builder().default_headers(headers).build().unwrap()
        }
    }
}






#[cfg(test)]
mod tests{
    use crate::{config::Config, subreddit::feedoptions::FeedFilter};
    use log::{info,debug};
    use super::Me;


    #[tokio::test]
    async fn test_no_auth_reddit_client(){
        let _ = env_logger::try_init();
        info!("Test : Default Reddit client without credentials");

        let reddit_client:Me=Me::default();
        let rust_subreddit=reddit_client.get_subreddit(
            "r/rust",
            None,
            FeedFilter::Hot
        ).await;
        assert_eq!(rust_subreddit.name,"r/rust");
        assert_ne!(rust_subreddit.about.clone(),None);
        debug!("Checking r/rust about part {}",rust_subreddit.about.unwrap());
    }
}
