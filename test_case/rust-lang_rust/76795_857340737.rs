
William@DESKTOP-H0PMN4M MINGW64 ~
$ rustup update
info: syncing channel updates for 'stable-x86_64-pc-windows-gnu'
info: latest update on 2021-05-10, rust version 1.52.1 (9bc8c42bb 2021-05-09)
info: downloading component 'cargo'
info: downloading component 'clippy'
info: downloading component 'rust-docs'
info: downloading component 'rust-mingw'
info: downloading component 'rust-std'
info: downloading component 'rustc'
info: downloading component 'rustfmt'
info: removing previous version of component 'cargo'
info: removing previous version of component 'clippy'
info: removing previous version of component 'rust-docs'
info: removing previous version of component 'rust-mingw'
info: removing previous version of component 'rust-std'
info: removing previous version of component 'rustc'
info: removing previous version of component 'rustfmt'
info: installing component 'cargo'
info: using up to 500.0 MiB of RAM to unpack components
info: installing component 'clippy'
info: installing component 'rust-docs'
info: installing component 'rust-mingw'
info: installing component 'rust-std'
info: installing component 'rustc'
info: installing component 'rustfmt'
info: syncing channel updates for 'nightly-x86_64-pc-windows-gnu'
info: latest update on 2021-06-09, rust version 1.54.0-nightly (ed597e7e1 2021-06-08)
info: downloading component 'rust-std' for 'riscv32i-unknown-none-elf'
info: downloading component 'clippy'
info: downloading component 'rust-std' for 'thumbv7em-none-eabihf'
info: downloading component 'rust-src'
info: downloading component 'cargo'
info: downloading component 'rust-docs'
info: downloading component 'rust-std'
info: downloading component 'rustc'
info: downloading component 'rustfmt'
info: removing previous version of component 'rust-std' for 'riscv32i-unknown-none-elf'
info: removing previous version of component 'clippy'
info: removing previous version of component 'rust-std' for 'thumbv7em-none-eabihf'
info: removing previous version of component 'rust-src'
info: removing previous version of component 'cargo'
info: removing previous version of component 'rust-docs'
info: removing previous version of component 'rust-std'
info: removing previous version of component 'rustc'
info: removing previous version of component 'rustfmt'
info: installing component 'rust-std' for 'riscv32i-unknown-none-elf'
info: installing component 'clippy'
info: installing component 'rust-std' for 'thumbv7em-none-eabihf'
info: installing component 'rust-src'
info: installing component 'cargo'
info: installing component 'rust-docs'
info: installing component 'rust-std'
info: installing component 'rustc'
error: error: 'File too big rustc-nightly-x86_64-pc-windows-gnu/rustc/bin/rustc_driver-3fd3f7c94716c1a7.dll 443569798'
info: installing component 'rustfmt'
info: checking for self-updates
info: downloading self-update

   stable-x86_64-pc-windows-gnu updated - rustc 1.52.1 (9bc8c42bb 2021-05-09) (from rustc 1.51.0 (2fd73fabe 2021-03-23))
  nightly-x86_64-pc-windows-gnu updated - rustc 1.54.0-nightly (ed597e7e1 2021-06-08) (from (timeout reading rustc version))

info: cleaning up downloads & tmp directories

William@DESKTOP-H0PMN4M MINGW64 ~
$ rustup update
C:\msys64\usr\bin\bash: /c/Users/William/.cargo/bin/rustup: Device or resource busy

William@DESKTOP-H0PMN4M MINGW64 ~
$ rustup update
info: syncing channel updates for 'stable-x86_64-pc-windows-gnu'
info: syncing channel updates for 'nightly-x86_64-pc-windows-gnu'
info: checking for self-updates

   stable-x86_64-pc-windows-gnu unchanged - rustc 1.52.1 (9bc8c42bb 2021-05-09)
  nightly-x86_64-pc-windows-gnu unchanged - rustc 1.54.0-nightly (ed597e7e1 2021-06-08)

info: cleaning up downloads & tmp directories

William@DESKTOP-H0PMN4M MINGW64 ~
$ rustc -V
rustc 1.54.0-nightly (ed597e7e1 2021-06-08)

William@DESKTOP-H0PMN4M MINGW64 ~
$ rustup -V
rustup 1.24.3 (ce5817a94 2021-05-31)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: The currently active `rustc` version is `rustc 1.54.0-nightly (ed597e7e1 2021-06-08)`
