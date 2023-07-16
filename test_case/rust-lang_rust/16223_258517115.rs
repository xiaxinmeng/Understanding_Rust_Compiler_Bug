 rust
struct A {
    some_string: String,
    some_num: u32,
}

impl A {
    fn some_method(self: Box<Self>) {
        some_function(self.some_string, self.some_num)
    }
}

fn some_function(_: String, _: u32) {}

fn main() {}
