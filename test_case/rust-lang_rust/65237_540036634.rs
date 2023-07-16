plain
2019-10-09T14:29:27.3598218Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-09T14:29:27.3784849Z ##[command]git config gc.auto 0
2019-10-09T14:29:27.3869983Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-09T14:29:27.3934341Z ##[command]git config --get-all http.proxy
2019-10-09T14:29:27.4062590Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65237/merge:refs/remotes/pull/65237/merge
---
2019-10-09T14:36:32.1364108Z    Compiling serde_json v1.0.40
2019-10-09T14:36:33.8796220Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-09T14:36:45.1402832Z     Finished release [optimized] target(s) in 1m 27s
2019-10-09T14:36:45.1486124Z tidy check
2019-10-09T14:36:45.8389967Z tidy error: /checkout/src/libcore/tests/fmt/builders.rs:344: trailing whitespace
2019-10-09T14:36:47.2171220Z Found 482 error codes
2019-10-09T14:36:47.2171396Z Found 0 error codes with no tests
2019-10-09T14:36:47.2171434Z Done!
2019-10-09T14:36:47.2174530Z some tidy checks failed
2019-10-09T14:36:47.2174530Z some tidy checks failed
2019-10-09T14:36:47.2178213Z 
2019-10-09T14:36:47.2178278Z 
2019-10-09T14:36:47.2179096Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-09T14:36:47.2179427Z 
2019-10-09T14:36:47.2179450Z 
2019-10-09T14:36:47.2182450Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-09T14:36:47.2182760Z Build completed unsuccessfully in 0:01:30
2019-10-09T14:36:47.2182760Z Build completed unsuccessfully in 0:01:30
2019-10-09T14:36:47.2231446Z == clock drift check ==
2019-10-09T14:36:47.2247701Z   local time: Wed Oct  9 14:36:47 UTC 2019
2019-10-09T14:36:47.3868781Z   network time: Wed, 09 Oct 2019 14:36:47 GMT
2019-10-09T14:36:47.3870751Z == end clock drift check ==
2019-10-09T14:36:48.7670035Z ##[error]Bash exited with code '1'.
2019-10-09T14:36:48.7708290Z ##[section]Starting: Checkout
2019-10-09T14:36:48.7709858Z ==============================================================================
2019-10-09T14:36:48.7709920Z Task         : Get sources
2019-10-09T14:36:48.7709957Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
