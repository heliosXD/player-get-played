
use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};


use hyper::body;
use hyper::{ 
    Client,
    Body,
    Method,
    Request,
    Uri
};

use hyper_tls::HttpsConnector;

use serde::{ 
    Deserialize,
    Serialize
};

#[derive(Serialize, Deserialize)]
pub struct btcInfo {
    Price: Vec<Value>, 
}

use chrono;
use std::error::Error;
use std::{
    thread,
    time::{Duration, Instant},
};

use serde_json::{ 
    Result,
    Value
};


#[tokio::main]
pub async fn main() -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
    
    
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let URL: hyper::Uri = "https://sochain.com/api/v2/get_info/BTC".parse()?;
    let res = client
        .get(URL.clone())
        .await?;
    let mut btccc = Vec::new();


    // parse conversion
    let bod_byte = body::to_bytes(res.into_body()).await?;
    let body = String::from_utf8(bod_byte.to_vec())
        .expect("resp not utf8");
    let v: Value = serde_json::from_str(&body).unwrap();
    for e in v["data"].as_object().unwrap(){
        btccc.push(e);
      }

    let price = btccc[6];
    println!("{:?}", price);
    //println!("{:?}", v["data"]["price"]);
    //let INFO = v["data"].as_array().unwrap();
    //println!("{}", tester.Price);
    //println!("{:#?}", INFO);

    //TODO how do people update lastest fetch carible
      //println!("{:?}", price)

    Ok(())
}
