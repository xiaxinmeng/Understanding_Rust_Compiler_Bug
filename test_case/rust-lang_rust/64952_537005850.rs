plain
2019-10-01T12:03:41.7201026Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-01T12:03:41.7405718Z ##[command]git config gc.auto 0
2019-10-01T12:03:41.7492903Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-01T12:03:41.7943647Z ##[command]git config --get-all http.proxy
2019-10-01T12:03:41.8083774Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64952/merge:refs/remotes/pull/64952/merge
---
2019-10-01T12:07:24.3747495Z extracting /checkout/obj/build/cache/2019-09-25/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-10-01T12:07:24.4494887Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-10-01T12:07:24.4495916Z 
2019-10-01T12:07:24.4496316Z Caused by:
2019-10-01T12:07:24.4497281Z   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2019-10-01T12:07:24.4508016Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml
2019-10-01T12:07:24.4564295Z == clock drift check ==
2019-10-01T12:07:24.4574409Z   local time: Tue Oct  1 12:07:24 UTC 2019
2019-10-01T12:07:24.4817653Z   network time: Tue, 01 Oct 2019 12:07:24 GMT
2019-10-01T12:07:24.4824648Z == end clock drift check ==
2019-10-01T12:07:24.4824648Z == end clock drift check ==
2019-10-01T12:07:45.7184274Z ##[error]Bash exited with code '1'.
2019-10-01T12:07:45.7233735Z ##[section]Starting: Checkout
2019-10-01T12:07:45.7235745Z ==============================================================================
2019-10-01T12:07:45.7235803Z Task         : Get sources
2019-10-01T12:07:45.7235851Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
