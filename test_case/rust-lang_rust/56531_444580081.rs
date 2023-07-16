plain
[01:30:54] 
[01:30:54] error: Could not document `rustc_mir`.
[01:30:54] 
[01:30:54] Caused by:
[01:30:54]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --crate-name rustc_mir src/librustc_mir/lib.rs --color always --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/doc -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-b82b065d803ba059.rmeta --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-295e5ae5e71d1639.rmeta --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-152a8cf6f1a33e2b.rmeta --extern either=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libeither-2de610a4f3267a39.rmeta --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-20e044368fa7145d.rmeta --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-59b02de3b36a70e9.rmeta --extern log_settings=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog_settings-7c8b0529b1428b32.rmeta --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-417751586b0fd078.rmeta --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-80bf13e3a564cda2.rmeta --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-c7c0526e5f7aba39.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-6d02415bd0aa2a1d.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-c2ec1cd1ce56af47.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-087c5c29b2b1d529.rmeta --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-842d6678f47fdbed.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-330e25a268d4cfb5.rmeta --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-31ed2dad90fc1e74.rmeta --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-55f0802cdd0774d5.rmeta --document-private-items` (exit code: 1)
[01:30:54] warning: Backing out of syntax highlighting
[01:30:54]   |
[01:30:54]   = note: You probably did not intend to render this as a rust code-block
[01:30:54] 
---
[01:30:57] [RUSTC-TIMING] rustc_mir test:false 30.116
[01:30:57] error: build failed
[01:30:57] 
[01:30:57] 
[01:30:57] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--no-deps" "-p" "rustc_save_analysis" "-p" "syntax" "-p" "rustc_llvm" "-p" "serialize" "-p" "syntax_pos" "-p" "rustc_codegen_utils" "-p" "rustc" "-p" "rustc_traits" "-p" "arena" "-p" "rustc_privacy" "-p" "rustc_codegen_ssa" "-p" "rustc_allocator" "-p" "rustc_lint" "-p" "rustc_mir" "-p" "rustc_errors" "-p" "rustc_incremental" "-p" "rustc_passes" "-p" "rustc_platform_intrinsics" "-p" "rustc_resolve" "-p" "syntax_ext" "-p" "rustc_typeck" "-p" "rustc_codegen_llvm" "-p" "rustc_apfloat" "-p" "build_helper" "-p" "rustc_plugin" "-p" "rustc_borrowck" "-p" "rustc_fs_util" "-p" "rustc_data_structures" "-p" "rustc_metadata" "-p" "rustc_driver" "-p" "fmt_macros" "-p" "rustc_target" "-p" "graphviz"
[01:30:57] 
[01:30:57] 
[01:30:57] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
[01:30:57] Build completed unsuccessfully in 1:25:42
---
travis_time:end:1816aecb:start=1544032401672116524,finish=1544032401681992358,duration=9875834
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0c840484
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0c5e5798
travis_time:start:0c5e5798
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:03d15b30
$ dmesg | grep -i kill
