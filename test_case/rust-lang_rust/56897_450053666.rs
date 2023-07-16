plain
travis_time:end:02986bcc:start=1545869689405673940,finish=1545869691578997872,duration=2173323932
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
[01:06:24] 
[01:06:24] running 118 tests
[01:06:47] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii..........i...ii...i.......ii.i.i.i 100/118
[01:06:51] ......iii.i.....ii
[01:06:51] 
[01:06:51]  finished in 26.640
[01:06:51] travis_fold:end:test_debuginfo

---
[01:35:03] 
[01:35:03] 12 3 | no
[01:35:03] 13   | ^^ not found in this scope
[01:35:03] 14 
[01:35:03] - thread '$DIR/failed-doctest-output.rs - OtherStruct (line 27)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:331:13
[01:35:03] + thread '$DIR/failed-doctest-output.rs - OtherStruct (line 17)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:321:13
[01:35:03] 17 
[01:35:03] 17 
[01:35:03] 18 ---- $DIR/failed-doctest-output.rs - SomeStruct (line 11) stdout ----
[01:35:03] 
[01:35:03] 21 thread 'main' panicked at 'oh no', $DIR/failed-doctest-output.rs:3:1
[01:35:03] 23 
[01:35:03] - ', src/librustdoc/test.rs:366:17
[01:35:03] + ', src/librustdoc/test.rs:356:17
[01:35:03] 25 
[01:35:03] 25 
[01:35:03] 26 
[01:35:03] 27 failures:
[01:35:03] 
[01:35:03] 
[01:35:03] The actual stdout differed from the expected stdout.
[01:35:03] Actual stdout saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/failed-doctest-output/failed-doctest-output.stdout
[01:35:03] To update references, rerun the tests and pass the `--bless` flag
[travis_time:end:19591c20:start=1545869699710454534,finish=1545875403153125169,duration=5703442670635
travis_time:start:0dcb7a94
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu Dec 27 01:50:03 UTC 2018
Thu, 27 Dec 2018 01:50:03 GMT
