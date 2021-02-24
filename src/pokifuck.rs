

use tokio;
use std::io;

use reqwest;
use reqwest::{Error, header};
use reqwest::header::{HeaderName};
use std::io::Read;
use std::collections::HashMap;
use std::fs::File;
use std::{
    thread,
    time::{Duration, Instant},
};

use serde::{ 
    Deserialize,
    Serialize
};



#[tokio::main]
async fn main() -> Result<(), Error>{

    eprintln!("

/$$$$$$$           /$$       /$$ /$$$$$$$$                  /$$      
| $$__  $$         | $$      |__/| $$_____/                 | $$      
| $$  \\ $$ /$$$$$$ | $$   /$$ /$$| $$    /$$   /$$  /$$$$$$$| $$   /$$
| $$$$$$$//$$__  $$| $$  /$$/| $$| $$$$$| $$  | $$ /$$_____/| $$  /$$/
| $$____/| $$  \\ $$| $$$$$$/ | $$| $$__/| $$  | $$| $$      | $$$$$$/ 
| $$     | $$  | $$| $$_  $$ | $$| $$   | $$  | $$| $$      | $$_  $$ 
| $$     |  $$$$$$/| $$ \\  $$| $$| $$   |  $$$$$$/|  $$$$$$$| $$ \\  $$
|__/      \\______/ |__/  \\__/|__/|__/    \\______/  \\_______/|__/  \\__/

1) Fetch Product info
2) Monitor Product/Category
------------------------------------------------------------------------------
");

    let mut line = String::new();
    println!("Please enter a command to proceed: ");
        
    loop {
        io::stdin().read_line(&mut line);

        match line.trim() {
            "1" => option1(),
            _ =>  panic!("invalid command")
        }
    }

}




fn option1() {
    println!("this is option 1")
}

fn optionERR() {
    println!("Damn no work")
}



#[tokio::main]
async fn monitor() -> Result<(), Error> {

    let mut headers = header::HeaderMap::new();
    println!("these are my Headers i need to format good\n{:#?}\n", headers);


    let url = "https://www.pokemoncenter.com/tpci-ecommweb-api/product/701-08514?format=zoom.nodatalinks";
    let client = reqwest::Client::builder()
       .build()?;

    let res = client.get(url)   
       .send()
       .await?;

    println!("Status: {:?}\n", res.status());   
    println!("Headers: {:?}\n", res.headers());

    let body = res.text().await?;   
    println!("Body: {:?}\n", body);

  Ok(())                            
}

