rust
use std::sync::Arc;

fn f() {
    println!("f called");
}

fn main() {
    let f = Arc::new(f);
    f();
}
