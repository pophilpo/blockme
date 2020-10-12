extern crate csv;
extern crate serde;

use reqwest::blocking::Client;
use data_writer::{DataWriter, Product};

mod request;
mod parser;
mod data_writer;

fn main() {


    let url = "https://thoughtbot.githu.io/rcm/rcm.7.html";

    let client = Client::new();
    let response = request::get_response(url, &client).unwrap();

    let html = parser::response_to_html(response);

    let h_selector = parser::create_css_selector("h1");


    for element in html.select(&h_selector){


        let text = parser::get_element_text(element);
        println!("{}", text);


    }

    let mut writer = DataWriter::new("test.csv".to_owned(), "test.json".to_owned());

    let product = Product {

        sku: "100B12RRE".to_string(),
        title: Some("Vortex Tab 90".to_string()),
        price: None,
        main_offer_link: "https://www.google.com".to_string(),
        main_image_link: "phil.jpg".to_string(),
        ..Default::default()

    };

    writer.populate(product);


}
