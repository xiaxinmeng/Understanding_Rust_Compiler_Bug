
[2020-09-17 13:39:53] 0 x10an14@x10-desktop:~/nav_github/rust-poc
-> $ rustup show profile
complete
[2020-09-17 13:40:24] 0 x10an14@x10-desktop:~/nav_github/rust-poc
-> $ rustup set profile default
info: profile set to 'default'
[2020-09-17 13:40:34] 0 x10an14@x10-desktop:~/nav_github/rust-poc
-> $ rustup override set nightly
info: syncing channel updates for 'nightly-x86_64-unknown-linux-gnu'
info: latest update on 2020-09-17, rust version 1.48.0-nightly (285fc7d70 2020-09-16)
info: downloading component 'cargo'
info: downloading component 'clippy'
info: downloading component 'rust-docs'
info: downloading component 'rust-std'
info: downloading component 'rustc'
 55.2 MiB /  55.2 MiB (100 %)  27.1 MiB/s in  2s ETA:  0s
info: downloading component 'rustfmt'
info: installing component 'cargo'
info: Defaulting to 500.0 MiB unpack ram
info: installing component 'clippy'
info: installing component 'rust-docs'
info: installing component 'rust-std'
 21.2 MiB /  21.2 MiB (100 %)  12.7 MiB/s in  1s ETA:  0s
info: installing component 'rustc'
 55.2 MiB /  55.2 MiB (100 %)  14.1 MiB/s in  3s ETA:  0s
info: installing component 'rustfmt'
info: override toolchain for '/home/x10an14/Documents/nav/github/rust-poc' set to 'nightly-x86_64-unknown-linux-gnu'

  nightly-x86_64-unknown-linux-gnu installed - rustc 1.48.0-nightly (285fc7d70 2020-09-16)

[2020-09-17 13:42:30] 0 x10an14@x10-desktop:~/nav_github/rust-poc                                                                                                                                                                                             (main=)
-> $ cargo build
   Compiling typenum v1.12.0
(...)
   Compiling rocket_http v0.4.5
   Compiling rust-poc v0.1.0 (/home/x10an14/Documents/nav/github/rust-poc)
    Finished dev [unoptimized + debuginfo] target(s) in 19.96s
[2020-09-17 13:45:00] 0 x10an14@x10-desktop:~/nav_github/rust-poc                                                                                                                                                                                             (main=)
->
