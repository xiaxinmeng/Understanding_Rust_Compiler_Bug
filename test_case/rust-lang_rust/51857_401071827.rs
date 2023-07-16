plain
[00:07:51]     Checking syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:09:34]     Checking rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:09:34]     Checking rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:09:36]     Checking rustc_allocator v0.0.0 (file:///checkout/src/librustc_allocator)
[00:09:36] error[E0412]: cannot find type `Mac` in this scope
[00:09:36]    --> librustc_allocator/expand.rs:175:33
[00:09:36]     |
[00:09:36] 175 |     fn fold_mac(&mut self, mac: Mac) -> Mac {
[00:09:36] help: possible candidate is found in another module, you can import it into scope
[00:09:36]     |
[00:09:36] 11  | use syntax::ast::Mac;
[00:09:36]     |
[00:09:36]     |
[00:09:36] 
[00:09:36] error[E0412]: cannot find type `Mac` in this scope
[00:09:36]    --> librustc_allocator/expand.rs:175:41
[00:09:36]     |
[00:09:36] 175 |     fn fold_mac(&mut self, mac: Mac) -> Mac {
[00:09:36] help: possible candidate is found in another module, you can import it into scope
[00:09:36]     |
[00:09:36] 11  | use syntax::ast::Mac;
[00:09:36]     |
[00:09:36]     |
[00:09:36] 
[00:09:36] error[E0425]: cannot find function `dummy_spanned` in this scope
[00:09:36]    --> librustc_allocator/expand.rs:207:13
[00:09:36]     |
[00:09:36] 207 |             dummy_spanned(Constness::NotConst),
[00:09:36] help: possible candidates are found in other modules, you can import them into scope
[00:09:36]     |
[00:09:36] 11  | use syntax::codemap::dummy_spanned;
[00:09:36]     |
[00:09:36]     |
[00:09:36] 11  | use syntax::ext::quote::rt::dummy_spanned;
[00:09:36] 
[00:09:37] error: aborting due to 3 previous errors
[00:09:37] 
[00:09:37] Some errors occurred: E0412, E0425.
[00:09:37] Some errors occurred: E0412, E0425.
[00:09:37] For more information about an error, try `rustc --explain E0412`.
[00:09:37] error: Could not compile `rustc_allocator`.
[00:09:37] 
[00:09:37] Caused by:
[00:09:37]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_allocator librustc_allocator/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,metadata -C prefer-dynamic -C opt-level=2 -C metadata=c469157163587070 -C extra-filename=-c469157163587070 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-1523773c8ffdbbe0.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-9d29f545d0225581.rmeta --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-b8a9671eea3447b6.rmeta --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-4ba6be0ac2ad7d43.rmeta --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-86c39ff3f662b7e2.rmeta --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-8684dc69a14122fa.rmeta -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-b694aac2c244ef25/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-0a0159574092a14e/out` (exit code: 101)
[00:10:19] error: build failed
[00:10:19] error: build failed
[00:10:19] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:10:19] expected success, got: exit code: 101
[00:10:19] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:10:19] travis_fold:end:stage0-rustc

[00:10:19] travis_time:end:stage0-rustc:start=1530198421593105497,finish=1530198616874505720,duration=195281400223

---
travis_time:end:0db502a4:start=1530198617569609039,finish=1530198617581668230,duration=12059191
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:05c0daf8
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:2047c49f
$ dmesg | grep -i kill
