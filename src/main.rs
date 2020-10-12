use reqwest::blocking::Client;

mod request;
mod parser;

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

}
