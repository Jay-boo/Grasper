use sof_wrapper::client::stackExchangeClient::StackExchangeSimpleClient;
use log::{info,warn, error};

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("Starting up ...");
    
    let mut sof_client=StackExchangeSimpleClient::new();
    sof_client.fetch_all_tags().await.unwrap();
    // match sof_client.fetch_tag_popularity("python"){
    //     Ok(_)=>{},
    //     Err(e)=>error!("{:?}",e)
    // }
}

