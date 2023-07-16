
error[E0433]: failed to resolve. Use of undeclared type or module `nope`
 --> expanded.rs:7:11
  |
7 | impl From<nope::Thing> for Error {
  |           ^^^^^^^^^^^ Use of undeclared type or module `nope`

error[E0433]: failed to resolve. Use of undeclared type or module `nope`
 --> expanded.rs:8:16
  |
8 |     fn from(_: nope::Thing) -> Self {
  |                ^^^^^^^^^^^ Use of undeclared type or module `nope`

error[E0119]: conflicting implementations of trait `std::convert::From<[type error]>` for type `Error`:
  --> expanded.rs:13:1
   |
7  | / impl From<nope::Thing> for Error {
8  | |     fn from(_: nope::Thing) -> Self {
9  | |         unimplemented!()
10 | |     }
11 | | }
   | |_- first implementation here
12 |
13 | / impl From<ErrorKind> for Error {
14 | |     fn from(_: ErrorKind) -> Self {
15 | |         unimplemented!()
16 | |     }
17 | | }
   | |_^ conflicting implementation for `Error`
