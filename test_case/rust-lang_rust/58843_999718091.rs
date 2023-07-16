rust
use std::sync::Mutex;

struct Foo {
    bar: String,
    baz: String,
}

fn main() {
    // breaks
    let mutex = Mutex::new(Foo { bar: "hi".into(), baz: "hello".into() });
    let mut x = mutex.lock().unwrap();
    
    // works fine
    // let mut x = Foo { bar: "hi".into(), baz: "hello".into() };
    let bar = &mut x.bar;
    let baz = &mut x.baz;
    *bar = "new".into();
}
