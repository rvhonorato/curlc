use crate::url::UrlComponents;
use core::fmt;

#[derive(Debug)]
pub struct Request {
    method: String,
    pub url: UrlComponents,
    data: String,
    content_type: String,
}

impl Request {
    pub fn new(method: &str, url: UrlComponents, data: &str, content_type: &str) -> Self {
        Request {
            url,
            method: method.to_string(),
            data: data.to_string(),
            content_type: content_type.to_string(),
        }
    }
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut request = format!(
            "{} /{} {}\r\nHost: {}\r\n",
            self.method, self.url.pathname, self.url.protocol, self.url.hostname
        );

        // Add headers
        let headers = vec![
            ("User-Agent".to_string(), "RustHttpClient/1.0".to_string()),
            ("Accept".to_string(), "*/*".to_string()),
            ("Connection".to_string(), "close".to_string()),
        ];
        for (k, v) in headers {
            request.push_str(&format!("{}: {}\r\n", k, v));
        }

        // Process data
        if !self.data.is_empty() {
            request.push_str(&format!("Content-Type: {}\r\n", self.content_type));
            request.push_str(&format!("Content-Length: {}\r\n", self.data.len()));
            request.push_str("\r\n");
            request.push_str(&self.data);
        } else {
            request.push_str("\r\n");
        }

        write!(f, "{}", request)
    }
}
