plain
2019-08-25T08:35:51.2363920Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-25T08:35:51.2562107Z ##[command]git config gc.auto 0
2019-08-25T08:35:52.1058168Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-25T08:35:52.1069551Z ##[command]git config --get-all http.proxy
2019-08-25T08:35:52.1074419Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63127/merge:refs/remotes/pull/63127/merge
---
2019-08-25T08:36:26.9508651Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-25T08:36:26.9508907Z 
2019-08-25T08:36:26.9509331Z   git checkout -b <new-branch-name>
2019-08-25T08:36:26.9509706Z 
2019-08-25T08:36:26.9510036Z HEAD is now at f0349bd54 Merge 2389278fdc1708f9b0a66273cb69f636c3ddc8fb into 783469ca09005d135c3204a55069707d1cd705a9
2019-08-25T08:36:26.9672809Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-25T08:36:26.9675057Z ==============================================================================
2019-08-25T08:36:26.9675121Z Task         : Bash
2019-08-25T08:36:26.9675157Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-25T08:39:42.1863372Z extracting /checkout/obj/build/cache/2019-08-13/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-08-25T08:39:42.2486635Z error: failed to resolve patches for `https://github.com/rust-lang/cargo`
2019-08-25T08:39:42.2486695Z 
2019-08-25T08:39:42.2486737Z Caused by:
2019-08-25T08:39:42.2487647Z   patch for `cargo` in `https://github.com/rust-lang/cargo` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2019-08-25T08:39:42.2491339Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml
2019-08-25T08:39:42.2541201Z == clock drift check ==
2019-08-25T08:39:42.2550062Z   local time: Sun Aug 25 08:39:42 UTC 2019
2019-08-25T08:39:42.3375544Z   network time: Sun, 25 Aug 2019 08:39:42 GMT
2019-08-25T08:39:42.3380382Z == end clock drift check ==
2019-08-25T08:39:42.3380382Z == end clock drift check ==
2019-08-25T08:40:02.2591765Z ##[error]Bash exited with code '1'.
2019-08-25T08:40:02.2633111Z ##[section]Starting: Checkout
2019-08-25T08:40:02.2635421Z ==============================================================================
2019-08-25T08:40:02.2635665Z Task         : Get sources
2019-08-25T08:40:02.2635710Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
