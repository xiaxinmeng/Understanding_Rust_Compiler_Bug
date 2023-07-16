
struct Foo;

fn main() {
    let f = Foo;
    let _g = f; // finalize called here
}

impl Drop for Foo {
    fn finalize(self) {
        let _g = self; // finalize not called here?
    }
}
