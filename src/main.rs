use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;

mod config;

#[derive(Deserialize)]
struct ApiResponse {
    bitcoin: HashMap<String, f64>,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::current();
    let sats = get_sats();

    let coingeko_api = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies={}",
        config.fiat
    );
    let client = Client::new();
    let response: ApiResponse = client.get(&coingeko_api).send().await?.json().await?;

    if let Some(btc_price) = response.bitcoin.get(&config.fiat) {
        let sats_price = btc_price / 100_000_000.0;
        println!("{}", sats * sats_price);
    } else {
        eprintln!("Error: Fiat currency not found");
    }

    Ok(())
}

fn get_sats() -> f64 {
    match env::args().nth(1) {
        Some(arg) => match arg.parse::<f64>() {
            Ok(value) => value,
            Err(_) => {
                eprintln!("Error: argument is not a valid satoshi value");
                std::process::exit(1);
            }
        },
        None => {
            eprintln!("Error: Expected a value in satoshi");
            std::process::exit(1);
        }
    }
}
