use super::*;
use rocket::local::Client;
use rocket::http::Status;

#[test]
fn levenshtein_match() {
    static JSON_URL: &str = "";
    static JSON_FILENAME: &str = "AllCards_test_data.json";
    let client = Client::new(rocket(JSON_URL, JSON_FILENAME)).expect("valid rocket instance");
    let mut response = client.get("/?card_name=darkar%20corn").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("Adarkar Unicorn".into()));
}
