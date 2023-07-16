 rust
// This version gives a warning:
type Foo<T: Copy> = T; // warning: trait bounds are not (yet) enforced in type definitions

// But this equivalent version doesn’t:
type Bar<T> where T: Copy = T; // no warning

fn main() {
    let _: Bar<Box<i32>> = Box::new(1); // compiles
}
