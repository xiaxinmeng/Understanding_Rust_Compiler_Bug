plain
travis_time:end:0c8d9641:start=1546215766170312280,finish=1546215767164664004,duration=994351724
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
[01:07:23] 
[01:07:23] running 118 tests
[01:07:45] .iiiii...i.....i..i...i..i.i..i.ii..i.....i..i....i..........iiii..........i...ii...i.......ii.i.i.i 100/118
[01:07:49] ......iii.i.....ii
[01:07:49] 
[01:07:49]  finished in 26.850
[01:07:49] travis_fold:end:test_debuginfo

---
[01:19:37] 
[01:19:37]    Doc-tests core
[01:19:42] 
[01:19:42] running 2229 tests
[01:19:52] ......iiiii.........................................................................F............... 100/2229
[01:20:15] .................................................................................................... 300/2229
[01:20:28] .......................................................i............................................ 400/2229
[01:20:39] .................................................................................................... 500/2229
[01:20:50] .................................................................................................... 600/2229
---
[01:22:52] .................................................................................................... 1700/2229
[01:23:04] .................................................................................................... 1800/2229
[01:23:17] .................................................................................................... 1900/2229
[01:23:29] .................................................................................................... 2000/2229
[01:23:43] ................................................................................F................... 2100/2229
[01:23:58] .......F....................................i....................................................... 2200/2229
stable library feature 'eq_ignore_case' (see issue #57221)
[01:24:01]  --> char/methods.rs:903:13
[01:24:01]   |
[01:24:01] 8 | assert!('ö'.eq_ignore_case(&'Ö'));
[01:24:01]   |
[01:24:01]   |
[01:24:01]   = help: add #![feature(eq_ignore_case)] to the crate attributes to enable
[01:24:01] thread 'char/methods.rs - char::methods::char::eq_ignore_case (line 898)' panicked at 'couldn't compile the test', src/librustdoc/test.rs:316:13
[01:24:01] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:24:01] 
[01:24:01] ---- str/mod.rs - str::Chars<'a>::eq_ignore_case (line 639) stdout ----
[01:24:01] ---- str/mod.rs - str::Chars<'a>::eq_ignore_case (line 639) stdout ----
[01:24:01] error[E0658]: use of unstable library feature 'eq_ignore_case' (see issue #57221)
[01:24:01]   |
[01:24:01]   |
[01:24:01] 4 | assert!("Ferris".chars().eq_ignore_case("FERRIS".chars()));
[01:24:01]   |
[01:24:01]   |
[01:24:01]   = help: add #![feature(eq_ignore_case)] to the crate attributes to enable
[01:24:01] 
[01:24:01] error[E0658]: use of unstable library feature 'eq_ignore_case' (see issue #57221)
[01:24:01]   |
[01:24:01]   |
[01:24:01] 5 | assert!("Ferrös".chars().eq_ignore_case("FERRöS".chars()));
[01:24:01]   |
[01:24:01]   |
[01:24:01]   = help: add #![feature(eq_ignore_case)] to the crate attributes to enable
[01:24:01] 
[01:24:01] error[E0658]: use of unstable library feature 'eq_ignore_case' (see issue #57221)
[01:24:01]   |
[01:24:01]   |
[01:24:01] 6 | assert!("Ferrös".chars().eq_travis_time:end:12f41f6c:start=1546215775228117764,finish=1546220817234800799,duration=5042006683035
travis_time:start:08fe95a4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Mon Dec 31 01:46:57 UTC 2018
Mon, 31 Dec 2018 01:46:57 GMT
