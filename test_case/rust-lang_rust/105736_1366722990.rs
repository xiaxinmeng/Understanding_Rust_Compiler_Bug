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
........................................................................................ 1496/1523
...........................
failures:

---- alloc::layout_array_edge_cases stdout ----
thread 'alloc::layout_array_edge_cases' panicked at 'assertion failed: `(left == right)`
  left: `false`,
 right: `true`', library/core/tests/alloc.rs:54:13


failures:
    alloc::layout_array_edge_cases
