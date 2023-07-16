
$ rustc +nightly long.rs
error[E0275]: overflow evaluating the requirement `S0: std::marker::Send`
  --> long.rs:67:13
   |
67 | fn main() { is_send::<S63>(); }
   |             ^^^^^^^^^^^^^^
   |
   = help: consider adding a `#![recursion_limit="128"]` attribute to your crate
   = note: required because it appears within the type `S1`
   = note: required because it appears within the type `S2`
   = note: required because it appears within the type `S3`
   = note: required because it appears within the type `S4`
   = note: ... blah blah blah ...
   = note: required because it appears within the type `S61`
   = note: required because it appears within the type `S62`
   = note: required because it appears within the type `S63`
note: required by `is_send`
  --> long.rs:66:1
   |
66 | fn is_send<T: Send>() {}
   | ^^^^^^^^^^^^^^^^^^^^^
