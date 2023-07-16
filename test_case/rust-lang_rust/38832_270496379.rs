rust
#![allow(dead_code)]

#[derive(Debug)]
#[allow(deprecated)]
enum Example {
    Var1,
    #[deprecated]
    Var2,
}

fn main() {
}
