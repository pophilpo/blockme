#![allow(dead_code)]
#![allow(unused_imports)]
extern crate csv;
extern crate serde;
extern crate serde_json;

use data_writer::{DataWriter, Product};
use reqwest::blocking::{Client, Response};

mod crawler;
mod data_writer;
mod parser;

fn crawl_rust_header(response: Response) -> Product {
    let mut product = Product {
        ..Default::default()
    };

    let html = parser::response_to_html(response);

    let h_selector = parser::create_css_selector("h1");

    for element in html.select(&h_selector) {
        let lang_name = parser::get_element_text(element);
        product.title = lang_name;
    }
    product
}

fn main() {
    /*
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

    let product = Product {
        sku: "AA0B11cJ".to_string(),
        title: "Vortex Tab 90".to_string(),
        price: "100 EUR".to_string(),
        main_offer_link: "https://google.com".to_string(),
        main_image_link: "vortex.jpg".to_string(),
        images: String::from("a.jpg;b.jpg"),
        customer_images: String::from(""),
        images_360: String::from(""),
        desc: "Awesome keyboard.".to_string(),
    };

    writer.populate(product);
    writer.write_to_csv();
    */

    let crawler = crawler::Crawler::new(vec!["simple".to_string()], crawl_rust_header);

    let product = crawler.crawl_product("https://www.rust-lang.org");
    println!("{}", product);
}
