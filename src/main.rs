#![feature(proc_macro_hygiene, decl_macro, const_vec_new, mpsc_select)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;

mod mtg_data;
mod json_storage;

use std::path::Path;
use rocket::response::content;
use rocket::http::RawStr;
use rocket::http::uri::Uri;

use std::sync::mpsc::channel;
use std::{thread, fs};
use std::time::Duration;
use rocket::Rocket;

fn refresh_json_storage(json_url: &str, json_filename: &str) {
    println!("\u{1F4BE} Clearing old data");

    if Path::new(json_filename).exists() == true {
        fs::remove_file(json_filename).expect("Error while removing the file");
    }

    mtg_data::download_file(json_url, json_filename);
    json_storage::set_data(mtg_data::read_json(json_filename));
}

#[get("/?<card_name>")]
fn json(card_name: &RawStr) -> content::Json<String> {
    let decoded_card_name = Uri::percent_decode(card_name.as_bytes()).expect("decoded");
    return content::Json(json_storage::get_card_name_by_query(decoded_card_name.to_string()));
}

fn rocket(json_url: &str, json_filename: &str) -> Rocket {
    if Path::new(json_filename).exists() == true {
        json_storage::set_data(mtg_data::read_json(json_filename));
    } else {
        refresh_json_storage(json_url, json_filename);
    }

    rocket::ignite().mount("/", routes![json])
}

fn main() {
    static JSON_URL: &str = "https://mtgjson.com/json/AllCards.json";
    static JSON_FILENAME: &str = "AllCards.json";
    let (send, recv) = channel();

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(21600));
            send.send("\u{1F4BE} Starting to update json_storage data").unwrap();
        }
    });

    thread::spawn(move || {
        loop {
            println!("{}", recv.recv().unwrap());
            refresh_json_storage(JSON_URL, JSON_FILENAME);
        }
    });

    rocket(JSON_URL, JSON_FILENAME).launch();
}

#[cfg(test)]
mod integration_test;
