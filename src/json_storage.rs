extern crate rocket;
extern crate strsim;

use std::collections::HashMap;
use strsim::normalized_levenshtein;
use std::sync::RwLock;
use crate::mtg_data::Card;

lazy_static! {
    static ref CARDS: RwLock<Vec<Card>> = RwLock::new(vec![]);
}

fn get_hashmap_values<T>(hashmap: HashMap<String, T>) -> Vec<T> where T: Clone {
    let mut vector: Vec<T> = vec![];
    for value in hashmap.values() {
        vector.push(value.clone())
    }
    vector
}

pub fn set_data(cards: HashMap<String, Card>) {
    println!("{} {}", "\u{1F4BE}", "Parsing data");
    CARDS.write().unwrap().clone_from(&get_hashmap_values(cards));
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
