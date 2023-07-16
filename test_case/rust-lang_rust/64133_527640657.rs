plain
2019-09-03T21:02:29.6897354Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-03T21:02:29.7093142Z ##[command]git config gc.auto 0
2019-09-03T21:02:29.7167218Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-03T21:02:29.7215054Z ##[command]git config --get-all http.proxy
2019-09-03T21:02:29.7345903Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64133/merge:refs/remotes/pull/64133/merge
---
2019-09-03T21:06:20.5028415Z ##                                                                         3.3%
2019-09-03T21:06:20.5704832Z ###################                                                       26.4%
2019-09-03T21:06:20.5705073Z ######################################################################## 100.0%
2019-09-03T21:06:21.6766893Z extracting /checkout/obj/build/cache/2019-08-13/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-09-03T21:06:21.7348358Z error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
2019-09-03T21:06:21.7349344Z Caused by:
2019-09-03T21:06:21.7349522Z   failed to load source for a dependency on `backtrace`
2019-09-03T21:06:21.7349748Z 
2019-09-03T21:06:21.7349924Z Caused by:
2019-09-03T21:06:21.7349924Z Caused by:
2019-09-03T21:06:21.7350349Z   Unable to update /backtrace-rs
2019-09-03T21:06:21.7350798Z Caused by:
2019-09-03T21:06:21.7350798Z Caused by:
2019-09-03T21:06:21.7351168Z   failed to read `/backtrace-rs/Cargo.toml`
2019-09-03T21:06:21.7351604Z Caused by:
2019-09-03T21:06:21.7351776Z   No such file or directory (os error 2)
2019-09-03T21:06:21.7352439Z failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml
2019-09-03T21:06:21.7353122Z Build completed unsuccessfully in 0:00:27
2019-09-03T21:06:21.7353122Z Build completed unsuccessfully in 0:00:27
2019-09-03T21:06:21.7377629Z == clock drift check ==
2019-09-03T21:06:21.7384617Z   local time: Tue Sep  3 21:06:21 UTC 2019
2019-09-03T21:06:21.8316942Z   network time: Tue, 03 Sep 2019 21:06:21 GMT
2019-09-03T21:06:21.8320322Z == end clock drift check ==
2019-09-03T21:06:42.2675295Z ##[error]Bash exited with code '1'.
2019-09-03T21:06:42.2711173Z ##[section]Starting: Checkout
2019-09-03T21:06:42.2712799Z ==============================================================================
2019-09-03T21:06:42.2712843Z Task         : Get sources
2019-09-03T21:06:42.2712918Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
