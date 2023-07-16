plain
2019-11-21T15:08:39.9460283Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-21T15:08:39.9681933Z ##[command]git config gc.auto 0
2019-11-21T15:08:39.9746673Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-21T15:08:39.9805519Z ##[command]git config --get-all http.proxy
2019-11-21T15:08:39.9954754Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66608/merge:refs/remotes/pull/66608/merge
---
2019-11-21T15:11:59.3291645Z #################################################                         68.8%
2019-11-21T15:11:59.3291744Z ######################################################################## 100.0%
2019-11-21T15:11:59.7710818Z extracting /checkout/obj/build/cache/2019-11-06/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-11-21T15:11:59.8599561Z     Updating crates.io index
2019-11-21T15:12:06.7479880Z     Updating git repository `https://github.com/spastorino/rustc-rayon`
---
2019-11-21T15:14:56.8193061Z * highest error code: E0744
2019-11-21T15:14:57.2204444Z * 270 features
2019-11-21T15:14:57.9343470Z Dependencies not on the whitelist:
2019-11-21T15:14:57.9344883Z * crossbeam-channel 
2019-11-21T15:14:57.9350799Z invalid source: "git+https://github.com/spastorino/rustc-rayon?branch=latch-target-thread-rustc#dd85f978633ae564515b7d1c82ba3d1cff76b1f1"
2019-11-21T15:14:57.9351597Z invalid source: "git+https://github.com/spastorino/rustc-rayon?branch=latch-target-thread-rustc#dd85f978633ae564515b7d1c82ba3d1cff76b1f1"
2019-11-21T15:14:58.1940954Z some tidy checks failed
2019-11-21T15:14:58.1946020Z Found 441 error codes
2019-11-21T15:14:58.1946332Z Found 0 error codes with no tests
2019-11-21T15:14:58.1946764Z Done!
2019-11-21T15:14:58.1946764Z Done!
2019-11-21T15:14:58.1952902Z 
2019-11-21T15:14:58.1953180Z 
2019-11-21T15:14:58.1954664Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-21T15:14:58.1955964Z 
2019-11-21T15:14:58.1956275Z 
2019-11-21T15:14:58.1956478Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-21T15:14:58.1956630Z Build completed unsuccessfully in 0:01:33
2019-11-21T15:14:58.1956630Z Build completed unsuccessfully in 0:01:33
2019-11-21T15:14:58.1999680Z == clock drift check ==
2019-11-21T15:14:58.2008292Z   local time: Thu Nov 21 15:14:58 UTC 2019
2019-11-21T15:14:58.4859302Z   network time: Thu, 21 Nov 2019 15:14:58 GMT
2019-11-21T15:14:58.4859411Z == end clock drift check ==
2019-11-21T15:14:59.7557073Z 
2019-11-21T15:14:59.7670767Z ##[error]Bash exited with code '1'.
2019-11-21T15:14:59.7702390Z ##[section]Starting: Checkout
2019-11-21T15:14:59.7704091Z ==============================================================================
2019-11-21T15:14:59.7704147Z Task         : Get sources
2019-11-21T15:14:59.7704209Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
