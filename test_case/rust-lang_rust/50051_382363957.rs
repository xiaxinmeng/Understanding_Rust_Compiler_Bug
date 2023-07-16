plain
[00:00:46] configure: rust.quiet-tests     := True
---
[00:01:26] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:26] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:26] Build completed unsuccessfully in 0:00:40
[00:01:26] make: *** [prepare] Error 1
[00:01:26] Makefile:81: recipe for target 'prepare' failed
[00:01:26] Command failed. Attempt 2/5:
[00:01:26]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:27] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:27] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:27] Build completed unsuccessfully in 0:00:00
[00:01:27] Makefile:81: recipe for target 'prepare' failed
[00:01:27] make: *** [prepare] Error 1
[00:01:27] Command failed. Attempt 3/5:
[00:01:27]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:28] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:28] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:28] Build completed unsuccessfully in 0:00:00
[00:01:28] Makefile:81: recipe for target 'prepare' failed
[00:01:28] make: *** [prepare] Error 1
[00:01:28] Command failed. Attempt 4/5:
[00:01:28]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:28] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:28] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:28] Build completed unsuccessfully in 0:00:00
[00:01:28] make: *** [prepare] Error 1
[00:01:28] Makefile:81: recipe for target 'prepare' failed
[00:01:28] Command failed. Attempt 5/5:
[00:01:28]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:01:29] error: the lock file needs to be updated but --locked was passed to prevent this
[00:01:29] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:01:29] Build completed unsuccessfully in 0:00:00
[00:01:29] Makefile:81: recipe for target 'prepare' failed
[00:01:29] make: *** [prepare] Error 1
[00:01:29] The command has failed after 5 attempts.
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:318dc532:start=1524053154214451560,finish=1524053154221301761,duration=6850201
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:0968533b
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:0968533b:start=1524053154227585569,finish=1524053154233938051,duration=6352482
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:12f9e700
$ dmesg | grep -i kill
[   10.813862] init: failsafe main process (1094) killed by TERM signal
