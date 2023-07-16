plain
configure: rust.debug-assertions := True
configure: rust.overflow-checks := True
configure: llvm.assertions      := True
configure: dist.missing-tools   := True
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
* highest error code: E0787
Found 504 error codes
Found 0 error(s) in error codes
Done!
tidy error: /checkout/src/test/rustdoc-json/fn_pointer/abi.rs:12: line longer than 100 chars
tidy error: /checkout/src/test/rustdoc-json/fn_pointer/abi.rs:18: line longer than 100 chars
tidy error: /checkout/src/test/rustdoc-json/fn_pointer/abi.rs:21: line longer than 100 chars
tidy error: /checkout/src/test/rustdoc-json/fn_pointer/abi.rs:24: line longer than 100 chars
tidy error: /checkout/src/test/rustdoc-json/methods/abi.rs:22: line longer than 100 chars
tidy error: /checkout/src/test/rustdoc-json/methods/abi.rs:28: line longer than 100 chars
tidy error: /checkout/src/test/rustdoc-json/methods/abi.rs:39: line longer than 100 chars
tidy error: /checkout/src/test/rustdoc-json/methods/abi.rs:42: line longer than 100 chars
tidy error: /checkout/src/test/rustdoc-json/methods/abi.rs:45: line longer than 100 chars
tidy error: /checkout/src/test/rustdoc-json/methods/abi.rs:48: line longer than 100 chars
tidy error: /checkout/src/test/rustdoc-json/methods/abi.rs:51: line longer than 100 chars
tidy error: /checkout/src/test/rustdoc-json/fns/abi.rs:16: line longer than 100 chars
tidy error: /checkout/src/test/rustdoc-json/fns/abi.rs:22: line longer than 100 chars
some tidy checks failed
Build completed unsuccessfully in 0:00:12
