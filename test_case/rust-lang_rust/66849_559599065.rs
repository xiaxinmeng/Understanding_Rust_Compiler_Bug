plain
2019-11-28T20:20:18.8449914Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-28T20:20:19.6355285Z ##[command]git config gc.auto 0
2019-11-28T20:20:19.6359648Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-28T20:20:19.6362460Z ##[command]git config --get-all http.proxy
2019-11-28T20:20:19.6366105Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66849/merge:refs/remotes/pull/66849/merge
---
2019-11-28T20:26:20.7713112Z Found 0 error codes with no tests
2019-11-28T20:26:20.7713161Z Done!
2019-11-28T20:26:20.7713186Z 
2019-11-28T20:26:20.7713208Z 
2019-11-28T20:26:20.7714386Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-28T20:26:20.7714483Z 
2019-11-28T20:26:20.7714698Z 
2019-11-28T20:26:20.7720224Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-28T20:26:20.7721071Z Build completed unsuccessfully in 0:01:26
2019-11-28T20:26:20.7721071Z Build completed unsuccessfully in 0:01:26
2019-11-28T20:26:20.7779100Z == clock drift check ==
2019-11-28T20:26:20.7790556Z   local time: Thu Nov 28 20:26:20 UTC 2019
2019-11-28T20:26:21.0557976Z   network time: Thu, 28 Nov 2019 20:26:21 GMT
2019-11-28T20:26:21.0562178Z == end clock drift check ==
2019-11-28T20:26:22.3577804Z 
2019-11-28T20:26:22.3692610Z ##[error]Bash exited with code '1'.
2019-11-28T20:26:22.3724553Z ##[section]Starting: Checkout
2019-11-28T20:26:22.3726804Z ==============================================================================
2019-11-28T20:26:22.3726862Z Task         : Get sources
2019-11-28T20:26:22.3726911Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
