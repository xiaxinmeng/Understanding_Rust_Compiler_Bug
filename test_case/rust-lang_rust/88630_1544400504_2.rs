rust
// some library crate `foo`
// lib.rs

impl PartialEq<Foo> for u8 {
    fn eq(&self, other: &Foo) -> bool {
        todo!()
    }
}

pub struct Foo;

// our binary crate with a `Cargo.toml` dep on `foo`
// main.rs

fn evil() {
    does_not_exist(); // comment and uncomment this to trigger the inference error
}

fn innocent_bystander() {
    "x".as_bytes() != &[];
}

fn main() {}


