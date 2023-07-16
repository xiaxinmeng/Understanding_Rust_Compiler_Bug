
extern mod std;
use std::json;
use std::serialization::{Deserializable, deserialize};

fn main() {
    do io::with_str_reader("[1]") |rdr| {
        let deser = result::unwrap(json::Deserializer(rdr));
        let x: ~[int] = deserialize(&deser);
        io::println(fmt!("%?", x));
    }
}
