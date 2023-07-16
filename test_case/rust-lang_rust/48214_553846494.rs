rust
#![allow(dead_code, unused_variables,)]
#![feature(trait_alias, trivial_bounds)]

trait T = Fn(&i32, &i32) -> bool;
fn impl_trait_fn() -> impl T {
	|a: &i32, b: &i32| true
}
