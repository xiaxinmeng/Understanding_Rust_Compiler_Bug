plain
travis_time:end:0cf42d8c:start=1550847506732434047,finish=1550847647656849386,duration=140924415339
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
#######################################################################   98.9%
######################################################################## 100.0%
[00:01:29] extracting /checkout/obj/build/cache/2019-02-17/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
[00:01:29]     Updating crates.io index
[00:01:41]     Updating git repository `https://github.com/Amanieu/hashbrown`
[00:01:42]   Downloaded filetime v0.2.4
[00:01:42]   Downloaded libc v0.2.46
[00:01:42]   Downloaded toml v0.4.10
[00:01:42]   Downloaded serde_derive v1.0.81
---
[00:04:09]    Compiling rustc-demangle v0.1.10
[00:04:10]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:04:14]    Compiling rustc-std-workspace-alloc v1.0.0 (/checkout/src/tools/rustc-std-workspace-alloc)
[00:04:14]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:04:14]    Compiling hashbrown v0.1.8 (https://github.com/Amanieu/hashbrown?branch=libstd#4f1cfb0a)
[00:04:31] Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:04:31] travis_fold:end:stage0-std

[00:04:31] travis_time:end:stage0-std:start=1550847892598802010,finish=1550847947562097407,duration=54963295397
---
[00:26:36]    Compiling rustc-demangle v0.1.10
[00:26:36]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:26:41]    Compiling rustc-std-workspace-alloc v1.0.0 (/checkout/src/tools/rustc-std-workspace-alloc)
[00:26:41]    Compiling panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
[00:26:41]    Compiling hashbrown v0.1.8 (https://github.com/Amanieu/hashbrown?branch=libstd#4f1cfb0a)
[00:27:02] Copying stage1 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[00:27:02] travis_fold:end:stage1-std

[00:27:02] travis_time:end:stage1-std:start=1550849231240092681,finish=1550849298840313806,duration=67600221125
---
[01:01:48]     Checking rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[01:01:48]     Checking rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[01:01:48]     Checking rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[01:01:48]     Checking rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[01:01:48]     Checking hashbrown v0.1.8 (https://github.com/Amanieu/hashbrown?branch=libstd#4f1cfb0a)
[01:02:07]     Finished release [optimized] target(s) in 24.97s
[01:02:07] Documenting stage2 test (x86_64-unknown-linux-gnu)
[01:02:08]     Checking proc_macro v0.0.0 (/checkout/src/libproc_macro)
[01:02:08]     Checking getopts v0.2.17
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:17:06] 
[01:17:06] running 119 tests
[01:17:31] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:17:35] i......iii.i.....ii
[01:17:35] 
[01:17:35]  finished in 28.736
[01:17:35] travis_fold:end:test_debuginfo

---
[01:32:44] travis_fold:start:test_stage1-std
travis_time:start:test_stage1-std
Testing std stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:32:44]    Compiling std v0.0.0 (/checkout/src/libstd)
[01:32:50] error[E0599]: no method named `raw_capacity` found for type `collections::hash::map::HashMap<_, _>` in the current scope
[01:32:50]      |
[01:32:50]      |
[01:32:50] 205  | pub struct HashMap<K, V, S = RandomState> {
[01:32:50]      | ----------------------------------------- method `raw_capacity` not found for this
[01:32:50] ...
[01:32:50] 3121 |         let old_raw_cap = m.raw_capacity();
[01:32:50]      |                             ^^^^^^^^^^^^ help: did you mean: `capacity`
[01:32:50] 
[01:32:50] error[E0599]: no method named `raw_capacity` found for type `collections::hash::map::HashMap<_, _>` in the current scope
[01:32:50]      |
[01:32:50]      |
[01:32:50] 205  | pub struct HashMap<K, V, S = RandomState> {
[01:32:50]      | ----------------------------------------- method `raw_capacity` not found for this
[01:32:50] ...
[01:32:50] 3122 |         while old_raw_cap == m.raw_capacity() {
[01:32:50]      |                                ^^^^^^^^^^^^ help: did you mean: `capacity`
[01:32:50] 
[01:32:50] error[E0599]: no method named `raw_capacity` found for type `collections::hash::map::HashMap<_, _>` in the current scope
[01:32:50]      |
[01:32:50]      |
[01:32:50] 205  | pub struct HashMap<K, V, S = RandomState> {
[01:32:50]      | ----------------------------------------- method `raw_capacity` not found for this
[01:32:50] ...
[01:32:50] 3136 |         assert_eq!(m.raw_capacity(), 1);
[01:32:50]      |                      ^^^^^^^^^^^^ help: did you mean: `capacity`
[01:32:50] 
[01:32:50] error[E0599]: no method named `raw_capacity` found for type `collections::hash::map::HashMap<{integer}, {integer}>` in the current scope
[01:32:50]      |
[01:32:50]      |
[01:32:50] 205  | pub struct HashMap<K, V, S = RandomState> {
[01:32:50]      | ----------------------------------------- method `raw_capacity` not found for this
[01:32:50] ...
[01:32:50] 3142 |         let initial_raw_cap = m.raw_capacity();
[01:32:50]      |                                 ^^^^^^^^^^^^ help: did you mean: `capacity`
[01:32:50] 
[01:32:50] error[E0599]: no method named `raw_capacity` found for type `collections::hash::map::HashMap<{integer}, {integer}>` in the current scope
[01:32:50]      |
[01:32:50]      |
[01:32:50] 205  | pub struct HashMap<K, V, S = RandomState> {
[01:32:50]      | ----------------------------------------- method `raw_capacity` not found for this
[01:32:50] ...
[01:32:50] 3144 |         let raw_cap = m.raw_capacity();
[01:32:50]      |                         ^^^^^^^^^^^^ help: did you mean: `capacity`
[01:32:50] 
[01:32:50] error[E0599]: no method named `raw_capacity` found for type `collections::hash::map::HashMap<usize, usize>` in the current scope
[01:32:50]      |
[01:32:50]      |
[01:32:50] 205  | pub struct HashMap<K, V, S = RandomState> {
[01:32:50]      | ----------------------------------------- method `raw_capacity` not found for this
[01:32:50] ...
[01:32:50] 3156 |         assert_eq!(m.raw_capacity(), raw_cap);
[01:32:50]      |                      ^^^^^^^^^^^^ help: did you mean: `capacity`
[01:32:50] 
[01:32:50] error[E0599]: no method named `raw_capacity` found for type `collections::hash::map::HashMap<usize, usize>` in the current scope
[01:32:50]      |
[01:32:50]      |
[01:32:50] 205  | pub struct HashMap<K, V, S = RandomState> {
[01:32:50]      | ----------------------------------------- method `raw_capacity` not found for this
[01:32:50] ...
[01:32:50] 3164 |         let new_raw_cap = m.raw_capacity();
[01:32:50]      |                             ^^^^^^^^^^^^ help: did you mean: `capacity`
[01:32:50] 
[01:32:50] error[E0599]: no method named `raw_capacity` found for type `collections::hash::map::HashMap<usize, usize>` in the current scope
[01:32:50]      |
[01:32:50]      |
[01:32:50] 205  | pub struct HashMap<K, V, S = RandomState> {
[01:32:50]      | ----------------------------------------- method `raw_capacity` not found for this
[01:32:50] ...
[01:32:50] 3170 |             assert_eq!(m.raw_capacity(), new_raw_cap);
[01:32:50]      |                          ^^^^^^^^^^^^ help: did you mean: `capacity`
[01:32:50] 
[01:32:50] error[E0599]: no method named `raw_capacity` found for type `collections::hash::map::HashMap<usize, usize>` in the current scope
[01:32:50]      |
[01:32:50]      |
[01:32:50] 205  | pub struct HashMap<K, V, S = RandomState> {
[01:32:50]      | ----------------------------------------- method `raw_capacity` not found for this
[01:32:50] ...
[01:32:50] 3174 |         assert_eq!(m.raw_capacity(), raw_cap);
[01:32:50]      |                      ^^^^^^^^^^^^ help: did you mean: `capacity`
[01:32:50] 
[01:32:50] error[E0599]: no method named `raw_capacity` found for type `collections::hash::map::HashMap<usize, usize>` in the current scope
[01:32:50]      |
[01:32:50]      |
[01:32:50] 205  | pub struct HashMap<K, V, S = RandomState> {
[01:32:50]      | ----------------------------------------- method `raw_capacity` not found for this
[01:32:50] ...
[01:32:50] 3184 |         assert_eq!(m.raw_capacity(), initial_raw_cap);
[01:32:50]      |                      ^^^^^^^^^^^^ help: did you mean: `capacity`
[01:32:56] error: aborting due to 10 previous errors
[01:32:56] 
[01:32:56] For more information about this error, try `rustc --explain E0599`.
[01:32:56] error: Could not compile `std`.
[01:32:56] error: Could not compile `std`.
[01:32:56] 
[01:32:56] To learn more, run the command again with --verbose.
[01:32:56] 
[01:32:56] 
[01:32:56] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "std" "--" "--quiet"
[01:32:56] 
[01:32:56] 
[01:32:56] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:32:56] Build completed unsuccessfully in 0:27:25
[01:32:56] Build completed unsuccessfully in 0:27:25
[01:32:56] make: *** [check] Error 1
[01:32:56] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1f97819a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Feb 22 16:34:13 UTC 2019
