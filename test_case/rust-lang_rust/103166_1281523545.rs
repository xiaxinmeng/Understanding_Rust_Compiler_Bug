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
........................................................................................ 352/409
.........................................................
failures:

---- iter::bench_copied_array_chunks stdout ----
thread 'iter::bench_copied_array_chunks' panicked at 'attempt to add with overflow', /checkout/library/core/src/iter/traits/accum.rs:141:1


failures:
    iter::bench_copied_array_chunks
    iter::bench_copied_array_chunks

test result: FAILED. 408 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

error: test failed, to rerun pass `-p core --bench corebenches`
