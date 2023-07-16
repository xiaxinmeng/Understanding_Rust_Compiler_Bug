 rust
if req.method().as_ref() == "GET" {
    Next::write()
} else {
    panic!("only can handle GET")
}
