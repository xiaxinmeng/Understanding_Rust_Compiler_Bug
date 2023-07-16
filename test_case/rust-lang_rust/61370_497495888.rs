plain
travis_time:end:1d61a4ee:start=1559246077039464983,finish=1559246078126141873,duration=1086676890
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
[01:20:20] 
[01:20:20] running 143 tests
[01:20:23] i..iii.....iii...iiii....i......................i...i................i.....i..........ii.i..i..i.ii. 100/143
[01:20:25] test result: ok. 113 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:20:25] 
[01:20:25]  finished in 4.890
[01:20:25] travis_fold:end:test_codegen
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:20:27] 
[01:20:27] running 9 tests
[01:20:27] iiiiiiiii
[01:20:27] 
[01:20:27]  finished in 0.153
[01:20:27] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-gdb+lldb (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:20:43] 
[01:20:43] running 122 tests
[01:21:09] .iiiii...i.....i..i...i..i.i.i..i.ii..i.i.....i..i....i..........iiii..........i...ii...i.......ii.i 100/122
[01:21:14] .i.i......iii.i.....ii
[01:21:14] 
[01:21:14]  finished in 30.537
[01:21:14] travis_fold:end:test_debuginfo

---
[01:45:04]    Compiling ansi_term v0.11.0
[01:45:04]    Compiling difference v2.0.0
[01:45:04]    Compiling pretty_assertions v0.5.1
[01:45:05]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[01:45:08] error[E0063]: missing field `distcheck_make` in initializer of `flags::Subcommand`
[01:45:08]      |
[01:45:08] 1926 |         config.cmd = Subcommand::Test {
[01:45:08]      |                      ^^^^^^^^^^^^^^^^ missing `distcheck_make`
[01:45:08] 
[01:45:08] 
[01:45:08] error[E0063]: missing field `distcheck_make` in initializer of `flags::Subcommand`
[01:45:08]      |
[01:45:08] 1968 |         config.cmd = Subcommand::Test {
[01:45:08]      |                      ^^^^^^^^^^^^^^^^ missing `distcheck_make`
[01:45:08] 
---
[01:45:08] 
[01:45:08] 
[01:45:08] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:45:08] Build completed unsuccessfully in 0:36:50
[01:45:08] make: *** [check] Error 1
[01:45:08] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:061237fb
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Thu May 30 21:39:56 UTC 2019
