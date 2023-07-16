
error[E0277]: `main` has invalid return type `std::result::Result<f32, std::num::ParseIntError>`
  --> $DIR/termination-trait-test-wrong-type.rs:18:1
   |
LL | fn can_parse_zero_as_f32() -> Result<f32, ParseIntError> { //~ ERROR
   |                               ^^^^^^^^^^^^^^^^^^^^^^^^^^ doesn't implement `std::process::Termination`
   |
   = help: the trait `std::process::Termination` is not implemented for `std::result::Result<f32, std::num::ParseIntError>`
   = note: required by `__test::test::assert_test_result`
