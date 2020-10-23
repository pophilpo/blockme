use reqwest::blocking::{Client, Response};
use std::error::Error;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]

    fn request_success() {
        let url = "https://www.rust-lang.org/";
        let response = get_response(url);

        match response {
            Ok(r) => assert_eq!(200, r.status().as_u16()),
            Err(e) => println!("{}", e),
        }
    }
}

pub fn get_response(url: &str, client: &Client) -> Result<Response, Box<dyn Error>> {
    // Simple blocking request

    let response = client.get(url).send()?;

    Ok(response)
}

pub fn get_scraperapi_response(
    url: &str,
    client: &Client,
    api_key: &str,
) -> Result<Response, Box<dyn Error>> {
    // Simple blocking request using ScraperApi. Api key is a very secret thing!

    let request_url = format!("http://api.scraperapi.com/?api_key={}&url={}", api_key, url);

    let response = client.get(url).send()?;

    Ok(response)
}
