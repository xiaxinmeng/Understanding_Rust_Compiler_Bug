plain
travis_time:end:0454c1a1:start=1553874358648060511,finish=1553874359535441079,duration=887380568
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
[01:20:38] 
[01:20:38] running 9 tests
[01:20:38] iiiiiiiii
[01:20:38] 
[01:20:38]  finished in 0.154
[01:20:38] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:20:54] 
[01:20:54] running 120 tests
[01:21:21] .iiiii...i.....i..i...i..i.i...i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i 100/120
[01:21:25] .i......iii.i.....ii
[01:21:25] 
[01:21:25]  finished in 31.018
[01:21:25] travis_fold:end:test_debuginfo

---
[01:48:23] failures:
[01:48:23] 
[01:48:23] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0038::Method_has_generic_type_parameters (line 911) stdout ----
[01:48:23] thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
[01:48:23]   left: `ThreadId(463)`,
[01:48:23]  right: `ThreadId(462)`', src/librustc_data_structures/sync.rs:707:9
[01:48:23] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:48:23] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0038::Method_has_generic_type_parameters (line 911)' panicked at 'Some expected error codes were not found: ["E0038"]', src/librustdoc/test.rs:315:9
[01:48:23] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0038::The_trait_cannot_contain_associated_constants (line 975) stdout ----
[01:48:23] thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
[01:48:23]   left: `ThreadId(551)`,
[01:48:23]   left: `ThreadId(551)`,
[01:48:23]  right: `ThreadId(553)`', src/librustc_data_structures/sync.rs:707:9
[01:48:23] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0038::The_trait_cannot_contain_associated_constants (line 975)' panicked at 'Some expected error codes were not found: ["E0038"]', src/librustdoc/test.rs:315:9
[01:48:23] 
[01:48:23] failures:
[01:48:23]     /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0038::Method_has_generic_type_parameters (line 911)
[01:48:23]     /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0038::The_trait_cannot_contain_associated_constants (line 975)
---
[01:48:23] 
[01:48:23] 
[01:48:23] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:48:23] Build completed unsuccessfully in 0:39:48
[01:48:23] make: *** [check] Error 1
[01:48:23] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:117f0cf8
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Mar 29 17:34:33 UTC 2019
