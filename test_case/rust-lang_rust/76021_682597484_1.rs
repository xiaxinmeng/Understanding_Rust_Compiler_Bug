
info: syncing channel updates for '1.46.0-x86_64-unknown-linux-musl'
info: latest update on 2020-08-27, rust version 1.46.0 (04488afe3 2020-08-24)
info: downloading component 'cargo'
info: downloading component 'clippy'
info: downloading component 'rust-std'
info: downloading component 'rustc'
info: downloading component 'rustfmt'
info: installing component 'cargo'
info: Defaulting to 500.0 MiB unpack ram
info: installing component 'clippy'
info: installing component 'rust-std'
info: installing component 'rustc'
info: installing component 'rustfmt'
    Updating crates.io index
 Downloading crates ...
  Downloaded rand_chacha v0.2.2
  Downloaded rand_core v0.5.1
  Downloaded cfg-if v0.1.10
  Downloaded getrandom v0.1.14
  Downloaded ppv-lite86 v0.2.9
  Downloaded libc v0.2.76
  Downloaded rand v0.7.3
   Compiling libc v0.2.76
   Compiling getrandom v0.1.14
   Compiling cfg-if v0.1.10
   Compiling ppv-lite86 v0.2.9
error: failed to run custom build command for `getrandom v0.1.14`

Caused by:
  process didn't exit successfully: `/tmp/app/target/debug/build/getrandom-64ae542341c0eb62/build-script-build` (signal: 11, SIGSEGV: invalid memory reference)
warning: build failed, waiting for other jobs to finish...
