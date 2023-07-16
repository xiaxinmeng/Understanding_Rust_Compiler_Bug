
[2020-09-16 19:50:50] 0 x10an14@x10-desktop:~/nav_github/rust-poc                                               (main=)
-> $ rustup show
Default host: x86_64-unknown-linux-gnu
rustup home:  /home/x10an14/.rustup

stable-x86_64-unknown-linux-gnu (default)
rustc 1.46.0 (04488afe3 2020-08-24)
[2020-09-16 19:52:21] 0 x10an14@x10-desktop:~/nav_github/rust-poc                                                                                                                                                                                             (main=)
-> $ la .git
total 52K
4.0K drwxr-xr-x 8 x10an14 x10an14 4.0K Sep 16 19:41 ./
4.0K drwxr-xr-x 5 x10an14 x10an14 4.0K Sep 16 19:44 ../
4.0K drwxr-xr-x 2 x10an14 x10an14 4.0K Sep 16 19:41 branches/
4.0K -rw-r--r-- 1 x10an14 x10an14  255 Sep 16 19:41 config
4.0K -rw-r--r-- 1 x10an14 x10an14   73 Sep 16 19:41 description
4.0K -rw-r--r-- 1 x10an14 x10an14   21 Sep 16 19:41 HEAD
4.0K drwxr-xr-x 2 x10an14 x10an14 4.0K Sep 16 19:41 hooks/
4.0K -rw-r--r-- 1 x10an14 x10an14  889 Sep 16 19:41 index
4.0K drwxr-xr-x 2 x10an14 x10an14 4.0K Sep 16 19:41 info/
4.0K drwxr-xr-x 3 x10an14 x10an14 4.0K Sep 16 19:41 logs/
4.0K drwxr-xr-x 4 x10an14 x10an14 4.0K Sep 16 19:41 objects/
4.0K -rw-r--r-- 1 x10an14 x10an14  112 Sep 16 19:41 packed-refs
4.0K drwxr-xr-x 5 x10an14 x10an14 4.0K Sep 16 19:41 refs/
[2020-09-16 19:52:27] 0 x10an14@x10-desktop:~/nav_github/rust-poc                                                                                                                                                                                             (main=)
-> $ rustup override set nightly
info: syncing channel updates for 'nightly-x86_64-unknown-linux-gnu'
info: latest update on 2020-09-16, rust version 1.48.0-nightly (6af1bdda5 2020-09-15)
info: downloading component 'cargo'
info: downloading component 'clippy'
info: downloading component 'llvm-tools-preview'
info: downloading component 'miri'
info: downloading component 'rls'
info: downloading component 'rust-analysis'
info: downloading component 'rust-analyzer-preview'
info: downloading component 'rust-docs'
info: downloading component 'rust-src'
info: downloading component 'rust-std'
info: downloading component 'rustc'
info: downloading component 'rustc-dev'
info: downloading component 'rustc-docs'
info: downloading component 'rustfmt'
info: installing component 'cargo'
info: Defaulting to 500.0 MiB unpack ram
info: installing component 'clippy'
info: installing component 'llvm-tools-preview'
 20.0 MiB /  20.0 MiB (100 %)  14.7 MiB/s in  1s ETA:  0s
info: installing component 'miri'
info: installing component 'rls'
info: installing component 'rust-analysis'
info: installing component 'rust-analyzer-preview'
info: installing component 'rust-docs'
 13.1 MiB /  13.1 MiB (100 %)  13.1 MiB/s in  1s ETA:  0s
info: installing component 'rust-src'
info: installing component 'rust-std'
 21.2 MiB /  21.2 MiB (100 %)  12.6 MiB/s in  1s ETA:  0s
info: installing component 'rustc'
 55.2 MiB /  55.2 MiB (100 %)  14.1 MiB/s in  3s ETA:  0s
info: installing component 'rustc-dev'
 89.5 MiB /  89.5 MiB (100 %)  13.6 MiB/s in  6s ETA:  0s
info: installing component 'rustc-docs'
info: rolling back changes
error: failed to install component: 'rustc-docs-x86_64-unknown-linux-gnu', detected conflict: '"share/doc/rust/html/rustc"'
[2020-09-16 19:52:52] 1 x10an14@x10-desktop:~/nav_github/rust-poc                                                                                                                                                                                             (main=)
-> $ 
