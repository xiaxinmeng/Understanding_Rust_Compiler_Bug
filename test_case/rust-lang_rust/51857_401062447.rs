plain
[00:06:23]     Checking syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:07:44]     Checking rustc_typeck v0.0.0 (file:///checkout/src/librustc_typeck)
[00:07:44]     Checking rustc_mir v0.0.0 (file:///checkout/src/librustc_mir)
[00:07:46]     Checking rustc_allocator v0.0.0 (file:///checkout/src/librustc_allocator)
[00:07:46] error[E0432]: unresolved import `syntax::ast::FnHeader`
[00:07:46]   --> librustc_allocator/expand.rs:16:44
[00:07:46]    |
[00:07:46] 16 |         self, Arg, Attribute, Crate, Expr, FnHeader, Generics, Ident, Item, ItemKind,
[00:07:46]    |                                            ^^^^^^^^ no `FnHeader` in `ast`
[00:07:46] 
[00:07:46] error[E0433]: failed to resolve. Use of undeclared type or module `Constness`
[00:07:46]    --> librustc_allocator/expand.rs:207:27
[00:07:46]     |
[00:07:46] 207 |             dummy_spanned(Constness::NotConst),
[00:07:46]     |                           ^^^^^^^^^ Use of undeclared type or module `Constness`
[00:07:46] 
[00:07:46] error[E0422]: cannot find struct, variant or union type `NameAndSpan` in this scope
[00:07:46]    --> librustc_allocator/expand.rs:103:21
[00:07:46]     |
[00:07:46] 103 |             callee: NameAndSpan {
[00:07:46] help: possible candidates are found in other modules, you can import them into scope
[00:07:46]     |
[00:07:46]     |
[00:07:46] 11  | use syntax::codemap::NameAndSpan;
[00:07:46]     |
[00:07:46] 11  | use syntax_pos::NameAndSpan;
[00:07:46]     |
[00:07:46] 11  | use syntax_pos::hygiene::NameAndSpan;
[00:07:46] 
[00:07:46] 
[00:07:46] error[E0425]: cannot find function `dummy_spanned` in this scope
[00:07:46]    --> librustc_allocator/expand.rs:207:13
[00:07:46]     |
[00:07:46] 207 |             dummy_spanned(Constness::NotConst),
[00:07:46] help: possible candidates are found in other modules, you can import them into scope
[00:07:46]     |
[00:07:46] 11  | use syntax::codemap::dummy_spanned;
[00:07:46]     |
[00:07:46]     |
[00:07:46] 11  | use syntax::ext::quote::rt::dummy_spanned;
[00:07:46] 
[00:07:46] 
[00:07:46] error: unused import: `FnHeader`
[00:07:46]   --> librustc_allocator/expand.rs:16:44
[00:07:46]    |
[00:07:46] 16 |         self, Arg, Attribute, Crate, Expr, FnHeader, Generics, Ident, Item, ItemKind,
[00:07:46]    |
[00:07:46]    = note: `-D unused-imports` implied by `-D warnings`
[00:07:46] 
[00:07:46] error: aborting due to 5 previous errors
[00:07:46] error: aborting due to 5 previous errors
[00:07:46] 
[00:07:46] Some errors occurred: E0422, E0425, E0432, E0433.
[00:07:46] For more information about an error, try `rustc --explain E0422`.
[00:07:46] error: Could not compile `rustc_allocator`.
[00:07:46] 
[00:07:46] Caused by:
[00:07:46]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_allocator librustc_allocator/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,metadata -C prefer-dynamic -C opt-level=2 -C metadata=c469157163587070 -C extra-filename=-c469157163587070 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-9d29f545d0225581.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-1523773c8ffdbbe0.rmeta --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-b8a9671eea3447b6.rmeta --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-4ba6be0ac2ad7d43.rmeta --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-8684dc69a14122fa.rmeta --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-86c39ff3f662b7e2.rmeta -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-b694aac2c244ef25/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-0a0159574092a14e/out` (exit code: 101)
[00:08:17] error: build failed
[00:08:17] error: build failed
[00:08:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:08:17] expected success, got: exit code: 101
[00:08:17] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:08:17] travis_fold:end:stage0-rustc

[00:08:17] travis_time:end:stage0-rustc:start=1530197263689232096,finish=1530197416431247273,duration=152742015177

---
travis_time:end:22185d56:start=1530197417009698669,finish=1530197417018852833,duration=9154164
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1570fbe2
$ head -30 ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
head: cannot open ‘./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers’ for reading: No such file or directory
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:16ddc38e
$ dmesg | grep -i kill
