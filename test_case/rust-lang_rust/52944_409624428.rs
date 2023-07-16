plain
[00:58:06] [RUSTC-TIMING] error_chain test:false 0.632
[00:58:31]    Compiling cargo_metadata v0.5.8
[00:58:44]    Compiling clippy_lints v0.0.212 (file:///checkout/src/tools/clippy/clippy_lints)
[00:58:45] [RUSTC-TIMING] cargo_metadata test:false 13.954
[00:58:51] error[E0004]: non-exhaustive patterns: `ToolMod` and `NonMacroAttr` not covered
[00:58:51]     |
[00:58:51] 936 |     match def {
[00:58:51] 936 |     match def {
[00:58:51]     |           ^^^ patterns `ToolMod` and `NonMacroAttr` not covered
[00:58:52] error: aborting due to previous error
[00:58:52] 
[00:58:52] For more information about this error, try `rustc --explain E0004`.
[00:58:52] error: Could not compile `clippy_lints`.
[00:58:52] error: Could not compile `clippy_lints`.
[00:58:52] 
[00:58:52] Caused by:
[00:58:52]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --edition=2018 --crate-name clippy_lints tools/clippy/clippy_lints/src/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=2 -C metadata=4f5368c39111a9e2 -C extra-filename=-4f5368c39111a9e2 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps --extern cargo_metadata=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libcargo_metadata-3e7ee31a39ad4d90.rlib --extern if_chain=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libif_chain-522b97d8c10ce346.rlib --extern itertools=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libitertools-884844f566f13cd5.rlib --extern lazy_static=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/liblazy_static-f47c61e4bddb4dc5.rlib --extern matches=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libmatches-3f68686179032f7e.rlib --extern pulldown_cmark=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libpulldown_cmark-fa418dc6b710dcd0.rlib --extern quine_mc_cluskey=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libquine_mc_cluskey-1c5c672bcb3cbbef.rlib --extern regex_syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libregex_syntax-d845c52fdc016715.rlib --extern semver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libsemver-dff62db54336e99c.rlib --extern serde=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde-91b4a05764bf2349.rlib --extern serde_derive=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libserde_derive-7ca48a6ccc017769.so --extern toml=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libtoml-cfc16693dc4e9f72.rlib --extern unicode_normalization=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libunicode_normalization-9e94b87f19a0d93e.rlib --extern url=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/liburl-338c99e445431005.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/build/backtrace-sys-5e3ffd0c26bbe51e/out` (exit code: 1)
[00:58:52] warning: build failed, waiting for other jobs to finish...
[00:58:52] error[E0004]: non-exhaustive patterns: `ToolMod` and `NonMacroAttr` not covered
[00:58:52]     |
[00:58:52] 936 |     match def {
[00:58:52] 936 |     match def {
[00:58:52]     |           ^^^ patterns `ToolMod` and `NonMacroAttr` not covered
[00:58:52] error: aborting due to previous error
[00:58:52] 
[00:58:52] For more information about this error, try `rustc --explain E0004`.
[00:58:52] [RUSTC-TIMING] clippy_lints test:false 7.236
---
[01:05:11] [RUSTC-TIMING] libgit2_sys test:false 0.852
[01:05:11]    Compiling rls-analysis v0.14.0
[01:05:15] [RUSTC-TIMING] cargo_metadata test:false 20.785
[01:05:17] [RUSTC-TIMING] rustfix test:false 6.214
[01:05:21] error[E0004]: non-exhaustive patterns: `ToolMod` and `NonMacroAttr` not covered
[01:05:21]     |
[01:05:21] 936 |     match def {
[01:05:21] 936 |     match def {
[01:05:21]     |           ^^^ patterns `ToolMod` and `NonMacroAttr` not covered
[01:05:21] error: aborting due to previous error
[01:05:21] 
[01:05:21] For more information about this error, try `rustc --explain E0004`.
[01:05:21] [RUSTC-TIMING] clippy_lints test:false 10.841
---
travis_time:end:10f10841:start=1533138960175676606,finish=1533138960182051957,duration=6375351
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:11cbff30
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:032b2d8f
travis_time:start:032b2d8f
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0f6a5030
$ dmesg | grep -i kill
