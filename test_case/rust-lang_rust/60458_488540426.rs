plain
travis_time:end:06a19a86:start=1556760968689770524,finish=1556761056723445100,duration=88033674576
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
[01:21:15] 
[01:21:15] running 9 tests
[01:21:15] iiiiiiiii
[01:21:15] 
[01:21:15]  finished in 0.157
[01:21:15] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:21:31] 
[01:21:31] running 121 tests
[01:21:57] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:22:02] i.i......iii.i.....ii
[01:22:02] 
[01:22:02]  finished in 30.490
[01:22:02] travis_fold:end:test_debuginfo

---
[01:32:22] travis_fold:start:test_stage1-core
travis_time:start:test_stage1-core
Testing core stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:32:22]    Compiling core v0.0.0 (/checkout/src/libcore)
[01:32:27] error[E0658]: use of unstable library feature 'debug_map_key_value': recently added, needs an RFC
[01:32:27]    --> src/libcore/../libcore/tests/fmt/builders.rs:229:22
[01:32:27]     |
[01:32:27] 229 |                     .key(&"bar").value(&true)
[01:32:27]     |
[01:32:27]     |
[01:32:27]     = help: add #![feature(debug_map_key_value)] to the crate attributes to enable
[01:32:27] 
[01:32:27] error[E0658]: use of unstable library feature 'debug_map_key_value': recently added, needs an RFC
[01:32:27]    --> src/libcore/../libcore/tests/fmt/builders.rs:229:34
[01:32:27]     |
[01:32:27] 229 |                     .key(&"bar").value(&true)
[01:32:27]     |
[01:32:27]     |
[01:32:27]     = help: add #![feature(debug_map_key_value)] to the crate attributes to enable
[01:32:27] 
[01:32:27] error[E0658]: use of unstable library feature 'debug_map_key_value': recently added, needs an RFC
[01:32:27]    --> src/libcore/../libcore/tests/fmt/builders.rs:263:22
[01:32:27]     |
[01:32:27] 263 |                     .key(&"bar").value(&true)
[01:32:27]     |
[01:32:27]     |
[01:32:27]     = help: add #![feature(debug_map_key_value)] to the crate attributes to enable
[01:32:27] 
[01:32:27] error[E0658]: use of unstable library feature 'debug_map_key_value': recently added, needs an RFC
[01:32:27]    --> src/libcore/../libcore/tests/fmt/builders.rs:263:34
[01:32:27]     |
[01:32:27] 263 |                     .key(&"bar").value(&true)
[01:32:27]     |
[01:32:27]     |
[01:32:27]     = help: add #![feature(debug_map_key_value)] to the crate attributes to enable
[01:32:27] 
[01:32:27] error[E0658]: use of unstable library feature 'debug_map_key_value': recently added, needs an RFC
[01:32:27]    --> src/libcore/../libcore/tests/fmt/builders.rs:264:22
[01:32:27]     |
[01:32:27] 264 |                     .key(&10).value(&format_args!("{}/{}", 10, 20))
[01:32:27]     |
[01:32:27]     |
[01:32:27]     = help: add #![feature(debug_map_key_value)] to the crate attributes to enable
[01:32:27] 
[01:32:27] error[E0658]: use of unstable library feature 'debug_map_key_value': recently added, needs an RFC
[01:32:27]    --> src/libcore/../libcore/tests/fmt/builders.rs:264:31
[01:32:27]     |
[01:32:27] 264 |                     .key(&10).value(&format_args!("{}/{}", 10, 20))
[01:32:27]     |
[01:32:27]     |
[01:32:27]     = help: add #![feature(debug_map_key_value)] to the crate attributes to enable
[01:32:27] 
[01:32:27] error[E0658]: use of unstable library feature 'debug_map_key_value': recently added, needs an RFC
[01:32:27]    --> src/libcore/../libcore/tests/fmt/builders.rs:330:22
[01:32:27]     |
[01:32:27] 330 |                     .key(&"bar")
[01:32:27]     |
[01:32:27]     |
[01:32:27]     = help: add #![feature(debug_map_key_value)] to the crate attributes to enable
[01:32:27] 
[01:32:27] error[E0658]: use of unstable library feature 'debug_map_key_value': recently added, needs an RFC
[01:32:27]    --> src/libcore/../libcore/tests/fmt/builders.rs:331:22
[01:32:27]     |
[01:32:27] 331 |                     .key(&"invalid")
[01:32:27]     |
[01:32:27]     |
[01:32:27]     = help: add #![feature(debug_map_key_value)] to the crate attributes to enable
[01:32:27] 
[01:32:27] error[E0658]: use of unstable library feature 'debug_map_key_value': recently added, needs an RFC
[01:32:27]    --> src/libcore/../libcore/tests/fmt/builders.rs:347:22
[01:32:27]     |
[01:32:27] 347 |                     .key(&"bar")
[01:32:27]     |
[01:32:27]     |
[01:32:27]     = help: add #![feature(debug_map_key_value)] to the crate attributes to enable
[01:32:27] 
[01:32:27] error[E0658]: use of unstable library feature 'debug_map_key_value': recently added, needs an RFC
[01:32:27]    --> src/libcore/../libcore/tests/fmt/builders.rs:363:22
[01:32:27]     |
[01:32:27] 363 |                     .value(&"invalid")
[01:32:27]     |
[01:32:27]     |
[01:32:27]     = help: add #![feature(debug_map_key_value)] to the crate attributes to enable
[01:32:27] 
[01:32:27] error[E0658]: use of unstable library feature 'debug_map_key_value': recently added, needs an RFC
[01:32:27]    --> src/libcore/../libcore/tests/fmt/builders.rs:364:22
[01:32:27]     |
[01:32:27] 364 |                     .key(&"bar")
[01:32:27]     |
[01:32:27]     |
[01:32:27]     = help: add #![feature(debug_map_key_value)] to the crate attributes to enable
[01:32:36] error: aborting due to 11 previous errors
[01:32:36] 
[01:32:36] For more information about this error, try `rustc --explain E0658`.
[01:32:36] error: Could not compile `core`.
[01:32:36] error: Could not compile `core`.
[01:32:36] 
[01:32:36] To learn more, run the command again with --verbose.
[01:32:36] 
[01:32:36] 
[01:32:36] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:32:36] 
[01:32:36] 
[01:32:36] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:32:36] Build completed unsuccessfully in 0:23:12
[01:32:36] Build completed unsuccessfully in 0:23:12
[01:32:36] make: *** [check] Error 1
[01:32:36] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:007c3715
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu May  2 03:10:23 UTC 2019
