plain
2019-11-25T17:32:31.5965590Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-25T17:32:31.6156061Z ##[command]git config gc.auto 0
2019-11-25T17:32:31.6224065Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-25T17:32:31.6309130Z ##[command]git config --get-all http.proxy
2019-11-25T17:32:31.6473412Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66750/merge:refs/remotes/pull/66750/merge
---
2019-11-25T17:35:33.7520118Z 
2019-11-25T17:35:33.7570656Z ####################################################################      94.8%
2019-11-25T17:35:33.7570780Z ######################################################################## 100.0%
2019-11-25T17:35:34.1821946Z extracting /checkout/obj/build/cache/2019-11-06/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-11-25T17:35:34.2588157Z     Updating git repository `https://github.com/bytecodealliance/wasi`
---
2019-11-25T17:38:34.6915938Z tidy check
2019-11-25T17:38:36.1058729Z * 589 error codes
2019-11-25T17:38:36.1059565Z * highest error code: E0745
2019-11-25T17:38:36.4987584Z * 273 features
2019-11-25T17:38:37.2369622Z invalid source: "git+https://github.com/bytecodealliance/wasi#686e1345862e4efd2630b779f5b992d418658cf2"
2019-11-25T17:38:37.5184687Z some tidy checks failed
2019-11-25T17:38:37.5184840Z Found 486 error codes
2019-11-25T17:38:37.5184895Z Found 0 error codes with no tests
2019-11-25T17:38:37.5184978Z Done!
2019-11-25T17:38:37.5184978Z Done!
2019-11-25T17:38:37.5185012Z 
2019-11-25T17:38:37.5185042Z 
2019-11-25T17:38:37.5185977Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-25T17:38:37.5186124Z 
2019-11-25T17:38:37.5186153Z 
2019-11-25T17:38:37.5216130Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-25T17:38:37.5216328Z Build completed unsuccessfully in 0:01:33
2019-11-25T17:38:37.5216328Z Build completed unsuccessfully in 0:01:33
2019-11-25T17:38:37.5242678Z == clock drift check ==
2019-11-25T17:38:37.5254015Z   local time: Mon Nov 25 17:38:37 UTC 2019
2019-11-25T17:38:38.0656139Z   network time: Mon, 25 Nov 2019 17:38:38 GMT
2019-11-25T17:38:38.0658769Z == end clock drift check ==
2019-11-25T17:38:39.3999177Z 
2019-11-25T17:38:39.4122723Z ##[error]Bash exited with code '1'.
2019-11-25T17:38:39.4152571Z ##[section]Starting: Checkout
2019-11-25T17:38:39.4154345Z ==============================================================================
2019-11-25T17:38:39.4154404Z Task         : Get sources
2019-11-25T17:38:39.4154475Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
