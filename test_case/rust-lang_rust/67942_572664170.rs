
   Compiling rustc_lint v0.0.0 (/home/mental/Desktop/open_projects/rust/src/librustc_lint)
   Compiling rustc_interface v0.0.0 (/home/mental/Desktop/open_projects/rust/src/librustc_interface)
   Compiling rustc_driver v0.0.0 (/home/mental/Desktop/open_projects/rust/src/librustc_driver)
   Compiling rustc-main v0.0.0 (/home/mental/Desktop/open_projects/rust/src/rustc)
    Finished release [optimized] target(s) in 10m 43s
Installing libLLVM.so to stage 0 (x86_64-unknown-linux-gnu)
Copying stage0 rustc from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Assembling stage1 compiler (x86_64-unknown-linux-gnu)
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling core v0.0.0 (/home/mental/Desktop/open_projects/rust/src/libcore)
error: unnecessary parentheses around const item
  --> src/libcore/../stdarch/crates/core_arch/src/x86/sse41.rs:38:35
   |
38 | pub const _MM_FROUND_FLOOR: i32 = (_MM_FROUND_RAISE_EXC | _MM_FROUND_TO_NEG_INF);
   |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove these parentheses
   |
   = note: `-D unused-parens` implied by `-D warnings`

error: unnecessary parentheses around const item
  --> src/libcore/../stdarch/crates/core_arch/src/x86/sse41.rs:41:34
   |
41 | pub const _MM_FROUND_CEIL: i32 = (_MM_FROUND_RAISE_EXC | _MM_FROUND_TO_POS_INF);
   |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove these parentheses

error: unnecessary parentheses around const item
  --> src/libcore/../stdarch/crates/core_arch/src/x86/sse41.rs:44:35
   |
44 | pub const _MM_FROUND_TRUNC: i32 = (_MM_FROUND_RAISE_EXC | _MM_FROUND_TO_ZERO);
   |                                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove these parentheses

error: unnecessary parentheses around const item
  --> src/libcore/../stdarch/crates/core_arch/src/x86/sse41.rs:48:34
   |
48 | pub const _MM_FROUND_RINT: i32 = (_MM_FROUND_RAISE_EXC | _MM_FROUND_CUR_DIRECTION);
   |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove these parentheses

error: unnecessary parentheses around const item
  --> src/libcore/../stdarch/crates/core_arch/src/x86/sse41.rs:51:39
   |
51 | pub const _MM_FROUND_NEARBYINT: i32 = (_MM_FROUND_NO_EXC | _MM_FROUND_CUR_DIRECTION);
   |                                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove these parentheses

error: aborting due to 5 previous errors

error: could not compile `core`.

To learn more, run the command again with --verbose.
command did not execute successfully: "/home/mental/Desktop/open_projects/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/home/mental/Desktop/open_projects/rust/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/home/mental/Desktop/open_projects/rust/src/libtest/Cargo.toml" "--message-format" "json-render-diagnostics"
expected success, got: exit code: 101
failed to run: /home/mental/Desktop/open_projects/rust/build/bootstrap/debug/bootstrap build
Build completed unsuccessfully in 0:13:03
