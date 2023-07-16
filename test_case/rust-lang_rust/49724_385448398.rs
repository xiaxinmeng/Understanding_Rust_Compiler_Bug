plain
[00:19:45]    Compiling rustc_trans v0.0.0 (file:///checkout/src/librustc_trans)
[00:19:45]    Compiling cc v1.0.10
[00:19:45]    Compiling num_cpus v1.8.0
[00:19:48]    Compiling rustc_llvm v0.0.0 (file:///checkout/src/librustc_llvm)
[00:19:59] error[E0599]: no method named `start` found for type `std::ops::Range<u128>` in the current scope
[00:19:59]   --> librustc_trans/mir/place.rs:98:30
[00:19:59]    |
[00:19:59] 98 |                     if range.start() != range.end() {
[00:19:59]    |                              ^^^^^ field, not a method
[00:19:59]    |
[00:19:59]    = help: did you mean to write `range.start` instead of `range.start(...)`?
[00:19:59]    = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[00:19:59]    = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[00:19:59]            candidate #1: `use std::ops::RangeBounds;`
[00:19:59] 
[00:19:59] error[E0599]: no method named `end` found for type `std::ops::Range<u128>` in the current scope
[00:19:59]   --> librustc_trans/mir/place.rs:98:47
[00:19:59]    |
[00:19:59] 98 |                     if range.start() != range.end() {
[00:19:59]    |                                               ^^^ field, not a method
[00:19:59]    |
[00:19:59]    = help: did you mean to write `range.end` instead of `range.end(...)`?
[00:19:59]    = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[00:19:59]    = note: the following trait is implemented but not in scope, perhaps add a `use` for it:
[00:19:59]            candidate #1: `use std::ops::RangeBounds;`
[00:19:59] 
ld/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-af41b93aec4c1e93.rlib --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-5a636a569660b030.so --extern rustc_trans_utils=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_trans_utils-933c30d4c9d5f293.so --extern tempdir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libtempdir-0c7cf905e0cd6081.rlib --extern rustc_mir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_mir-b916cbd03439f0e3.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-cf2c0cb22fec89b8.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-cf2c0cb22fec89b8.rlib --extern rustc_platform_intrinsics=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_platform_intrinsics-72b23abc9e5791ba.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-40a8b14ac2445c60.so --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libflate2-1a0253e187e6500f.rlib --extern rustc_allocator=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_allocator-7434a28be6690a60.so --extern cc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libcc-58331467d9165b44.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-7cdf848b4ea36a34.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-6ed50b2d1f28e8a9.so --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-6870b7044c859a70.rlib --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-ab2cf718bbbe7ba7.rlib --extern rustc=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc-f44070055fa5d2f8.so --extern rustc_const_math=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_const_math-48257f9c510f796f.so --extern rustc_llvm=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_llvm-38c1456f766597c0.rlib --extern rustc_incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_incremental-7d314c6f6eeeb003.so --extern num_cpus=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libnum_cpus-ced35b4b5b9925d9.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-c12600360c64db1e/out -L native=2018
travis_time:end:027b2004:start=1525104861429005746,finish=1525104861492945353,duration=63939607

The command "date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
" exited with 0.
