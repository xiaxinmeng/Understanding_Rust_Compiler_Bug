rust
#![feature(associated_type_bounds)]

trait A: B<Assoc: Fn(u32)> {}
impl<T> A for T where T: B<Assoc: Fn(u32)> {}

trait B {
    type Assoc;
}
impl<T> B for T {
    type Assoc = fn(u32);
}

fn needs_a(t: impl A + Fn(i32)) {}

fn main() {
    needs_a(|x| {});
}
