use std::env;

mod http;
mod request;
mod response;
mod url;

use clap::{command, Arg, ArgAction};
use http::HttpExchange;
use request::Request;
use url::parse_url;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = command!()
        .arg(Arg::new("url_input").required(true).help("help goes here"))
        .arg(
            Arg::new("method")
                .short('X')
                .action(ArgAction::Set)
                .default_value("GET"),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(ArgAction::SetTrue)
                .help("help goes here"),
        )
        .get_matches();

    // Parse arguments
    let url_input = matches.get_one::<String>("url_input").unwrap();
    let method = matches.get_one::<String>("method").unwrap();
    let verbose = matches.get_flag("verbose");

    // Parse URL
    let url = parse_url(url_input);
    let request = Request::new(method, url);

    // Setup exchange
    let mut http_exchange = HttpExchange::new(request);

    // Send
    http_exchange.send()?;

    if verbose {
        http_exchange.pprint_request();
        http_exchange.pprint_response_header();
    }

    // Print body
    println!("{}", http_exchange.response.body);

    Ok(())
}
