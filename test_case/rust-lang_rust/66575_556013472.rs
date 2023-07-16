plain
2019-11-20T13:38:03.6331010Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-20T13:38:03.6517745Z ##[command]git config gc.auto 0
2019-11-20T13:38:03.6592374Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-20T13:38:03.6638978Z ##[command]git config --get-all http.proxy
2019-11-20T13:38:03.6786414Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66575/merge:refs/remotes/pull/66575/merge
---
2019-11-20T13:44:06.2819235Z    Compiling serde_json v1.0.40
2019-11-20T13:44:08.0898913Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-11-20T13:44:19.3236867Z     Finished release [optimized] target(s) in 1m 28s
2019-11-20T13:44:19.3344789Z tidy check
2019-11-20T13:44:20.0580719Z tidy error: /checkout/src/librustc/session/config.rs: ignoring file length unnecessarily
2019-11-20T13:44:22.0931403Z some tidy checks failed
2019-11-20T13:44:22.0931496Z Found 441 error codes
2019-11-20T13:44:22.0931833Z Found 0 error codes with no tests
2019-11-20T13:44:22.0932092Z Done!
2019-11-20T13:44:22.0932092Z Done!
2019-11-20T13:44:22.0932124Z 
2019-11-20T13:44:22.0932149Z 
2019-11-20T13:44:22.0933015Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-20T13:44:22.0933122Z 
2019-11-20T13:44:22.0933147Z 
2019-11-20T13:44:22.0943482Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-20T13:44:22.0943563Z Build completed unsuccessfully in 0:01:32
2019-11-20T13:44:22.0943563Z Build completed unsuccessfully in 0:01:32
2019-11-20T13:44:22.0984186Z == clock drift check ==
2019-11-20T13:44:22.0993607Z   local time: Wed Nov 20 13:44:22 UTC 2019
2019-11-20T13:44:22.2518955Z   network time: Wed, 20 Nov 2019 13:44:22 GMT
2019-11-20T13:44:22.2524507Z == end clock drift check ==
2019-11-20T13:44:23.5607663Z 
2019-11-20T13:44:23.5762047Z ##[error]Bash exited with code '1'.
2019-11-20T13:44:23.5801685Z ##[section]Starting: Checkout
2019-11-20T13:44:23.5803597Z ==============================================================================
2019-11-20T13:44:23.5803674Z Task         : Get sources
2019-11-20T13:44:23.5803728Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
