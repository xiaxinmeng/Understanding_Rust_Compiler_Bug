 rust
// main.rs
$ cat main.rs
#[macro_use] extern crate mywrite;

fn main() {
    let mut v = vec![];
    mywrite!(&v, "Hello world");
}
