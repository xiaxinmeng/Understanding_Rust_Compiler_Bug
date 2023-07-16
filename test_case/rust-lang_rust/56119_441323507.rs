plain
travis_time:end:08399d5a:start=1543005138292520995,finish=1543005192160381682,duration=53867860687
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:50:45] .................................................................................................... 100/5050
[00:50:48] .................................................................................................... 200/5050
[00:50:50] .............................ii............................................ii...................ii.. 300/5050
[00:50:53] ..............................................................................................iii... 400/5050
[00:50:56] .....iiiiiiii.iii............................iii...........................................i........ 500/5050
[00:51:03] .................................................................................................... 700/5050
[00:51:09] ...................................................................................i...........i.... 800/5050
[00:51:11] .................................................................................................... 900/5050
[00:51:14] ..iiiii..................ii.iiii.................................................................... 1000/5050
---
[00:51:45] .................................................................................................... 2200/5050
[00:51:49] .................................................................................................... 2300/5050
[00:51:52] .................................................................................................... 2400/5050
[00:51:56] .................................................................................................... 2500/5050
[00:51:59] ...........................................................................................iiiiiiiii 2600/5050
[00:52:06] .........................................................ii......................................... 2800/5050
[00:52:09] .................................................................................................... 2900/5050
[00:52:13] .................................................................................................... 3000/5050
[00:52:16] ......................................................i............................................. 3100/5050
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:06] 
[01:06:06] running 117 tests
[01:06:09] i..ii...iii..iiii.....i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..i 100/117
[01:06:10] i.i.....iiii.....
[01:06:10] 
[01:06:10]  finished in 3.708
[01:06:10] travis_fold:end:test_codegen

---
travis_time:start:test_debuginfo
Check compiletest suite=debuginfo mode=debuginfo-both (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:06:24] 
[01:06:24] running 118 tests
[01:06:48] .iiiii...i.....i..i...i..i.i..i.i..i.....i..i....i..........iiii.........i.i....i...i.......ii.i.i.i 100/118
[01:06:52] ......iii.i.....ii
[01:06:52] 
[01:06:52]  finished in 27.500
[01:06:52] travis_fold:end:test_debuginfo

---
[01:35:17] travis_fold:start:stage0-linkchecker
travis_time:start:stage0-linkchecker
Building stage0 tool linkchecker (x86_64-unknown-linux-gnu)
[01:35:18]    Compiling linkchecker v0.1.0 (/checkout/src/tools/linkchecker)
[01:35:18] error: expected one of `.`, `;`, `?`, or an operator, found `redirect_line`
[01:35:18]     |
[01:35:18]     |
[01:35:18] 335 |     let redirect_line = lines.nth(6)?
[01:35:18]     |                                      - expected one of `.`, `;`, `?`, or an operator here
[01:35:18] 336 | 
[01:35:18] 337 |     redirect_line.find(REDIRECT).map(|i| {
[01:35:18]     |     ^^^^^^^^^^^^^ unexpected token
[01:35:18] error: aborting due to previous error
[01:35:18] 
[01:35:18] error: Could not compile `linkchecker`.
[01:35:18] 
[01:35:18] 
[01:35:18] To learn more, run the command again with --verbose.
[01:35:18] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/linkchecker/Cargo.toml" "--message-format" "json"
[01:35:18] expected success, got: exit code: 101
[01:35:18] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:35:18] Build completed unsuccessfully in 0:48:29
[01:35:18] Makefile:58: recipe for target 'check' failed
[01:35:18] make: *** [check] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1f9b5b8c
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Nov 23 22:08:39 UTC 2018
---
148504 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu
148500 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release
141220 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps
134660 ./obj/build/bootstrap/debug/incremental/bootstrap-1plb86h2refwc
134656 ./obj/build/bootstrap/debug/incremental/bootstrap-1plb86h2refwc/s-f6y6v4wmnn-etei7d-3bcgkskzxpex9
130776 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release
127616 ./obj/build/x86_64-unknown-linux-gnu/test/mir-opt
123700 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps
115736 ./obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/release
---
travis_time:end:344ba47e:start=1543010921649506117,finish=1543010921656941681,duration=7435564
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:021b5ee2
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:15d7cdd8
travis_time:start:15d7cdd8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0a040010
$ dmesg | grep -i kill
