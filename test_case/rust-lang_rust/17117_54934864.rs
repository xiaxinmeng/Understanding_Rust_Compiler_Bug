
extern crate url;
extern crate http;

use url::Url;
use http::client::request::{RequestWriter};
use http::method::Get;

fn get_page(uri: &str) -> str {
    let url = Url::parse(uri).unwrap();
    let request: RequestWriter = match RequestWriter::new(Get, url) {
        Ok(request) => request,
        Err(error) => fail!("{}", error),
    };
    let mut response = match request.read_response() {
        Ok(response) => response,
        Err((_request, error)) => fail!("{}", error),
    };
    let body = match response.read_to_string() {
        Ok(body) => body,
        Err(error) => fail!("Failed to read response body {}", error),
    };
    *body.as_slice()
}

fn main() {
     let body = get_page("https://www.google.com/");
     println!("{:s}", body);
}
