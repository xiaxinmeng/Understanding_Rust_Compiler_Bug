rust
#![feature(auto_traits, negative_impls)]

unsafe auto trait Unmanaged {}

unsafe trait Trace {}

struct GcPtr(*const ());

unsafe impl Trace for GcPtr {}

// It seems like rustc ignores the `T: Trace` bound.
impl<T: Trace> !Unmanaged for T {}

fn check<T: Unmanaged>(_: T) {}

fn main() {
    let a = (0, 0);
    // error: the trait bound `({integer}, {integer}): Unmanaged` is not satisfied
    check(a);
}
