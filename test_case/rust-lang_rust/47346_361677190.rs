rust
#[macro_use]
extern crate repro;

#[macro_use]
extern crate serde_derive;

#[derive(Serialize, MyCustomDerive)]
struct MyStruct;

my_macro!();

fn main() {}
