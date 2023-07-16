
> x clippy
Checking stage0 library artifacts (aarch64-unknown-linux-gnu -> aarch64-unknown-linux-gnu)
error: failed to run `rustc` to learn about target-specific information

Caused by:
  process didn't exit successfully: `/home/gh-jyn514/.local/lib/rustup/toolchains/nightly-aarch64-unknown-linux-gnu/bin/clippy-driver rustc - --crate-name ___ --print=file-names --crate-type bin --crate-type rlib --crate-type dylib --crate-type cdylib --crate-type staticlib --crate-type proc-macro --print=sysroot --print=cfg` (exit status: 1)
  --- stderr
  error: Unrecognized option: 'message-format'
