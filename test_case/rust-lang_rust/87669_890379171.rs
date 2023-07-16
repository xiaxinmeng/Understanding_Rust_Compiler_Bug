plain
   Compiling alloc v0.0.0 (/checkout/library/alloc)
   Compiling cfg-if v0.1.10
   Compiling adler v0.2.3
   Compiling rustc-demangle v0.1.18
error[E0658]: pattern bindings after an `@` are unstable
    |
    |
872 |         let src @ Range { start, end } = slice::range(src, ..self.len());
    |
    = note: see issue #65490 <https://github.com/rust-lang/rust/issues/65490> for more information
    = help: add `#![feature(bindings_after_at)]` to the crate attributes to enable


error[E0658]: pattern bindings after an `@` are unstable
    |
    |
872 |         let src @ Range { start, end } = slice::range(src, ..self.len());
    |
    = note: see issue #65490 <https://github.com/rust-lang/rust/issues/65490> for more information
    = help: add `#![feature(bindings_after_at)]` to the crate attributes to enable

