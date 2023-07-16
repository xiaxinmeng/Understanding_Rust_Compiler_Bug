plain
2020-01-09T23:06:28.7036483Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-09T23:06:28.7047499Z ##[command]git config gc.auto 0
2020-01-09T23:06:28.7049796Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-09T23:06:28.7051880Z ##[command]git config --get-all http.proxy
2020-01-09T23:06:28.7055022Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68071/merge:refs/remotes/pull/68071/merge
---
2020-01-09T23:12:18.3399719Z Found 0 error codes with no tests
2020-01-09T23:12:18.3399763Z Done!
2020-01-09T23:12:18.3406119Z 
2020-01-09T23:12:18.3406426Z 
2020-01-09T23:12:18.3407868Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-01-09T23:12:18.3408302Z 
2020-01-09T23:12:18.3408586Z 
2020-01-09T23:12:18.3417796Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-01-09T23:12:18.3417887Z Build completed unsuccessfully in 0:01:39
2020-01-09T23:12:18.3417887Z Build completed unsuccessfully in 0:01:39
2020-01-09T23:12:18.3471086Z == clock drift check ==
2020-01-09T23:12:18.3483315Z   local time: Thu Jan  9 23:12:18 UTC 2020
2020-01-09T23:12:18.5057933Z   network time: Thu, 09 Jan 2020 23:12:18 GMT
2020-01-09T23:12:18.5063695Z == end clock drift check ==
2020-01-09T23:12:19.2524090Z 
2020-01-09T23:12:19.2640356Z ##[error]Bash exited with code '1'.
2020-01-09T23:12:19.2671910Z ##[section]Starting: Checkout
2020-01-09T23:12:19.2673576Z ==============================================================================
2020-01-09T23:12:19.2673633Z Task         : Get sources
2020-01-09T23:12:19.2673696Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
