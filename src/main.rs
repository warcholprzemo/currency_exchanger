/*
To use this app we need to run a server on http://localhost:8000/currencies.json which
will give a JSON response like
{
    "PLN": "4.19",
    "EUR": "0.78"
}
*/

use std::{thread, time};
use std::collections::HashMap;
use chrono::{Utc, DateTime};
use reqwest::blocking::get;

#[derive(Debug)]
struct Currency {
    iso_code: String,
    value: f32,
    updated: DateTime<Utc>
}

impl Currency {
    fn new(iso_code: String, value: f32) -> Currency {
        Currency {
            iso_code: iso_code,
            value: value,
            updated: Utc::now()
        }
    }
    fn update(&mut self, new_value: f32) {
        self.value = new_value;
        self.updated = Utc::now();
    }
}

fn main() {
    let pln: Currency = Currency::new(String::from("PLN"), 4.04);
    let eur: Currency = Currency::new(String::from("EUR"), 0.77);
    let mut currencies = Vec::new();
    currencies.push(pln);
    currencies.push(eur);

    loop {
        let resp = get("http://localhost:8000/currencies.json");
        let good_response = match resp {  // <Result>
            Ok(data) => data,
            Err(e) => { panic!("Problem with the response {:?}", e) }
        };
        let data = good_response.json::<HashMap<String, String>>().unwrap();

        for currency in &mut currencies {
            let value_as_result = data.get(&currency.iso_code); // <Option>
            let value_as_str = match value_as_result {
                Some(value) => value,
                None => {
                    println!("Value does not exist in data for currency {}", currency.iso_code);
                    continue;
                }
            };
            let value_as_float = value_as_str.parse::<f32>().unwrap();

            if currency.value != value_as_float {
                currency.update(value_as_float);
                println!("Updated {:?}", currency);
            } else {
                println!("No update for {}", currency.iso_code);
            }
        }
        println!("---------------");
        let seconds = time::Duration::from_secs(5);
        thread::sleep(seconds);
    }
}
