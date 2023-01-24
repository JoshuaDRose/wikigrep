use http::{Request, Response};
use http::client::Client;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let request = Request::get("http://httpbin.org/ip").body(())?;
    let response = client.send(request)?;
    println!("{:?}", response);
    Ok(())
}
