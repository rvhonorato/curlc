pub struct UrlComponents {
    pub protocol: String,
    pub hostname: String,
    pub pathname: String,
    pub socket_addr: String,
}

/// This function recieves a "normal" url and retrieves the relevant fields for the UrlComponents
pub fn parse_url(url: &str) -> UrlComponents {
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
