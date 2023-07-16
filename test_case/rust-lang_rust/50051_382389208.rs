plain
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:01:59] configure: rust.quiet-tests     := True
---
[00:02:34] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:34] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:34] Build completed unsuccessfully in 0:00:35
[00:02:34] make: *** [prepare] Error 1
[00:02:34] Command failed. Attempt 2/5:
[00:02:34]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:35] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:35] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:35] Build completed unsuccessfully in 0:00:00
[00:02:35] make: *** [prepare] Error 1
[00:02:35] Command failed. Attempt 3/5:
[00:02:35]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:35] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:35] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:35] Build completed unsuccessfully in 0:00:00
[00:02:35] make: *** [prepare] Error 1
[00:02:35] Command failed. Attempt 4/5:
[00:02:35]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:36] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:36] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:36] Build completed unsuccessfully in 0:00:00
[00:02:36] make: *** [prepare] Error 1
[00:02:36] Command failed. Attempt 5/5:
[00:02:36]     Updating registry `https://github.com/rust-lang/crates.io-index`
[00:02:36] error: the lock file needs to be updated but --locked was passed to prevent this
[00:02:36] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[00:02:36] Build completed unsuccessfully in 0:00:00
[00:02:36] make: *** [prepare] Error 1
[00:02:36] The command has failed after 5 attempts.
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:0142af8c:start=1524058659328268684,finish=1524058659338198109,duration=9929425
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:0f1eab80
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:0f1eab80:start=1524058659345189776,finish=1524058659354493066,duration=9303290
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2ee9494e
$ dmesg | grep -i kill
[   10.388111] init: failsafe main process (1093) killed by TERM signal
