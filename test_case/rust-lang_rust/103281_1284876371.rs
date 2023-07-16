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
........................................................................................ 1496/1506
..........
failures:

---- mem::test_transmute_copy_grow_panics stdout ----
thread 'mem::test_transmute_copy_grow_panics' panicked at 'cannot transmute_copy if Dst is larger than Src', /checkout/library/core/src/mem/mod.rs:1056:5

failures:
    mem::test_transmute_copy_grow_panics
error: test failed, to rerun pass `-p core --test coretests`
