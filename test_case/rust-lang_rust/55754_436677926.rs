plain
travis_time:end:01155b76:start=1541601430146560918,finish=1541601483991947598,duration=53845386680
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:52:58] .................................................................................................... 100/4997
[00:53:01] .................................................................................................... 200/4997
[00:53:04] ........................................................................ii...................ii..... 300/4997
[00:53:07] ...........................................................................................iii...... 400/4997
[00:53:10] ..iiiiiiii.iii...........................iii...........................................i...........i 500/4997
[00:53:17] .................................................................................................... 700/4997
[00:53:23] .....................................................................i...........i.................. 800/4997
[00:53:26] ........................................................................................iiiii....... 900/4997
[00:53:30] ...........ii.iiii.................................................................................. 1000/4997
---
[00:54:05] .................................................................................................... 2200/4997
[00:54:09] .................................................................................................... 2300/4997
[00:54:13] .................................................................................................... 2400/4997
[00:54:17] .................................................................................................... 2500/4997
[00:54:20] ..............................................................................iiiiiiiii............. 2600/4997
[00:54:27] .............................ii..................................................................... 2800/4997
[00:54:30] .................................................................................................... 2900/4997
[00:54:34] .................................................................................................... 3000/4997
[00:54:37] ........................i........................................................................... 3100/4997
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:08:36] 
[01:08:36] running 115 tests
[01:08:39] i..ii...iii..iii.....i...i.........i..iii...........i.....i.....ii...i..i.ii...............i..ii..ii 100/115
[01:08:39] .i....iiii.....
[01:08:39] 
[01:08:39]  finished in 3.453
[01:08:39] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:08:53] 
[01:08:53] running 118 tests
[01:09:16] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:09:20] ......iii.i.....ii
[01:09:20] 
[01:09:20]  finished in 26.528
[01:09:20] travis_fold:end:test_debuginfo

---
[01:30:13] .................................................................................................... 400/988
[01:30:20] ......................i..i.....................................iiii........ii....................... 500/988
[01:30:27] .................................................................................................... 600/988
[01:30:34] .................................................................................................... 700/988
[01:30:43] ...........iiii.............................................F..........F............................ 800/988
[01:31:01] ....................................iiii................................................
[01:31:01] failures:
[01:31:01] 
[01:31:01] ---- process.rs - process::Command::output (line 765) stdout ----
[01:31:01] ---- process.rs - process::Command::output (line 765) stdout ----
[01:31:01] error[E0599]: no method named `write` found for type `std::io::Stdout` in the current scope
[01:31:01]    |
[01:31:01]    |
[01:31:01] 12 | io::stdout().write(&output.stdout);
[01:31:01]    |
[01:31:01]    = help: items from traits can only be used if the trait is in scope
[01:31:01] help: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:31:01]    |
[01:31:01]    |
[01:31:01] 3  | use std::io::Write;
[01:31:01]    |
[01:31:01] 
[01:31:01] error[E0599]: no method named `write` found for type `std::io::Stderr` in the current scope
[01:31:01]    |
[01:31:01]    |
[01:31:01] 13 | io::stderr().write(&output.stderr);
[01:31:01]    |
[01:31:01]    = help: items from traits can only be used if the trait is in scope
[01:31:01] help: the following trait is implemented but not in scope, perhaps add a `use` for it:
[01:31:01]    |
[01:31:01]    |
[01:31:01] 3  | use std::io::Write;
[01:31:01]    |
[01:31:01] 
[01:31:01] thread 'process.rs - process::Command::output (line 765)' panicked at 'couldn't compile the test', librustdoc/test.rs:323:13
[01:31:01] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:31:01] 
[01:31:01] ---- process.rs - process::Stdio::inherit (line 953) stdout ----
[01:31:01] error[E0599]: no method named `write` found for type `std::io::Stdout` in the cu

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:28309c58
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0033793a:start=1541606957609038434,finish=1541606957613290651,duration=4252217
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:03946401
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
