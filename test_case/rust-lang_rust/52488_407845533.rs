plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:01c3b798
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:56:06]    |
[00:56:06]    = note: #[warn(intra_doc_link_resolution_failure)] on by default
[00:56:06]    = help: to escape `[` and `]` characters, just add '\' before them like `\[` or `\]`
[00:56:06] 
[00:56:06] error: character literal may only contain one codepoint: '0
[00:56:06]   |
[00:56:06]   |
[00:56:06] 2 |    let y: &'0 u32 = x; // let's call this `'0`
[00:56:06] 
[00:56:06] warning: `[rayon::prelude]` cannot be resolved, ignoring it...
[00:56:06]   --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-rayon-0.1.1/src/lib.rs:45:17
[00:56:06]    |
---
[00:56:06] 
[00:56:06] error: Could not document `rustc_mir`.
[00:56:06] 
[00:56:06] Caused by:
[00:56:06]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustdoc --crate-name rustc_mir librustc_mir/lib.rs --target x86_64-unknown-linux-gnu -o /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/doc -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-0bd69ee9dc79ce46.rmeta --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-6ee8eacb3d8d570e.rmeta --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-933f9775f6ff92b4.rmeta --extern either=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libeither-9f83dad4e8a02083.rmeta --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-c0444a2db6d82ace.rmeta --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-0c8d7e70bf760e58.rmeta --extern log_settings=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/liblog_settings-9985e4269add965d.rmeta --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-3f89ea052dfccbf7.rmeta --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-7efe41410b14ae59.rmeta --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-35b60181bff7d40d.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-4002e4bed6df6f51.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-d318997c1dc2adeb.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-eda515317019a2da.rmeta --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-cf6fbd7507d87841.rmeta --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-b17b73e52ca7e26e.rmeta --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-fcb171cff9cde4aa.rmeta --document-private-items` (exit code: 1)
[00:56:15] error: build failed
[00:56:15] 
[00:56:15] 
[00:56:15] 
[00:56:15] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "doc" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--no-deps" "-p" "rustc_errors" "-p" "rustc" "-p" "rustc_data_structures" "-p" "rustc_lint" "-p" "rustc_allocator" "-p" "syntax_pos" "-p" "proc_macro" "-p" "rustc_platform_intrinsics" "-p" "rustc_driver" "-p" "rustc_typeck" "-p" "rustc_incremental" "-p" "rustc_save_analysis" "-p" "rustc_apfloat" "-p" "rustc_passes" "-p" "syntax" "-p" "rustc_borrowck" "-p" "rustc_mir" "-p" "fmt_macros" "-p" "graphviz" "-p" "rustc_traits" "-p" "rustc_resolve" "-p" "syntax_ext" "-p" "serialize" "-p" "rustc_codegen_utils" "-p" "rustc_target" "-p" "rustc_privacy" "-p" "arena" "-p" "rustc_metadata" "-p" "rustc_plugin"
[00:56:15] 
[00:56:15] 
[00:56:15] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu
[00:56:15] Build completed unsuccessfully in 0:49:56
---
travis_time:end:0b4e1f08:start=1532541696382644665,finish=1532541696390963678,duration=8319013
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:11b51632
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:090b41e3
travis_time:start:090b41e3
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:03c19cc1
$ dmesg | grep -i kill
