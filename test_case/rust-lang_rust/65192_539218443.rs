plain
2019-10-07T21:27:26.4275315Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-07T21:27:26.4451790Z ##[command]git config gc.auto 0
2019-10-07T21:27:26.4519643Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-07T21:27:26.4574367Z ##[command]git config --get-all http.proxy
2019-10-07T21:27:26.4709765Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65192/merge:refs/remotes/pull/65192/merge
---
2019-10-07T21:34:02.8145210Z    Compiling serde_json v1.0.40
2019-10-07T21:34:04.6201092Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-07T21:34:15.5858243Z     Finished release [optimized] target(s) in 1m 28s
2019-10-07T21:34:15.5929491Z tidy check
2019-10-07T21:34:16.2143380Z tidy error: /checkout/src/librustc/traits/error_reporting.rs:721: line longer than 100 chars
2019-10-07T21:34:17.4830599Z some tidy checks failed
2019-10-07T21:34:17.4836659Z 
2019-10-07T21:34:17.4836659Z 
2019-10-07T21:34:17.4837601Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-07T21:34:17.4837781Z 
2019-10-07T21:34:17.4837806Z 
2019-10-07T21:34:17.4845118Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-07T21:34:17.4845195Z Build completed unsuccessfully in 0:01:31
2019-10-07T21:34:17.4845195Z Build completed unsuccessfully in 0:01:31
2019-10-07T21:34:17.4897140Z == clock drift check ==
2019-10-07T21:34:17.4912234Z   local time: Mon Oct  7 21:34:17 UTC 2019
2019-10-07T21:34:17.5889463Z   network time: Mon, 07 Oct 2019 21:34:17 GMT
2019-10-07T21:34:17.5893374Z == end clock drift check ==
2019-10-07T21:34:18.9577456Z ##[error]Bash exited with code '1'.
2019-10-07T21:34:18.9641677Z ##[section]Starting: Checkout
2019-10-07T21:34:18.9643783Z ==============================================================================
2019-10-07T21:34:18.9643851Z Task         : Get sources
2019-10-07T21:34:18.9643904Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
