rust
let _bind_str = format! ("0.0.0.0:{}", 2233);
match HttpServer::new (|| {
    App::new ()
}).bind (&_bind_str [..]) {
    Ok (mut _server) => { _server.run ().await; () },
    Err (_e) => println! ("http listen failed: {}", _e.to_string ()),
};
