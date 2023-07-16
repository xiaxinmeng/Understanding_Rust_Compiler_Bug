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
........................................................................................ 616/651
...................................
failures:

error: test failed, to rerun pass `-p alloc --test collectionstests`
---- slice::test_in_place_iterator_specialization stdout ----
thread 'slice::test_in_place_iterator_specialization' panicked at 'assertion failed: `(left == right)`
  left: `0x7f12040014c0`,
 right: `0x7f1204001940`', library/alloc/tests/slice.rs:1407:5

failures:
    slice::test_in_place_iterator_specialization

