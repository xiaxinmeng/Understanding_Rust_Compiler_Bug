
Generating unstable book md files (x86_64-pc-solaris)
Building stage0 tool unstable-book-gen (x86_64-pc-solaris)
    Updating crates.io index
warning: spurious network error (3 tries remaining): [35] SSL connect error (OpenSSL/1.0.2zg: error:140770FC:SSL routines:SSL23_GET_SERVER_HELLO:unknown protocol)
error: failed to get `opener` as a dependency of package `clippy_dev v0.0.1 (/builds/psumbera/rust-lang-build/src/tools/clippy/clippy_dev)`

Caused by:
  failed to query replaced source registry `crates-io`

Caused by:
  download of op/en/opener failed

Caused by:
  failed to download from `https://index.crates.io/op/en/opener`

Caused by:
  [1] Unsupported protocol (Received HTTP/0.9 when not allowed)
Building bootstrap
Build completed unsuccessfully in 0:06:43
gmake: *** [Makefile:49: dist] Error 1
