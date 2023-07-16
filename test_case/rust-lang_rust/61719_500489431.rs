plain
travis_time:end:09302cf4:start=1560178630403447032,finish=1560178719651019857,duration=89247572825
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:47] 
[01:05:47] running 144 tests
[01:05:50] i..iii.....iii..iiii.....i......................i...i................i.....i..........ii.i..i..i.ii. 100/144
[01:05:52] test result: ok. 114 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:05:52] 
[01:05:52]  finished in 4.714
[01:05:52] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:54] 
[01:05:54] running 9 tests
[01:05:54] iiiiiiiii
[01:05:54] 
[01:05:54]  finished in 0.149
[01:05:54] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:10] 
[01:06:10] running 122 tests
[01:06:36] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:06:41] .i.i......iii.i.....ii
[01:06:41] 
[01:06:41]  finished in 30.729
[01:06:41] travis_fold:end:test_debuginfo

