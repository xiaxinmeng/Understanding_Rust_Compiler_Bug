plain
2019-11-04T02:59:48.0675705Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-04T02:59:48.0888102Z ##[command]git config gc.auto 0
2019-11-04T02:59:48.0963503Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-04T02:59:48.1018526Z ##[command]git config --get-all http.proxy
2019-11-04T02:59:48.1176083Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66072/merge:refs/remotes/pull/66072/merge
---
2019-11-04T03:06:11.8713236Z    Compiling serde_json v1.0.40
2019-11-04T03:06:13.8548970Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-11-04T03:06:26.0011400Z     Finished release [optimized] target(s) in 1m 34s
2019-11-04T03:06:26.0095722Z tidy check
2019-11-04T03:06:26.5489726Z tidy error: /checkout/src/librustc_interface/pretty.rs: empty file
2019-11-04T03:06:26.5490566Z tidy error: /checkout/src/librustc_interface/pretty.rs: leading newline
2019-11-04T03:06:26.6915224Z tidy error: /checkout/src/librustc/session/config.rs:2625: line longer than 100 chars
2019-11-04T03:06:26.6916196Z tidy error: /checkout/src/librustc/session/config.rs: too many lines (3040) (add `// ignore-tidy-filelength` to the file to suppress this error)
2019-11-04T03:06:28.5993865Z Found 485 error codes
2019-11-04T03:06:28.5996216Z Found 0 error codes with no tests
2019-11-04T03:06:28.5997013Z Done!
2019-11-04T03:06:28.5997306Z some tidy checks failed
2019-11-04T03:06:28.5997306Z some tidy checks failed
2019-11-04T03:06:28.5997476Z 
2019-11-04T03:06:28.5997625Z 
2019-11-04T03:06:28.5998856Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-04T03:06:28.5999378Z 
2019-11-04T03:06:28.5999548Z 
2019-11-04T03:06:28.6005886Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-04T03:06:28.6006208Z Build completed unsuccessfully in 0:01:38
2019-11-04T03:06:28.6006208Z Build completed unsuccessfully in 0:01:38
2019-11-04T03:06:28.6059993Z == clock drift check ==
2019-11-04T03:06:28.6069627Z   local time: Mon Nov  4 03:06:28 UTC 2019
2019-11-04T03:06:28.7579880Z   network time: Mon, 04 Nov 2019 03:06:28 GMT
2019-11-04T03:06:28.7580025Z == end clock drift check ==
2019-11-04T03:06:30.2093586Z 
2019-11-04T03:06:30.2209414Z ##[error]Bash exited with code '1'.
2019-11-04T03:06:30.2236532Z ##[section]Starting: Checkout
2019-11-04T03:06:30.2238286Z ==============================================================================
2019-11-04T03:06:30.2238356Z Task         : Get sources
2019-11-04T03:06:30.2238405Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
