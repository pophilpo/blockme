use reqwest::blocking::Client;

mod request;

fn main() {


    let url = "https://www.rust-lang.org/";

    let client = Client::new();

    let response = request::get_response(url, &client);
    println!("Got response!!!");

    match response {

        Ok(r) => println!("{}", r.status()),
        Err(e) => println!("{}", e)
    }

}
