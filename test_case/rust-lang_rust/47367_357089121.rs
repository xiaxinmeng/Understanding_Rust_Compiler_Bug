rust
#![crate_type = "lib"]
#![feature(conservative_impl_trait, universal_impl_trait)]
#![feature(never_type)]

trait T {}

fn f(x: !) -> impl T {
    x      // ^^^^^^ the trait `T` is not implemented for `!`
}
