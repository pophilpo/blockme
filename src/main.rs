use blockme::Parser;
fn main() {


    let url = "https://www.google.com/";


    let response = Parser::get_response(url);

    match response {
        Ok(r) => println!("{}", r.status_code),
        Err(e) => println!("{}", e)
    }



}
