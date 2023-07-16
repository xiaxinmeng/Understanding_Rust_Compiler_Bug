
error[E0308]: `if` and `else` have incompatible types
 --> fil3.rs:5:9
  |
2 | /     if false {
3 | |         0i32
  | |         ---- expected because of this
4 | |     } else {
5 | |         1u32 //~ ERROR `if` and `else` have incompatible types
  | |         ^^^^ expected `i32`, found `u32`
6 | |     }
  | |_____- `if` and `else` have incompatible types

error[E0746]: return type cannot have an unboxed trait object
 --> fil3.rs:1:13
  |
1 | fn qux() -> dyn std::fmt::Display {
  |             ^^^^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
  |
  = note: for information on trait objects, see <https://doc.rust-lang.org/book/ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types>
  = note: if all the returned values were of the same type you could use `impl std::fmt::Display` as the return type
  = note: for information on `impl Trait`, see <https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits>
  = note: you can create a new `enum` with a variant for each returned type
help: return a boxed trait object instead
  |
1 | fn qux() -> Box<dyn std::fmt::Display> {
2 |     if false {
3 |         Box::new(0i32)
4 |     } else {
5 |         Box::new(1u32) //~ ERROR `if` and `else` have incompatible types
  |

error: aborting due to 2 previous errors
