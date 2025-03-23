use std::env;

mod request;
mod url;
use request::Request;
use url::parse_url;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let url = parse_url(&args[1]);

    let request = Request {
        url,
        method: "GET".to_string(),
        headers: vec![
            ("User-Agent".to_string(), "RustHttpClient/1.0".to_string()),
            ("Accept".to_string(), "*/*".to_string()),
            ("Connection".to_string(), "close".to_string()),
        ],
        body: String::new(),
    };

    request.send()?;

    Ok(())
}
