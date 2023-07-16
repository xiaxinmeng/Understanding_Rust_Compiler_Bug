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
........................................................................................ 1408/1494
......................................................................................
failures:

---- fmt::test_estimated_capacity stdout ----
thread 'fmt::test_estimated_capacity' panicked at 'assertion failed: `(left == right)`
 right: `0`', library/core/tests/fmt/mod.rs:27:5


failures:
