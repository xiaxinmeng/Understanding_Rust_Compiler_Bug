plain
travis_time:end:0d355238:start=1553443553537255520,finish=1553443554484076371,duration=946820851
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
Setting environment variables from .travis.yml
---
travis_time:start:test_assembly
Check compiletest suite=assembly mode=assembly (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:21:45] 
[01:21:45] running 9 tests
[01:21:45] iiiiiiiii
[01:21:45] 
[01:21:45]  finished in 0.158
[01:21:45] travis_fold:end:test_assembly

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:22:01] 
[01:22:01] running 120 tests
[01:22:27] .iiiii...i.....i..i...i..i.i...i.ii...i.....i..i....i..........iiii..........i...ii...i.......ii.i.i 100/120
[01:22:32] .i......iii.i.....ii
[01:22:32] 
[01:22:32]  finished in 30.954
[01:22:32] travis_fold:end:test_debuginfo

---
[01:51:51]    Compiling ansi_term v0.11.0
[01:51:51]    Compiling difference v2.0.0
[01:51:52]    Compiling pretty_assertions v0.5.1
[01:51:52]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[01:51:56] error[E0063]: missing field `rustfix_coverage` in initializer of `flags::Subcommand`
[01:51:56]      |
[01:51:56] 1851 |         config.cmd = Subcommand::Test {
[01:51:56] 1851 |         config.cmd = Subcommand::Test {
[01:51:56]      |                      ^^^^^^^^^^^^^^^^ missing `rustfix_coverage`
[01:51:56] 
[01:51:56] error[E0063]: missing field `rustfix_coverage` in initializer of `flags::Subcommand`
[01:51:56]      |
[01:51:56] 1892 |         config.cmd = Subcommand::Test {
[01:51:56] 1892 |         config.cmd = Subcommand::Test {
[01:51:56]      |                      ^^^^^^^^^^^^^^^^ missing `rustfix_coverage`
[01:51:56] error: aborting due to 2 previous errors
[01:51:56] 
[01:51:56] For more information about this error, try `rustc --explain E0063`.
[01:51:56] error: Could not compile `bootstrap`.
---
[01:51:56] 
[01:51:56] 
[01:51:56] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:51:56] Build completed unsuccessfully in 0:41:59
[01:51:56] make: *** [check] Error 1
[01:51:56] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2938cf98
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Mar 24 17:58:01 UTC 2019
---
43088 ./obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnutravis_time:end:13bc5fdf:start=1553450283476555580,finish=1553450283533642021,duration=57086441
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0925885e
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:10e0a26c
$ dmesg | grep -i kill
