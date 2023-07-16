 rust
use std::cast;
enum Foo { }
struct Bar;

#[cfg(ex6)]
fn main() {
    let b : Bar = Bar;
    let i : Foo = unsafe { cast::transmute(b) };
    println!("b: {:?}", b);
    match i {
    }
}

#[cfg(ex7)]
fn main() {
    let b : Bar = Bar;
    let i : Foo = unsafe { cast::transmute(b) };
    println!("b: {:?}", b);
    match i {
        _ => { println!("The impossible!"); }
    }
}
