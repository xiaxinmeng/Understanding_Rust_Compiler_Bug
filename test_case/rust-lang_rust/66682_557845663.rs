plain
2019-11-24T00:08:25.8465561Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-24T00:08:25.8679252Z ##[command]git config gc.auto 0
2019-11-24T00:08:26.6026035Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-24T00:08:26.6028455Z ##[command]git config --get-all http.proxy
2019-11-24T00:08:26.6031065Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66682/merge:refs/remotes/pull/66682/merge
---
2019-11-24T00:14:02.5945629Z   = note: `#[warn(unused_imports)]` on by default
2019-11-24T00:14:02.5948754Z 
2019-11-24T00:14:12.6248536Z     Finished release [optimized] target(s) in 1m 20s
2019-11-24T00:14:12.6339222Z tidy check
2019-11-24T00:14:14.1871238Z tidy error: /checkout/src/librustc/ty/print/pretty.rs:1083: trailing whitespace
2019-11-24T00:14:14.1877233Z tidy error: /checkout/src/librustc/ty/print/pretty.rs:1501: line longer than 100 chars
2019-11-24T00:14:14.1877488Z tidy error: /checkout/src/librustc/infer/error_reporting/mod.rs:967: trailing whitespace
2019-11-24T00:14:15.1646201Z Found 485 error codes
2019-11-24T00:14:15.1648517Z Found 0 error codes with no tests
2019-11-24T00:14:15.1648936Z Done!
2019-11-24T00:14:15.1649317Z some tidy checks failed
2019-11-24T00:14:15.1649317Z some tidy checks failed
2019-11-24T00:14:15.1649681Z 
2019-11-24T00:14:15.1649911Z 
2019-11-24T00:14:15.1651289Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-24T00:14:15.1654670Z 
2019-11-24T00:14:15.1655036Z 
2019-11-24T00:14:15.1655394Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-24T00:14:15.1655679Z Build completed unsuccessfully in 0:01:24
2019-11-24T00:14:15.1655679Z Build completed unsuccessfully in 0:01:24
2019-11-24T00:14:15.1701543Z == clock drift check ==
2019-11-24T00:14:15.1709575Z   local time: Sun Nov 24 00:14:15 UTC 2019
2019-11-24T00:14:15.3071238Z   network time: Sun, 24 Nov 2019 00:14:15 GMT
2019-11-24T00:14:15.3076954Z == end clock drift check ==
2019-11-24T00:14:16.6887548Z 
2019-11-24T00:14:16.6986245Z ##[error]Bash exited with code '1'.
2019-11-24T00:14:16.7012733Z ##[section]Starting: Checkout
2019-11-24T00:14:16.7014840Z ==============================================================================
2019-11-24T00:14:16.7014919Z Task         : Get sources
2019-11-24T00:14:16.7014968Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
