plain
travis_time:end:002da92e:start=1550957233901543875,finish=1550957234886969035,duration=985425160
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
[01:13:03] 
[01:13:03] running 119 tests
[01:13:31] .iiiii...i.....i..i...i..i.i..i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i. 100/119
[01:13:36] i......iii.i.....ii
[01:13:36] 
[01:13:36]  finished in 32.613
[01:13:36] travis_fold:end:test_debuginfo

---
[01:40:43] 
[01:40:43] failures:
[01:40:43] 
[01:40:43] ---- /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0670 (line 11002) stdout ----
[01:40:43] error: expected one of `!`, `.`, `::`, `;`, `?`, `{`, `}`, or an operator, found `fn`
[01:40:43]   |
[01:40:43]   |
[01:40:43] 3 | async fn foo() {}
[01:40:43]   |       ^^ expected one of 8 possible tokens here
[01:40:43] 
[01:40:43] error[E0425]: cannot find value `async` in this scope
[01:40:43]   |
[01:40:43]   |
[01:40:43] 3 | async fn foo() {}
[01:40:43] 
[01:40:43] 
[01:40:43] thread '/checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0670 (line 11002)' panicked at 'Some expected error codes were not found: ["E0670"]', src/librustdoc/test.rs:356:9
[01:40:43] 
[01:40:43] 
[01:40:43] failures:
[01:40:43]     /checkout/obj/build/x86_64-unknown-linux-gnu/test/error-index.md - Rust_Compiler_Error_Index::E0670 (line 11002)
---
[01:40:43] 
[01:40:43] 
[01:40:43] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:40:43] Build completed unsuccessfully in 0:39:50
[01:40:43] Makefile:48: recipe for target 'check' failed
[01:40:43] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:327d5d8e
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sat Feb 23 23:08:10 UTC 2019
