plain
2019-09-20T21:01:07.6189439Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-20T21:01:07.6412863Z ##[command]git config gc.auto 0
2019-09-20T21:01:07.6479093Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-20T21:01:07.6534166Z ##[command]git config --get-all http.proxy
2019-09-20T21:01:07.6699914Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64619/merge:refs/remotes/pull/64619/merge
---
2019-09-20T21:08:02.2981018Z    Compiling serde_json v1.0.40
2019-09-20T21:08:04.0816124Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-09-20T21:08:14.6584804Z     Finished release [optimized] target(s) in 1m 29s
2019-09-20T21:08:14.6659012Z tidy check
2019-09-20T21:08:15.0545137Z tidy error: /checkout/src/librustc_typeck/check/pat.rs:729: line longer than 100 chars
2019-09-20T21:08:16.6336994Z some tidy checks failed
2019-09-20T21:08:16.6337168Z 
2019-09-20T21:08:16.6337168Z 
2019-09-20T21:08:16.6338331Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-20T21:08:16.6338466Z 
2019-09-20T21:08:16.6338487Z 
2019-09-20T21:08:16.6349581Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-20T21:08:16.6349845Z Build completed unsuccessfully in 0:01:32
2019-09-20T21:08:16.6349845Z Build completed unsuccessfully in 0:01:32
2019-09-20T21:08:16.6397124Z == clock drift check ==
2019-09-20T21:08:16.6412979Z   local time: Fri Sep 20 21:08:16 UTC 2019
2019-09-20T21:08:16.7293126Z   network time: Fri, 20 Sep 2019 21:08:16 GMT
2019-09-20T21:08:16.7293261Z == end clock drift check ==
2019-09-20T21:08:18.0533579Z ##[error]Bash exited with code '1'.
2019-09-20T21:08:18.0567417Z ##[section]Starting: Checkout
2019-09-20T21:08:18.0570421Z ==============================================================================
2019-09-20T21:08:18.0570471Z Task         : Get sources
2019-09-20T21:08:18.0570513Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
