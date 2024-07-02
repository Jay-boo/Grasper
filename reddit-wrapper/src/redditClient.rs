use std::{io, str::FromStr};
use reqwest::{header::{USER_AGENT, self,  AUTHORIZATION, HeaderValue}, Client};
use serde::Deserialize;
use core::fmt;
use crate::{config::Config, me::me::{self, Me}};







#[derive(Deserialize,Debug)]
#[serde(untagged)]
enum AuthResponse{
    AuthData { access_token:String},
    ErrorData { error: String },
}
impl fmt::Display for AuthResponse{
    
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            AuthResponse::AuthData { access_token }=>{
                write!(f,"Access Token: {}",access_token)
            },
                AuthResponse::ErrorData { error }=>{
                write!(f,"Error Auth : {}",error)
            }

        }
    }
}

pub struct RedditClient{
    config:Config,
    client:reqwest::Client
}

impl RedditClient {
    pub fn new(user_agent:&str,client_id:&str,client_secret:&str)-> RedditClient{
        RedditClient{
            config:Config::new(
                user_agent,
                client_id,
                client_secret
            ),
            client:reqwest::Client::new()
        }

    }

    pub async fn login(&mut self,username:&str,password:&str) ->Result<me::Me, std::io::Error>{
        let url:&str="https://www.reddit.com/api/v1/access_token";
        let form = [
                ("grant_type", "password"),
                (
                    "username",username
                ),
                (
                    "password",password
                ),
            ];
        let request=self.client
            .post(url)
            .header(USER_AGENT, &self.config.user_agent)
            .basic_auth(&self.config.client_id, Some(&self.config.client_secret))
            .form(&form);

        let response:reqwest::Response=match request.send().await{
            Ok(response)=>response,
            Err(_e)=>return Err(io::Error::new(io::ErrorKind::NotConnected, format!("Authentication request failed to {}  \nwith{:#?}!",url,form)))
        };
        if response.status()==200{
            log::debug!("code query 200");
            let auth_data:AuthResponse=match response.json::<AuthResponse>().await{
                Ok(data)=>data,
                Err(_e)=> return Err(io::Error::new(io::ErrorKind::NotConnected, format!("Not found user :{}",username)))

            };
            let access_token:String=match auth_data{
                AuthResponse::AuthData { access_token }=>access_token,
                AuthResponse::ErrorData { error }=>return Err(io::Error::new(io::ErrorKind::NotConnected, format!("No access_token found ->{}",error)))
            };
            let mut headers=header::HeaderMap::new();
            headers.insert(
                USER_AGENT,
                HeaderValue::from_str(&self.config.user_agent).unwrap(),
            );
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&format!("Bearer {}",access_token)).unwrap()
            );
            self.config.username=Some(username.to_string());
            self.config.password=Some(password.to_string());
            self.config.access_token=Some(access_token.to_string());
            self.client=Client::builder()
                .default_headers(headers)
                .build().unwrap();
            log::debug!("{}",self.config);
            return Ok(Me::new(&self.config, &self.client))

        }else{
            return Err(io::Error::new(io::ErrorKind::NotConnected, format!("Not found user :{}",username)))
        }
    }
    
}
