plain
travis_time:end:151e55a0:start=1557932370976576990,finish=1557932625412762032,duration=254436185042
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[00:00:00] +src/ci/init_repo.sh . /home/travis/rustsrc
[00:03:42] From https://github.com/rust-lang/rust
[00:03:42]  * branch                    beta       -> FETCH_HEAD
[00:03:42]  * branch                    master     -> FETCH_HEAD
[00:03:44]    f2951e6fd7f..7158ed9cbea  master     -> origin/master
[00:03:44] warning: 80ee94c1f8640e866840dbcafe76ccdcbfe20fd8:.gitmodules, multiple configurations found for 'submodule.src/llvm.url'. Skipping second one!
[00:03:44] warning: 80ee94c1f8640e866840dbcafe76ccdcbfe20fd8:.gitmodules, multiple configurations found for 'submodule.src/llvm.branch'. Skipping second one!
[00:03:44] warning: 80ee94c1f8640e866840dbcafe76ccdcbfe20fd8:.gitmodules, multiple configurations found for 'submodule.src/rust-installer.path'. Skipping second one!
[00:03:44] warning: 80ee94c1f8640e866840dbcafe76ccdcbfe20fd8:.gitmodules, multiple configurations found for 'submodule.src/rust-installer.url'. Skipping second one!
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:24:28] 
[01:24:28] running 9 tests
[01:24:28] iiiiiiiii
[01:24:28] 
[01:24:28]  finished in 0.160
[01:24:28] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:24:44] 
[01:24:44] running 121 tests
[01:25:11] .iiiii...i.....i..i...i..i.i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i. 100/121
[01:25:15] i.i......iii.i.....ii
[01:25:15] 
[01:25:15]  finished in 31.329
[01:25:15] travis_fold:end:test_debuginfo

---
[01:51:36] failures:
[01:51:36] 
[01:51:36] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0038::The_trait_cannot_contain_associated_constants (line 975) stdout ----
[01:51:36] thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
[01:51:36]   left: `ThreadId(542)`,
[01:51:36]  right: `ThreadId(543)`', src/librustc_data_structures/sync.rs:707:9
[01:51:36] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0038::The_trait_cannot_contain_associated_constants (line 975)' panicked at 'Some expected error codes were not found: ["E0038"]', src/librustdoc/test.rs:315:9
[01:51:36] 
[01:51:36] failures:
[01:51:36]     /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0038::The_trait_cannot_contain_associated_constants (line 975)
[01:51:36] 
---
[01:51:36] 
[01:51:36] 
[01:51:36] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:51:36] Build completed unsuccessfully in 0:39:03
[01:51:36] Makefile:48: recipe for target 'check' failed
[01:51:36] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:177d5a2c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Wed May 15 16:55:31 UTC 2019
