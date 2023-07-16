rust
error[E0658]: use of extern prelude names introduced with `extern crate` items is unstable (see issue #54658)
  --> interp-macros/src/lib.rs:12:5
   |
12 | use proc_macro::TokenStream;
   |     ^^^^^^^^^^
   |
   = help: add #![feature(extern_crate_item_prelude)] to the crate attributes to enable
