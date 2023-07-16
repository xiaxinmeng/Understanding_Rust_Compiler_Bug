bash
# in the RA repo
$ git branch -D sync-from-rust

# in the Rust repo
$ git checkout rust-analyzer-2023-02-20
$ git subtree push -P src/tools/rust-analyzer rust-analyzer-local sync-from-rust
$ git checkout -b ra-new-sync
$ git subtree pull -P src/tools/rust-analyzer rust-analyzer-local sync-from-rust
$ git log --format=oneline
04c4eda971e945fa533880eb14fc6db314d8ed3e (HEAD -> ra-new-sync) Merge commit '7e711da2f07778d62f6411de5da520f1e260d761' into ra-new-sync
03288ebba35defc807952e6e55a0ab8f5f77aa83 (origin/rust-analyzer-2023-02-20, rust-analyzer-2023-02-20) :arrow_up: rust-analyzer
7e711da2f07778d62f6411de5da520f1e260d761 (rust-analyzer-local/sync-from-rust) :arrow_up: rust-analyzer
824f915cbc32c0942122389274a1b6fbe2ffc51e (upstream/master, upstream/HEAD, master) Auto merge of #108235 - tmiasko:read-buf, r=the8472
[snip]
