use crate::url::UrlComponents;

use std::{
    io::{Read, Write},
    net::TcpStream,
    time::Duration,
};

pub struct Request {
    pub url: UrlComponents,
    pub method: String,
    pub headers: Vec<(String, String)>,
    pub body: String,
}

impl Request {
    pub fn send(&self) -> Result<String, std::io::Error> {
        let req = self.create();

        if let Ok(mut stream) = TcpStream::connect(&self.url.socket_addr) {
            // println!("Connected to {:?}", &self.url.socket_addr);
            stream.set_read_timeout(Some(Duration::from_secs(2)))?;
            stream.write_all(req.as_bytes())?;
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

            println!("Response: {}", response_string);

            Ok(response_string)
        } else {
            println!("Could not connect!");
            Err(std::io::Error::new(
                std::io::ErrorKind::ConnectionRefused,
                "Could not connect to the server",
            ))
        }
    }

    fn create(&self) -> String {
        let mut request = format!(
            "{} /{} {}\r\nHost: {}\r\n",
            self.method, self.url.pathname, self.url.protocol, self.url.hostname
        );

        // Add headers
        for (k, v) in &self.headers {
            request.push_str(&format!("{}: {}\r\n", k, v));
        }

        // Signal header termination
        request.push_str("\r\n");

        // Add body if any
        if !self.body.is_empty() {
            request.push_str(&self.body);
        }

        println!("Request: {}", request);
        request
    }
}
