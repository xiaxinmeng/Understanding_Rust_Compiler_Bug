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
..............................
failures:

---- string::test_try_reserve stdout ----
thread 'string::test_try_reserve' panicked at 'attempt to add with overflow', library/alloc/src/raw_vec.rs:405:24
---- vec::test_push_growth_strategy stdout ----
---- vec::test_push_growth_strategy stdout ----
thread 'vec::test_push_growth_strategy' panicked at 'assertion failed: alloc_size > 1', /checkout/library/alloc/src/raw_vec.rs:403:9
---- vec::test_try_reserve stdout ----
---- vec::test_try_reserve stdout ----
thread 'vec::test_try_reserve' panicked at 'attempt to add with overflow', /checkout/library/alloc/src/raw_vec.rs:405:24

failures:
    string::test_try_reserve
    vec::test_push_growth_strategy
