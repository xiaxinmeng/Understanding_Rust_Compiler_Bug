 rust
struct Foo;
struct Bar { x: int }
struct Baz(int);

impl Drop for Foo {
    fn drop(&self) {
        println("finalize Foo");
    }
}
impl Drop for Bar {
    fn drop(&self) {
        println("finalize Bar");
    }
}
impl Drop for Baz {
    fn drop(&self) {
        println("finalize Baz");
    }
}

fn main() {
    { let _x = Foo; }
    { let _x = Bar { x: 21 }; }
    { let _x = Baz(21); }

    println("------------");

    { let _ = Foo; }
    { let _ = Bar { x: 21 }; }
    { let _ = Baz(21); }
}
