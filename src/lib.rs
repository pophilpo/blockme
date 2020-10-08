use reqwest;


pub struct Response {

    pub status_code: u16,
    html: String
}

impl Response{

    pub fn new(status_code: u16, html: String) -> Response {
        Response{status_code, html}
    }
}

pub struct Parser {

    use_scraperapi: bool,
    project_name: String,


}

impl Parser {


    pub fn new(use_scraperapi: bool, project_name: String) -> Parser {
        Parser{use_scraperapi, project_name}
    }


    pub fn get_response(url: &str) -> Result<Response, reqwest::Error> {

        let response = reqwest::blocking::get(url)?;
        let status_code = response.status().as_u16();
        let html = response.text()?;

        let response = Response::new(status_code, html);

        Ok(response)
    }
}
