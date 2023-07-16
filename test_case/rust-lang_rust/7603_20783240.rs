 rust
#[allow(default_methods)];

pub trait TraitWithDefaultMethod {
    pub fn method(self) {
        ()
    }
}

struct MyStruct;

impl TraitWithDefaultMethod for MyStruct { }

fn main() {
    MyStruct.method();
}
