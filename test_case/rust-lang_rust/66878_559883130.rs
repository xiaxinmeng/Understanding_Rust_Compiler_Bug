plain
2019-11-29T21:21:36.4258933Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-29T21:21:36.4269521Z ##[command]git config gc.auto 0
2019-11-29T21:21:36.4314596Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-29T21:21:37.4218496Z ##[command]git config --get-all http.proxy
2019-11-29T21:21:38.4796432Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66878/merge:refs/remotes/pull/66878/merge
---
2019-11-29T21:27:12.5341208Z    Compiling serde_json v1.0.40
2019-11-29T21:27:14.0635882Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-11-29T21:27:23.8605297Z     Finished release [optimized] target(s) in 1m 19s
2019-11-29T21:27:23.8696819Z tidy check
2019-11-29T21:27:24.5034899Z tidy error: /checkout/src/librustc_session/config.rs:2385: line longer than 100 chars
2019-11-29T21:27:24.5035632Z tidy error: /checkout/src/librustc_session/config.rs:2397: line longer than 100 chars
2019-11-29T21:27:25.1714910Z thread 'main' panicked at 'fs::read_to_string(&path) failed with No such file or directory (os error 2)', src/tools/tidy/src/features.rs:235:20
2019-11-29T21:27:25.1769036Z 
2019-11-29T21:27:25.1769305Z 
2019-11-29T21:27:25.1769305Z 
2019-11-29T21:27:25.1770379Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-29T21:27:25.1770587Z 
2019-11-29T21:27:25.1770611Z 
2019-11-29T21:27:25.1770657Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-29T21:27:25.1770732Z Build completed unsuccessfully in 0:01:21
2019-11-29T21:27:25.1770732Z Build completed unsuccessfully in 0:01:21
2019-11-29T21:27:25.1777146Z == clock drift check ==
2019-11-29T21:27:25.1784049Z   local time: Fri Nov 29 21:27:25 UTC 2019
2019-11-29T21:27:25.4559242Z   network time: Fri, 29 Nov 2019 21:27:25 GMT
2019-11-29T21:27:25.4561433Z == end clock drift check ==
2019-11-29T21:27:26.8757716Z 
2019-11-29T21:27:26.8840795Z ##[error]Bash exited with code '1'.
2019-11-29T21:27:26.8873775Z ##[section]Starting: Checkout
2019-11-29T21:27:26.8876388Z ==============================================================================
2019-11-29T21:27:26.8876440Z Task         : Get sources
2019-11-29T21:27:26.8876499Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
