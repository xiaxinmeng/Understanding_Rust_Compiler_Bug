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
tidy: Skipping binary file check, read-only filesystem
Checking which error codes lack tests...
* 629 error codes
* highest error code: E0787
tidy error: /checkout/src/doc/rustc/src/platform-support/armv7-unknown-linux-uclibceabi.md:13: trailing whitespace
tidy error: /checkout/src/doc/rustc/src/platform-support/armv7-unknown-linux-uclibceabi.md:18: trailing whitespace
tidy error: /checkout/src/doc/rustc/src/platform-support/armv7-unknown-linux-uclibceabi.md:22: trailing whitespace
tidy error: /checkout/src/doc/rustc/src/platform-support/armv7-unknown-linux-uclibceabi.md:44: trailing whitespace
tidy error: /checkout/src/doc/rustc/src/platform-support/armv7-unknown-linux-uclibceabi.md:56: trailing whitespace
tidy error: /checkout/src/doc/rustc/src/platform-support/armv7-unknown-linux-uclibceabi.md:60: trailing whitespace
tidy error: /checkout/src/doc/rustc/src/platform-support/armv7-unknown-linux-uclibceabi.md:66: trailing whitespace
tidy error: /checkout/src/doc/rustc/src/platform-support/armv7-unknown-linux-uclibceabi.md:84: trailing whitespace
tidy error: /checkout/src/doc/rustc/src/platform-support/armv7-unknown-linux-uclibceabi.md:96: trailing whitespace
tidy error: /checkout/src/doc/rustc/src/platform-support/armv7-unknown-linux-uclibceabi.md:116: trailing whitespace
Found 0 error codes with no tests
Done!
* 363 features
some tidy checks failed
