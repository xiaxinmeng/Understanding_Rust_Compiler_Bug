plain
travis_time:end:031699f4:start=1545001132271443988,finish=1545001133361084262,duration=1089640274
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:54:58] 
[00:54:58] running 121 tests
[00:55:01] i..ii...iii..iiii.....i...i..........i..iii.............i.....i......ii...i..i.ii..............i...i 100/121
[00:55:01] i..ii.i.....iiii.....
[00:55:01] 
[00:55:01]  finished in 3.561
[00:55:01] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[00:55:17] 
[00:55:17] running 119 tests
[00:55:42] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii.........i.i...ii...i.......ii.i.i. 100/119
[00:55:47] i......iii.i.....ii
[00:55:47] 
[00:55:47]  finished in 29.740
[00:55:47] travis_fold:end:test_debuginfo

---
[01:27:26] 
[01:27:26] 12 3 | no
[01:27:26] 13   | ^^ not found in this scope
[01:27:26] 14 
[01:27:26] - thread '$DIR/failed-doctest-output.rs - OtherStruct (line 27)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:326:13
[01:27:26] + thread '$DIR/failed-doctest-output.rs - OtherStruct (line 27)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:331:13
[01:27:26] 17 
[01:27:26] 17 
[01:27:26] 18 ---- $DIR/failed-doctest-output.rs - SomeStruct (line 21) stdout ----
[01:27:26] 
[01:27:26] 21 thread 'main' panicked at 'oh no', $DIR/failed-doctest-output.rs:3:1
[01:27:26] 23 
[01:27:26] - ', src/librustdoc/test.rs:361:17
[01:27:26] + ', src/librustdoc/test.rs:366:17
[01:27:26] 25 
[01:27:26] 25 
[01:27:26] 26 
[01:27:26] 27 failures:
[01:27:26] 
[01:27:26] 
[01:27:26] The actual stdout differed from the expected stdout.
[01:27:26] Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/failed-doctest-output.stdout
[01:27:26] To update references, rerun the tests and pass the `--bless` flag
3328908 ./obj
3313596 ./obj/build
2473504 ./obj/build/x86_64-unknown-linux-gnu
1119404 ./src
---
182824 ./obj/build/x86_64-unknown-linux-gnu/test/ui
153280 ./src/tools/clang
148664 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc
144288 ./obj/build/bootstrap/debug/incremental/bootstrap-2x7szixskz2uj
144284 ./obj/build/bootstrap/debug/incremental/bootstrap-2x7szixskz2uj/s-f7nnrvz83k-1ewg3cw-3qov0d07xkydp
140204 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
140200 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
133684 ./obj/build/x86_64-unknown-linux-gnu/stage1-rus
