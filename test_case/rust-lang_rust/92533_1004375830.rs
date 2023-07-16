plain
    Checking ar v0.8.0
    Checking gccjit_sys v0.0.1 (https://github.com/antoyo/gccjit.rs#0672b78d)
    Checking gccjit v1.0.0 (https://github.com/antoyo/gccjit.rs#0672b78d)
    Checking rustc_codegen_gcc v0.1.0 (/checkout/compiler/rustc_codegen_gcc)
error[E0615]: attempted to take value of method `ident` on type `VariantDef`
   |
   |
60 |                     write!(&mut name, "::{}", def.variants[index].ident).unwrap();
   |
help: use parentheses to call the method
   |
   |
60 |                     write!(&mut name, "::{}", def.variants[index].ident(_)).unwrap();

error[E0277]: the size for values of type `str` cannot be known at compilation time
   --> src/type_of.rs:86:17
    |
    |
86  |                 Some(ref name) => {
    |                 ^^^^^^^^^^^^^^ doesn't have a size known at compile-time
    |
    = help: the trait `std::marker::Sized` is not implemented for `str`
note: required by a bound in `std::prelude::v1::Some`
    |
515 | pub enum Option<T> {
515 | pub enum Option<T> {
    |                 ^ required by this bound in `std::prelude::v1::Some`
error[E0277]: the size for values of type `str` cannot be known at compilation time
   --> src/type_of.rs:85:17
    |
    |
85  |                 None => cx.type_struct(&[fill], packed),
    |
    = help: the trait `std::marker::Sized` is not implemented for `str`
note: required by a bound in `std::prelude::v1::None`
   --> /checkout/library/core/src/option.rs:515:17
   --> /checkout/library/core/src/option.rs:515:17
    |
515 | pub enum Option<T> {
    |                 ^ required by this bound in `std::prelude::v1::None`
error[E0277]: the size for values of type `str` cannot be known at compilation time
   --> src/type_of.rs:96:17
    |
96  |                 None => {
96  |                 None => {
    |                 ^^^^ doesn't have a size known at compile-time
    |
    = help: the trait `std::marker::Sized` is not implemented for `str`
note: required by a bound in `std::prelude::v1::None`
   --> /checkout/library/core/src/option.rs:515:17
    |
515 | pub enum Option<T> {
    |                 ^ required by this bound in `std::prelude::v1::None`
error[E0277]: the size for values of type `str` cannot be known at compilation time
   --> src/type_of.rs:100:17
    |
100 |                 Some(ref name) => {
100 |                 Some(ref name) => {
    |                 ^^^^^^^^^^^^^^ doesn't have a size known at compile-time
    |
    = help: the trait `std::marker::Sized` is not implemented for `str`
note: required by a bound in `std::prelude::v1::Some`
    |
515 | pub enum Option<T> {
515 | pub enum Option<T> {
    |                 ^ required by this bound in `std::prelude::v1::Some`
Some errors have detailed explanations: E0277, E0615.
For more information about an error, try `rustc --explain E0277`.
error: could not compile `rustc_codegen_gcc` due to 5 previous errors
Build completed unsuccessfully in 0:03:05
