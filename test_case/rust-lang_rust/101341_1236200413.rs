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
..FF.................................................................................... 352/373
.....................
failures:

---- raw_vec::tests::allocator_param stdout ----
thread 'raw_vec::tests::allocator_param' panicked at 'assertion failed: `(left == right)`
 right: `250`', library/alloc/src/raw_vec/tests.rs:46:5

---- raw_vec::tests::reserve_does_not_overallocate stdout ----
thread 'raw_vec::tests::reserve_does_not_overallocate' panicked at 'assertion failed: `(left == right)`
---
    raw_vec::tests::reserve_does_not_overallocate

test result: FAILED. 371 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.13s

error: test failed, to rerun pass '-p alloc --lib'
