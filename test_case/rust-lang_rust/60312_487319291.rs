rust
use futures::{Future, future::ok};

fn main() {
    ok(()).then(|_a: Result<(), ()>| -> Result<(), ()> {
        std::process::exit(1);
    }).wait().unwrap();
}
