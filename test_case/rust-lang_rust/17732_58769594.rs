
#![feature(associated_types)]

trait Deserialize {
    type Visitor;
}

trait Deserializer {
    fn deserialize<D: Deserialize>();
}

fn main() { }
