 rust
extern crate serialize;
use serialize::json::ToJson;

fn f<Sized? T: ToJson>(x: &T) {
    println!("{}", x.to_json().to_pretty_str());
}

fn main() {
    f(&123i32); // fine
    f(&vec![123i32, 4, 5, 6]); // fine
    f("foo") //
}
