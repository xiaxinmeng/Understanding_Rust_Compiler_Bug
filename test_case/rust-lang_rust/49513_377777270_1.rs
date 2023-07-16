\n\nEnsure that the expressions given can be evaluated as the desired integer type.\nSee the FFI section of the Reference for more information about using a custom\ninteger type:\n\nhttps://doc.rust-lang.org/reference.html#ffi-attributes\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/uninhabited_enum_discriminant2.rs","byte_start":521,"byte_end":522,"line_start":14,"line_end":14,"column_start":9,"column_end":10,"is_primary":true,"text":[{"text":"    B = A, //~ ERROR enums without inhabited variants do not have discriminants","highlight_start":9,"highlight_end":10}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error[E0080]: constant evaluation of enum discriminant resulted in non-integer\n  --> /checkout/src/test/ui/uninhabited_enum_discriminant2.rs:14:9\n   |\nLL |     B = A, //~ ERROR enums without inhabited variants do not have discriminants\n   |         ^\n\n"}
[00:39:58] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:39:58] {"message":"For more information about this error, try `rustc --explain E0080`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0080`.\n"}
---
[00:39:essfully in 0:01:58
[00:39:58] Makefile:58: recipe for target 'check' failed
[00:39:58] make: *** [check] Error 1
---
56656 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental/syntax-2bjr8lg1wd06d/s-ezpishk4pc-1m7ty9p-2t6tvq8n3uqxw
---
$ cat obj/tmp/sccache.log
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:282c949e:start=1522578398095573314,finish=1522578398101322510,duration=5749196
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:083d0d38
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:083d0d38:start=1522578398106515779,finish=1522578398111893709,duration=5377930
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01330c4c
$ dmesg | grep -i kill
[   10.237355] init: failsafe main process (1093) killed by TERM signal
