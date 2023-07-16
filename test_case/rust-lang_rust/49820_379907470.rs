plain
[00:00:47] configure: rust.quiet-tests     := True
---
[00:04:47] tidy error: /checkout/src/libsyntax/ext/base.rs:234: line longer than 100 chars
---
$ dmesg | grep -i kill
[   10.536843] init: failsafe main process (1093) killed by TERM signal
