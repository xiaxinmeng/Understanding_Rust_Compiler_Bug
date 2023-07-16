plain
2019-08-24T22:50:57.5893231Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-24T22:50:57.6065372Z ##[command]git config gc.auto 0
2019-08-24T22:50:57.6134098Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-24T22:50:57.6195574Z ##[command]git config --get-all http.proxy
2019-08-24T22:50:57.6329184Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63676/merge:refs/remotes/pull/63676/merge
---
2019-08-24T22:51:31.4530203Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-24T22:51:31.4530228Z 
2019-08-24T22:51:31.4530420Z   git checkout -b <new-branch-name>
2019-08-24T22:51:31.4530444Z 
2019-08-24T22:51:31.4530482Z HEAD is now at e5d10b8ac Merge 37721461d47d3840adc6d931b848a9db8e66ceaa into eeba189cfb2cfc5c5898513352d4ca8f1df06e05
2019-08-24T22:51:31.4684306Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-24T22:51:31.4687028Z ==============================================================================
2019-08-24T22:51:31.4687105Z Task         : Bash
2019-08-24T22:51:31.4687150Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-24T22:54:45.9341685Z ####################                                                      28.3%
2019-08-24T22:54:45.9345543Z ######################################################################## 100.0%
2019-08-24T22:54:46.3031725Z extracting /checkout/obj/build/cache/2019-08-13/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-08-24T22:54:46.3717261Z     Updating crates.io index
2019-08-24T22:55:15.7622410Z     Updating git repository `https://github.com/newpavlov/rust-wasi`
2019-08-24T22:55:16.4899738Z error: failed to write /checkout/Cargo.lock
2019-08-24T22:55:16.4900078Z Caused by:
2019-08-24T22:55:16.4900120Z   failed to open: /checkout/Cargo.lock
2019-08-24T22:55:16.4900246Z 
2019-08-24T22:55:16.4900278Z Caused by:
2019-08-24T22:55:16.4900278Z Caused by:
2019-08-24T22:55:16.4900922Z   Read-only file system (os error 30)
2019-08-24T22:55:16.4930162Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml
2019-08-24T22:55:16.4972896Z == clock drift check ==
2019-08-24T22:55:16.4991854Z   local time: Sat Aug 24 22:55:16 UTC 2019
2019-08-24T22:55:16.6459611Z   network time: Sat, 24 Aug 2019 22:55:16 GMT
2019-08-24T22:55:16.6465100Z == end clock drift check ==
2019-08-24T22:55:16.6465100Z == end clock drift check ==
2019-08-24T22:55:23.3181912Z ##[error]Bash exited with code '1'.
2019-08-24T22:55:23.3235363Z ##[section]Starting: Checkout
2019-08-24T22:55:23.3237358Z ==============================================================================
2019-08-24T22:55:23.3237419Z Task         : Get sources
2019-08-24T22:55:23.3237467Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
