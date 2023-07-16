rust
trait Tr { fn f(&self) {} }
fn f<T: Tr>(t: T) {
    t.f();
    //^ the `f` in the `impl Tr for T` (note that this could be in a downstream crate)
    // has *no effect* on the resolution of this `f`.
} 