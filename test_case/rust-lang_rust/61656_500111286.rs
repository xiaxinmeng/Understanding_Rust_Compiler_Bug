plain
[00:02:44]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:45] error[E0425]: cannot find value `stage` in this scope
[00:02:45]     --> src/bootstrap/dist.rs:1277:57
[00:02:45]      |
[00:02:45] 1277 |             println!("skipping Dist Miri stage{} ({})", stage, target);
[00:02:45] 
[00:02:47] error[E0609]: no field `toolstate` on type `config::Config`
[00:02:47]     --> src/bootstrap/dist.rs:1276:28
[00:02:47]      |
[00:02:47]      |
[00:02:47] 1276 |         if !builder.config.toolstate.miri.testing() {
[00:02:47]      |
[00:02:47]      |
[00:02:47]      = note: available fields are: `ccache`, `ninja`, `verbose`, `submodules`, `fast_submodules` ... and 98 others
[00:02:48] error: aborting due to 2 previous errors
[00:02:48] 
[00:02:48] Some errors have detailed explanations: E0425, E0609.
[00:02:48] For more information about an error, try `rustc --explain E0425`.
---
[00:02:49]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:50] error[E0425]: cannot find value `stage` in this scope
[00:02:50]     --> src/bootstrap/dist.rs:1277:57
[00:02:50]      |
[00:02:50] 1277 |             println!("skipping Dist Miri stage{} ({})", stage, target);
[00:02:50] 
[00:02:52] error[E0609]: no field `toolstate` on type `config::Config`
[00:02:52]     --> src/bootstrap/dist.rs:1276:28
[00:02:52]      |
[00:02:52]      |
[00:02:52] 1276 |         if !builder.config.toolstate.miri.testing() {
[00:02:52]      |
[00:02:52]      |
[00:02:52]      = note: available fields are: `ccache`, `ninja`, `verbose`, `submodules`, `fast_submodules` ... and 98 others
[00:02:53] error: aborting due to 2 previous errors
[00:02:53] 
[00:02:53] Some errors have detailed explanations: E0425, E0609.
[00:02:53] For more information about an error, try `rustc --explain E0425`.
---
[00:02:55]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:02:56] error[E0425]: cannot find value `stage` in this scope
[00:02:56]     --> src/bootstrap/dist.rs:1277:57
[00:02:56]      |
[00:02:56] 1277 |             println!("skipping Dist Miri stage{} ({})", stage, target);
[00:02:56] 
[00:02:58] error[E0609]: no field `toolstate` on type `config::Config`
[00:02:58]     --> src/bootstrap/dist.rs:1276:28
[00:02:58]      |
[00:02:58]      |
[00:02:58] 1276 |         if !builder.config.toolstate.miri.testing() {
[00:02:58]      |
[00:02:58]      |
[00:02:58]      = note: available fields are: `ccache`, `ninja`, `verbose`, `submodules`, `fast_submodules` ... and 98 others
[00:02:59] error: aborting due to 2 previous errors
[00:02:59] 
[00:02:59] Some errors have detailed explanations: E0425, E0609.
[00:02:59] For more information about an error, try `rustc --explain E0425`.
---
[00:03:02]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:03:03] error[E0425]: cannot find value `stage` in this scope
[00:03:03]     --> src/bootstrap/dist.rs:1277:57
[00:03:03]      |
[00:03:03] 1277 |             println!("skipping Dist Miri stage{} ({})", stage, target);
[00:03:03] 
[00:03:05] error[E0609]: no field `toolstate` on type `config::Config`
[00:03:05]     --> src/bootstrap/dist.rs:1276:28
[00:03:05]      |
[00:03:05]      |
[00:03:05] 1276 |         if !builder.config.toolstate.miri.testing() {
[00:03:05]      |
[00:03:05]      |
[00:03:05]      = note: available fields are: `ccache`, `ninja`, `verbose`, `submodules`, `fast_submodules` ... and 98 others
[00:03:06] error: aborting due to 2 previous errors
[00:03:06] 
[00:03:06] Some errors have detailed explanations: E0425, E0609.
[00:03:06] For more information about an error, try `rustc --explain E0425`.
---
[00:03:10]    Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
[00:03:11] error[E0425]: cannot find value `stage` in this scope
[00:03:11]     --> src/bootstrap/dist.rs:1277:57
[00:03:11]      |
[00:03:11] 1277 |             println!("skipping Dist Miri stage{} ({})", stage, target);
[00:03:11] 
[00:03:13] error[E0609]: no field `toolstate` on type `config::Config`
[00:03:13]     --> src/bootstrap/dist.rs:1276:28
[00:03:13]      |
[00:03:13]      |
[00:03:13] 1276 |         if !builder.config.toolstate.miri.testing() {
[00:03:13]      |
[00:03:13]      |
[00:03:13]      = note: available fields are: `ccache`, `ninja`, `verbose`, `submodules`, `fast_submodules` ... and 98 others
travis_time:end:14b42bc0:start=1559987031636884105,finish=1559987226868178960,duration=195231294855
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:0af5abd6
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:2c16f557:start=1559987227627291099,finish=1559987227642407050,duration=15115951
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00faab68
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:11021554
travis_time:start:11021554
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:3d014b16
$ dmesg | grep -i kill
