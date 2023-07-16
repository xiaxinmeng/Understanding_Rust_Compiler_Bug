plain
2020-01-16T04:53:15.3124896Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-16T04:53:15.3141501Z ##[command]git config gc.auto 0
2020-01-16T04:53:15.3143893Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-16T04:53:15.3145898Z ##[command]git config --get-all http.proxy
2020-01-16T04:53:15.3150368Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68269/merge:refs/remotes/pull/68269/merge
---
2020-01-16T04:59:10.5447489Z    Compiling serde_json v1.0.40
2020-01-16T04:59:12.2341787Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-01-16T04:59:21.9923871Z     Finished release [optimized] target(s) in 1m 22s
2020-01-16T04:59:22.0034686Z tidy check
2020-01-16T04:59:23.1954936Z tidy error: /checkout/src/librustc/traits/error_reporting.rs: too many lines (3018) (add `// ignore-tidy-filelength` to the file to suppress this error)
2020-01-16T04:59:24.8549029Z some tidy checks failed
2020-01-16T04:59:24.8551223Z Found 486 error codes
2020-01-16T04:59:24.8555931Z Found 0 error codes with no tests
2020-01-16T04:59:24.8561348Z Done!
2020-01-16T04:59:24.8561348Z Done!
2020-01-16T04:59:24.8561410Z 
2020-01-16T04:59:24.8561442Z 
2020-01-16T04:59:24.8562451Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-01-16T04:59:24.8562673Z 
2020-01-16T04:59:24.8562702Z 
2020-01-16T04:59:24.8562776Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-01-16T04:59:24.8563193Z Build completed unsuccessfully in 0:01:34
2020-01-16T04:59:24.8563193Z Build completed unsuccessfully in 0:01:34
2020-01-16T04:59:24.8606563Z == clock drift check ==
2020-01-16T04:59:24.8620782Z   local time: Thu Jan 16 04:59:24 UTC 2020
2020-01-16T04:59:25.1560361Z   network time: Thu, 16 Jan 2020 04:59:25 GMT
2020-01-16T04:59:25.1560477Z == end clock drift check ==
2020-01-16T04:59:25.9484015Z 
2020-01-16T04:59:25.9579065Z ##[error]Bash exited with code '1'.
2020-01-16T04:59:25.9610268Z ##[section]Starting: Checkout
2020-01-16T04:59:25.9612160Z ==============================================================================
2020-01-16T04:59:25.9612223Z Task         : Get sources
2020-01-16T04:59:25.9612277Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
