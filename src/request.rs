use reqwest::blocking::{Client, Response};
use std::error::Error;

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

    let _request_url = format!("http://api.scraperapi.com/?api_key={}&url={}", api_key, url);

    let response = client.get(url).send()?;

    Ok(response)
}
