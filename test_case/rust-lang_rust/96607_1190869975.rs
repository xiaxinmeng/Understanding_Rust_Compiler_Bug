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
........................................................................................ 352/373
.....................
failures:

---- box_storage::tests::zst stdout ----
thread 'box_storage::tests::zst' panicked at 'assertion failed: `(left == right)`
  left: `0x564d45d310e0`,
 right: `0x1`', library/alloc/src/box_storage/tests.rs:88:5
---- box_storage::tests::zst_reserve_panic stdout ----
thread 'box_storage::tests::zst_reserve_panic' panicked at 'assertion failed: `(left == right)`
thread 'box_storage::tests::zst_reserve_panic' panicked at 'assertion failed: `(left == right)`
  left: `0x564d45d310e0`,
 right: `0x1`', library/alloc/src/box_storage/tests.rs:88:5
note: panic did not contain expected string
      panic message: `"assertion failed: `(left == right)`\n  left: `0x564d45d310e0`,\n right: `0x1`"`,
 expected substring: `"capacity overflow"`
thread 'box_storage::tests::zst_reserve_exact_panic' panicked at 'assertion failed: `(left == right)`
thread 'box_storage::tests::zst_reserve_exact_panic' panicked at 'assertion failed: `(left == right)`
  left: `0x564d45d310e0`,
 right: `0x1`', library/alloc/src/box_storage/tests.rs:88:5
note: panic did not contain expected string
note: panic did not contain expected string
      panic message: `"assertion failed: `(left == right)`\n  left: `0x564d45d310e0`,\n right: `0x1`"`,
 expected substring: `"capacity overflow"`
failures:
    box_storage::tests::zst
    box_storage::tests::zst_reserve_exact_panic
    box_storage::tests::zst_reserve_panic
    box_storage::tests::zst_reserve_panic

test result: FAILED. 370 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.13s

error: test failed, to rerun pass '-p alloc --lib'
Build completed unsuccessfully in 0:01:20
