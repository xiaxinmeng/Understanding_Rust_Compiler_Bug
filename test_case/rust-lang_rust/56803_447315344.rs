plain
[01:17:49] 
[01:17:49]  Documenting rustc_codegen_llvm v0.0.0 (/checkout/src/librustc_codegen_llvm)
[01:17:50] [RUSTC-TIMING] rustc_errors test:false 1.754
[01:17:51]  Documenting syntax v0.0.0 (/checkout/src/libsyntax)
[01:17:55] thread '<unnamed>' panicked at 'assertion failed: output.is_none()', src/librustdoc/clean/simplify.rs:108:21
[01:17:55] 
[01:17:55] error: internal compiler error: unexpected panic
[01:17:55] 
[01:17:55] error: Unrecognized option: 'document-private-items'
[01:17:55] error: Unrecognized option: 'document-private-items'
[01:17:55] 
[01:17:55] error: Could not document `rustc_codegen_llvm`.
[01:17:55] 
[01:17:55] Caused by:
[01:17:55]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --crate-name rustc_codegen_llvm src/librustc_codegen_llvm/lib.rs --color always --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/doc -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern cc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libcc-78014087a3fc60e4.rmeta --extern memmap=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libmemmap-a8ad7b43a562b016.rmeta --extern num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libnum_cpus-bdab322fe114a2e2.rmeta --extern rustc_demangle=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_demangle-9cc9c185e5855eb0.rmeta --extern rustc_llvm=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_llvm-c6100b2254ad59ec.rmeta --document-private-items` (exit code: 1)
[01:17:57] warning: `[stable]` cannot be resolved, ignoring it...
[01:17:57]    --> src/libsyntax/attr/builtin.rs:121:22
[01:17:57]     |
[01:17:57] 121 | /// Represents the #[stable], #[unstable], #[rustc_{deprecated,const_unstable}] attributes.
---
[01:18:04] [RUSTC-TIMING] syntax test:false 14.164
[01:18:04] error: build failed
[01:18:04] 
[01:18:04] 
[01:18:04] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--no-deps" "-p" "serialize" "-p" "syntax_pos" "-p" "syntax" "-p" "rustc_save_analysis" "-p" "arena" "-p" "rustc" "-p" "graphviz" "-p" "rustc_apfloat" "-p" "rustc_passes" "-p" "rustc_traits" "-p" "rustc_codegen_ssa" "-p" "rustc_metadata" "-p" "rustc_codegen_utils" "-p" "rustc_typeck" "-p" "rustc_platform_intrinsics" "-p" "rustc_fs_util" "-p" "fmt_macros" "-p" "rustc_driver" "-p" "rustc_llvm" "-p" "rustc_codegen_llvm" "-p" "rustc_allocator" "-p" "rustc_data_structures" "-p" "rustc_incremental" "-p" "build_helper" "-p" "rustc_lint" "-p" "rustc_borrowck" "-p" "rustc_target" "-p" "rustc_mir" "-p" "rustc_plugin" "-p" "rustc_resolve" "-p" "syntax_ext" "-p" "rustc_privacy" "-p" "rustc_errors"
[01:18:04] 
[01:18:04] 
[01:18:04] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
[01:18:04] Build completed unsuccessfully in 1:13:09
---
travis_time:end:1832a191:start=1544791651062123469,finish=1544791651070194870,duration=8071401
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0bf29c05
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:003daaf6
travis_time:start:003daaf6
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1b3b7b00
$ dmesg | grep -i kill
