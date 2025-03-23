pub struct Response {
    pub body: String,
    pub headers: String,
}

impl Response {
    pub fn new() -> Self {
        Response {
            body: String::new(),
            headers: String::new(),
        }
    }
}
