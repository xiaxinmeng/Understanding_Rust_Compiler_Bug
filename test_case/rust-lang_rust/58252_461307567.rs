plain
travis_time:end:0b29bffc:start=1549516593770273829,finish=1549516749488569797,duration=155718295968
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
[01:11:14] 
[01:11:14] running 119 tests
[01:11:42] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:11:47] i......iii.i.....ii
[01:11:47] 
[01:11:47]  finished in 33.299
[01:11:47] travis_fold:end:test_debuginfo

---
[01:35:17]      |
[01:35:17] 3549 |         use Decodable;
[01:35:17]      |             ^^^^^^^^^
[01:35:17] 
[01:35:17] error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
[01:35:17]      |
[01:35:17]      |
[01:35:17] 2603 |     #[derive(RustcDecodable, Eq, PartialEq, Debug)]
[01:35:17]      |              ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
[01:35:17] 
[01:35:17] error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
[01:35:17]      |
[01:35:17]      |
[01:35:17] 2630 |     #[derive(PartialEq, RustcEncodable, RustcDecodable, Debug)]
[01:35:17]      |                         ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
[01:35:17] 
[01:35:17] error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
[01:35:17]      |
[01:35:17]      |
[01:35:17] 2630 |     #[derive(PartialEq, RustcEncodable, RustcDecodable, Debug)]
[01:35:17]      |                                         ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
[01:35:17] 
[01:35:17] error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
[01:35:17]      |
[01:35:17]      |
[01:35:17] 2636 |     #[derive(PartialEq, RustcEncodable, RustcDecodable, Debug)]
[01:35:17]      |                         ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
[01:35:17] 
[01:35:17] error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
[01:35:17]      |
[01:35:17]      |
[01:35:17] 2636 |     #[derive(PartialEq, RustcEncodable, RustcDecodable, Debug)]
[01:35:17]      |                                         ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
[01:35:17] 
[01:35:17] error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
[01:35:17]      |
[01:35:17]      |
[01:35:17] 2643 |     #[derive(PartialEq, RustcEncodable, RustcDecodable, Debug)]
[01:35:17]      |                         ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
[01:35:17] 
[01:35:17] error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
[01:35:17]      |
[01:35:17]      |
[01:35:17] 2643 |     #[derive(PartialEq, RustcEncodable, RustcDecodable, Debug)]
[01:35:17]      |                                         ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
[01:35:17] 
[01:35:17] error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
[01:35:17]      |
[01:35:17]      |
[01:35:17] 3163 |     #[derive(RustcDecodable)]
[01:35:17]      |              ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
[01:35:17] 
[01:35:17] error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
[01:35:17]      |
[01:35:17]      |
[01:35:17] 3212 |     #[derive(RustcDecodable)]
[01:35:17]      |              ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
[01:35:17] 
[01:35:17] error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
[01:35:17]      |
[01:35:17]      |
[01:35:17] 3220 |     #[derive(RustcDecodable)]
[01:35:17]      |              ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
[01:35:17] 
[01:35:17] error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
[01:35:17]      |
[01:35:17]      |
[01:35:17] 3519 |         #[derive(RustcEncodable, Eq, Hash, PartialEq, RustcDecodable, Debug)]
[01:35:17]      |                  ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
[01:35:17] 
[01:35:17] error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
[01:35:17]      |
[01:35:17]      |
[01:35:17] 3519 |         #[derive(RustcEncodable, Eq, Hash, PartialEq, RustcDecodable, Debug)]
[01:35:17]      |                                                       ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
[01:35:17] 
[01:35:17] error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
[01:35:17]      |
[01:35:17]      |
[01:35:17] 3922 |         #[derive(PartialEq, Eq, Hash, RustcEncodable)]
[01:35:17]      |                                       ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
[01:35:17] 
[01:35:17] error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
[01:35:17]    --> src/libserialize/opaque.rs:335:39
[01:35:17]     |
[01:35:17] 335 |     #[derive(PartialEq, Clone, Debug, RustcEncodable, RustcDecodable)]
[01:35:17]     |                                       ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
[01:35:17] 
[01:35:17] error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
[01:35:17]    --> src/libserialize/opaque.rs:335:55
[01:35:17]     |
[01:35:17] 335 |     #[derive(PartialEq, Clone, Debug, RustcEncodable, RustcDecodable)]
[01:35:17]     |                                                       ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
[01:35:17] 
[01:35:17] error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
[01:35:17]    --> src/libserialize/opaque.rs:555:39
[01:35:17]     |
[01:35:17] 555 |     #[derive(PartialEq, Clone, Debug, RustcEncodable, RustcDecodable)]
[01:35:17]     |                                       ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
[01:35:17] 
[01:35:17] error[E0433]: failed to resolve: could not find `rustc_serialize` in `{{root}}`
[01:35:17]    --> src/libserialize/opaque.rs:555:55
[01:35:17]     |
[01:35:17] 555 |     #[derive(PartialEq, Clone, Debug, RustcEncodable, RustcDecodable)]
[01:35:17]     |                                                       ^^^^^^^^^^^^^^ could not find `rustc_serialize` in `{{root}}`
[01:35:17] error: hidden lifetime parameters in types are deprecated
[01:35:17]     --> src/libserialize/json.rs:3561:58
[01:35:17]      |
[01:35:17]      |
[01:35:17] 3561 |                            expected: Vec<(JsonEvent, Vec<StackElement>)>) {
[01:35:17] 
[01:35:17] error: aborting due to 20 previous errors
[01:35:17] 
[01:35:17] For more information about this error, try `rustc --explain E0433`.
[01:35:17] For more information about this error, try `rustc --explain E0433`.
[01:35:17] error: Could not compile `serialize`.
[01:35:17] 
[01:35:17] To learn more, run the command again with --verbose.
[01:35:17] 
[01:35:17] 
[01:35:17] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "serialize" "--" "--quiet"
[01:35:17] 
[01:35:17] 
[01:35:17] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:35:17] Build completed unsuccessfully in 0:36:25
[01:35:17] Build completed unsuccessfully in 0:36:25
[01:35:17] Makefile:48: recipe for target 'check' failed
[01:35:17] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0d6555ce
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Feb  7 06:54:37 UTC 2019
---
travis_time:end:013cb537:start=1549522479351607564,finish=1549522479357612807,duration=6005243
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0ef1c408
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$
