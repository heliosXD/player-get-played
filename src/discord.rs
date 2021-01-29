
#[path = "parse.rs"]
mod parse;

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

use tokio;

const HELP_MESSAGE: &str = "ok";

println!("{:?}", Price);
const HELP_COMMAND: &str = "!help";

const token: &str = "NzY4NjQ0NzI1NDE0Mjk3NjIy.X5DeLw.hXX6sp0Ns2WArSGAXMBZai4j3dc";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == HELP_COMMAND {
            if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
pub async fn starter() {

    let mut clientDiscord = serenity::Client::new(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = clientDiscord.start().await {
        println!("Client error: {:?}", why);
    }
}