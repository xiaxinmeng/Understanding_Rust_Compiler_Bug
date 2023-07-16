
warning: a method with this name will be added to the standard library in the future
  --> src/test/compile-fail/binop-logic-int.rs:11:21
   |
11 | fn main() { let x = 1 && 2; }
   |                     ^
   |
   = note: #[warn(unstable_name_collision)] on by default
   = warning: once this method is added to the standard library, there will be ambiguity here, which will cause a hard error!
   = note: for more information, see issue #48919 <https://github.com/rust-lang/rust/issues/48919>
   = help: call with fully qualified syntax `std::sys::windows::IsZero::is_zero(...)` to keep using the current method
   = note: add #![feature(nonzero)] to the crate attributes to enable `core::nonzero::Zeroable::is_zero`

error[E0308]: mismatched types
  --> src/test/compile-fail/binop-logic-int.rs:11:21
   |
11 | fn main() { let x = 1 && 2; }
   |                     ^ expected bool, found integral variable
   |
   = note: expected type `bool`
              found type `{integer}`

warning: a method with this name will be added to the standard library in the future
  --> src/test/compile-fail/binop-logic-int.rs:11:26
   |
11 | fn main() { let x = 1 && 2; }
   |                          ^
   |
   = warning: once this method is added to the standard library, there will be ambiguity here, which will cause a hard error!
   = note: for more information, see issue #48919 <https://github.com/rust-lang/rust/issues/48919>
   = help: call with fully qualified syntax `std::sys::windows::IsZero::is_zero(...)` to keep using the current method
   = note: add #![feature(nonzero)] to the crate attributes to enable `core::nonzero::Zeroable::is_zero`

error[E0308]: mismatched types
  --> src/test/compile-fail/binop-logic-int.rs:11:26
   |
11 | fn main() { let x = 1 && 2; }
   |                          ^ expected bool, found integral variable
   |
   = note: expected type `bool`
              found type `{integer}`

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.
