extern crate csv;
extern crate serde;
extern crate serde_json;

use data_writer::{DataWriter, Product};
use reqwest::blocking::Client;

mod data_writer;
mod parser;
mod request;

fn main() {
    let url = "https://thoughtbot.githu.io/rcm/rcm.7.html";

    let client = Client::new();
    let response = request::get_response(url, &client).unwrap();

    let html = parser::response_to_html(response);

    let h_selector = parser::create_css_selector("h1");

    for element in html.select(&h_selector) {
        let text = parser::get_element_text(element);
        println!("{}", text);
    }

    let mut writer = DataWriter::new("test.csv".to_owned(), "test.json".to_owned(), Vec::new());

    let mut product = Product {
        sku: "".to_string(),
        title: "".to_string(),
        price: "100 EUR".to_string(),
        main_offer_link: "https://google.com".to_string(),
        main_image_link: "vortex.jpg".to_string(),
        images: String::from("a.jpg;b.jpg"),
        customer_images: String::from(""),
        images_360: String::from(""),
        desc: "Awesome keyboard.".to_string(),
    };

    println!("{}", product);
}
