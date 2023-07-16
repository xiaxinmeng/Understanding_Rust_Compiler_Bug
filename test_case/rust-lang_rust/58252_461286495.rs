plain
travis_time:end:1d9e3c99:start=1549508790317961830,finish=1549508791273175733,duration=955213903
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:12:30] 
[01:12:30] running 119 tests
[01:12:59] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:13:03] i......iii.i.....ii
[01:13:03] 
[01:13:03]  finished in 33.533
[01:13:03] travis_fold:end:test_debuginfo

---
[01:36:32] 
[01:36:32] error[E0432]: unresolved import `json`
[01:36:32]     --> src/libserialize/json.rs:3518:13
[01:36:32]      |
[01:36:32] 3518 |         use json;
[01:36:32]      |             ^^^^ no `json` external crate
[01:36:32] 
[01:36:32] error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
[01:36:32]      |
[01:36:32]      |
[01:36:32] 2603 |     #[derive(RustcDecodable, Eq, PartialEq, Debug)]
[01:36:32]      |              ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
[01:36:32] 
[01:36:32] error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
[01:36:32]      |
[01:36:32]      |
[01:36:32] 2630 |     #[derive(PartialEq, RustcEncodable, RustcDecodable, Debug)]
[01:36:32]      |                         ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
[01:36:32] 
[01:36:32] error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
[01:36:32]      |
[01:36:32]      |
[01:36:32] 2630 |     #[derive(PartialEq, RustcEncodable, RustcDecodable, Debug)]
[01:36:32]      |                                         ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
[01:36:32] 
[01:36:32] error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
[01:36:32]      |
[01:36:32]      |
[01:36:32] 2636 |     #[derive(PartialEq, RustcEncodable, RustcDecodable, Debug)]
[01:36:32]      |                         ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
[01:36:32] 
[01:36:32] error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
[01:36:32]      |
[01:36:32]      |
[01:36:32] 2636 |     #[derive(PartialEq, RustcEncodable, RustcDecodable, Debug)]
[01:36:32]      |                                         ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
[01:36:32] 
[01:36:32] error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
[01:36:32]      |
[01:36:32]      |
[01:36:32] 2643 |     #[derive(PartialEq, RustcEncodable, RustcDecodable, Debug)]
[01:36:32]      |                         ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
[01:36:32] 
[01:36:32] error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
[01:36:32]      |
[01:36:32]      |
[01:36:32] 2643 |     #[derive(PartialEq, RustcEncodable, RustcDecodable, Debug)]
[01:36:32]      |                                         ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
[01:36:32] 
[01:36:32] error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
[01:36:32]      |
[01:36:32]      |
[01:36:32] 3163 |     #[derive(RustcDecodable)]
[01:36:32]      |              ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
[01:36:32] 
[01:36:32] error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
[01:36:32]      |
[01:36:32]      |
[01:36:32] 3212 |     #[derive(RustcDecodable)]
[01:36:32]      |              ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
[01:36:32] 
[01:36:32] error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
[01:36:32]      |
[01:36:32]      |
[01:36:32] 3220 |     #[derive(RustcDecodable)]
[01:36:32]      |              ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
[01:36:32] 
[01:36:32] error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
[01:36:32]      |
[01:36:32]      |
[01:36:32] 3519 |         #[derive(RustcEncodable, Eq, Hash, PartialEq, RustcDecodable, Debug)]
[01:36:32]      |                  ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
[01:36:32] 
[01:36:32] error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
[01:36:32]      |
[01:36:32]      |
[01:36:32] 3519 |         #[derive(RustcEncodable, Eq, Hash, PartialEq, RustcDecodable, Debug)]
[01:36:32]      |                                                       ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
[01:36:32] 
[01:36:32] error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
[01:36:32]      |
[01:36:32]      |
[01:36:32] 3922 |         #[derive(PartialEq, Eq, Hash, RustcEncodable)]
[01:36:32]      |                                       ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
[01:36:32] 
[01:36:32] error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
[01:36:32]    --> src/libserialize/opaque.rs:335:39
[01:36:32]     |
[01:36:32] 335 |     #[derive(PartialEq, Clone, Debug, RustcEncodable, RustcDecodable)]
[01:36:32]     |                                       ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
[01:36:32] 
[01:36:32] error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
[01:36:32]    --> src/libserialize/opaque.rs:335:55
[01:36:32]     |
[01:36:32] 335 |     #[derive(PartialEq, Clone, Debug, RustcEncodable, RustcDecodable)]
[01:36:32]     |                                                       ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
[01:36:32] 
[01:36:32] error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
[01:36:32]    --> src/libserialize/opaque.rs:555:39
[01:36:32]     |
[01:36:32] 555 |     #[derive(PartialEq, Clone, Debug, RustcEncodable, RustcDecodable)]
[01:36:32]     |                                       ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
[01:36:32] 
[01:36:32] error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
[01:36:32]    --> src/libserialize/opaque.rs:555:55
[01:36:32]     |
[01:36:32] 555 |     #[derive(PartialEq, Clone, Debug, RustcEncodable, RustcDecodable)]
[01:36:32]     |                                                       ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
[01:36:32] error: hidden lifetime parameters in types are deprecated
[01:36:32]     --> src/libserialize/json.rs:3561:58
[01:36:32]      |
[01:36:32]      |
[01:36:32] 3561 |                            expected: Vec<(JsonEvent, Vec<StackElement>)>) {
[01:36:32] 
[01:36:33] error: aborting due to 21 previous errors
[01:36:33] 
[01:36:33] Some errors occurred: E0432, E0433.
[01:36:33] Some errors occurred: E0432, E0433.
[01:36:33] For more information about an error, try `rustc --explain E0432`.
[01:36:33] error: Could not compile `serialize`.
[01:36:33] 
[01:36:33] To learn more, run the command again with --verbose.
[01:36:33] 
[01:36:33] 
[01:36:33] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "serialize" "--" "--quiet"
[01:36:33] 
[01:36:33] 
[01:36:33] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:36:33] Build completed unsuccessfully in 0:36:29
[01:36:33] Build completed unsuccessfully in 0:36:29
[01:36:33] Makefile:48: recipe for target 'check' failed
[01:36:33] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0d4cd5c8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Feb  7 04:43:14 UTC 2019
---
travis_time:end:0713cb5c:start=1549514597119581798,finish=1549514597179836696,duration=60254898
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:05a5cd00
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00eca450
$ dmesg | grep -i kill
