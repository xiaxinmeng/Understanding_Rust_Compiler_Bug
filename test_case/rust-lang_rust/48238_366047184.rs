rust
#![feature(nll)]

fn main() {
    let a = "Hello".to_string();
    move || {
        a.rsplit("")
    };
}
