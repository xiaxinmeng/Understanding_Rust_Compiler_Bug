
error: failed to run `rustc` to learn about target-specific information

Caused by:
  process didn't exit successfully: `/data/omnios-build/omniosorg/r151042/_extra/rust-1.62.0/rustc-1.62.0-src/build/bootstrap/debug/rustc - --crate-name ___ --print=file-names --cfg=bootstrap -Csymbol-mangling-version=v0 -Zmacro-backtrace -Clink-args=-Wl,-z,origin '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Csplit-debuginfo=off -Ztls-model=initial-exec --target x86_64-unknown-illumos --crate-type bin --crate-type rlib --crate-type dylib --crate-type cdylib --crate-type staticlib --crate-type proc-macro --print=sysroot --print=cfg` (exit status: 1)
  --- stderr
  error: `-Csplit-debuginfo` is unstable on this platform
