
use std::string::String;

#[derive(Clone)]
struct Foo {
 a: String,
 b: String,
 // Possibly some more fields here
}

#[derive(Clone)]
struct FooThing(Vec<Foo>);

impl FooThing {
    fn filter(&mut self, bar: &Foo) {
        // Oops. Shouldn't be a `ref` there
        let FooThing(ref buf) = self.clone();

        let new_buf: Vec<Foo> = buf.into_iter()
                                   .filter(|f| f.a == bar.a && f.b == bar.b)
                                   .collect();
    }
}

fn main() {}
