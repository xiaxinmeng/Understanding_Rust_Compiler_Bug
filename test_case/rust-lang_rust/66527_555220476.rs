plain
2019-11-18T21:21:32.6856165Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-18T21:21:32.7009947Z ##[command]git config gc.auto 0
2019-11-18T21:21:32.7073274Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-18T21:21:32.7116203Z ##[command]git config --get-all http.proxy
2019-11-18T21:21:32.7236102Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66527/merge:refs/remotes/pull/66527/merge
---
2019-11-18T21:24:26.1598335Z ##########                                                                14.9%
2019-11-18T21:24:26.1599294Z ######################################################################## 100.0%
2019-11-18T21:24:26.4820282Z extracting /checkout/obj/build/cache/2019-11-06/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-11-18T21:24:26.5513032Z     Updating crates.io index
2019-11-18T21:24:31.9734009Z     Updating git repository `https://github.com/spastorino/rustc-rayon`
---
2019-11-18T21:27:04.8004322Z * highest error code: E0744
2019-11-18T21:27:05.1239822Z * 270 features
2019-11-18T21:27:05.6891324Z Dependencies not on the whitelist:
2019-11-18T21:27:05.6892209Z * crossbeam-channel 
2019-11-18T21:27:05.6898195Z invalid source: "git+https://github.com/spastorino/rustc-rayon?branch=latch-target-thread-rustc#a8dc7ab6e4515aae06a3982f495c12acf59e2bc0"
2019-11-18T21:27:05.6898557Z invalid source: "git+https://github.com/spastorino/rustc-rayon?branch=latch-target-thread-rustc#a8dc7ab6e4515aae06a3982f495c12acf59e2bc0"
2019-11-18T21:27:06.7132706Z some tidy checks failed
2019-11-18T21:27:06.7132988Z Found 441 error codes
2019-11-18T21:27:06.7133226Z Found 0 error codes with no tests
2019-11-18T21:27:06.7133418Z Done!
2019-11-18T21:27:06.7133418Z Done!
2019-11-18T21:27:06.7133574Z 
2019-11-18T21:27:07.0117016Z 
2019-11-18T21:27:07.0117943Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-18T21:27:07.0118073Z 
2019-11-18T21:27:07.0118099Z 
2019-11-18T21:27:07.0118146Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-18T21:27:07.0118193Z Build completed unsuccessfully in 0:01:20
2019-11-18T21:27:07.0118193Z Build completed unsuccessfully in 0:01:20
2019-11-18T21:27:07.0118254Z == clock drift check ==
2019-11-18T21:27:07.0118297Z   local time: Mon Nov 18 21:27:05 UTC 2019
2019-11-18T21:27:07.0118342Z   network time: Mon, 18 Nov 2019 21:27:06 GMT
2019-11-18T21:27:07.0118402Z == end clock drift check ==
2019-11-18T21:27:07.5830348Z 
2019-11-18T21:27:07.5916820Z ##[error]Bash exited with code '1'.
2019-11-18T21:27:07.5941074Z ##[section]Starting: Checkout
2019-11-18T21:27:07.5942938Z ==============================================================================
2019-11-18T21:27:07.5942989Z Task         : Get sources
2019-11-18T21:27:07.5943033Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
