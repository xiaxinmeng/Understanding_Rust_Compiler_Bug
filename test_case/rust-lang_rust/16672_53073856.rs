 rust
#![feature(unboxed_closures)]

fn main() {
    let f = |&: x:uint| {x};
    let bar = foo(f);
    println!("{}", bar.call((1,)));
}

fn foo<F: Fn<(uint,), uint>>(f: F) -> Box<Fn<(uint,),uint>> {
   ( box |&: x: uint| f.call((x,))) as Box<Fn<(uint,), uint>>
}
