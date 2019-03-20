#![feature(proc_macro_hygiene, decl_macro, const_vec_new)]
#[macro_use]
extern crate rocket;
extern crate strsim;

use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use rocket::response::content;
use rocket::http::RawStr;
use rocket::http::uri::Uri;
use serde_json::Value;
use strsim::normalized_levenshtein;

static mut CARD_NAMES: Vec<String> = Vec::new();

fn parse_data() {
    let mut file = File::open("AllCards.json").expect("Error while opening");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Error while parsing");

    let cards: HashMap<String, HashMap<String, Value>> = serde_json::from_str(&data).expect("Error while serializing");

    for (card_name, _value) in cards.iter() {
        unsafe {
            let decoded_card_name = Uri::percent_decode(card_name.as_bytes()).expect("Error while decoding");
            CARD_NAMES.push(decoded_card_name.to_string());
        }
    }
}

fn get_card_name_by_query(query_card_name: String) -> String {
    unsafe {
        let mut highest_leven: Option<f64> = None;
        let mut highest_leven_card_name: String = String::from("");

        for card_name in CARD_NAMES.iter() {
            let leven_score = normalized_levenshtein(card_name, query_card_name.as_str());
            if highest_leven.is_none() || highest_leven.unwrap() < leven_score {
                highest_leven = Option::from(leven_score);
                highest_leven_card_name = card_name.clone();
            }
        }
        return highest_leven_card_name;
    }
}

fn main() {
    println!("{} {}", "\u{1F4BE}", "Parsing data");
    parse_data();
    println!("{} {}", "\u{2728}", "Data parsed!");

    #[get("/?<card_name>")]
    fn json(card_name: &RawStr) -> content::Json<String> {
        let decoded_card_name = Uri::percent_decode(card_name.as_bytes()).expect("decoded");
        return content::Json(get_card_name_by_query(decoded_card_name.to_string()));
    }

    rocket::ignite().mount("/", routes![json]).launch();
}
