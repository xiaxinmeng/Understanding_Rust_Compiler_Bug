plain
Resolving deltas: 100% (611672/611672), completed with 4861 local objects.
---
[00:00:46] configure: rust.quiet-tests     := True
---
[00:01:18] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:18] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:18] Build completed unsuccessfully in 0:00:31
[00:01:18] make: *** [prepare] Error 1
[00:01:18] Makefile:81: recipe for target 'prepare' failed
[00:01:18] Command failed. Attempt 2/5:
[00:01:18] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:18] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:18] Build completed unsuccessfully in 0:00:00
[00:01:18] make: *** [prepare] Error 1
[00:01:18] Makefile:81: recipe for target 'prepare' failed
[00:01:18] Command failed. Attempt 3/5:
[00:01:18] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:18] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:18] Build completed unsuccessfully in 0:00:00
[00:01:18] make: *** [prepare] Error 1
[00:01:18] Makefile:81: recipe for target 'prepare' failed
[00:01:18] Command failed. Attempt 4/5:
[00:01:18] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:18] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:18] Build completed unsuccessfully in 0:00:00
[00:01:18] make: *** [prepare] Error 1
[00:01:18] Makefile:81: recipe for target 'prepare' failed
[00:01:18] Command failed. Attempt 5/5:
[00:01:18] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:18] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:18] Build completed unsuccessfully in 0:00:00
[00:01:18] make: *** [prepare] Error 1
[00:01:18] Makefile:81: recipe for target 'prepare' failed
[00:01:18] The command has failed after 5 attempts.
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:0e9514fc:start=1523031062354581336,finish=1523031062362205548,duration=7624212
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:0866ba90
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:0866ba90:start=1523031062368936878,finish=1523031062375297956,duration=6361078
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:02177eb8
$ dmesg | grep -i kill
[   10.613799] init: failsafe main process (1094) killed by TERM signal
