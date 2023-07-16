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
tidy: Skipping binary file check, read-only filesystem
Checking which error codes lack tests...
* 632 error codes
* highest error code: E0790
tidy error: /checkout/library/core/src/slice/sort.rs:773: undocumented unsafe
tidy error: /checkout/library/core/src/slice/sort.rs:880: TODO is deprecated; use FIXME
tidy error: /checkout/library/core/src/slice/sort.rs:890: undocumented unsafe
tidy error: /checkout/library/core/src/slice/sort.rs:957: undocumented unsafe
tidy error: /checkout/library/core/src/slice/sort.rs:1056: undocumented unsafe
tidy error: /checkout/library/core/src/slice/sort.rs:1080: undocumented unsafe
tidy error: /checkout/library/core/src/slice/sort.rs:1103: undocumented unsafe
tidy error: /checkout/library/core/src/slice/sort.rs:1116: undocumented unsafe
tidy error: /checkout/library/core/src/slice/sort.rs:1127: undocumented unsafe
tidy error: /checkout/library/core/src/slice/sort.rs:1140: undocumented unsafe
tidy error: /checkout/library/core/src/slice/sort.rs:1155: undocumented unsafe
tidy error: /checkout/library/core/src/slice/sort.rs:1160: undocumented unsafe
tidy error: /checkout/library/core/src/slice/sort.rs:1174: undocumented unsafe
tidy error: /checkout/library/core/src/slice/sort.rs:1492: TODO is deprecated; use FIXME
Found 0 error(s) in error codes
Done!
* 389 features
some tidy checks failed
