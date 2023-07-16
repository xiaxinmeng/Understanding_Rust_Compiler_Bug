plain
2019-11-04T06:00:54.3190569Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-04T06:00:54.3372766Z ##[command]git config gc.auto 0
2019-11-04T06:00:54.3444092Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-04T06:00:54.3506436Z ##[command]git config --get-all http.proxy
2019-11-04T06:00:55.2576830Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66069/merge:refs/remotes/pull/66069/merge
---
2019-11-04T06:07:01.4304845Z    Compiling serde_json v1.0.40
2019-11-04T06:07:03.1591868Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-11-04T06:07:14.6153294Z     Finished release [optimized] target(s) in 1m 28s
2019-11-04T06:07:14.6236272Z tidy check
2019-11-04T06:07:14.7492711Z tidy error: /checkout/src/liballoc/vec.rs: too many lines (3029) (add `// ignore-tidy-filelength` to the file to suppress this error)
2019-11-04T06:07:16.9038987Z Found 485 error codes
2019-11-04T06:07:16.9041635Z Found 0 error codes with no tests
2019-11-04T06:07:16.9042194Z Done!
2019-11-04T06:07:16.9042473Z some tidy checks failed
2019-11-04T06:07:16.9042473Z some tidy checks failed
2019-11-04T06:07:16.9042804Z 
2019-11-04T06:07:16.9043082Z 
2019-11-04T06:07:16.9044266Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-04T06:07:16.9045327Z 
2019-11-04T06:07:16.9045543Z 
2019-11-04T06:07:16.9053066Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-04T06:07:16.9053176Z Build completed unsuccessfully in 0:01:32
2019-11-04T06:07:16.9053176Z Build completed unsuccessfully in 0:01:32
2019-11-04T06:07:16.9116802Z == clock drift check ==
2019-11-04T06:07:16.9127294Z   local time: Mon Nov  4 06:07:16 UTC 2019
2019-11-04T06:07:17.2051161Z   network time: Mon, 04 Nov 2019 06:07:17 GMT
2019-11-04T06:07:17.2057231Z == end clock drift check ==
2019-11-04T06:07:18.6086136Z 
2019-11-04T06:07:18.6216962Z ##[error]Bash exited with code '1'.
2019-11-04T06:07:18.6245314Z ##[section]Starting: Checkout
2019-11-04T06:07:18.6247904Z ==============================================================================
2019-11-04T06:07:18.6247965Z Task         : Get sources
2019-11-04T06:07:18.6248044Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
