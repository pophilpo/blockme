use crate::data_writer;
use reqwest::blocking::{Client, Response};
use std::error::Error;

pub struct Crawler {
    pub urls: Vec<String>,

    // Crawler takes the crawling function, since all websites are different. This might be a bad
    // practice
    pub parse_function: fn(Response) -> data_writer::Product,

    pub client: Client,
}

impl Crawler {
    pub fn new(urls: Vec<String>, parse_function: fn(Response) -> data_writer::Product) -> Crawler {
        Crawler {
            urls,
            parse_function,
            client: Client::new(),
        }
    }

    pub fn get_response(&self, url: &str) -> Result<Response, Box<dyn Error>> {
        let response = self.client.get(url).send()?;

        Ok(response)
    }

    pub fn crawl_product(&self, url: &str) -> data_writer::Product {
        let response = self.get_response(url).unwrap();

        (self.parse_function)(response)
    }
}
