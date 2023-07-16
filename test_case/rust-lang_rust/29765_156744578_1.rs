 Rust
//main.rs
mod foo;
mod bar;
fn main() {}

//foo.rs
mod inner { mod copy; }

//bar.rs
mod inner { mod copy; }
