plain
2019-11-20T04:13:23.1717002Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-20T04:13:23.1904076Z ##[command]git config gc.auto 0
2019-11-20T04:13:23.1986391Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-20T04:13:23.2060600Z ##[command]git config --get-all http.proxy
2019-11-20T04:13:23.2197521Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66527/merge:refs/remotes/pull/66527/merge
---
2019-11-20T04:16:19.9566573Z #################################################                         69.3%
2019-11-20T04:16:19.9567624Z ######################################################################## 100.0%
2019-11-20T04:16:20.3532368Z extracting /checkout/obj/build/cache/2019-11-06/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-11-20T04:16:20.4355388Z     Updating crates.io index
2019-11-20T04:16:26.5158840Z     Updating git repository `https://github.com/spastorino/rustc-rayon`
---
2019-11-20T04:19:28.6128603Z * highest error code: E0744
2019-11-20T04:19:28.9989370Z * 270 features
2019-11-20T04:19:29.6717509Z Dependencies not on the whitelist:
2019-11-20T04:19:29.6718293Z * crossbeam-channel 
2019-11-20T04:19:29.6726720Z invalid source: "git+https://github.com/spastorino/rustc-rayon?branch=latch-target-thread-rustc#826207502d9b26fdf71ea9c7db63f05ab04486d0"
2019-11-20T04:19:29.6727138Z invalid source: "git+https://github.com/spastorino/rustc-rayon?branch=latch-target-thread-rustc#826207502d9b26fdf71ea9c7db63f05ab04486d0"
2019-11-20T04:19:29.9216245Z some tidy checks failed
2019-11-20T04:19:29.9224319Z Found 441 error codes
2019-11-20T04:19:29.9226043Z Found 0 error codes with no tests
2019-11-20T04:19:29.9226265Z Done!
2019-11-20T04:19:29.9226265Z Done!
2019-11-20T04:19:29.9226400Z 
2019-11-20T04:19:29.9226804Z 
2019-11-20T04:19:29.9227850Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-20T04:19:29.9228233Z 
2019-11-20T04:19:29.9228347Z 
2019-11-20T04:19:29.9234944Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-20T04:19:29.9235040Z Build completed unsuccessfully in 0:01:32
2019-11-20T04:19:29.9235040Z Build completed unsuccessfully in 0:01:32
2019-11-20T04:19:29.9276602Z == clock drift check ==
2019-11-20T04:19:29.9284154Z   local time: Wed Nov 20 04:19:29 UTC 2019
2019-11-20T04:19:30.1973287Z   network time: Wed, 20 Nov 2019 04:19:30 GMT
2019-11-20T04:19:30.1977332Z == end clock drift check ==
2019-11-20T04:19:31.5092685Z 
2019-11-20T04:19:31.5191007Z ##[error]Bash exited with code '1'.
2019-11-20T04:19:31.5241238Z ##[section]Starting: Checkout
2019-11-20T04:19:31.5242920Z ==============================================================================
2019-11-20T04:19:31.5242975Z Task         : Get sources
2019-11-20T04:19:31.5243042Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
