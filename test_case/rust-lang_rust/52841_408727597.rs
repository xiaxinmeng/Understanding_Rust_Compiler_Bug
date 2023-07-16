plain
[01:18:25] 
[01:18:25] failures:
[01:18:25] 
[01:18:25] ---- /checkout/src/doc/unstable-book/src/language-features/tool-attributes.md - tool_attributes::An_example (line 17) stdout ----
[01:18:25] error[E0658]: The attribute `rustfmt::skip` is currently unknown to the compiler and may have meaning added to it in the future (see issue #29642)
[01:18:25]   |
[01:18:25] 4 | #[rustfmt::skip]
[01:18:25]   | ^^^^^^^^^^^^^^^^
[01:18:25]   |
[01:18:25]   |
[01:18:25]   = help: add #![feature(custom_attribute)] to the crate attributes to enable
[01:18:25] thread '/checkout/src/doc/unstable-book/src/language-features/tool-attributes.md - tool_attributes::An_example (line 17)' panicked at 'couldn't compile the test', librustdoc/test.rs:332:13
[01:18:25] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:18:25] 
[01:18:25] 
