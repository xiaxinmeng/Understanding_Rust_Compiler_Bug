
error[E0631]: type mismatch in closure arguments
  --> src/net/mod.rs:77:9
   |
77 | /         Box::new(
78 | |             writer
79 | |                 .send_all(reader.and_then(move |req| future::ok(req)))
80 | |                 .then(move |_| {
...  |
84 | |                 .then(|_: ()| Ok(())),
   | |                       -------------- found signature of `fn(()) -> _`
85 | |         )
   | |_________^ expected signature of `fn(std::result::Result<(), _>) -> _`
