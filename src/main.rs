use std::{
    io::{Read, Write},
    net::TcpStream,
};

struct UrlComponents {
    protocol: String,
    hostname: String,
    pathname: String,
    socket_addr: String,
}

struct Request {
    url: UrlComponents,
    method: String,
    headers: Vec<(String, String)>,
    body: String,
}

impl Request {
    fn send(&self) -> Result<String, std::io::Error> {
        let req = self.create();

        println!("Request: {}", req);

        match TcpStream::connect(&self.url.socket_addr) {
            Ok(mut stream) => {
                stream.write_all(req.as_bytes())?;
                stream.flush()?;

                let mut response = String::new();
                let _ = stream.read_to_string(&mut response);

                Ok(response)
            }
            Err(e) => panic!("{:?}", e),
        }
    }

    fn create(&self) -> String {
        let mut request = format!(
            "{} /{} {}\r\nHost: {}\r\n",
            self.method, self.url.pathname, self.url.protocol, self.url.hostname
        );

        for (k, v) in &self.headers {
            request.push_str(&format!("{}: {}\r\n", k, v));
        }

        if !self.body.is_empty() {
            request.push_str(&self.body);
        }

        request
    }
}

/// This function recieves a "normal" url and retrieves the relevant fields for the UrlComponents
fn parse_url(url: &str) -> UrlComponents {
    let (_protocol, rest) = url.split_once("://").unwrap();
    let (tmp_hostname, pathname) = rest.split_once('/').unwrap();
    let (hostname, port) = if tmp_hostname.contains(':') {
        tmp_hostname.split_once(':').expect("Invalid hostname")
    } else {
        (tmp_hostname, "80")
    };
    UrlComponents {
        protocol: "HTTP/1.1".to_string(),
        hostname: hostname.to_string(),
        pathname: pathname.to_string(),
        socket_addr: format!("{}:{}", hostname, port),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = parse_url("https://eu.httpbin.org/get");

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

    match request.send() {
        Ok(response) => println!("Response:\n{}", response),
        Err(e) => eprintln!("Error: {}", e),
    }

    Ok(())
}
