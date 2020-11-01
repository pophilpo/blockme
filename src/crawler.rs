use crate::data_writer;
use reqwest::blocking::{Client, Response};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

pub struct Crawler {
    pub urls: Vec<String>,

    // Crawler takes the crawling function, since all websites are different. This might be a bad
    // practice
    pub parse_function: fn(Response) -> data_writer::Product,

    pub client: Client,

    pub headers: HeaderMap,
}

impl Crawler {
    pub fn new(urls: Vec<String>, parse_function: fn(Response) -> data_writer::Product) -> Crawler {
        Crawler {
            urls,
            parse_function,
            client: Client::new(),
            headers: HeaderMap::new(),
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

    pub fn read_headers(&mut self, filename: &str) {
        let file = File::open(filename).expect("Failed to read file");

        let lines = io::BufReader::new(file).lines();

        for line in lines {
            let line = line.unwrap();
            let mut values: Vec<&str> = line.split(": ").collect();
            // Pile of sh*t
            let header_value: HeaderValue = values.pop().unwrap().parse().unwrap();
            let header_name: HeaderName = values.pop().unwrap().parse().unwrap();

            self.headers.insert(header_name, header_value);
        }
    }
}
