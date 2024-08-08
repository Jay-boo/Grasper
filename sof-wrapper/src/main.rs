use core::panic;
use chrono;
use std::time::Duration;
use std::env;
use dotenv;
use serde_json;

use sof_wrapper::client::stackExchangeClient::StackExchangeSimpleClient;
use log::{info,error, debug};
use futures::StreamExt;
use rdkafka::{producer::{self, BaseProducer, FutureProducer, FutureRecord}, ClientConfig, util::Timeout};

#[tokio::main]
async fn main()->Result<(),std::io::Error> {
    dotenv::dotenv().ok();
    env_logger::init();
    let kafka_host:String=match env::var("KAFKA_HOST"){
        Ok(kafka_host)=>{
            debug!("KAFKA_HOST = {}",kafka_host);
            kafka_host
        },
        Err(_)=>{
            error!("No KAFKA_HOST set");
            panic!()
        }

    };
    let producer:&FutureProducer=&ClientConfig::new()
        .set("bootstrap.servers", kafka_host)
        .set("request.required.acks", "1")
        .set("message.timeout.ms", "5000")
        .set("request.timeout.ms","1000")
        .create().unwrap();



    info!("Starting up ...");
    
    let sof_client=StackExchangeSimpleClient::new();
    let (mut stream_tags,join_handle)=sof_client.stream(Duration::from_secs(86000), Some(Duration::from_secs(10)));
    info!("Streaming ...");
    while let Some(items)=stream_tags.next().await{
        match items{
            Ok(tag_items)=>{
                let record_payload=serde_json::to_string(&tag_items).unwrap();
                let record: FutureRecord<'_, String, String>=FutureRecord::to("sof")
                    .payload( 
                        &record_payload
                    ).timestamp( 
                        chrono::Local::now().timestamp_millis()
                ); 
                info!("Trying to send");
                producer.send(record, Duration::from_secs(0)).await;
            },
            Err(stream_error)=>{
                log::warn!("StreamError: Not send data -> {}",stream_error);
            }
        }

        // producer.send(record, Timeout::Never).await;
        
    }
    Ok(())

}

