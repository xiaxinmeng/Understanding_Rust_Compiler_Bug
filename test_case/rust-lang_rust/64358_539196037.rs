plain
2019-10-07T20:24:52.0038856Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-07T20:24:52.0299661Z ##[command]git config gc.auto 0
2019-10-07T20:24:52.0380635Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-07T20:24:52.0436580Z ##[command]git config --get-all http.proxy
2019-10-07T20:24:52.0579651Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64358/merge:refs/remotes/pull/64358/merge
---
2019-10-07T20:31:57.8478355Z * crossbeam-queue 
2019-10-07T20:31:57.8939953Z some tidy checks failed
2019-10-07T20:31:57.8940022Z 
2019-10-07T20:31:57.8940047Z 
2019-10-07T20:31:57.8941142Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-07T20:31:57.8941323Z 
2019-10-07T20:31:57.8941350Z 
2019-10-07T20:31:57.8995316Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-07T20:31:57.8995563Z Build completed unsuccessfully in 0:01:31
2019-10-07T20:31:57.8995563Z Build completed unsuccessfully in 0:01:31
2019-10-07T20:31:57.9001450Z == clock drift check ==
2019-10-07T20:31:57.9016364Z   local time: Mon Oct  7 20:31:57 UTC 2019
2019-10-07T20:31:57.9979543Z   network time: Mon, 07 Oct 2019 20:31:57 GMT
2019-10-07T20:31:59.4248337Z == end clock drift check ==
2019-10-07T20:31:59.4337921Z ##[error]Bash exited with code '1'.
2019-10-07T20:31:59.4381277Z ##[section]Starting: Checkout
2019-10-07T20:31:59.4383161Z ==============================================================================
2019-10-07T20:31:59.4383220Z Task         : Get sources
2019-10-07T20:31:59.4383276Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
