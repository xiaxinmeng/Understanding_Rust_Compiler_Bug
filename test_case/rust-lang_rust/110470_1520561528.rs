plain
   Compiling basic-toml v0.1.2
   Compiling askama_derive v0.12.1
    Checking askama v0.12.0
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error: unused imports: `LayoutS`, `Primitive`, `TagEncoding`, `Variants`
  --> src/librustdoc/html/render/print_item.rs:12:25
   |
12 | use rustc_target::abi::{LayoutS, Primitive, TagEncoding, Variants};
   |
   |
   = note: `-D unused-imports` implied by `-D warnings`
error: could not compile `rustdoc` due to previous error
Build completed unsuccessfully in 0:01:16
