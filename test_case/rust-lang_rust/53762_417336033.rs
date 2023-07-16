plain
[00:59:20] [RUSTC-TIMING] regex test:false 59.544
[00:59:49]    Compiling cargo_metadata v0.6.0
[01:00:02]    Compiling clippy_lints v0.0.212 (file:///checkout/src/tools/clippy/clippy_lints)
[01:00:03] [RUSTC-TIMING] cargo_metadata test:false 13.653
[01:00:05] error: cannot find macro `declare_tool_lint!` in this scope
[01:00:05]    |
[01:00:05]    |
[01:00:05] 27 |           declare_tool_lint! { pub clippy::$name, Warn, $description, report_in_external_macro: true }
[01:00:05]    | 
[01:00:05]   ::: tools/clippy/clippy_lints/src/ptr_offset_with_cast.rs:31:1
[01:00:05]    |
[01:00:05]    |
[01:00:05] 31 | / declare_clippy_lint! {
[01:00:05] 32 | |     pub PTR_OFFSET_WITH_CAST,
[01:00:05] 33 | |     complexity,
[01:00:05] 34 | |     "uneeded pointer offset cast"
[01:00:05]    | |_- in this macro invocation
[01:00:05] 
[01:00:05] error[E0425]: cannot find value `PTR_OFFSET_WITH_CAST` in this scope
[01:00:05]   --> tools/clippy/clippy_lints/src/ptr_offset_with_cast.rs:42:21
[01:00:05]   --> tools/clippy/clippy_lints/src/ptr_offset_with_cast.rs:42:21
[01:00:05]    |
[01:00:05] 42 |         lint_array!(PTR_OFFSET_WITH_CAST)
[01:00:05] 
[01:00:05] error[E0425]: cannot find value `PTR_OFFSET_WITH_CAST` in this scope
[01:00:05]   --> tools/clippy/clippy_lints/src/ptr_offset_with_cast.rs:62:43
[01:00:05]    |
[01:00:05]    |
[01:00:05] 62 |             utils::span_lint_and_sugg(cx, PTR_OFFSET_WITH_CAST, expr.span, &msg, "try", sugg);
[01:00:05] 
[01:00:05] error[E0425]: cannot find value `PTR_OFFSET_WITH_CAST` in this scope
[01:00:05]   --> tools/clippy/clippy_lints/src/ptr_offset_with_cast.rs:64:34
[01:00:05]    |
[01:00:05]    |
[01:00:05] 64 |             utils::span_lint(cx, PTR_OFFSET_WITH_CAST, expr.span, &msg);
[01:00:05]    |                                  ^^^^^^^^^^^^^^^^^^^^ not found in this scope
[01:00:05] 
[01:00:05] error[E0425]: cannot find value `PTR_OFFSET_WITH_CAST` in module `ptr_offset_with_cast`
[01:00:05]     |
[01:00:05] 637 |         ptr_offset_with_cast::PTR_OFFSET_WITH_CAST,
[01:00:05]     |                               ^^^^^^^^^^^^^^^^^^^^ not found in `ptr_offset_with_cast`
[01:00:05] 
[01:00:05] 
[01:00:05] error[E0425]: cannot find value `PTR_OFFSET_WITH_CAST` in module `ptr_offset_with_cast`
[01:00:05]     |
[01:00:05] 822 |         ptr_offset_with_cast::PTR_OFFSET_WITH_CAST,
[01:00:05]     |                               ^^^^^^^^^^^^^^^^^^^^ not found in `ptr_offset_with_cast`
[01:00:05] 
---
[01:00:05] 
[01:00:05] Caused by:
[01:00:05]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --edition=2018 --crate-name clippy_lints tools/clippy/clippy_lints/src/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=eabb50878b391657 -C extra-filename=-eabb50878b391657 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps --extern cargo_metadata=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libcargo_metadata-12af0bcd847f0d23.rlib --extern if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libif_chain-07b3664d787c7572.rlib --extern itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libitertools-2d91947ad2507a77.rlib --extern lazy_static=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/liblazy_static-870f2eb0a64ba289.rlib --extern matches=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libmatches-8a101770c4388b9c.rlib --extern pulldown_cmark=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libpulldown_cmark-74e9288dce480f90.rlib --extern quine_mc_cluskey=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libquine_mc_cluskey-2ee1c41d9ae864f4.rlib --extern regex_syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libregex_syntax-4e485525325216a3.rlib --extern semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libsemver-6af0386c7b598b95.rlib --extern serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde-df457623811513dd.rlib --extern serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-d7c0f676a3203f32.so --extern toml=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libtoml-f0b066ee4279cf94.rlib --extern unicode_normalization=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libunicode_normalization-f5faf42c64d3e9c9.rlib --extern url=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/liburl-dfec5b8d1c987bce.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/build/backtrace-sys-14658f1bf6019970/out` (exit code: 1)
[01:00:05] warning: build failed, waiting for other jobs to finish...
[01:00:05] error: cannot find macro `declare_tool_lint!` in this scope
[01:00:05]    |
[01:00:05]    |
[01:00:05] 27 |           declare_tool_lint! { pub clippy::$name, Warn, $description, report_in_external_macro: true }
[01:00:05]    | 
[01:00:05]   ::: tools/clippy/clippy_lints/src/ptr_offset_with_cast.rs:31:1
[01:00:05]    |
[01:00:05]    |
[01:00:05] 31 | / declare_clippy_lint! {
[01:00:05] 32 | |     pub PTR_OFFSET_WITH_CAST,
[01:00:05] 33 | |     complexity,
[01:00:05] 34 | |     "uneeded pointer offset cast"
[01:00:05]    | |_- in this macro invocation
[01:00:05] 
[01:00:05] error[E0425]: cannot find value `PTR_OFFSET_WITH_CAST` in this scope
[01:00:05]   --> tools/clippy/clippy_lints/src/ptr_offset_with_cast.rs:42:21
[01:00:05]   --> tools/clippy/clippy_lints/src/ptr_offset_with_cast.rs:42:21
[01:00:05]    |
[01:00:05] 42 |         lint_array!(PTR_OFFSET_WITH_CAST)
[01:00:05] 
[01:00:05] error[E0425]: cannot find value `PTR_OFFSET_WITH_CAST` in this scope
[01:00:05]   --> tools/clippy/clippy_lints/src/ptr_offset_with_cast.rs:62:43
[01:00:05]    |
[01:00:05]    |
[01:00:05] 62 |             utils::span_lint_and_sugg(cx, PTR_OFFSET_WITH_CAST, expr.span, &msg, "try", sugg);
[01:00:05] 
[01:00:05] error[E0425]: cannot find value `PTR_OFFSET_WITH_CAST` in this scope
[01:00:05]   --> tools/clippy/clippy_lints/src/ptr_offset_with_cast.rs:64:34
[01:00:05]    |
[01:00:05]    |
[01:00:05] 64 |             utils::span_lint(cx, PTR_OFFSET_WITH_CAST, expr.span, &msg);
[01:00:05]    |                                  ^^^^^^^^^^^^^^^^^^^^ not found in this scope
[01:00:05] 
[01:00:05] error[E0425]: cannot find value `PTR_OFFSET_WITH_CAST` in module `ptr_offset_with_cast`
[01:00:05]     |
[01:00:05] 637 |         ptr_offset_with_cast::PTR_OFFSET_WITH_CAST,
[01:00:05]     |                               ^^^^^^^^^^^^^^^^^^^^ not found in `ptr_offset_with_cast`
[01:00:05] 
[01:00:05] 
[01:00:05] error[E0425]: cannot find value `PTR_OFFSET_WITH_CAST` in module `ptr_offset_with_cast`
[01:00:05]     |
[01:00:05] 822 |         ptr_offset_with_cast::PTR_OFFSET_WITH_CAST,
[01:00:05]     |                               ^^^^^^^^^^^^^^^^^^^^ not found in `ptr_offset_with_cast`
[01:00:05] 
---
[01:06:25] [RUSTC-TIMING] ignore test:false 34.454
[01:06:25]    Compiling clippy_lints v0.0.212 (file:///checkout/src/tools/clippy/clippy_lints)
[01:06:27] [RUSTC-TIMING] failure test:false 1.824
[01:06:27]    Compiling rayon v1.0.1
[01:06:29] error: cannot find macro `declare_tool_lint!` in this scope
[01:06:29]    |
[01:06:29]    |
[01:06:29] 27 |           declare_tool_lint! { pub clippy::$name, Warn, $description, report_in_external_macro: true }
[01:06:29]    | 
[01:06:29]   ::: tools/clippy/clippy_lints/src/ptr_offset_with_cast.rs:31:1
[01:06:29]    |
[01:06:29]    |
[01:06:29] 31 | / declare_clippy_lint! {
[01:06:29] 32 | |     pub PTR_OFFSET_WITH_CAST,
[01:06:29] 33 | |     complexity,
[01:06:29] 34 | |     "uneeded pointer offset cast"
[01:06:29]    | |_- in this macro invocation
[01:06:29] 
[01:06:29] error[E0425]: cannot find value `PTR_OFFSET_WITH_CAST` in this scope
[01:06:29]   --> tools/clippy/clippy_lints/src/ptr_offset_with_cast.rs:42:21
[01:06:29]   --> tools/clippy/clippy_lints/src/ptr_offset_with_cast.rs:42:21
[01:06:29]    |
[01:06:29] 42 |         lint_array!(PTR_OFFSET_WITH_CAST)
[01:06:29] 
[01:06:29] [RUSTC-TIMING] rls_vfs test:false 4.168
[01:06:29] error[E0425]: cannot find value `PTR_OFFSET_WITH_CAST` in this scope
[01:06:29]   --> tools/clippy/clippy_lints/src/ptr_offset_with_cast.rs:62:43
[01:06:29]   --> tools/clippy/clippy_lints/src/ptr_offset_with_cast.rs:62:43
[01:06:29]    |
[01:06:29] 62 |             utils::span_lint_and_sugg(cx, PTR_OFFSET_WITH_CAST, expr.span, &msg, "try", sugg);
[01:06:29] 
[01:06:29] error[E0425]: cannot find value `PTR_OFFSET_WITH_CAST` in this scope
[01:06:29]   --> tools/clippy/clippy_lints/src/ptr_offset_with_cast.rs:64:34
[01:06:29]    |
[01:06:29]    |
[01:06:29] 64 |             utils::span_lint(cx, PTR_OFFSET_WITH_CAST, expr.span, &msg);
[01:06:29]    |                                  ^^^^^^^^^^^^^^^^^^^^ not found in this scope
[01:06:29] 
[01:06:29] error[E0425]: cannot find value `PTR_OFFSET_WITH_CAST` in module `ptr_offset_with_cast`
[01:06:29]     |
[01:06:29] 637 |         ptr_offset_with_cast::PTR_OFFSET_WITH_CAST,
[01:06:29]     |                               ^^^^^^^^^^^^^^^^^^^^ not found in `ptr_offset_with_cast`
[01:06:29] 
[01:06:29] 
[01:06:29] error[E0425]: cannot find value `PTR_OFFSET_WITH_CAST` in module `ptr_offset_with_cast`
[01:06:29]     |
[01:06:29] 822 |         ptr_offset_with_cast::PTR_OFFSET_WITH_CAST,
[01:06:29]     |                               ^^^^^^^^^^^^^^^^^^^^ not found in `ptr_offset_with_cast`
[01:06:29] 
---
[01:12:07] Verifying status of rustfmt...
[01:12:07] Verifying status of clippy-driver...
[01:12:07] This PR updated 'src/tools/clippy', verifying if status is 'test-pass'...
[01:12:07] 
[01:12:07] ⚠️ We detected that this PR updated 'clippy-driver', but its tests failed.
[01:12:07] 
[01:12:07] If you do intend to update 'clippy-driver', please check the error messages above and
[01:12:07] commit another update.
[01:12:07] 
[01:12:07] If you do NOT intend to update 'clippy-driver', please ensure you did not accidentally
[01:12:07] change the submodule at 'src/tools/clippy'. You may ask your reviewer for the
[01:12:07] proper steps.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 3.
travis_time:start:309ee0e1
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:151f4f16:start=1535638664918714260,finish=1535638664924457200,duration=5742940
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:18e716ce
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:04f7fc6d
travis_time:start:04f7fc6d
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:27f98c7c
$ dmesg | grep -i kill
