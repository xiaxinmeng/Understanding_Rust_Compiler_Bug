plain
[01:24:39]                                                         ^^^^^^^^^^^^
[01:24:39]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:24:39] 
[01:24:40]  Documenting arena v0.0.0 (/checkout/src/libarena)
[01:24:41] thread '<unnamed>' panicked at 'assertion failed: output.is_none()', src/librustdoc/clean/simplify.rs:108:21
[01:24:41] 
[01:24:41] error: internal compiler error: unexpected panic
[01:24:41] 
[01:24:41] error: Unrecognized option: 'document-private-items'
[01:24:41] error: Unrecognized option: 'document-private-items'
[01:24:41] 
[01:24:41] error: Could not document `rustc_codegen_llvm`.
[01:24:41] 
[01:24:41] Caused by:
[01:24:41]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --crate-name rustc_codegen_llvm src/librustc_codegen_llvm/lib.rs --color always --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/doc -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern cc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libcc-c6f14bfbea71a5d2.rmeta --extern memmap=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libmemmap-f1ce0e7d8602fc1f.rmeta --extern num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libnum_cpus-2f51f01ec1efde74.rmeta --extern rustc_demangle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_demangle-d7f65c515e37a89e.rmeta --extern rustc_llvm=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_llvm-98200b85279ce65f.rmeta --document-private-items` (exit code: 1)
[01:24:42] [RUSTC-TIMING] rustc_target test:false 4.272
[01:24:44] error: build failed
[01:24:44] 
[01:24:44] 
[01:24:44] 
[01:24:44] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--no-deps" "-p" "rustc" "-p" "rustc_codegen_ssa" "-p" "syntax_pos" "-p" "rustc_apfloat" "-p" "rustc_mir" "-p" "graphviz" "-p" "serialize" "-p" "rustc_typeck" "-p" "rustc_errors" "-p" "rustc_plugin" "-p" "fmt_macros" "-p" "rustc_fs_util" "-p" "rustc_save_analysis" "-p" "rustc_target" "-p" "rustc_driver" "-p" "syntax_ext" "-p" "rustc_llvm" "-p" "rustc_data_structures" "-p" "rustc_allocator" "-p" "rustc_privacy" "-p" "rustc_borrowck" "-p" "syntax" "-p" "rustc_platform_intrinsics" "-p" "arena" "-p" "rustc_resolve" "-p" "rustc_passes" "-p" "rustc_codegen_llvm" "-p" "rustc_incremental" "-p" "rustc_metadata" "-p" "rustc_codegen_utils" "-p" "rustc_lint" "-p" "build_helper" "-p" "rustc_traits"
[01:24:44] 
[01:24:44] 
[01:24:44] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
[01:24:44] Build completed unsuccessfully in 1:19:46
---
travis_time:end:033626c3:start=1544737670285052753,finish=1544737670291532419,duration=6479666
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0059858a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:086109e2
travis_time:start:086109e2
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:15736908
$ dmesg | grep -i kill
