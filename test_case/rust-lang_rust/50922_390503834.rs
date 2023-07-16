plain
[00:02:54]    Compiling cc v1.0.15
[00:02:54]    Compiling core v0.0.0 (file:///checkout/src/libcore)
[00:02:54]    Compiling build_helper v0.1.0 (file:///checkout/src/build_helper)
[00:02:54]    Compiling unwind v0.0.0 (file:///checkout/src/libunwind)
[00:02:56] error[E0432]: unresolved import `unicode::mapping_table`
[00:02:56]   --> libcore/char/mod.rs:61:14
[00:02:56] 61 | use unicode::mapping_table::Lookup;
[00:02:56] 61 | use unicode::mapping_table::Lookup;
[00:02:56]    |              ^^^^^^^^^^^^^ Could not find `mapping_table` in `unicode`
[00:02:56] 
[00:02:56] error[E0432]: unresolved import `unicode::mapping_table`
[00:02:56]   --> libcore/unicode/tables.rs:17:14
[00:02:56]    |
[00:02:56] 17 | use unicode::mapping_table::MappingTable;
[00:02:56]    |              ^^^^^^^^^^^^^ Could not find `mapping_table` in `unicode`
[00:02:56] 
[00:02:56] error[E0412]: cannot find type `CaseMappingIter` in this scope
[00:02:56]    --> libcore/char/mod.rs:436:24
[00:02:56]     |
[00:02:56] 436 | pub struct ToUppercase(CaseMappingIter);
[00:02:56] 
[00:02:56] 
[00:02:56] error[E0422]: cannot find struct, variant or union type `MappingTable` in this scope
[00:02:56]     --> libcore/unicode/tables.rs:1143:44
[00:02:56]      |
[00:02:56] 1143 |     const Lowercase: super::MappingTable = MappingTable {
[00:02:56] 
[00:02:56] 
[00:02:56] error[E0422]: cannot find struct, variant or union type `MappingTable` in this scope
[00:02:56]     --> libcore/unicode/tables.rs:1773:44
[00:02:56]      |
[00:02:56] 1773 |     const Uppercase: super::MappingTable = MappingTable {
[00:02:56] 
[00:02:56] 
[00:02:56] error[E0603]: constant `Lowercase` is private
[00:02:56]    --> libcore/char/methods.rs:782:21
[00:02:56]     |
[00:02:56] 782 |         ToLowercase(conversions::Lowercase.lookup(self))
[00:02:56] 
[00:02:56] 
[00:02:56] error[E0603]: constant `Uppercase` is private
[00:02:56]    --> libcore/char/methods.rs:868:21
[00:02:56]     |
[00:02:56] 868 |         ToUppercase(conversions::Uppercase.lookup(self))
[00:02:56] 
[00:02:56]    Compiling std v0.0.0 (file:///checkout/src/libstd)
[00:03:00]    Compiling compiler_builtins v0.0.0 (file:///checkout/src/rustc/compiler_builtins_shim)
[00:03:00]    Compiling cmake v0.1.30
---
[00:03:08] Caused by:
[00:03:08]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core libcore/lib.rs --color always --error-format json --crate-type lib --emit=dep-info,link -C opt-level=3 -C metadata=0d1ebef792b1d9ca -C extra-filename=-0d1ebef792b1d9ca --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/deps` (exit code: 101)
[00:03:08] warning: build failed, waiting for other jobs to finish...
[00:03:18] error: build failed
[00:03:18] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:18] expected success, got: exit code: 101
[00:03:18] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1091:9
[00:03:18] travis_fold:end:stage0-std

[00:03:18] travis_time:end:stage0-std:start=1526843051271625051,finish=1526843075740658075,duration=24469033024


[00:03:18] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:03:18] Build completed unsuccessfully in 0:00:25
[00:03:18] make: *** [tidy] Error 1
[00:03:18] Makefile:79: recipe for target 'tidy' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1c22feb8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
