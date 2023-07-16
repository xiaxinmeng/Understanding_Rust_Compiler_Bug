rust
#![deny(unconditional_recursion)]

struct Object;

trait ToObject {
    fn to_object(&self) -> &Object;
}

impl ToObject for Object {
    fn to_object(&self) -> &Object {
        self
    }
}

impl<T: ToObject> ObjectOps for T {}

trait ObjectOps : ToObject {
    fn foo(&self) -> f32 {
        self.to_object().foo()
    }
}

fn main() {
    let x = Object;
    println!("{}", x.foo());
}
