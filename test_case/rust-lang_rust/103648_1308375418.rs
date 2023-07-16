
; RUSTUP_DIST_SERVER=https://dev-static.rust-lang.org rustup toolchain install nightly-2022-11-08
info: syncing channel updates for 'nightly-2022-11-08-x86_64-unknown-linux-gnu'
info: latest update on 2022-11-08, rust version 1.66.0-nightly (880ff5e01 2022-10-29)
info: downloading component 'cargo'
info: downloading component 'clippy'
info: downloading component 'rust-docs'
 19.0 MiB /  19.0 MiB (100 %)  12.5 MiB/s in  2s ETA:  0s
info: downloading component 'rust-std'
 29.6 MiB /  29.6 MiB (100 %)  12.4 MiB/s in  3s ETA:  0s
info: downloading component 'rustc'
 67.9 MiB /  67.9 MiB (100 %)  11.2 MiB/s in  6s ETA:  0s
info: downloading component 'rustfmt'
info: installing component 'cargo'
info: installing component 'clippy'
info: installing component 'rust-docs'
 19.0 MiB /  19.0 MiB (100 %) 894.4 KiB/s in 59s ETA:  0s    
info: installing component 'rust-std'
 29.6 MiB /  29.6 MiB (100 %)   8.2 MiB/s in  6s ETA:  0s
  6 IO-ops /   6 IO-ops (100 %)   0 IOPS in  6s ETA: Unknown
info: installing component 'rustc'
 67.9 MiB /  67.9 MiB (100 %)  13.3 MiB/s in  5s ETA:  0s
  5 IO-ops /   5 IO-ops (100 %)   0 IOPS in 16s ETA: Unknown
info: installing component 'rustfmt'

  nightly-2022-11-08-x86_64-unknown-linux-gnu installed - rustc 1.66.0-nightly (880ff5e01 2022-10-29)

info: checking for self-updates
(bash@pop-os) ~/rust-lang/rust2 [02:14:53, dry-run-progress]
; cargo +nightly-2022-11-08-x86_64-unknown-linux-gnu clippy --version
clippy 0.1.66 (880ff5e 2022-10-29)
(bash@pop-os) ~/rust-lang/rust2 [02:16:14, dry-run-progress]
; cargo +nightly-2022-11-08-x86_64-unknown-linux-gnu rustfmt --version
(bash@pop-os) ~/rust-lang/rust2 [02:16:24, dry-run-progress]
; cargo +nightly-2022-11-08-x86_64-unknown-linux-gnu fmt --version
rustfmt 1.5.1-nightly (880ff5e 2022-10-29)
