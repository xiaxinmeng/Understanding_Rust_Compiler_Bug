rust
//error[E0562]: `impl Trait` not allowed outside of function and inherent method return types
//fn f1() -> impl FnOnce() -> impl Clone
//OK with #![feature(unboxed_closures)]
fn f2() -> impl FnOnce<(), Output=impl Clone> {...}
