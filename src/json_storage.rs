extern crate rocket;
extern crate strsim;

use std::collections::HashMap;
use rocket::http::uri::Uri;
use serde_json::Value;
use strsim::normalized_levenshtein;
use std::sync::RwLock;

lazy_static! {
    static ref CARD_NAMES: RwLock<Vec<String>> = RwLock::new(vec![]);
}

fn vector_includes(decoded_card_name: &String) -> bool {
    return CARD_NAMES.read().unwrap().iter().find(|&x| x == decoded_card_name).is_some();
}

pub fn set_data(cards: HashMap<String, HashMap<String, Value>>) {
    println!("{} {}", "\u{1F4BE}", "Parsing data");
    for (card_name, _value) in cards.iter() {
        let decoded_card_name = Uri::percent_decode(card_name.as_bytes()).expect("Error while decoding").to_string();
        if vector_includes(&decoded_card_name) == false {
            CARD_NAMES.write().unwrap().push(decoded_card_name);
        }
    }
    println!("{} {}", "\u{2728}", "Data parsed!");
}

pub fn get_card_name_by_query(query_card_name: String) -> String {
    let mut highest_leven: f64 = 0.0;
    let mut highest_leven_card_name: String = String::from("");

    for card_name in CARD_NAMES.read().unwrap().iter() {
        let leven_score = normalized_levenshtein(card_name, query_card_name.as_str());
        if highest_leven < leven_score {
            highest_leven = leven_score;
            highest_leven_card_name = card_name.clone();
        }
    }
    return highest_leven_card_name;
}
