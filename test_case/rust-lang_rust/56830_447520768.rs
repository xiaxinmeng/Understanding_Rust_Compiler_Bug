plain
[01:24:18]     = note: #[warn(intra_doc_link_resolution_failure)] on by default
[01:24:18]     = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[01:24:18] 
[01:24:21]  Documenting syntax v0.0.0 (/checkout/src/libsyntax)
[01:24:24] thread '<unnamed>' panicked at 'assertion failed: output.is_none()', src/librustdoc/clean/simplify.rs:108:21
[01:24:24] 
[01:24:24] error: internal compiler error: unexpected panic
[01:24:24] 
[01:24:24] error: Unrecognized option: 'document-private-items'
[01:24:24] error: Unrecognized option: 'document-private-items'
[01:24:24] 
[01:24:24] error: Could not document `rustc_codegen_llvm`.
[01:24:24] 
[01:24:24] Caused by:
[01:24:24]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --crate-name rustc_codegen_llvm src/librustc_codegen_llvm/lib.rs --color always --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/doc -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern cc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libcc-0680b0eaf52437b2.rmeta --extern memmap=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libmemmap-63e78290f037994b.rmeta --extern num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libnum_cpus-e95e5c640eac3a13.rmeta --extern rustc_demangle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_demangle-39fd5a7e85491bb6.rmeta --extern rustc_llvm=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_llvm-7f01c3999d8715e1.rmeta --document-private-items` (exit code: 1)
[01:24:28] warning: `[stable]` cannot be resolved, ignoring it...
[01:24:28]    --> src/libsyntax/attr/builtin.rs:121:22
[01:24:28]     |
[01:24:28] 121 | /// Represents the #[stable], #[unstable], #[rustc_{deprecated,const_unstable}] attributes.
---
[01:24:33] [RUSTC-TIMING] syntax test:false 15.342
[01:24:33] error: build failed
[01:24:33] 
[01:24:33] 
[01:24:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--no-deps" "-p" "build_helper" "-p" "rustc_resolve" "-p" "rustc_passes" "-p" "rustc_save_analysis" "-p" "rustc_codegen_utils" "-p" "serialize" "-p" "rustc_plugin" "-p" "syntax_ext" "-p" "rustc_metadata" "-p" "rustc_incremental" "-p" "rustc_fs_util" "-p" "rustc_mir" "-p" "rustc_driver" "-p" "rustc_allocator" "-p" "rustc_lint" "-p" "rustc_borrowck" "-p" "rustc_platform_intrinsics" "-p" "graphviz" "-p" "rustc_llvm" "-p" "rustc_apfloat" "-p" "rustc_traits" "-p" "rustc_typeck" "-p" "arena" "-p" "rustc_errors" "-p" "syntax" "-p" "rustc_codegen_llvm" "-p" "rustc" "-p" "rustc_target" "-p" "syntax_pos" "-p" "rustc_codegen_ssa" "-p" "rustc_data_structures" "-p" "fmt_macros" "-p" "rustc_privacy"
[01:24:33] 
[01:24:33] 
[01:24:34] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
[01:24:34] Build completed unsuccessfully in 1:19:54
---
travis_time:end:16216980:start=1544834958562822667,finish=1544834958570558822,duration=7736155
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:056bf8b6
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1a925ab0
travis_time:start:1a925ab0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:13fa375c
$ dmesg | grep -i kill
