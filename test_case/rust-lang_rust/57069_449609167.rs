plain
travis_time:end:1eb06b4a:start=1545525159190793511,finish=1545525160305920687,duration=1115127176
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
[01:07:52] 
[01:07:52] running 118 tests
[01:08:15] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii..........i...ii...i.......ii.i.i.i 100/118
[01:08:19] ......iii.i.....ii
[01:08:19] 
[01:08:19]  finished in 27.440
[01:08:19] travis_fold:end:test_debuginfo

---
[01:28:26]    --> src/libfmt_macros/lib.rs:630:22
[01:28:26]     |
[01:28:26] 229 | /     pub fn new(
[01:28:26] 230 | |         s: &'a str,
[01:28:26] 231 | |         style: Option<usize>,
[01:28:26] 232 | |         skips: Vec<usize>,
[01:28:26] 245 | |         }
[01:28:26] 246 | |     }
[01:28:26]     | |_____- defined here
[01:28:26] ...
