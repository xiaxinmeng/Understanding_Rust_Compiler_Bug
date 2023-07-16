
error[E0277]: the size for values of type `(dyn Trait + 'static)` cannot be known at compilation time
  --> src/main.rs:12:21
   |
12 |     fn index(&self, index: Trait) -> &i32 {
   |                     ^^^^^ doesn't have a size known at compile-time
   |
   = help: the trait `std::marker::Sized` is not implemented for `(dyn Trait + 'static)`
   = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
   = note: all local variables must have a statically known size
   = help: unsized locals are gated as an unstable feature
