extern crate rocket;
extern crate strsim;

use std::collections::HashMap;
use rocket::http::uri::Uri;
use strsim::normalized_levenshtein;
use std::sync::RwLock;
use crate::mtg_data::Card;

lazy_static! {
    static ref CARDS: RwLock<Vec<Card>> = RwLock::new(vec![]);
}

fn cards_includes(decoded_card_name: String) -> bool {
    return CARDS.read().unwrap().iter().find(|&x| x.name == decoded_card_name).is_some();
}

pub fn set_data(cards: HashMap<String, Card>) {
    println!("{} {}", "\u{1F4BE}", "Parsing data");
    for (card_name, value) in cards.iter() {
        let decoded_card_name = Uri::percent_decode(card_name.as_bytes()).unwrap();
        if cards_includes(decoded_card_name.to_string()) == false {
            CARDS.write().unwrap().push(value.clone());
        }
    }
    println!("{} {}", "\u{2728}", "Data parsed!");
}

pub fn get_card_name_by_query(query_card_name: String) -> Option<Card> {
    let mut highest_leven: f64 = 0.0;
    let mut highest_card: Option<Card> = None;

    for card in CARDS.read().unwrap().iter() {
        let leven_score = normalized_levenshtein(card.name.as_str(), query_card_name.as_str());
        if highest_leven < leven_score {
            highest_leven = leven_score;
            highest_card = Option::from(card.clone());
        }
    }

    return highest_card;
}
