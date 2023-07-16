plain
   Compiling libc v0.2.98
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.49
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error[E0658]: pattern bindings after an `@` are unstable
   --> library/core/src/iter/adapters/enumerate.rs:123:23
    |
123 |             ret @ Err(advanced) => {
    |
    = note: see issue #65490 <https://github.com/rust-lang/rust/issues/65490> for more information
    = help: add `#![feature(bindings_after_at)]` to the crate attributes to enable


error[E0658]: pattern bindings after an `@` are unstable
   --> library/core/src/iter/adapters/take.rs:124:23
    |
124 |             ret @ Err(advanced) => {
    |
    = note: see issue #65490 <https://github.com/rust-lang/rust/issues/65490> for more information
    = help: add `#![feature(bindings_after_at)]` to the crate attributes to enable

