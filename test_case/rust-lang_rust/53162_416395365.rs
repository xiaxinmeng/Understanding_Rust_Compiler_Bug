plain
[00:55:56]     |                     ^^^^^^^^^^^^ cannot be resolved, ignoring
[00:55:56]     |
[00:55:56]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:55:56] 
[00:55:56] thread '<unnamed>' panicked at 'no entry found for key', libcore/option.rs:1000:5
[00:55:56] 
[00:55:56] error: internal compiler error: unexpected panic
[00:55:56] 
[00:55:56] error: Unrecognized option: 'document-private-items'
[00:55:56] error: Unrecognized option: 'document-private-items'
[00:55:56] 
[00:55:57] [RUSTC-TIMING] chalk_engine test:false 4.317
[00:55:57] error: Could not document `rustc_apfloat`.
[00:55:57] 
[00:55:57] Caused by:
[00:55:57]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --crate-name rustc_apfloat librustc_apfloat/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/doc -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-31b76e0e7ad44ce0.rmeta --extern rustc_cratesio_shim=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_cratesio_shim-8045ad198aef2da0.rmeta --document-private-items` (exit code: 1)
[00:55:59] 
[00:55:59] 
[00:55:59] 
[00:55:59] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--no-deps" "-p" "syntax_pos" "-p" "rustc_data_structures" "-p" "fmt_macros" "-p" "rustc_typeck" "-p" "syntax" "-p" "rustc_driver" "-p" "rustc_metadata" "-p" "rustc_borrowck" "-p" "rustc_metadata_utils" "-p" "rustc_allocator" "-p" "rustc_codegen_utils" "-p" "rustc_save_analysis" "-p" "rustc_apfloat" "-p" "rustc_passes" "-p" "graphviz" "-p" "rustc_mir" "-p" "rustc_incremental" "-p" "build_helper" "-p" "rustc_target" "-p" "rustc_errors" "-p" "arena" "-p" "rustc_llvm" "-p" "rustc_resolve" "-p" "syntax_ext" "-p" "rustc_platform_intrinsics" "-p" "rustc_privacy" "-p" "rustc" "-p" "rustc_plugin" "-p" "rustc_codegen_llvm" "-p" "rustc_traits" "-p" "serialize" "-p" "rustc_lint" "-p" "rustc_fs_util" "-p" "proc_macro"
[00:55:59] 
[00:55:59] 
[00:55:59] error: build failed
[00:55:59] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
---
travis_time:end:0189905d:start=1535410514765446272,finish=1535410514772093935,duration=6647663
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1f0a5a7c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:14058211
travis_time:start:14058211
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:11d38966
$ dmesg | grep -i kill
$ dmesg | grep -i kill
[   10.819491] init: failsafe main process (1093) killed by TERM signal
[   42.187403] init: plymouth-upstart-bridge main process (509) killed by TERM signal
travis_fold:end:after_failure.6

Done. Your build exited with 1.
