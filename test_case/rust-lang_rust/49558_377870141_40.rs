\n\nHere, `Grams2` is a not equivalent to `Grams` -- the former transparently wraps\na (non-transparent) struct containing a single float, while `Grams` is a\ntransparent wrapper around a float. This can make a difference for the ABI.\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/compile-fail/repr-transparent-other-reprs.rs","byte_start":774,"byte_end":785,"line_start":r(transparent)] //~ ERROR cannot have other repr\n   |        ^^^^^^^^^^^\nLL | #[repr(C)]\n   |        ^\n\n"}
[00:54:11] thread 'rustc' panicked at 'value was not set', libcore/option.rs:916:5
[00:54:11] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:54:11] {"message":"aborting due to 5 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 5 previous errors\n\n"}
[00:54:11] {"message":"Some errors occurred: E0601, E0692.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0601, E0692.\n"}
[00:54:11] {"message":"For more information about an error, try `rustc --explain E0601`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0601`.\n"}
[00:54:11]
[00:54:11] error: internal compiler error: unexpected panic
[00:54:11]
[00:54:11] note: the compiler unexpectedly panicked. this is a bug.
[00:54:11]
[00:54:11] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:54:11]
[00:54:11] note: rustc 1.26.0-dev running on x86_64-unknown-linux-gnu
[00:54:11]
[00:54:11] note: compiler flags: -Z ui-testing -Z miri -Z unstable-options -C prefer-dynamic -C rpath
[00:54:11]
[00:54:11]
[00:54:11] ------------------------------------------
[00:54:11]
[00:54:11] thread '[compile-fail] compile-fail/repr-transparent-other-reprs.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2901:9
[00:54:11]
[00:54:11] ---- [compile-fail] compile-fail/resolve_self_super_hint.rs stdout ----
[00:54:11]
[00:54:11] error: compiler encountered internal error
[00:54:11] status: exit code: 101
[00:54:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/resolve_self_super_hint.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/resolve_self_super_hint.stage2-x86_64-unknown-linux-gnu" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail/resolve_self_super_hint.stage2-x86_64-unknown-linux-gnu.aux" "-A" "unused"
---
[00:54:11] {"message":"unresolved import `alloc`","code":{"code":"E0432","explanation":"\nAn import was unresolved.\n\nErroneous code example:\n\n