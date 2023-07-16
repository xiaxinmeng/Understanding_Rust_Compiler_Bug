
[00:50:41] failures:
[00:50:41] 
[00:50:41] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index (line 7427) stdout ----
[00:50:41] 	error[E0455]: native frameworks are only available on macOS targets
[00:50:41]  --> <anon>:2:1
[00:50:41]   |
[00:50:41] 2 | #[link(name = "FooCoreServices", kind = "framework")] extern {}
[00:50:41]   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:50:41] 
[00:50:41] error: aborting due to previous error(s)
[00:50:41] 
[00:50:41] thread 'rustc' panicked at 'Box<Any>', /checkout/src/librustc/session/mod.rs:218
[00:50:41] 
[00:50:41] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index (line 7887) stdout ----
[00:50:41] 	error[E0455]: native frameworks are only available on macOS targets
[00:50:41]  --> <anon>:2:1
[00:50:41]   |
[00:50:41] 2 | #[link(name = "FooCoreServices", kind = "framework")] extern {}
[00:50:41]   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
[00:50:41] 
[00:50:41] error: aborting due to previous error(s)
[00:50:41] 
[00:50:41] thread 'rustc' panicked at 'Box<Any>', /checkout/src/librustc/session/mod.rs:218
[00:50:41] 
[00:50:41] 
[00:50:41] failures:
[00:50:41]     /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index (line 7427)
[00:50:41]     /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index (line 7887)
[00:50:41] 
[00:50:41] test result: FAILED. 1138 passed; 2 failed; 18 ignored; 0 measured; 0 filtered out
