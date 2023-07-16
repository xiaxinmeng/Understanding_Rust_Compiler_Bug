
error[E0277]: the size for values of type `(dyn Trait + 'static)` cannot be known at compilation time
  --> file18.rs:16:13
   |
16 | fn ban() -> dyn Trait { Struct } //~ ERROR E0277
   |             ^^^^^^^^^   ------ return place
   |             |
   |             doesn't have a size known at compile-time
   |             help: use `impl Trait`: `impl Trait`
   |
   = help: the trait `std::marker::Sized` is not implemented for `(dyn Trait + 'static)`
   = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
   = note: the return type of a function must have a statically known size
