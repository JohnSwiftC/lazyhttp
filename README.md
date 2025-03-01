# lazyhttp

An easy library to handle stream objects (TcpStream, TlsStream, etc) transferring HTTP data, providing functions which return a Request from the [http](https://crates.io/crates/http) crate.

This library does not handle networking or responding. This is really intended to remove a snippet of code that I find myself copying and pasting over and over again.

**Important:** This library will also only accept bodies if they are sent with a `Content-Length`
header.

# Example

```rust
let listener = TcpListener::bind("addr");

for stream in listener.incoming() {
    let stream = stream.unwrap();

    if let Ok(req) = lazyhttp::handle_stream(&stream) {
        // Do something with req
    }
}
```

Note, this is not a good example for production code. Remember proper error handling and some form of async or multithreading for a server environment.
