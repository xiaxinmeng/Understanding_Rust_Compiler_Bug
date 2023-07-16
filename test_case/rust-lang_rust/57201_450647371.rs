rust
#![feature(impl_trait_in_bindings)]

fn main() {
    let a = vec![1, 2, 3];
    
    let b: Box<impl Iterator> = Box::new(a.iter());
}
