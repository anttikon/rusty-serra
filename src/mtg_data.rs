extern crate reqwest;

use std::fs::File;
use std::io::{Read, Error, copy};
use std::collections::HashMap;

fn create_file(filename: &str) -> Result<File, Error> {
    return {
        File::create(filename)
    };
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Card {
    pub colors: Vec<String>,
    pub name: String,
}

pub fn download_file(url: &str, filename: &str) {
    println!("{} {} {}", "\u{1F4BE}", "Requesting", url);
    let mut response = match reqwest::get(url) {
        Ok(val) => val,
        Err(err) => panic!("Cannot download {} {}", url, err),
    };

    println!("{} {} {}", "\u{1F4BE}", "Creating", filename);
    let mut dest: File = match create_file(filename) {
        Ok(val) => val,
        Err(err) => panic!("Cannot create file {} {}", filename, err),
    };

    println!("{} {} {}", "\u{1F4BE}", "Copying data to", filename);
    match copy(&mut response, &mut dest) {
        Ok(val) => val,
        Err(err) => panic!("Copy content to file {}", err),
    };
}

pub fn read_json(filename: &str) -> HashMap<String, Card> {
    let mut file = File::open(filename).expect("Error while opening");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Error while parsing");

    return serde_json::from_str::<HashMap<String, Card>>(&data).unwrap();
}
