
error[E0277]: the trait bound `std::string::String: From<&str>` is not satisfied
  --> src/main.rs:74:10
   |
74 | #[derive(StructOpt)]
   |          ^^^^^^^^^ the trait `From<&str>` is not implemented for `std::string::String`
   |
   = note: required because of the requirements on the impl of `Into<std::string::String>` for `&str`
note: required by a bound in `App::<'a, 'b>::new`
  --> /home/npmccallum/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-2.33.3/src/app/mod.rs:80:19
   |
80 |     pub fn new<S: Into<String>>(n: S) -> Self {
   |                   ^^^^^^^^^^^^ required by this bound in `App::<'a, 'b>::new`
   = note: this error originates in the derive macro `StructOpt` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider introducing a `where` bound, but there might be an alternative better way to express this requirement
   |
75 | struct Info where std::string::String: From<&str> {}
   |             +++++++++++++++++++++++++++++++++++++

error[E0277]: the trait bound `std::string::String: From<&str>` is not satisfied
  --> src/main.rs:78:10
   |
78 | #[derive(StructOpt)]
   |          ^^^^^^^^^ the trait `From<&str>` is not implemented for `std::string::String`
   |
   = note: required because of the requirements on the impl of `Into<std::string::String>` for `&str`
note: required by a bound in `App::<'a, 'b>::new`
  --> /home/npmccallum/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-2.33.3/src/app/mod.rs:80:19
   |
80 |     pub fn new<S: Into<String>>(n: S) -> Self {
   |                   ^^^^^^^^^^^^ required by this bound in `App::<'a, 'b>::new`
   = note: this error originates in the derive macro `StructOpt` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider introducing a `where` bound, but there might be an alternative better way to express this requirement
   |
79 | struct Exec where std::string::String: From<&str> {
   |             +++++++++++++++++++++++++++++++++++++

error[E0277]: the trait bound `std::string::String: From<&str>` is not satisfied
  --> src/main.rs:84:10
   |
84 | #[derive(StructOpt)]
   |          ^^^^^^^^^ the trait `From<&str>` is not implemented for `std::string::String`
   |
   = note: required because of the requirements on the impl of `Into<std::string::String>` for `&str`
note: required by a bound in `App::<'a, 'b>::new`
  --> /home/npmccallum/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-2.33.3/src/app/mod.rs:80:19
   |
80 |     pub fn new<S: Into<String>>(n: S) -> Self {
   |                   ^^^^^^^^^^^^ required by this bound in `App::<'a, 'b>::new`
   = note: this error originates in the derive macro `StructOpt` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider introducing a `where` bound, but there might be an alternative better way to express this requirement
   |
86 | enum Options where std::string::String: From<&str> {
   |              +++++++++++++++++++++++++++++++++++++

For more information about this error, try `rustc --explain E0277`.
error: could not compile `enarx-keepldr` due to 12 previous errors
