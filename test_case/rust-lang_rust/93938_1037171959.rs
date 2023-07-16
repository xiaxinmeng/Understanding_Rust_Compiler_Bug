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
Found 503 error codes
Found 0 error codes with no tests
Done!
tidy error: /checkout/compiler/rustc_hir/src/def.rs:278: trailing whitespace
tidy error: /checkout/compiler/rustc_hir/src/def.rs:283: trailing whitespace
tidy error: /checkout/compiler/rustc_hir/src/def.rs:290: trailing whitespace
tidy error: /checkout/compiler/rustc_hir/src/def.rs:296: trailing whitespace
tidy error: /checkout/compiler/rustc_hir/src/def.rs:310: trailing whitespace
tidy error: /checkout/compiler/rustc_hir/src/def.rs:326: trailing whitespace
some tidy checks failed
Build completed unsuccessfully in 0:00:11
