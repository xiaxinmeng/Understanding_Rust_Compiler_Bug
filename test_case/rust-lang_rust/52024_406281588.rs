plain
[00:54:21]     |
[00:54:21]     = note: #[warn(intra_doc_link_resolution_failure)] on by default
[00:54:21]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:54:21] 
[00:54:21] thread '<unnamed>' panicked at 'internal error: entered unreachable code', librustdoc/html/render.rs:2172:17
[00:54:21] 
[00:54:21] error: internal compiler error: unexpected panic
[00:54:21] 
[00:54:21] 
[00:54:21] error: Unrecognized option: 'document-private-items'
[00:54:21] error: Could not document `rustc_target`.
[00:54:21] 
[00:54:21] Caused by:
[00:54:21] Caused by:
[00:54:21]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --crate-name rustc_target librustc_target/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/doc --cfg feature="jemalloc" -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-7e9f3919d4f774b0.rmeta --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-905f2125667bc71f.rmeta --extern rustc_cratesio_shim=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_cratesio_shim-b1b8868001efaf4b.rmeta --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-af91440bac3001e5.rmeta --document-private-items` (exit code: 101)
[00:54:21] [RUSTC-TIMING] rls_span test:false 0.563
[00:54:22] [RUSTC-TIMING] flate2 test:false 1.106
[00:54:22] [RUSTC-TIMING] chalk_engine test:false 4.244
[00:54:22] error: build failed
[00:54:22] error: build failed
[00:54:22] 
[00:54:22] 
[00:54:22] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--no-deps" "-p" "rustc" "-p" "rustc_save_analysis" "-p" "rustc_apfloat" "-p" "serialize" "-p" "fmt_macros" "-p" "graphviz" "-p" "rustc_privacy" "-p" "rustc_passes" "-p" "rustc_allocator" "-p" "syntax_ext" "-p" "rustc_resolve" "-p" "rustc_incremental" "-p" "arena" "-p" "rustc_typeck" "-p" "rustc_errors" "-p" "syntax" "-p" "rustc_codegen_utils" "-p" "rustc_mir" "-p" "rustc_target" "-p" "rustc_plugin" "-p" "proc_macro" "-p" "rustc_traits" "-p" "syntax_pos" "-p" "rustc_lint" "-p" "rustc_driver" "-p" "rustc_data_structures" "-p" "rustc_platform_intrinsics" "-p" "rustc_metadata" "-p" "rustc_borrowck"
[00:54:22] 
[00:54:22] 
[00:54:22] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
[00:54:22] Build completed unsuccessfully in 0:49:10
---
travis_time:end:0ad735a4:start=1532007960479417014,finish=1532007960485259817,duration=5842803
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1f594d88
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03456318
travis_time:start:03456318
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:29d82b37
$ dmesg | grep -i kill
