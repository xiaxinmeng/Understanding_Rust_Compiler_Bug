
[01:22:48] ---- /checkout/src/doc/embedded-book/src/c-tips/index.md - Tips_for_embedded_C_developers::Iterators_vs_Array_Access (line 183) stdout ----
[01:22:48] error[E0425]: cannot find function `process` in this scope
[01:22:48]  --> /checkout/src/doc/embedded-book/src/c-tips/index.md:186:5
[01:22:48]   |
[01:22:48] 5 |     process(*element);
[01:22:48]   |     ^^^^^^^ not found in this scope
[01:22:48] 
[01:22:48] thread '/checkout/src/doc/embedded-book/src/c-tips/index.md - Tips_for_embedded_C_developers::Iterators_vs_Array_Access (line 183)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:351:13
[01:22:48] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:22:48] 
[01:22:48] ---- /checkout/src/doc/embedded-book/src/c-tips/index.md - Tips_for_embedded_C_developers::Preprocessor::Compile_Time_Sizes_and_Computation (line 78) stdout ----
[01:22:48] error: incorrect close delimiter: `]`
[01:22:48]  --> /checkout/src/doc/embedded-book/src/c-tips/index.md:82:38
[01:22:48]   |
[01:22:48] 6 |     #[cfg(not(feature="use_more_ram")]
[01:22:48]   |      -   -                           ^ incorrect close delimiter
[01:22:48]   |      |   |
[01:22:48]   |      |   un-closed delimiter
[01:22:48]   |      close delimiter possibly meant for this
[01:22:48] 
[01:22:48] thread '/checkout/src/doc/embedded-book/src/c-tips/index.md - Tips_for_embedded_C_developers::Preprocessor::Compile_Time_Sizes_and_Computation (line 78)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:351:13
[01:22:48] 
[01:22:48] ---- /checkout/src/doc/embedded-book/src/c-tips/index.md - Tips_for_embedded_C_developers::Volatile_Access (line 262) stdout ----
[01:22:48] error[E0658]: The attribute `interrupt` is currently unknown to the compiler and may have meaning added to it in the future (see issue #29642)
[01:22:48]  --> /checkout/src/doc/embedded-book/src/c-tips/index.md:265:3
[01:22:48]   |
[01:22:48] 5 | #[interrupt]
[01:22:48]   |   ^^^^^^^^^
[01:22:48]   |
[01:22:48]   = help: add #![feature(custom_attribute)] to the crate attributes to enable
[01:22:48] 
[01:22:48] error[E0425]: cannot find function `run_task` in this scope
[01:22:48]   --> /checkout/src/doc/embedded-book/src/c-tips/index.md:280:9
[01:22:48]    |
[01:22:48] 20 |         run_task();
[01:22:48]    |         ^^^^^^^^ not found in this scope
[01:22:48] 
[01:22:48] thread '/checkout/src/doc/embedded-book/src/c-tips/index.md - Tips_for_embedded_C_developers::Volatile_Access (line 262)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:351:13
[01:22:48] 
