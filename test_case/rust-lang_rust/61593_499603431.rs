plain
[00:26:06]    Compiling autocfg v0.1.4
[00:26:07] error[E0433]: failed to resolve: use of undeclared type or module `core`
[00:26:07]    --> src/libcore/../stdsimd/crates/core_arch/src/powerpc/altivec.rs:288:14
[00:26:07]     |
[00:26:07] 286 | / macro_rules! impl_neg {
[00:26:07] 287 | |     ($s: ident : $zero: expr) => {
[00:26:07] 288 | |         impl core::ops::Neg for s_t_l!($s) {
[00:26:07]     | |              ^^^^ use of undeclared type or module `core`
[00:26:07] 289 | |             type Output = s_t_l!($s);
[00:26:07] 295 | |     };
[00:26:07] 296 | | }
[00:26:07] 296 | | }
[00:26:07]     | |_- in this expansion of `impl_neg!`
[00:26:07] 297 | 
[00:26:07] 298 |   impl_neg! { i8x16 : 0 }
[00:26:07] 
[00:26:07] error[E0433]: failed to resolve: use of undeclared type or module `core`
[00:26:07]    --> src/libcore/../stdsimd/crates/core_arch/src/powerpc/altivec.rs:288:14
[00:26:07]     |
[00:26:07]     |
[00:26:07] 286 | / macro_rules! impl_neg {
[00:26:07] 287 | |     ($s: ident : $zero: expr) => {
[00:26:07] 288 | |         impl core::ops::Neg for s_t_l!($s) {
[00:26:07]     | |              ^^^^ use of undeclared type or module `core`
[00:26:07] 289 | |             type Output = s_t_l!($s);
[00:26:07] 295 | |     };
[00:26:07] 296 | | }
[00:26:07] 296 | | }
[00:26:07]     | |_- in this expansion of `impl_neg!`
[00:26:07] ...
[00:26:07] 299 |   impl_neg! { i16x8 : 0 }
[00:26:07] 
[00:26:07] error[E0433]: failed to resolve: use of undeclared type or module `core`
[00:26:07]    --> src/libcore/../stdsimd/crates/core_arch/src/powerpc/altivec.rs:288:14
[00:26:07]     |
[00:26:07]     |
[00:26:07] 286 | / macro_rules! impl_neg {
[00:26:07] 287 | |     ($s: ident : $zero: expr) => {
[00:26:07] 288 | |         impl core::ops::Neg for s_t_l!($s) {
[00:26:07]     | |              ^^^^ use of undeclared type or module `core`
[00:26:07] 289 | |             type Output = s_t_l!($s);
[00:26:07] 295 | |     };
[00:26:07] 296 | | }
[00:26:07] 296 | | }
[00:26:07]     | |_- in this expansion of `impl_neg!`
[00:26:07] ...
[00:26:07] 300 |   impl_neg! { i32x4 : 0 }
[00:26:07] 
[00:26:07] error[E0433]: failed to resolve: use of undeclared type or module `core`
[00:26:07]    --> src/libcore/../stdsimd/crates/core_arch/src/powerpc/altivec.rs:288:14
[00:26:07]     |
[00:26:07]     |
[00:26:07] 286 | / macro_rules! impl_neg {
[00:26:07] 287 | |     ($s: ident : $zero: expr) => {
[00:26:07] 288 | |         impl core::ops::Neg for s_t_l!($s) {
[00:26:07]     | |              ^^^^ use of undeclared type or module `core`
[00:26:07] 289 | |             type Output = s_t_l!($s);
[00:26:07] 295 | |     };
[00:26:07] 296 | | }
[00:26:07] 296 | | }
[00:26:07]     | |_- in this expansion of `impl_neg!`
[00:26:07] ...
[00:26:07] 301 |   impl_neg! { f32x4 : 0f32 }
[00:26:07] 
[00:26:08]    Compiling backtrace v0.3.29
[00:26:10] error: aborting due to 4 previous errors
[00:26:10] 
---
travis_time:end:0648aee1:start=1559844076387520818,finish=1559844076406851390,duration=19330572
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:06769039
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:07469379
travis_time:start:07469379
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:053921e2
$ dmesg | grep -i kill
