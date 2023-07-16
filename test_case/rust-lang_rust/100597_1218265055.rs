
[dependencies]
 // capability is denied project-wide unless specified in the dependencies
hyper = { version = "0.14", features = ["full"],  capabilities = ["std::net"]}

...

use hyper::Client capabilities { std::net }; // capability is permitted for this file

or 

fn main() -> {
   use hyper::Client capabilities { std::net }; // capability is permitted for this scope
   let client = hyper::Client::new();
   // etc..
}

or

let client = hyper::Client::new() capabilities { std::net }; // capability exists only for this statement
