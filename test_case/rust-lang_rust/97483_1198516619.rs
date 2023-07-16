rust
error: an associated function with this name may be added to the standard library in the future
    --> compiler/rustc_codegen_llvm/src/debuginfo/metadata.rs:1499:25
     |
1499 |             vtable_name.as_ptr().cast(),
     |                         ^^^^^^
     |
     = warning: once this associated item is added to the standard library, the ambiguity may cause an error or change in behavior!
     = note: for more information, see issue #48919 <https://github.com/rust-lang/rust/issues/48919>
     = help: call with fully qualified syntax `bitflags::core::str::<impl str>::as_ptr(...)` to keep using the current method
     = help: add `#![feature(string_as_ptr)]` to the crate attributes to enable `std::string::String::as_ptr`
