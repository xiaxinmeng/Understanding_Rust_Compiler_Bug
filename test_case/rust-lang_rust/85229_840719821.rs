rust
#![feature(const_generics)]
#![allow(incomplete_features)]

fn foo<const X: &'static [usize]>() {
    
}

fn main() {
    foo::<{&[1, 2, 3, 4]}>();
}
