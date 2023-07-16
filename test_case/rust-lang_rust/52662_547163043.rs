rust
#![feature(associated_type_bounds)]

trait A {
    type T;
}

trait B: A<T: B> {}
