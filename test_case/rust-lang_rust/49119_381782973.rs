plain
[00:01:01] configure: rust.quiet-tests     := True
---
[00:05:26]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_errors librustc_errors/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=609e2421d03f9c9a -C extra-filename=-609e2421d03f9c9a --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_649
[00:05:26] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:05:26] Build completed unsuccessfully in 0:01:22
[00:05:26] Makefile:28: recipe for target 'all' failed
[00:05:26] make: *** [all] Error 1
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:034c68c6:start=1523921874241369783,finish=1523921874248881719,duration=7511936
travis_fold:end:after_failure.2
travis_fold:start:after_failure.3
travis_time:start:041dfc8c
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:041dfc8c:start=1523921874256626252,finish=1523921874264762915,duration=8136663
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1718cff4
$ dmesg | grep -i kill
[   11.034767] init: failsafe main process (1093) killed by TERM signal
