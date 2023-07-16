rust
#![recursion_limit = "3000"]
pub use vek::vec::repr_c::Vec2;

fn is_average<T: num_integer::Average>() {}

fn foo<T>() {
    is_average::<Vec2<T>>();
}

fn main() {}
