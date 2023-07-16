plain
    Checking tracing-tree v0.1.9
    Checking rustdoc-json-types v0.1.0 (/checkout/src/rustdoc-json-types)
    Checking tera v1.10.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error[E0432]: unresolved import `rustc_middle::ty::WithConstness`
 --> src/librustdoc/clean/blanket_impl.rs:6:37
  |
6 | use rustc_middle::ty::{ToPredicate, WithConstness};
  |                                     ^^^^^^^^^^^^^ no `WithConstness` in `ty`
For more information about this error, try `rustc --explain E0432`.
error: could not compile `rustdoc` due to previous error
Build completed unsuccessfully in 0:02:33
