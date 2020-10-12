use std::error::Error;
use std::fs::File;
use std::io;
use std::process;
use csv::Writer;
use std::io::prelude::*;

use serde::Serialize;


#[derive(Debug, Serialize)]
pub struct Product {

    pub sku: String,
    pub title: String,
    pub price: String,
    pub main_offer_link: String,
    pub main_image_link: String,
    pub images: Vec<String>,
    pub customer_images: Vec<String>,
    pub images_360: Vec<String>,
    pub desc: String,


}


impl Default for Product {

     fn default() -> Self {

            Product {
                sku: String::from(""),
                title: String::from(""),
                price: String::from(""),
                main_offer_link: String::from(""),
                main_image_link: String::from(""),
                images: Vec::new(),
                customer_images: Vec::new(),
                images_360: Vec::new(),
                desc: String::from("")

        }
    }


}


pub struct DataWriter {

    csv_filename: String,
    json_filename: String,

    pub storage: Vec<Product>,


}

impl DataWriter{

    // TODO: Store filepaths as std::...paths not Strings.


    pub fn new(csv_filename: String, json_filename: String, storage: Vec<Product>) -> DataWriter {

        DataWriter{csv_filename, json_filename, storage}

    }


    pub fn populate(&mut self, product: Product) {

        self.storage.push(product);

    }

    pub fn write_json(&mut self) {

        let mut json_file = File::create(&self.json_filename).unwrap();
        let json_data = serde_json::to_vec(&self.storage).unwrap();

        json_file.write(&json_data).unwrap();

    }



}
