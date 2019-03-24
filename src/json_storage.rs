extern crate rocket;
extern crate strsim;

use std::collections::HashMap;
use rocket::http::uri::Uri;
use serde_json::Value;
use strsim::normalized_levenshtein;

static mut CARD_NAMES: Vec<String> = Vec::new();

pub fn set_data(cards: HashMap<String, HashMap<String, Value>>) {
    println!("{} {}", "\u{1F4BE}", "Parsing data");
    unsafe {
        CARD_NAMES.clear();
    }
    for (card_name, _value) in cards.iter() {
        unsafe {
            let decoded_card_name = Uri::percent_decode(card_name.as_bytes()).expect("Error while decoding");
            CARD_NAMES.push(decoded_card_name.to_string());
        }
    }
    println!("{} {}", "\u{2728}", "Data parsed!");
}

pub fn get_card_name_by_query(query_card_name: String) -> String {
    unsafe {
        let mut highest_leven: f64 = 0.0;
        let mut highest_leven_card_name: String = String::from("");

        for card_name in CARD_NAMES.iter() {
            let leven_score = normalized_levenshtein(card_name, query_card_name.as_str());
            if highest_leven < leven_score {
                highest_leven = leven_score;
                highest_leven_card_name = card_name.clone();
            }
        }
        return highest_leven_card_name;
    }
}
