use crate::url::UrlComponents;
use core::fmt;

pub struct Request {
    method: String,
    pub url: UrlComponents,
}

impl Request {
    pub fn new(method: &str, url: UrlComponents) -> Self {
        Request {
            url,
            method: method.to_string(),
        }
    }
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut request = format!(
            "{} /{} {}\r\nHost: {}\r\n",
            self.method, self.url.pathname, self.url.protocol, self.url.hostname
        );

        let headers = vec![
            ("User-Agent".to_string(), "RustHttpClient/1.0".to_string()),
            ("Accept".to_string(), "*/*".to_string()),
            ("Connection".to_string(), "close".to_string()),
        ];

        // Add headers
        for (k, v) in headers {
            request.push_str(&format!("{}: {}\r\n", k, v));
        }

        // Signal header termination
        request.push_str("\r\n");

        write!(f, "{}", request)
    }
}
