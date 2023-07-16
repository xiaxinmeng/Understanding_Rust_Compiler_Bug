
error[E0308]: mismatched types
  --> src/main.rs:14:27
   |
12 | /     while let Some(stream) = listener.next().await {
13 | |         match stream {
14 | |             Ok(stream) => handle_client(stream),
   | |                           ^^^^^^^^^^^^^^^^^^^^^ expected `()`, found opaque type
15 | |             Err(err) => return Err(err.into()),
16 | |         }
17 | |     }
   | |_____- expected this to be `()`
...
22 |   async fn handle_client(stream: TcpStream) -> Result<(), Box<dyn Error>> {
   |                                                -------------------------- the `Output` of this `async fn`'s found opaque type
   |
   = note: expected unit type `()`
            found opaque type `impl std::future::Future`
help: try adding a semicolon
   |
14 |             Ok(stream) => handle_client(stream);,
   |                                                ^
help: consider using a semicolon here
   |
17 |     };
   |      ^
