
~ $ rustup update nightly
info: syncing channel updates for 'nightly-x86_64-unknown-linux-gnu'
info: downloading component 'rustc'
 53.2 MiB /  53.2 MiB (100 %)  23.8 MiB/s ETA:   0 s
info: downloading component 'rust-std'
 84.0 MiB /  84.0 MiB (100 %)  55.7 MiB/s ETA:   0 s
info: downloading component 'cargo'
info: downloading component 'rust-docs'
info: downloading component 'rust-std' for 'x86_64-unknown-linux-musl'
 17.9 MiB /  17.9 MiB (100 %)   3.5 MiB/s ETA:   0 s
info: installing component 'rustc'
info: installing component 'rust-std'
info: installing component 'cargo'
info: installing component 'rust-docs'
info: installing component 'rust-std' for 'x86_64-unknown-linux-musl'

  nightly-x86_64-unknown-linux-gnu updated - rustc 1.21.0-nightly (2aeb5930f 2017-08-25)

~ $ echo 'fn main() {}' > x.rs
~ $ rustc --target x86_64-unknown-linux-musl x.rs
~ $ ./x
~ $ ldd x
        not a dynamic executable
~ $ file x
x: ELF 64-bit LSB  executable, x86-64, version 1 (GNU/Linux), statically linked, BuildID[sha1]=76e65b3d762c26af39227c47141f4f67f7b20a55, not stripped
