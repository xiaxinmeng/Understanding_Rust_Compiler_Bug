
extern crate fcgi;
extern crate libc;
extern crate unix_socket;

use fcgi::{Request, DefaultRequest};
use unix_socket::{UnixListener};
use std::os::unix::io::{AsRawFd};

fn main() { 
    let listener = UnixListener::bind("/run/webapp/webapp.sock").unwrap();  
    println!("isCgi: {}", fcgi::is_cgi());
    let result = fcgi::initialize_fcgi();
    let mut request: DefaultRequest = Request::new_with_fd(listener.as_raw_fd()).unwrap(); 
    while request.accept() {
        println!("request uri    {:?}", request.get_param("REQUEST_URI"));
        println!("document root  {:?}", request.get_param("DOCUMENT_ROOT"));
        println!("script name    {:?}", request.get_param("SCRIPT_NAME"));
        println!("request method {:?}", request.get_param("REQUEST_METHOD"));
        let received = request.readall(); 
        println!("Received (size={})", received.len());
        if received.len() > 0 { 
            println!("8<------------------");
            println!("{}", received);
            println!("8<------------------");
        } 
        let body = "Content-type: text/html\r\n\r\n<header><title>Hello World</title></header>\r\n<body> <h1>Hello World!</h1>  </body>";
        let byte_count = request.write(body);
        request.error("Test error!");
        println!("number of bytes written {}", byte_count);
        request.flush(fcgi::StreamType::OutStream);
        request.finish();
    }
}
