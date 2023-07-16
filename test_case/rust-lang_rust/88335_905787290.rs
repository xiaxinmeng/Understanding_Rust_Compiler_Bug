plain
   Compiling parking_lot v0.11.1
   Compiling cstr v0.2.8
   Compiling rand_core v0.5.1
   Compiling regex-automata v0.1.10
error[E0428]: the name `extern_item` is defined multiple times
  --> /cargo/registry/src/github.com-1ecc6299db9ec823/psm-0.1.11/src/lib.rs:27:1
15 | macro_rules! extern_item {
15 | macro_rules! extern_item {
   | ------------------------ previous definition of the macro `extern_item` here
27 | macro_rules! extern_item {
27 | macro_rules! extern_item {
   | ^^^^^^^^^^^^^^^^^^^^^^^^ `extern_item` redefined here
   |
   = note: `extern_item` must be defined only once in the macro namespace of this crate
   Compiling regex v1.4.6
   Compiling rand_chacha v0.3.0
For more information about this error, try `rustc --explain E0428`.
error: could not compile `stacker` due to previous error
