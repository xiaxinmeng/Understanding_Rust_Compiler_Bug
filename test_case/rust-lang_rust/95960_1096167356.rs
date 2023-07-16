plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.70
   Compiling unwind v0.0.0 (/checkout/library/unwind)
error: `#[rustc_deprecated]` has been removed
  --> library/core/src/../../stdarch/crates/core_arch/src/x86/eflags.rs:28:1
28 | / #[rustc_deprecated(
29 | |     since = "1.29.0",
29 | |     since = "1.29.0",
30 | |     reason = "See issue #51810 - use inline assembly instead"
31 | | )]
   | |__^
   |
   = help: use `#[deprecated]` instead
error: `reason` has been renamed
  --> library/core/src/../../stdarch/crates/core_arch/src/x86/eflags.rs:30:5
   |
   |
30 |     reason = "See issue #51810 - use inline assembly instead"
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `note` instead: `note = "See issue #51810 - use inline assembly instead"`

error: `#[rustc_deprecated]` has been removed
  --> library/core/src/../../stdarch/crates/core_arch/src/x86/eflags.rs:60:1
60 | / #[rustc_deprecated(
61 | |     since = "1.29.0",
61 | |     since = "1.29.0",
62 | |     reason = "See issue #51810 - use inline assembly instead"
63 | | )]
   | |__^
   |
   = help: use `#[deprecated]` instead
error: `reason` has been renamed
  --> library/core/src/../../stdarch/crates/core_arch/src/x86/eflags.rs:62:5
   |
   |
62 |     reason = "See issue #51810 - use inline assembly instead"
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: use `note` instead: `note = "See issue #51810 - use inline assembly instead"`
error: could not compile `core` due to 4 previous errors
Build completed unsuccessfully in 0:03:34
