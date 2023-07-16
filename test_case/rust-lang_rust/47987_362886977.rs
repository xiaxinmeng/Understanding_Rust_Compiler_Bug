
[01:41:47] failures:
[01:41:47] 
[01:41:47] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0265 (line 4308) stdout ----
[01:41:47] 	error[E0391]: unsupported cyclic reference between types/traits detected
[01:41:47]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:4309:16
[01:41:47]   |
[01:41:47] 3 | const X: u32 = X;
[01:41:47]   |                ^ cyclic reference
[01:41:47]   |
[01:41:47] note: the cycle begins when processing `main::X`...
[01:41:47]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:4309:1
[01:41:47]   |
[01:41:47] 3 | const X: u32 = X;
[01:41:47]   | ^^^^^^^^^^^^^^^^^
[01:41:47]   = note: ...which then again requires processing `main::X`, completing the cycle.
[01:41:47] 
[01:41:47] error: aborting due to previous error
[01:41:47] 
[01:41:47] thread 'rustc' panicked at 'Some expected error codes were not found: ["E0265"]', librustdoc/test.rs:303:9
[01:41:47] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:41:47] 
[01:41:47] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0265 (line 4312) stdout ----
[01:41:47] 	error[E0391]: unsupported cyclic reference between types/traits detected
[01:41:47]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:4313:16
[01:41:47]   |
[01:41:47] 3 | const X: u32 = Y;
[01:41:47]   |                ^ cyclic reference
[01:41:47]   |
[01:41:47] note: the cycle begins when processing `main::Y`...
[01:41:47]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:4314:1
[01:41:47]   |
[01:41:47] 4 | const Y: u32 = X;
[01:41:47]   | ^^^^^^^^^^^^^^^^^
[01:41:47] note: ...which then requires processing `main::X`...
[01:41:47]  --> /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md:4314:16
[01:41:47]   |
[01:41:47] 4 | const Y: u32 = X;
[01:41:47]   |                ^
[01:41:47]   = note: ...which then again requires processing `main::Y`, completing the cycle.
[01:41:47] 
[01:41:47] error: aborting due to previous error
