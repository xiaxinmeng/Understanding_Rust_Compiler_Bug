plain
[00:25:54]    Compiling autocfg v0.1.4
[00:25:55] error[E0433]: failed to resolve: use of undeclared type or module `core`
[00:25:55]    --> src/libcore/../stdsimd/crates/core_arch/src/powerpc/altivec.rs:288:14
[00:25:55]     |
[00:25:55] 286 | / macro_rules! impl_neg {
[00:25:55] 287 | |     ($s: ident : $zero: expr) => {
[00:25:55] 288 | |         impl core::ops::Neg for s_t_l!($s) {
[00:25:55]     | |              ^^^^ use of undeclared type or module `core`
[00:25:55] 289 | |             type Output = s_t_l!($s);
[00:25:55] 295 | |     };
[00:25:55] 296 | | }
[00:25:55] 296 | | }
[00:25:55]     | |_- in this expansion of `impl_neg!`
[00:25:55] 297 | 
[00:25:55] 298 |   impl_neg! { i8x16 : 0 }
[00:25:55] 
[00:25:55] error[E0433]: failed to resolve: use of undeclared type or module `core`
[00:25:55]    --> src/libcore/../stdsimd/crates/core_arch/src/powerpc/altivec.rs:288:14
[00:25:55]     |
[00:25:55]     |
[00:25:55] 286 | / macro_rules! impl_neg {
[00:25:55] 287 | |     ($s: ident : $zero: expr) => {
[00:25:55] 288 | |         impl core::ops::Neg for s_t_l!($s) {
[00:25:55]     | |              ^^^^ use of undeclared type or module `core`
[00:25:55] 289 | |             type Output = s_t_l!($s);
[00:25:55] 295 | |     };
[00:25:55] 296 | | }
[00:25:55] 296 | | }
[00:25:55]     | |_- in this expansion of `impl_neg!`
[00:25:55] ...
[00:25:55] 299 |   impl_neg! { i16x8 : 0 }
[00:25:55] 
[00:25:55] error[E0433]: failed to resolve: use of undeclared type or module `core`
[00:25:55]    --> src/libcore/../stdsimd/crates/core_arch/src/powerpc/altivec.rs:288:14
[00:25:55]     |
[00:25:55]     |
[00:25:55] 286 | / macro_rules! impl_neg {
[00:25:55] 287 | |     ($s: ident : $zero: expr) => {
[00:25:55] 288 | |         impl core::ops::Neg for s_t_l!($s) {
[00:25:55]     | |              ^^^^ use of undeclared type or module `core`
[00:25:55] 289 | |             type Output = s_t_l!($s);
[00:25:55] 295 | |     };
[00:25:55] 296 | | }
[00:25:55] 296 | | }
[00:25:55]     | |_- in this expansion of `impl_neg!`
[00:25:55] ...
[00:25:55] 300 |   impl_neg! { i32x4 : 0 }
[00:25:55] 
[00:25:55] error[E0433]: failed to resolve: use of undeclared type or module `core`
[00:25:55]    --> src/libcore/../stdsimd/crates/core_arch/src/powerpc/altivec.rs:288:14
[00:25:55]     |
[00:25:55]     |
[00:25:55] 286 | / macro_rules! impl_neg {
[00:25:55] 287 | |     ($s: ident : $zero: expr) => {
[00:25:55] 288 | |         impl core::ops::Neg for s_t_l!($s) {
[00:25:55]     | |              ^^^^ use of undeclared type or module `core`
[00:25:55] 289 | |             type Output = s_t_l!($s);
[00:25:55] 295 | |     };
[00:25:55] 296 | | }
[00:25:55] 296 | | }
[00:25:55]     | |_- in this expansion of `impl_neg!`
[00:25:55] ...
[00:25:55] 301 |   impl_neg! { f32x4 : 0f32 }
[00:25:55] 
[00:25:56]    Compiling backtrace v0.3.29
[00:25:58] error: aborting due to 4 previous errors
[00:25:58] 
---
travis_time:end:22b23230:start=1559835830054356504,finish=1559835830061972603,duration=7616099
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:11f45d2f
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1326e9b8
travis_time:start:1326e9b8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0bc15940
$ dmesg | grep -i kill
