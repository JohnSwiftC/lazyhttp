use std::collections::HashMap;
use std::error::Error;
use std::io::{BufRead, BufReader, Read};

pub fn handle_stream<T>(stream: &T) -> Result<Request, Box<dyn Error + Send + Sync + 'static>>
where
    for<'a> &'a T: std::io::Read,
{
    let mut buf_reader = BufReader::new(stream);

    let mut line_buf = String::new();

    if let Err(_) = buf_reader.read_line(&mut line_buf) {
        panic!("Bad Request");
    }

    let mut request_parts: Vec<&str> = line_buf.split_whitespace().collect();

    // We only want POST requests being made

    let method = match request_parts.get(0) {
        Some(method) => method.to_string(),
        None => panic!("No request method"),
    };

    let mut headers = HashMap::new();

    loop {
        let mut line_buf = String::new();

        if let Err(_) = buf_reader.read_line(&mut line_buf) {
            panic!("Bad Request");
        }

        if line_buf.is_empty() || line_buf == "\n" || line_buf == "\r\n" {
            break;
        }

        let mut comps = line_buf.split(":");
        let key = comps.next().unwrap_or("None");
        let value = comps.next().unwrap_or("None").trim();

        headers.insert(key.to_string(), value.to_string());
    }

    let body;

    if let Some(length) = headers.get("Content-Length") {
        let mut bytes = vec![0_u8; length.parse().expect("Bad Content Length Header")];

        buf_reader
            .read_exact(&mut bytes)
            .expect("Failed to read content!");

        body = Some(String::from_utf8(bytes).expect("Invalid String!"));
    } else {
        body = None;
    }

    Ok(Request {
        method: method,
        route: request_parts.swap_remove(1).to_string(),
        version: request_parts.pop().unwrap().to_string(),
        headers: headers,
        body: body,
    })
}

pub struct Request {
    pub method: String,
    pub route: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl Request {
    /// Outputs a new owned string of the full request.
    pub fn as_text(&self) -> String{
        match &self.body {
            Some(body) => {
                let mut r = String::new();
                r.push_str(&format!("{} {} {}\r\n", self.method, self.route, self.version));
                for (k , v) in &self.headers {
                    r.push_str(&format!("{}: {}\r\n", k, v));
                }

                r.push_str("\r\n");
                r.push_str(&body);

                r
            }
            None => {
                let mut r = String::new();
                r.push_str(&format!("{} {} {}\r\n", self.method, self.route, self.version));
                for (k , v) in &self.headers {
                    r.push_str(&format!("{}: {}\r\n", k, v));
                }

                r.push_str("\r\n");

                r
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_text_body() {
        let mut headers = HashMap::new();
        headers.insert("Test".into(), "Value".into());

        let req = Request {
            method: "POST".to_string(),
            route: "/".to_string(),
            version: "HTTP/1".to_string(),
            headers: headers,
            body: Some("My body".into()),
        };

        assert_eq!(req.as_text(), "POST / HTTP/1\r\nTest: Value\r\n\r\nMy body");
    }

    #[test]
    fn test_as_text() {
        let mut headers = HashMap::new();
        headers.insert("Test".into(), "Value".into());

        let req = Request {
            method: "POST".to_string(),
            route: "/".to_string(),
            version: "HTTP/1".to_string(),
            headers: headers,
            body: None,
        };

        assert_eq!(req.as_text(), "POST / HTTP/1\r\nTest: Value\r\n\r\n");

    }
}