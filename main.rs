extern crate clap;
use clap::{App, SubCommand};

extern crate serde;
use serde::{Serialize, Deserialize};

extern crate reqwest;
use reqwest::blocking::Client;

#[derive(Serialize, Deserialize, Debug)]
struct Currency {
    id: String,
    name: String,
    min_size: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Currencies {
    data: Vec<Currency>
}

#[derive(Serialize, Deserialize, Debug)]
struct Price {
    base: String,
    currency: String,
    amount: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Prices {
    data: Price
}

impl Prices {
    fn get(&mut self) {
        let client = Client::new();
        let res = client.get("https://api.coinbase.com/v2/prices/BTC-USD/buy").send();
        match res {
            Ok(parsed) => {
                let price = parsed.json::<Prices>().unwrap();
                println!("🪙  {base} - {currency} - {amount}",
                    base=price.data.base,
                    currency=price.data.currency,
                    amount=price.data.amount);
            }
            Err(e) => println!("Err: {:?}", e),
        }
    }

}


impl Currencies {
    fn get(&mut self) {
        println!("Get currencies from Coinbase");
        let client = Client::new();
        let res = client.get("https://api.coinbase.com/v2/currencies")
            .send();
        match res {
            Ok(parsed) => {
                let currencies = parsed.json::<Currencies>().unwrap();
                for currency in &currencies.data {
                    println!("💰 {id}, {name}",
                        id=currency.id,
                        name=currency.name);
                }
                self.data = currencies.data;
            }
            Err(e) => println!("Err: {:?}", e),
        }
    }
}

fn main() {
    let app = App::new("coin")
        .version("v0.1")
        .author("Oscarmlage Live Twitch <info@oscarmlage.com>")
        .about("Testing tool Rust + Coinbase")
        .subcommand(
            // [currencies]
            SubCommand::with_name("currencies")
            .about("Get currencies from Coinbase")
        )
        .subcommand(
            // [prices]
            SubCommand::with_name("prices")
            .about("Get prices from Coinbase")
        );

    let matches = app.get_matches();

    match matches.subcommand() {
        ("currencies", Some(_matches)) => {
            println!("Currencies!");
            let mut currencies = Currencies {
                data: Vec::new(),
            };
            currencies.get();
        },

        ("prices", Some(_matches)) => {
            println!("Prices!");
            let mut prices = Prices {
                data: Price {
                    base: "".to_string(),
                    currency: "".to_string(),
                    amount: "".to_string()
                },
            };
            prices.get();
        },
        _ => println!("What!"),
    }
}
