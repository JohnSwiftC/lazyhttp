use std::error::Error;
use std::io::{BufRead, BufReader, Read};
use http::Request;
use http::Version;

pub fn handle_stream<T>(stream: &T) -> Result<Request<Option<String>>, Box<dyn Error + Send + Sync + 'static>>
where
    for<'a> &'a T: std::io::Read,
{

    let mut request = Request::builder();

    let mut buf_reader = BufReader::new(stream);
    let mut line_buf = String::new();
    let mut content_length: Option<usize> = None;

    buf_reader.read_line(&mut line_buf)?;

    let request_parts: Vec<&str> = line_buf.split_whitespace().collect();

    // We only want POST requests being made

    match request_parts.get(0) {
        Some(method) => {
            request = request.method(*method);
        }, 
        None => return Err("No Request Method Specified".into()),
    };

    match request_parts.get(1) {
        Some(uri) => {
            request = request.uri(*uri);
        },
        None => return Err("No Request URI Specified".into()),
    }

    match request_parts.get(2) {
        Some(v_str) => {
            request = request.version(parse_version(v_str));
        },
        None => return Err("No HTTP Version Specified".into()),
    }

    loop {
        let mut line_buf = String::new();

        buf_reader.read_line(&mut line_buf)?;

        if line_buf.is_empty() || line_buf == "\n" || line_buf == "\r\n" {
            break;
        }

        let mut comps = line_buf.split(":");
        let key = comps.next().unwrap_or("None");
        let value = comps.next().unwrap_or("None").trim();

        if key == "Content-Length" {
            content_length = Some(value.parse()?);
        }

        request = request.header(key, value);
    }


    if let Some(length) = content_length {
        let mut bytes = vec![0_u8; length];

        buf_reader
            .read_exact(&mut bytes)?;

        Ok(request.body(Some(String::from_utf8(bytes)?))?)
        
    } else {
        Ok(request.body(None)?)
    }

    
    
}

fn parse_version(v: &str) -> Version {
    match v {
        "HTTP/0.9" => Version::HTTP_09,
        "HTTP/1.0" => Version::HTTP_10,
        "HTTP/1.1" => Version::HTTP_11,
        "HTTP/2.0" => Version::HTTP_2,
        "HTTP/3.0" => Version::HTTP_3,
        &_ => Version::HTTP_11, // Hopefully this is ok to do
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpListener;
    // This test requires the tester to make an http request to the test machine
    #[test]
    fn recieve_requests() {
        let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

        let stream = listener.incoming().next().unwrap().unwrap();

        let request = handle_stream(&stream).unwrap();

        println!("{:?} {:?} {:?}", request.method(), request.uri(), request.version());

        for (key, value) in request.headers() {
            println!("{:?}: {:?}", key, value);
        }

        assert_eq!(request.into_body().unwrap(), "Hello Test".to_string());
    }
}
