 rust
#![feature(unboxed_closures)]

pub fn deeper<F: Fn()>(c: F) {
    c.call(());
}

pub fn deep<A,C:Fn(uint)>(_c: C) {
    deeper(|&:| {});
}

fn main() { 
    deep::<char,_>(|&: _:uint| {});
}
