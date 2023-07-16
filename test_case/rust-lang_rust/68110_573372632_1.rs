
error[E0308]: mismatched types
 --> file18.rs:9:35
  |
9 | fn fuz() -> (usize, Trait) { (42, Struct) } //~ ERROR E0277
  |                                   ^^^^^^ expected trait `Trait`, found struct `Struct`
  |
  = note: expected trait object `(dyn Trait + 'static)`
                   found struct `Struct`

error[E0277]: the size for values of type `(dyn Trait + 'static)` cannot be known at compilation time
 --> file18.rs:9:13
  |
9 | fn fuz() -> (usize, Trait) { (42, Struct) } //~ ERROR E0277
  |             ^^^^^^^^^^^^^^   ------------ return place
  |             |
  |             doesn't have a size known at compile-time
  |
  = help: within `(usize, (dyn Trait + 'static))`, the trait `std::marker::Sized` is not implemented for `(dyn Trait + 'static)`
  = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
  = note: required because it appears within the type `(usize, (dyn Trait + 'static))`
  = note: the return type of a function must have a statically known size

error[E0308]: mismatched types
  --> file18.rs:10:39
   |
10 | fn bar() -> (usize, dyn Trait) { (42, Struct) } //~ ERROR E0277
   |                                       ^^^^^^ expected trait `Trait`, found struct `Struct`
   |
   = note: expected trait object `(dyn Trait + 'static)`
                    found struct `Struct`

error[E0277]: the size for values of type `(dyn Trait + 'static)` cannot be known at compilation time
  --> file18.rs:10:13
   |
10 | fn bar() -> (usize, dyn Trait) { (42, Struct) } //~ ERROR E0277
   |             ^^^^^^^^^^^^^^^^^^   ------------ return place
   |             |
   |             doesn't have a size known at compile-time
   |
   = help: within `(usize, (dyn Trait + 'static))`, the trait `std::marker::Sized` is not implemented for `(dyn Trait + 'static)`
   = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
   = note: required because it appears within the type `(usize, (dyn Trait + 'static))`
   = note: the return type of a function must have a statically known size

error[E0308]: mismatched types
  --> file18.rs:15:21
   |
15 | fn bap() -> Trait { Struct } //~ ERROR E0277
   |             -----   ^^^^^^ expected trait `Trait`, found struct `Struct`
   |             |
   |             expected `(dyn Trait + 'static)` because of return type
   |
   = note: expected trait object `(dyn Trait + 'static)`
                    found struct `Struct`

error[E0277]: the size for values of type `(dyn Trait + 'static)` cannot be known at compilation time
  --> file18.rs:15:13
   |
15 | fn bap() -> Trait { Struct } //~ ERROR E0277
   |             ^^^^^   ------ return place
   |             |
   |             doesn't have a size known at compile-time
   |             help: use `impl Trait`: `impl Trait`
   |
   = help: the trait `std::marker::Sized` is not implemented for `(dyn Trait + 'static)`
   = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
   = note: the return type of a function must have a statically known size

error[E0308]: mismatched types
  --> file18.rs:16:25
   |
16 | fn ban() -> dyn Trait { Struct } //~ ERROR E0277
   |             ---------   ^^^^^^ expected trait `Trait`, found struct `Struct`
   |             |
   |             expected `(dyn Trait + 'static)` because of return type
   |
   = note: expected trait object `(dyn Trait + 'static)`
                    found struct `Struct`

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

error[E0277]: the size for values of type `(dyn Trait + 'static)` cannot be known at compilation time
  --> file18.rs:17:13
   |
17 | fn bak() -> dyn Trait { unimplemented!() } //~ ERROR E0277
   |             ^^^^^^^^^   ---------------- return place
   |             |
   |             doesn't have a size known at compile-time
   |             help: use `impl Trait`: `impl Trait`
   |
   = help: the trait `std::marker::Sized` is not implemented for `(dyn Trait + 'static)`
   = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
   = note: the return type of a function must have a statically known size
   = note: this error originates in a macro outside of the current crate (in Nightly builds, run with -Z external-macro-backtrace for more info)

error[E0308]: mismatched types
  --> file18.rs:20:16
   |
18 | fn bal() -> dyn Trait { //~ ERROR E0277
   |             --------- expected `(dyn Trait + 'static)` because of return type
19 |     if true {
20 |         return Struct;
   |                ^^^^^^ expected trait `Trait`, found struct `Struct`
   |
   = note: expected trait object `(dyn Trait + 'static)`
                    found struct `Struct`

error[E0308]: mismatched types
  --> file18.rs:22:5
   |
18 | fn bal() -> dyn Trait { //~ ERROR E0277
   |             --------- expected `(dyn Trait + 'static)` because of return type
...
22 |     Other
   |     ^^^^^ expected trait `Trait`, found struct `Other`
   |
   = note: expected trait object `(dyn Trait + 'static)`
                    found struct `Other`

error[E0277]: the size for values of type `(dyn Trait + 'static)` cannot be known at compilation time
  --> file18.rs:18:13
   |
18 | fn bal() -> dyn Trait { //~ ERROR E0277
   |             ^^^^^^^^^ doesn't have a size known at compile-time
19 |     if true {
20 |         return Struct;
   |                ------ return place
21 |     }
22 |     Other
   |     ----- return place
   |
   = help: the trait `std::marker::Sized` is not implemented for `(dyn Trait + 'static)`
   = note: to learn more, visit <https://doc.rust-lang.org/book/ch19-04-advanced-types.html#dynamically-sized-types-and-the-sized-trait>
   = note: the return type of a function must have a statically known size
help: use boxed traits
   |
18 | fn bal() -> Box<dyn Trait> { //~ ERROR E0277
19 |     if true {
20 |         return Box::new(Struct);
21 |     }
22 |     Box::new(Other)
   |
