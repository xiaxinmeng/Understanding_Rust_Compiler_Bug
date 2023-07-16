\n\nIf you don't know the basics of Rust, you can go look to the Rust Book to get\nstarted: https://doc.rust-lang.org/book/\n"},"level":"error","spans":[],"children":[{"message":"consider adding a `main` function to `/checkout/src/test/compile-fail/gated-bad-feature.rs`","code":null,"level":"note","spans":[],"children":[],"rendered":null}],"rendered":"error[E0601]: `main` function not found in crate `gated_bad_feature`\n   |\n   = note: consider adding a `main` function to `/checkout/src/test/compile-fail/gated-bad-feature.rs`\n\n"}
[00:54:11] thread 'rustc' panicked at 'value was not set', libcore/option.rs:916:5
[00:54:11] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:54:11] {"message":"aborting due to 6 previous errors","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to 6 previous errors\n\n"}
[00:54:11] {"message":"Some errors occurred: E0555, E0556, E0557, E0601.","code":null,"level":"","spans":[],"children":[],"rendered":"Some errors occurred: E0555, E0556, E0557, E0601.\n"}
[00:54:11] {"message":"For more information about an error, try `rustc --explain E0555`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0555`.\n"}
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
---
[00:54:11] error: compiler encountered internal error
[00:54:11] status: exit code: 101
[00:54:11] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/compile-fail/hygiene/no_implicit_prelude.rs" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/compile-fail" "--target=x86_64n() {\n    // Your program will start here.\n    println!(\"Hello world!\");\n}\n