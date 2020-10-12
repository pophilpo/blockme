
use std::error::Error;
use std::fs::File;
use std::io;
use std::process;
use csv::Writer;

use serde::Serialize;


#[derive(Default, Debug, Serialize)]
pub struct Product {

    // Really ugly pub's. Used cause idk how Derive Default works yet.
    // TODO: Create a ::new() impl.
    // TODO: Python ImageUpload Pipe requires 'images' field to be not NAN but '[]'

    pub sku: String,
    pub title: Option<String>,
    pub price: Option<String>,
    pub main_offer_link: String,
    pub main_image_link: String,
    pub images: Option<Vec<String>>,
    pub customer_images: Option<Vec<String>>,
    pub images_360: Option<Vec<String>>,
    pub desc: Option<String>,


}

pub struct DataWriter {

    csv_filename: String,
    json_filename: String,
    csv_writer: Writer<File>,


}

impl DataWriter{

    pub fn new(csv_filename: String, json_filename: String) -> DataWriter {

        let csv_writer = Writer::from_path(&csv_filename).unwrap();

        DataWriter {csv_filename, json_filename, csv_writer}
    }

    pub fn populate(&mut self, product: Product) {

        self.csv_writer.serialize(product);


    }
}
