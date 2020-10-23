use reqwest::blocking::Response;
use scraper::{ElementRef, Html, Selector};

pub fn response_to_html(response: Response) -> Html {
    // I don't like taking the ownership of Response here but I found no other way to parse it's text.
    // Seems like reading a body of html response requiers you to own it.

    let response_body = response.text().unwrap();

    Html::parse_document(&response_body)
}

pub fn create_css_selector(pattern: &str) -> Selector {
    Selector::parse(pattern).unwrap()
}

pub fn get_element_text(html_element: ElementRef) -> String {
    // TODO: Clean the string from all the shit there is

    html_element.text().collect::<String>()
}
