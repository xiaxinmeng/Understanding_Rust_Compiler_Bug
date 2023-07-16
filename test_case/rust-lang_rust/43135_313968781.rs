rust
fn msg() -> String { format!("Hello, {}!", "world") }

pub fn foo2() -> impl Future<Item=String, Error=()> {
    future::ok(()).and_then(|()| future::ok(msg()) )
}
