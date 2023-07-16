 rust
#![feature(unboxed_closures)]

fn main() {
    let f = |&: | { };
    let bar = foo(f);
    println!("{}" , bar.call (()) );
}

fn foo<F: Fn<(), ()> + 'static>(f: F) -> Box<Fn<(), ()> + 'static> {
    ( box |&: | f.call(())) as Box<Fn<(), ()>>
}
