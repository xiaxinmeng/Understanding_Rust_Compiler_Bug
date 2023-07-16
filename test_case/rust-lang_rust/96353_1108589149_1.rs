
error[E0746]: return type cannot have an unboxed trait object
  --> $DIR/point-to-type-err-cause-on-impl-trait-return.rs:77:13
   |
LL | fn pug() -> dyn std::fmt::Display {
   |             ^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
   |
   = note: for information on trait objects, see <https://doc.rust-lang.org/book/ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types>
   = note: if all the returned values were of the same type you could use `impl std::fmt::Display` as the return type
   = note: for information on `impl Trait`, see <https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits>
   = note: you can create a new `enum` with a variant for each returned type
help: return a boxed trait object instead
   |
LL | fn pug() -> Box<dyn std::fmt::Display> {
   |             ++++                     +
help: ... and box this value
   |
LL |         0 => Box::new(0i32),
   |              +++++++++    +
help: ... and box this value
   |
LL |         1 => Box::new(1u32),
   |              +++++++++    +
help: ... and box this value
   |
LL |         _ => Box::new(2u32),
   |              +++++++++    +
