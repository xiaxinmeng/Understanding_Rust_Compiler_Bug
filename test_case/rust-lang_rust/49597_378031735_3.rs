\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui-fulldeps/proc-macro/auxiliary/three-equals.rs","byte_start":800,"byte_end":804,"line_start":23,"line_end":23,"column_start":25,"column_end":29,"is_primary":true,"text":[{"text":"        let span = tree.span;","highlight_start":25,"highlight_end":29}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[{"message":"maybe a `()` to call it is missing?","code":null,"level":"help","spans":[],"children":[],"rendered":null}],"rendered":"error[E0615]: attempted to take value of method `span` on type `proc_macro::TokenTree`\n  --> /checkout/src/tenknown-linux-gnu/native
---
56668 ./obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/incremental/syntax-2bjr8lg1wd06d/s-ezr2bofnbx-1tjk4f8-3oe29bqr0b31g
---
$ ls -lat $HOME/Library/Logs/DiagnosticReports/
ls: cannot access /home/travis/Library/Logs/DiagnosticReports/: No such file or directory
travis_time:end:1c3df970:start=1522700286854383955,finish=1522700286862465213,duration=8081258
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0e66bd0a
$ find $HOME/Library/Logs/DiagnosticReports -type f -name '*.crash' -not -name '*.stage2-*.crash' -not -name 'com.apple.CoreSimulator.CoreSimulatorService-*.crash' -exec printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" {} \; -exec head -750 {} \; -exec echo travis_fold":"end:crashlog \; || true
find: `/home/travis/Library/Logs/DiagnosticReports': No such file or directory
travis_time:end:0e66bd0a:start=1522700286870059867,finish=1522700286878255196,duration=8195329
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:07b54980
$ dmesg | grep -i kill
[   10.986113] init: failsafe main process (1093) killed by TERM signal
