console
$ cat a.rs 
pub trait Deserialize {
    fn deserialize();
}
$ cat b.rs 
pub struct A {}

impl a::Deserialize for A {
    fn deserialize() {
        extern crate a as _a;
    }
}

fn main() {
    A::deserialize();
}
$ rustc --crate-type=lib --edition=2018 a.rs
$ rustc --crate-type=bin --edition=2018 b.rs -L. --extern a 
