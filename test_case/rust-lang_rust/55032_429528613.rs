plain
[00:37:58]    Compiling rustdoc v0.0.0 (/checkout/src/librustdoc)
[00:38:01] error[E0308]: mismatched types
[00:38:01]     --> librustdoc/clean/mod.rs:2635:24
[00:38:01]      |
[00:38:01] 2635 |                 if let Some(principal) = obj.principal() {
[00:38:01]      |                        ^^^^^^^^^^^^^^^ expected struct `rustc::ty::Binder`, found enum `std::option::Option`
[00:38:01]      |
[00:38:01]      = note: expected type `rustc::ty::Binder<rustc::ty::ExistentialTraitRef<'_>>`
[00:38:01] 
4057816 .
1798488 ./obj
1798448 ./obj/build
---
151484 ./obj/build/bootstrap/debug/incremental
151412 ./src/tools/clang
149120 ./src/llvm-emscripten/test
136028 ./obj/build/bootstrap/debug/incremental/bootstrap-3ivyub3ic2113
136024 ./obj/build/bootstrap/debug/incremental/bootstrap-3ivyub3ic2113/s-f5ohiaivx9-11os506-1vqgtfjdeq8xj
120432 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
118224 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
111072 ./src/llvm/test/CodeGen
107656 ./obj/build/x86_64-unknown-linux-gnu/stage0/lib/rustlib/x86_64-unknown-linux-gnu/codegen-backends
---
41756 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/rel425887465070062,duration=8750809
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1b3e6d38
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0ebc5d50
$ dmesg | grep -i kill
