
Building stage0 tool build-manifest (x86_64-unknown-linux-gnu(x86_64-unknown-linux-gnu))
error: failed to run `rustc` to learn about target-specific information

Caused by:
  process didn't exit successfully: `/home/jnelson/rust-lang/rust/target/bootstrap/debug/rustc - --crate-name ___ --print=file-names --cfg=bootstrap -Csymbol-mangling-version=v0 -Zmacro-backtrace -Clink-args=-Wl,-z,origin '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Ztls-model=initial-exec -Cllvm-args=-import-instr-limit=10 --target x86_64-unknown-linux-gnu --crate-type bin --crate-type rlib --crate-type dylib --crate-type cdylib --crate-type staticlib --crate-type proc-macro --print=sysroot --print=cfg` (exit status: 1)
  --- stderr
  error: the option `Z` is only accepted on the nightly compiler
