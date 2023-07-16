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
test tests::is_test_test ... ok
test header::tests::test_extract_version_range ... ok
test header::tests::revisions ... ok
test header::tests::aux_build ... ok
fatal runtime error: failed to initiate panic, error 5
error: test failed, to rerun pass `--bin compiletest`
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools/x86_64-unknown-linux-gnu/release/deps/compiletest-c2ecd793fe762a8c` (signal: 6, SIGABRT: process abort signal)
