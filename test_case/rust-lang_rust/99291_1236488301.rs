plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
Attempting with retry: make prepare
---
* highest error code: E0790
Found 506 error codes
Found 0 error(s) in error codes
Done!
tidy error: /checkout/src/test/ui/let-else/let-else-drop-order.rs:67: line longer than 100 chars
tidy error: /checkout/src/test/ui/let-else/let-else-drop-order.rs:70: line longer than 100 chars
some tidy checks failed
Build completed unsuccessfully in 0:00:09
