use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;

mod config;

#[derive(Deserialize)]
struct ApiResponse {
    bitcoin: HashMap<String, f64>, // Usando HashMap para permitir várias moedas
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::current();
    let fiat = get_fiat();

    // CoinGecko API
    let url = format!("https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies={}", config.fiat);
    let client = Client::new();
    let response: ApiResponse = client.get(&url).send().await?.json().await?;

    // Acessa o valor da moeda, independentemente do nome da chave (por exemplo, "brl" ou outro)
    if let Some(btc_price) = response.bitcoin.get(&config.fiat) {
        let satoshi_brl = btc_price / 100_000_000.0;

        // Imprimindo os resultados
        println!("{}", fiat * satoshi_brl );
    } else {
        eprintln!("Erro: A moeda especificada não foi encontrada na resposta.");
    }

    Ok(())
}


fn get_fiat() -> f64 {
    match env::args().nth(1) {
        Some(arg) => match arg.parse::<f64>() {
            Ok(value) => value,  // Se a conversão for bem-sucedida, retorna o valor
            Err(_) => {
                eprintln!("Erro: O argumento fornecido não pode ser convertido para um número.");
                std::process::exit(1); // Sai com código de erro
            }
        },
        None => {
            eprintln!("Erro: Esperado um argumento.");
            std::process::exit(1); // Sai com código de erro
        }
    }
}
