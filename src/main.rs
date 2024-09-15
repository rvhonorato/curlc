use std::net::TcpStream;

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
    fn new(url: UrlComponents) -> Request {
        Request {
            url,
            method: "".to_string(),
            headers: Vec::new(),
            body: "".to_string(),
        }
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

fn open_stream(socket_addr: &str) -> TcpStream {
    match TcpStream::connect(socket_addr) {
        Ok(stream) => {
            println!("connected!");
            stream
        }
        Err(e) => panic!("{:?}", e),
    }
}

fn main() {
    let url = parse_url("https://eu.httpbin.org/get");

    let _stream = open_stream(&url.socket_addr);

    let request = Request::new(url);
}
