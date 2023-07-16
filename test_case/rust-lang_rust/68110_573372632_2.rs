
error[E0308]: mismatched types
 --> file18.rs:9:35
  |
9 | fn fuz() -> (usize, Trait) { (42, Struct) } //~ ERROR E0277
  |                     -----         ^^^^^^ expected trait `Trait`, found struct `Struct`
  |                     |
  |                     help: `Struct` implements `dyn Trait`, consider using: `impl Trait`
  |
  = note: expected trait object `(dyn Trait + 'static)`
                   found struct `Struct`

error[E0308]: mismatched types
  --> file18.rs:10:39
   |
10 | fn bar() -> (usize, dyn Trait) { (42, Struct) } //~ ERROR E0277
   |                     ---------         ^^^^^^ expected trait `Trait`, found struct `Struct`
   |                     |
   |                     help: `Struct` implements `dyn Trait`, consider using: `impl Trait`
   |
   = note: expected trait object `(dyn Trait + 'static)`
                    found struct `Struct`

error[E0XX1]: the size for values of type `(dyn Trait + 'static)` cannot be known at compilation time
  --> file18.rs:17:13
   |
17 | fn bak() -> dyn Trait { unimplemented!() } //~ ERROR E0277
   |             ^^^^^^^^^   ---------------- if this implemented `Trait`, you could use `impl Trait` as the return type
   |             |
   |             doesn't have a size known at compile-time
   |             help: because there's a single return in this function, you could use `impl Trait`: `impl Trait`
   |
   = help: the trait `std::marker::Sized` is not implemented for `(dyn Trait + 'static)`
   = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
   = note: the return type of a function must have a statically known size
   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)

error[E0XX2]: the size for values of type `(dyn Trait + 'static)` cannot be known at compilation time
  --> file18.rs:18:13
   |
18 | fn bal() -> dyn Trait { //~ ERROR E0277
   |             ^^^^^^^^^ doesn't have a size known at compile-time
   |
   = help: the trait `std::marker::Sized` is not implemented for `(dyn Trait + 'static)`
   = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
   = note: the return type of a function must have a statically known size
note: because this function has multiple returns, it wouldn't be possible to use `impl Trait`
   |
19 |     if true {
20 |         return Struct;
   |                ^^^^^^ `bal` returns here
21 |     }
22 |     Other
   |     ^^^^^ `bal` returns here
   |
   = note: for more information on `impl Trait` and trait objects, visit <FOO>
help: consider using trait objects by using `Box`
   |
18 | fn bal() -> Box<dyn Trait> { //~ ERROR E0277
19 |     if true {
20 |         return Box::new(Struct);
21 |     }
22 |     Box::new(Other)
   |
