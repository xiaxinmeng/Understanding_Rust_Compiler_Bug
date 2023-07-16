plain
travis_time:end:02202400:start=1555769211527969977,finish=1555769212279229061,duration=751259084
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:13:53] 
[01:13:53] running 9 tests
[01:13:53] iiiiiiiii
[01:13:53] 
[01:13:53]  finished in 0.155
[01:13:53] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:14:09] 
[01:14:09] running 121 tests
[01:14:35] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:14:39] i.i......iii.i.....ii
[01:14:39] 
[01:14:39]  finished in 30.219
[01:14:39] travis_fold:end:test_debuginfo

---
[01:24:35]    Compiling core v0.0.0 (/checkout/src/libcore)
[01:24:37] error: hidden lifetime parameters in types are deprecated
[01:24:37]    --> src/libcore/../libcore/tests/cell.rs:142:17
[01:24:37]     |
[01:24:37] 142 |         let b1: Ref<Option<u32>> = x.borrow();
[01:24:37]     |                 ^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `Ref<'_, Option<u32>>`
[01:24:37] error: hidden lifetime parameters in types are deprecated
[01:24:37]    --> src/libcore/../libcore/tests/cell.rs:146:21
[01:24:37]     |
[01:24:37]     |
[01:24:37] 146 |             let b2: Ref<u32> = Ref::map(b1, |o| o.as_ref().unwrap());
[01:24:37]     |                     ^^^^^^^^ help: indicate the anonymous lifetime: `Ref<'_, u32>`
[01:24:37] error: hidden lifetime parameters in types are deprecated
[01:24:37]    --> src/libcore/../libcore/tests/cell.rs:225:12
[01:24:37]     |
[01:24:37]     |
[01:24:37] 225 |     let d: Ref<u32> = x.accessor();
[01:24:37]     |            ^^^^^^^^ help: indicate the anonymous lifetime: `Ref<'_, u32>`
[01:24:37] error: hidden lifetime parameters in types are deprecated
[01:24:37]    --> src/libcore/../libcore/tests/cell.rs:220:31
[01:24:37]     |
[01:24:37]     |
[01:24:37] 220 |         fn accessor(&self) -> Ref<u32> {
[01:24:37]     |                               ^^^^^^^^ help: indicate the anonymous lifetime: `Ref<'_, u32>`
[01:24:37] error: hidden lifetime parameters in types are deprecated
[01:24:37]    --> src/libcore/../libcore/tests/cell.rs:239:20
[01:24:37]     |
[01:24:37]     |
[01:24:37] 239 |         let mut d: RefMut<u32> = x.accessor();
[01:24:37]     |                    ^^^^^^^^^^^ help: indicate the anonymous lifetime: `RefMut<'_, u32>`
[01:24:37] error: hidden lifetime parameters in types are deprecated
[01:24:37]    --> src/libcore/../libcore/tests/cell.rs:233:31
[01:24:37]     |
[01:24:37]     |
[01:24:37] 233 |         fn accessor(&self) -> RefMut<u32> {
[01:24:37]     |                               ^^^^^^^^^^^ help: indicate the anonymous lifetime: `RefMut<'_, u32>`
[01:24:37] error: hidden lifetime parameters in types are deprecated
[01:24:37]    --> src/libcore/../libcore/tests/cell.rs:336:26
[01:24:37]     |
[01:24:37]     |
[01:24:37] 336 |         let mut cellref: RefMut<[i32; 3]> = cell.borrow_mut();
[01:24:37]     |                          ^^^^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `RefMut<'_, [i32; 3]>`
[01:24:37] error: hidden lifetime parameters in types are deprecated
[01:24:37]    --> src/libcore/../libcore/tests/cell.rs:338:26
[01:24:37]     |
[01:24:37]     |
[01:24:37] 338 |         let mut coerced: RefMut<[i32]> = cellref;
[01:24:37]     |                          ^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `RefMut<'_, [i32]>`
[01:24:37] error: hidden lifetime parameters in types are deprecated
[01:24:37]    --> src/libcore/../libcore/tests/cell.rs:343:22
[01:24:37]     |
[01:24:37]     |
[01:24:37] 343 |         let cellref: Ref<[i32; 3]> = cell.borrow();
[01:24:37]     |                      ^^^^^^^^^^^^^ help: indicate the anonymous lifetime: `Ref<'_, [i32; 3]>`
[01:24:37] error: hidden lifetime parameters in types are deprecated
[01:24:37]    --> src/libcore/../libcore/tests/cell.rs:345:22
[01:24:37]     |
[01:24:37]     |
[01:24:37] 345 |         let coerced: Ref<[i32]> = cellref;
[01:24:37]     |                      ^^^^^^^^^^ help: indicate the anonymous lifetime: `Ref<'_, [i32]>`
[01:24:37] error: hidden lifetime parameters in types are deprecated
[01:24:37]  --> src/libcore/../libcore/tests/fmt/builders.rs:9:37
[01:24:37]   |
[01:24:37] 9 |             fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
---
[01:24:37] 
[01:24:37] error: hidden lifetime parameters in types are deprecated
[01:24:37]    --> src/libcore/../libcore/tests/iter.rs:575:32
[01:24:37]     |
[01:24:37] 575 | pub fn cycle<T>(data: &[T]) -> CycleIter<T> {
[01:24:37]     |                                ^^^^^^^^^^^^ help: indicate the anonymous lifetime: `CycleIter<'_, T>`
[01:24:49] error: unused extern crate
[01:24:49]   --> src/libcore/../libcore/tests/lib.rs:36:1
[01:24:49]    |
[01:24:49] 36 | extern crate core;
[01:24:49] 36 | extern crate core;
[01:24:49]    | ^^^^^^^^^^^^^^^^^^ help: remove it
[01:24:49]    |
[01:24:49]    = note: `-D unused-extern-crates` implied by `-D rust-2018-idioms`
[01:24:49] error: aborting due to 37 previous errors
[01:24:49] 
[01:24:49] error: Could not compile `core`.
[01:24:49] 
[01:24:49] 
[01:24:49] To learn more, run the command again with --verbose.
[01:24:49] 
[01:24:49] 
[01:24:49] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:24:49] 
[01:24:49] 
[01:24:49] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:24:49] Build completed unsuccessfully in 0:22:38
[01:24:49] Build completed unsuccessfully in 0:22:38
[01:24:49] make: *** [check] Error 1
[01:24:49] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:184d34fc
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Apr 20 15:31:52 UTC 2019
---
travis_time:end:0b3a4960:start=1555774314670690126,finish=1555774314676725493,duration=6035367
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:028cbe08
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1aa65de0
travis_time:start:1aa65de0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:10e82848
$ dmesg | grep -i kill
