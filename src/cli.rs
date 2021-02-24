
use structopt::StructOpt;
use exitfailure::{ExitFailure};
use serde;
use serde_derive::{Deserialize, Serialize};
use reqwest::{Url, Client};
use serde_json::json;
use tokio;

#[derive(StructOpt)]
struct Cli {
    method: String
}

#[derive(Serialize, Deserialize, Debug)]
struct InfoData {
    data: Data
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    price: String
}
// method is what the btc is so like eth, doge, BTC etc.....


impl InfoData{
    async fn get(method: &String) -> Result<Self, ExitFailure> {
        let url = format!("https://sochain.com/api/v2/get_info/{}", method);
        let url = Url::parse(&*url)?;

        let resp = reqwest::get(url)
            .await?
            .json::<InfoData>()
            .await?;
        Ok(resp)

    }
}




#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let response = InfoData::get(&args.method).await?;
    println!("out method is: {}, and value of price is {}",args.method, response.data.price);

    Ok(())

}



