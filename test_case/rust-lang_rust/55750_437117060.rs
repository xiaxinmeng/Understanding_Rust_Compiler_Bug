plain
travis_time:end:170c5a64:start=1541697992439280097,finish=1541698046487232785,duration=54047952688
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:52:50] .................................................................................................... 100/4999
[00:52:53] .................................................................................................... 200/4999
[00:52:56] ........................................................................ii...................ii..... 300/4999
[00:52:58] ...........................................................................................iii...... 400/4999
[00:53:01] ..iiiiiiii.iii...........................iii...........................................i...........i 500/4999
[00:53:08] .................................................................................................... 700/4999
[00:53:15] .....................................................................i...........i.................. 800/4999
[00:53:18] ........................................................................................iiiii....... 900/4999
[00:53:21] ...........ii.iiii.................................................................................. 1000/4999
---
[00:53:57] .................................................................................................... 2200/4999
[00:54:02] .................................................................................................... 2300/4999
[00:54:05] .................................................................................................... 2400/4999
[00:54:09] .................................................................................................... 2500/4999
[00:54:13] ...................................................................iiiiiiiii........................ 2600/4999
[00:54:20] ...............................ii................................................................... 2800/4999
[00:54:22] .................................................................................................... 2900/4999
[00:54:26] .................................................................................................... 3000/4999
[00:54:29] ..........................i......................................................................... 3100/4999
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:08:33] 
[01:08:33] running 115 tests
[01:08:36] i..ii...iii..iii.....i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..ii 100/115
[01:08:37] .i....iiii.....
[01:08:37] 
[01:08:37]  finished in 3.498
[01:08:37] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:08:51] 
[01:08:51] running 118 tests
[01:09:14] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:09:17] ......iii.i.....ii
[01:09:17] 
[01:09:17]  finished in 26.735
[01:09:17] travis_fold:end:test_debuginfo

---
[01:34:05] travis_fold:start:test_stage1-rustc_driver
travis_time:start:test_stage1-rustc_driver
Testing rustc_driver stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:34:05]    Compiling rustc_driver v0.0.0 (/checkout/src/librustc_driver)
[01:34:06] error[E0423]: expected function, found struct `hir::ItemLocalId`
[01:34:06]    --> librustc_driver/test.rs:235:17
[01:34:06]     |
[01:34:06] 235 |             id: hir::ItemLocalId(1),
[01:34:06]     |                 ^^^^^^^^^^^^^^^^ did you mean `hir::ItemLocalId { /* fields */ }`?
[01:34:06] 
[01:34:06] error[E0423]: expected function, found struct `hir::ItemLocalId`
[01:34:06]    --> librustc_driver/test.rs:241:21
[01:34:06]     |
[01:34:06] 241 |                 id: hir::ItemLocalId(1),
[01:34:06]     |                     ^^^^^^^^^^^^^^^^ did you mean `hir::ItemLocalId { /* fields */ }`?
[01:34:06] 
[01:34:06] error[E0423]: expected function, found struct `hir::ItemLocalId`
[01:34:06]    --> librustc_driver/test.rs:244:29
[01:34:06]     |
[01:34:06] 244 |                         id: hir::ItemLocalId(10),
[01:34:06]     |                             ^^^^^^^^^^^^^^^^ did you mean `hir::ItemLocalId { /* fields */ }`?
[01:34:06] 
[01:34:06] error[E0423]: expected function, found struct `hir::ItemLocalId`
[01:34:06]    --> librustc_driver/test.rs:248:29
[01:34:06]     |
[01:34:06] 248 |                         id: hir::ItemLocalId(11),
[01:34:06]     |                             ^^^^^^^^^^^^^^^^ did you mean `hir::ItemLocalId { /* fields */ }`?
[01:34:06] 
[01:34:06] error[E0423]: expected function, found struct `hir::ItemLocalId`
[01:34:06]    --> librustc_driver/test.rs:403:17
[01:34:06]     |
[01:34:06] 403 |             id: hir::ItemLocalId(id),
[01:34:06]     |                 ^^^^^^^^^^^^^^^^ did you mean `hir::ItemLocalId { /* fields */ }`?
[01:34:07] error: aborting due to 5 previous errors
[01:34:07] 
[01:34:07] For more information about this error, try `rustc --explain E0423`.
[01:34:07] error: Could not compile `rustc_driver`.
[01:34:07] error: Could not compile `rustc_driver`.
[01:34:07] 
[01:34:07] To learn more, run the command again with --verbose.
[01:34:07] 
[01:34:07] 
[01:34:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "rustc_driver" "--" "--quiet"
[01:34:07] 
[01:34:07] 
[01:34:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:34:07] Build completed unsuccessfully in 0:45:04
[01:34:07] Build completed unsuccessfully in 0:45:04
[01:34:07] Makefile:58: recipe for target 'check' failed
[01:34:07] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:01632ced
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
