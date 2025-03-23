use crate::request::Request;
use crate::response::Response;

use std::{
    io::{Read, Write},
    net::TcpStream,
    time::Duration,
};

pub struct HttpExchange {
    socket_addr: String,
    pub request: Request,
    pub response: Response,
}

impl HttpExchange {
    pub fn new(request: Request) -> Self {
        HttpExchange {
            socket_addr: request.url.socket_addr.to_string(),
            request,
            response: Response::new(),
        }
    }

    pub fn send(&mut self) -> Result<(), std::io::Error> {
        if let Ok(mut stream) = TcpStream::connect(&self.socket_addr) {
            stream.set_read_timeout(Some(Duration::from_secs(2)))?;
            stream.write_all(self.request.to_string().as_bytes())?;
            stream.flush()?;

            let mut response = Vec::new();
            let mut buffer = [0; 1024];

            loop {
                let bytes_read = stream.read(&mut buffer)?;
                if bytes_read == 0 {
                    break;
                }
                // println!("Read {} butes", bytes_read);
                response.extend_from_slice(&buffer[..bytes_read]);
            }

            let response_string = String::from_utf8_lossy(&response).to_string();
            self.response = parse_response(&response_string);
            Ok(())
        } else {
            println!("Could not connect!");
            Err(std::io::Error::new(
                std::io::ErrorKind::ConnectionRefused,
                "Could not connect to the server",
            ))
        }
    }
}

fn parse_response(response: &str) -> Response {
    let mut headers = Vec::new();

    // Split the response into lines
    let mut lines = response.lines();

    // Skip the status line (e.g., "HTTP/1.1 200 OK")
    for line in lines.by_ref() {
        if line.is_empty() {
            // An empty line indicates the end of headers
            break;
        }
        if line.split_once(':').is_some() {
            // headers.push_str(line);
            headers.push(line.to_string());
        }
    }

    // The rest of the lines are the body
    let body = lines.collect::<Vec<_>>().join("\n");
    let headers = headers.join("\n");

    Response { headers, body }
}
