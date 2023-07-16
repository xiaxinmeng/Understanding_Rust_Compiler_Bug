plain
[01:26:25]              resulting code better than in the case of [`chunks_mut`].
[01:26:25]                                                         ^^^^^^^^^^^^
[01:26:25]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:26:25] 
[01:26:26] thread '<unnamed>' panicked at 'assertion failed: output.is_none()', src/librustdoc/clean/simplify.rs:108:21
[01:26:27] 
[01:26:27] error: internal compiler error: unexpected panic
[01:26:27] 
[01:26:27] error: Unrecognized option: 'document-private-items'
[01:26:27] error: Unrecognized option: 'document-private-items'
[01:26:27] 
[01:26:27] error: Could not document `rustc_codegen_llvm`.
[01:26:27] 
[01:26:27] Caused by:
[01:26:27]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --crate-name rustc_codegen_llvm src/librustc_codegen_llvm/lib.rs --color always --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/doc -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern cc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libcc-fb8a40aee05fd80e.rmeta --extern memmap=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libmemmap-79eb47eec50b34e8.rmeta --extern num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libnum_cpus-f6cf12e28c391422.rmeta --extern rustc_demangle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_demangle-38e75cf135dd3aa5.rmeta --extern rustc_llvm=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_llvm-9f98e6cde7a267cf.rmeta --document-private-items` (exit code: 1)
[01:26:31] [RUSTC-TIMING] syntax test:false 16.157
[01:26:31] error: build failed
[01:26:31] 
[01:26:31] 
[01:26:31] 
[01:26:31] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--no-deps" "-p" "rustc_metadata" "-p" "fmt_macros" "-p" "rustc_apfloat" "-p" "rustc_target" "-p" "rustc_allocator" "-p" "rustc_resolve" "-p" "rustc_platform_intrinsics" "-p" "rustc_llvm" "-p" "rustc_codegen_llvm" "-p" "rustc" "-p" "syntax_pos" "-p" "rustc_passes" "-p" "serialize" "-p" "rustc_codegen_ssa" "-p" "rustc_errors" "-p" "rustc_traits" "-p" "syntax" "-p" "rustc_driver" "-p" "rustc_borrowck" "-p" "rustc_codegen_utils" "-p" "rustc_plugin" "-p" "rustc_data_structures" "-p" "rustc_save_analysis" "-p" "arena" "-p" "rustc_incremental" "-p" "rustc_typeck" "-p" "rustc_fs_util" "-p" "rustc_lint" "-p" "rustc_privacy" "-p" "rustc_mir" "-p" "build_helper" "-p" "syntax_ext" "-p" "graphviz"
[01:26:31] 
[01:26:31] 
[01:26:31] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
[01:26:31] Build completed unsuccessfully in 1:21:46
---
travis_time:end:1122084e:start=1545249989113739658,finish=1545249989121415879,duration=7676221
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0326ab3f
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0d5e6505
travis_time:start:0d5e6505
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1ca224b3
$ dmesg | grep -i kill
