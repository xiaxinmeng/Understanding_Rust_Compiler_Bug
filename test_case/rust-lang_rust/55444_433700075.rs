plain
travis_time:end:00fe9e50:start=1540729002113336540,finish=1540729003139284146,duration=1025947606
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
---
[00:02:31]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:32] error[E0425]: cannot find value `build` in this scope
[00:02:32]    --> bootstrap/compile.rs:154:8
[00:02:32]     |
[00:02:32] 154 |     if build.build.build.contains("linux") && target == "i686-pc-windows-gnu" {
[00:02:32] help: possible candidates are found in other modules, you can import them into scope
[00:02:32]     |
[00:02:32] 19  | use cmake::build;
[00:02:32]     |
[00:02:32]     |
[00:02:32] 19  | use metadata::build;
[00:02:32]     |
[00:02:32] 
[00:02:32] error[E0425]: cannot find value `features` in this scope
[00:02:32]    --> bootstrap/compile.rs:155:9
[00:02:32]     |
[00:02:32] 155 |         features.push_str(" sjlj_eh");
[00:02:32] 
[00:02:35] error: aborting due to 2 previous errors
[00:02:35] 
[00:02:35] For more information about this error, try `rustc --explain E0425`.
---
[00:02:36]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:37] error[E0425]: cannot find value `build` in this scope
[00:02:37]    --> bootstrap/compile.rs:154:8
[00:02:37]     |
[00:02:37] 154 |     if build.build.build.contains("linux") && target == "i686-pc-windows-gnu" {
[00:02:37] help: possible candidates are found in other modules, you can import them into scope
[00:02:37]     |
[00:02:37] 19  | use cmake::build;
[00:02:37]     |
[00:02:37]     |
[00:02:37] 19  | use metadata::build;
[00:02:37]     |
[00:02:37] 
[00:02:37] error[E0425]: cannot find value `features` in this scope
[00:02:37]    --> bootstrap/compile.rs:155:9
[00:02:37]     |
[00:02:37] 155 |         features.push_str(" sjlj_eh");
[00:02:37] 
[00:02:41] error: aborting due to 2 previous errors
[00:02:41] 
[00:02:41] For more information about this error, try `rustc --explain E0425`.
---
[00:02:43]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:44] error[E0425]: cannot find value `build` in this scope
[00:02:44]    --> bootstrap/compile.rs:154:8
[00:02:44]     |
[00:02:44] 154 |     if build.build.build.contains("linux") && target == "i686-pc-windows-gnu" {
[00:02:44] help: possible candidates are found in other modules, you can import them into scope
[00:02:44]     |
[00:02:44] 19  | use cmake::build;
[00:02:44]     |
[00:02:44]     |
[00:02:44] 19  | use metadata::build;
[00:02:44]     |
[00:02:44] 
[00:02:44] error[E0425]: cannot find value `features` in this scope
[00:02:44]    --> bootstrap/compile.rs:155:9
[00:02:44]     |
[00:02:44] 155 |         features.push_str(" sjlj_eh");
[00:02:44] 
[00:02:47] error: aborting due to 2 previous errors
[00:02:47] 
[00:02:47] For more information about this error, try `rustc --explain E0425`.
---
[00:02:50]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:51] error[E0425]: cannot find value `build` in this scope
[00:02:51]    --> bootstrap/compile.rs:154:8
[00:02:51]     |
[00:02:51] 154 |     if build.build.build.contains("linux") && target == "i686-pc-windows-gnu" {
[00:02:51] help: possible candidates are found in other modules, you can import them into scope
[00:02:51]     |
[00:02:51] 19  | use cmake::build;
[00:02:51]     |
[00:02:51]     |
[00:02:51] 19  | use metadata::build;
[00:02:51]     |
[00:02:51] 
[00:02:51] error[E0425]: cannot find value `features` in this scope
[00:02:51]    --> bootstrap/compile.rs:155:9
[00:02:51]     |
[00:02:51] 155 |         features.push_str(" sjlj_eh");
[00:02:51] 
[00:02:55] error: aborting due to 2 previous errors
[00:02:55] 
[00:02:55] For more information about this error, try `rustc --explain E0425`.
---
[00:02:59]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:03:00] error[E0425]: cannot find value `build` in this scope
[00:03:00]    --> bootstrap/compile.rs:154:8
[00:03:00]     |
[00:03:00] 154 |     if build.build.build.contains("linux") && target == "i686-pc-windows-gnu" {
[00:03:00] help: possible candidates are found in other modules, you can import them into scope
[00:03:00]     |
[00:03:00] 19  | use cmake::build;
[00:03:00]     |
[00:03:00]     |
[00:03:00] 19  | use metadata::build;
[00:03:00]     |
[00:03:00] 
[00:03:00] error[E0425]: cannot find value `features` in this scope
[00:03:00]    --> bootstrap/compile.rs:155:9
[00:03:00]     |
[00:03:00] 155 |         features.push_str(" sjlj_eh");
[00:03:00] 
[00:03:03] error: aborting due to 2 previous errors
[00:03:03] 
[00:03:03] For more information about this error, try `rustc --explain E0425`.
---
travis_time:end:2615dc60:start=1540729197312735905,finish=1540729197319055544,duration=6319639
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:27147480
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:16127650
travis_time:start:16127650
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:29871adf
$ dmesg | grep -i kill
