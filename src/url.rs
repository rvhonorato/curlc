pub struct UrlComponents {
    pub protocol: String,
    pub hostname: String,
    pub pathname: String,
    pub socket_addr: String,
}

/// This function recieves a "normal" url and retrieves the relevant fields for the UrlComponents
pub fn parse_url(url: &str) -> UrlComponents {
    // protocol is http or https
    let (_protocol, addr) = url.split_once("://").unwrap();

    // extract the hostname and the path from the addr
    let (hostname_part, path_part) = addr.split_once('/').unwrap_or((addr, ""));

    // extract the port, if any
    let (hostname, port) = hostname_part
        .split_once(':')
        .unwrap_or((hostname_part, "80"));

    // process the path, if any
    let pathname = if path_part.is_empty() {
        String::new()
    } else {
        format!("/{}", path_part)
    };

    UrlComponents {
        protocol: "HTTP/1.1".to_string(),
        hostname: hostname.to_string(),
        pathname,
        socket_addr: format!("{}:{}", hostname, port),
    }
}
