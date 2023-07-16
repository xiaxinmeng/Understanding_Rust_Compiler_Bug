plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:1598d858
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
travis_fold:end:before_install.5
travis_fold:start:before_install.6
travis_time:start:0a36bc4d
$ sudo grep -rE 'shutdown|poweroff|halt' /var/spool/cron || true
/var/spool/cron/atjobs/a00001018601cc:poweroff
travis_fold:end:before_install.6
travis_fold:start:before_install.7
travis_time:start:06a1bc66
$ sudo grep -E 'google-clock-skew|ntpd|startup-script' /var/log/syslog || true
---
[00:44:27] Building stage1 codegen artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu, emscripten)
[00:44:28]    Compiling rustc_codegen_llvm v0.0.0 (file:///checkout/src/librustc_codegen_llvm)
[00:44:28]    Compiling rustc_llvm v0.0.0 (file:///checkout/src/librustc_llvm)

Broadcast message from root@travis-job-43d4f326-326c-45ea-94bf-ff5954307705
 (unknown) at 13:16 ...
The system is going down for power off NOW!
