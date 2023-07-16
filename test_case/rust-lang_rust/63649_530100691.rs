plain
2019-09-10T20:09:42.3065342Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-10T20:09:42.3248007Z ##[command]git config gc.auto 0
2019-09-10T20:09:42.3329747Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-10T20:09:42.3389677Z ##[command]git config --get-all http.proxy
2019-09-10T20:09:42.3544289Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63649/merge:refs/remotes/pull/63649/merge
---
2019-09-10T20:13:05.1963907Z ##############################################################            87.1%
2019-09-10T20:13:05.2109876Z ######################################################################    97.7%
2019-09-10T20:13:05.2111409Z ######################################################################## 100.0%
2019-09-10T20:13:05.6136017Z extracting /checkout/obj/build/cache/2019-08-13/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-09-10T20:13:05.6812218Z error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
2019-09-10T20:13:05.6812411Z Caused by:
2019-09-10T20:13:05.6812411Z Caused by:
2019-09-10T20:13:05.6812985Z   patch for `rustfmt-nightly` in `https://github.com/rust-lang/crates.io-index` did not resolve to any crates. If this is unexpected, you may wish to consult: https://github.com/rust-lang/cargo/issues/4678
2019-09-10T20:13:05.6859209Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml
2019-09-10T20:13:05.6874376Z == clock drift check ==
2019-09-10T20:13:05.6885878Z   local time: Tue Sep 10 20:13:05 UTC 2019
2019-09-10T20:13:05.8456023Z   network time: Tue, 10 Sep 2019 20:13:05 GMT
2019-09-10T20:13:05.8460749Z == end clock drift check ==
2019-09-10T20:13:05.8460749Z == end clock drift check ==
2019-09-10T20:13:25.7879988Z ##[error]Bash exited with code '1'.
2019-09-10T20:13:25.7916132Z ##[section]Starting: Checkout
2019-09-10T20:13:25.7918119Z ==============================================================================
2019-09-10T20:13:25.7918202Z Task         : Get sources
2019-09-10T20:13:25.7918254Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
