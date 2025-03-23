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
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(ArgAction::SetTrue)
                .help("help goes here"),
        )
        .get_matches();

    let url_input = matches.get_one::<String>("url_input").unwrap();
    let url = parse_url(url_input);

    let verbose = matches.get_flag("verbose");

    let request = Request::new("GET", url);

    let mut http_exchange = HttpExchange::new(request);

    http_exchange.send()?;

    if verbose {
        for line in http_exchange.request.to_string().lines() {
            println!("> {}", line);
        }
    }
    if verbose {
        for line in http_exchange.response.headers.lines() {
            println!("< {}", line)
        }
        println!("<")
    }
    println!("{}", http_exchange.response.body);

    Ok(())
}
