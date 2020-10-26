use csv::Writer;
use std::error::Error;
use std::fmt;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::process;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    // TODO: Maybe fix the skip serializing repetition?
    #[serde(skip_serializing_if = "String::is_empty")]
    pub sku: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub title: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub price: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub main_offer_link: String,

    #[serde(skip_serializing_if = "String::is_empty")]
    pub main_image_link: String,

    pub images: Vec<String>,
    pub customer_images: Vec<String>,
    pub images_360: Vec<String>,

    #[serde(skip_serializing_if = "String::is_empty")]
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
            desc: String::from(""),
        }
    }
}

impl fmt::Display for Product {
    // Wtf is this?
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Product: \n  sku: {} \n  title: {} \n  price: {} \n  main_offer_link: {} \n  main_image_link: {} \n  images: {:?} \n  customer_images: {:?} \n  images_360: {:?} \n  desc: {}",
            self.sku, self.title, self.price, self.main_offer_link, self.main_image_link, self.images, self.customer_images, self.images_360, self.desc
        )
    }
}

pub struct DataWriter {
    csv_filename: String,
    json_filename: String,

    pub storage: Vec<Product>,
}

impl DataWriter {
    // TODO: Store filepaths as std::...paths not Strings.

    pub fn new(csv_filename: String, json_filename: String, storage: Vec<Product>) -> DataWriter {
        DataWriter {
            csv_filename,
            json_filename,
            storage,
        }
    }

    pub fn populate(&mut self, product: Product) {
        self.storage.push(product);
    }

    pub fn popuplate_from_json(&mut self) {
        let data = fs::read_to_string(&self.json_filename).unwrap();

        let json_data: Vec<Product> = serde_json::from_str(&data).unwrap();

        for product in json_data {
            self.populate(product);
        }
    }

    pub fn write_to_json(&self) {
        let mut json_file = File::create(&self.json_filename).unwrap();
        let json_data = serde_json::to_vec(&self.storage).unwrap();

        json_file.write(&json_data).unwrap();
    }

    pub fn write_to_csv(&self) {
        let mut writer = Writer::from_path(&self.csv_filename).unwrap();
        for product in &self.storage {
            println!("{}", product);
            writer.serialize(&product).unwrap();
        }
    }
}
