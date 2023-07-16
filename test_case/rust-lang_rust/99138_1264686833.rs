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
........................................................................................ 1408/1495
.......................................................................................
failures:

---- fmt::num::test_format_debug_bases stdout ----
thread 'fmt::num::test_format_debug_bases' panicked at 'assertion failed: `(left == right)`
  left: `"[70, 111, 111, 00]"`,
 right: `"[46, 6f, 6f, 00]"`', library/core/tests/fmt/num.rs:223:5

failures:
error: test failed, to rerun pass `-p core --test coretests`
    fmt::num::test_format_debug_bases
