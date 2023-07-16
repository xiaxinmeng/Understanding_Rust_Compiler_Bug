rust
#![feature(unsize)]

trait Trait {}
struct A; impl Trait for A {}

fn f<T: ?Sized, U: ?Sized>() where T: std::marker::Unsize<U> {}

fn main() {
    // for traits:
    f::<A, Trait>(); // works normally
    f::<Trait, Trait>(); // works; Unsize is reflexive
    
    // for slices:
    f::<[usize; 4], [usize]>(); // works normally
    f::<[usize], [usize]>(); // error; Unsize isn't reflexive ?
}
