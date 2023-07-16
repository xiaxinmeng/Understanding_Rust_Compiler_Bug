plain
2019-11-12T10:47:13.1515924Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-12T10:47:13.1708716Z ##[command]git config gc.auto 0
2019-11-12T10:47:13.1759869Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-12T10:47:13.1814374Z ##[command]git config --get-all http.proxy
2019-11-12T10:47:13.1958774Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66314/merge:refs/remotes/pull/66314/merge
---
2019-11-12T10:53:20.5429696Z    Compiling serde_json v1.0.40
2019-11-12T10:53:22.2952635Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-11-12T10:53:33.5402796Z     Finished release [optimized] target(s) in 1m 28s
2019-11-12T10:53:33.5484110Z tidy check
2019-11-12T10:53:34.3394720Z tidy error: /checkout/src/librustc_error_codes/error_codes.rs: too many trailing newlines (2)
2019-11-12T10:53:34.3395565Z tidy error: /checkout/src/librustc_error_codes/error_codes.rs: ignoring file length unnecessarily
2019-11-12T10:53:36.1085212Z Found 440 error codes
2019-11-12T10:53:36.1085329Z Found 0 error codes with no tests
2019-11-12T10:53:36.1085410Z Done!
2019-11-12T10:53:36.1085700Z some tidy checks failed
2019-11-12T10:53:36.1085700Z some tidy checks failed
2019-11-12T10:53:36.1085730Z 
2019-11-12T10:53:36.1085761Z 
2019-11-12T10:53:36.1086671Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-12T10:53:36.1086787Z 
2019-11-12T10:53:36.1086813Z 
2019-11-12T10:53:36.1095245Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-12T10:53:36.1095531Z Build completed unsuccessfully in 0:01:32
2019-11-12T10:53:36.1095531Z Build completed unsuccessfully in 0:01:32
2019-11-12T10:53:36.1140246Z == clock drift check ==
2019-11-12T10:53:36.1153113Z   local time: Tue Nov 12 10:53:36 UTC 2019
2019-11-12T10:53:36.2642686Z   network time: Tue, 12 Nov 2019 10:53:36 GMT
2019-11-12T10:53:36.2642857Z == end clock drift check ==
2019-11-12T10:53:37.7478609Z 
2019-11-12T10:53:37.7570344Z ##[error]Bash exited with code '1'.
2019-11-12T10:53:37.7593471Z ##[section]Starting: Checkout
2019-11-12T10:53:37.7595946Z ==============================================================================
2019-11-12T10:53:37.7596004Z Task         : Get sources
2019-11-12T10:53:37.7596070Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
