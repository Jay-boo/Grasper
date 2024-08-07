use std::time::Duration;

use sof_wrapper::client::stackExchangeClient::StackExchangeSimpleClient;
use log::{info,debug};
use futures::StreamExt;

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("Starting up ...");
    
    let mut sof_client=StackExchangeSimpleClient::new();
    let (mut stream_tags,join_handle)=sof_client.stream(Duration::from_secs(30), Some(Duration::from_secs(10)));
    info!("Streaming ...");
    while let Some(items)=stream_tags.next().await{
        
    }

}

