plain
travis_time:end:27a467f2:start=1554749311199409299,finish=1554749313407358025,duration=2207948726
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:12:49] 
[01:12:49] running 9 tests
[01:12:49] iiiiiiiii
[01:12:49] 
[01:12:49]  finished in 0.152
[01:12:49] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:13:05] 
[01:13:05] running 121 tests
[01:13:30] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:13:34] i.i......iii.i.....ii
[01:13:34] 
[01:13:34]  finished in 29.723
[01:13:34] travis_fold:end:test_debuginfo

---
[01:39:18] failures:
[01:39:18] 
[01:39:18] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0038::The_trait_cannot_contain_associated_constants (line 975) stdout ----
[01:39:18] thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
[01:39:18]   left: `ThreadId(560)`,
[01:39:18]  right: `ThreadId(559)`', src/librustc_data_structures/sync.rs:707:9
[01:39:18] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0038::The_trait_cannot_contain_associated_constants (line 975)' panicked at 'Some expected error codes were not found: ["E0038"]', src/librustdoc/test.rs:315:9
[01:39:18] 
[01:39:18] failures:
[01:39:18]     /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0038::The_trait_cannot_contain_associated_constants (line 975)
[01:39:18] 
---
[01:39:18] 
[01:39:18] 
[01:39:18] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:39:18] Build completed unsuccessfully in 0:38:16
[01:39:18] make: *** [check] Error 1
[01:39:18] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2c93c930
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Apr  8 20:28:01 UTC 2019
