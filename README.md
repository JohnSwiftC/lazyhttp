# lazyhttp

An easy library to handle stream objects (TcpStream, TlsStream, etc) transferring HTTP data. Includes a function to parse and return an object representing the request.

> This library does not handle networking or responding. This is really intended to remove a snippet of code that I find myself copying and pasting over and over again.

# Example

{% highlight rust %}
let listener = TcpListener::bind("addr");

for stream in listener.incoming() {
    if let Ok(req) = lazyhttp::handle_stream(stream) {
        // Do something with req
    }
}
{% endhighlight %}
