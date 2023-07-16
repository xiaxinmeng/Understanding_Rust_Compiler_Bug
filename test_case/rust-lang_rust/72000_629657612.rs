
root@44e56aad1805:/# rustup component add rustc-dev --toolchain nightly
info: downloading component 'rustc-dev'
 87.3 MiB /  87.3 MiB (100 %)  39.8 MiB/s in  3s ETA:  0s
info: installing component 'rustc-dev'
 87.3 MiB /  87.3 MiB (100 %)   9.1 MiB/s in  9s ETA:  0s
root@44e56aad1805:/# rustup component add llvm-tools-preview --toolchain nightly
info: downloading component 'llvm-tools-preview'
info: installing component 'llvm-tools-preview'
 18.0 MiB /  18.0 MiB (100 %)   9.6 MiB/s in  1s ETA:  0s
info: rolling back changes
error: failed to install component: 'llvm-tools-preview-x86_64-unknown-linux-gnu', detected conflict: '"lib/rustlib/x86_64-unknown-linux-gnu/lib/libLLVM-9-rust-1.45.0-nightly.so"'
root@44e56aad1805:/#
