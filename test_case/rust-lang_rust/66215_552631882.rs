plain
2019-11-11T21:42:37.1428646Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-11T21:42:37.1644503Z ##[command]git config gc.auto 0
2019-11-11T21:42:37.1732514Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-11T21:42:37.1786780Z ##[command]git config --get-all http.proxy
2019-11-11T21:42:37.1939974Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66215/merge:refs/remotes/pull/66215/merge
---
2019-11-11T21:48:25.0404051Z    Compiling serde_json v1.0.40
2019-11-11T21:48:26.7247496Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-11-11T21:48:37.4019559Z     Finished release [optimized] target(s) in 1m 24s
2019-11-11T21:48:37.4095768Z tidy check
2019-11-11T21:48:37.5465617Z tidy error: /checkout/src/liballoc/string.rs:2003: line longer than 100 chars
2019-11-11T21:48:40.0506143Z some tidy checks failed
2019-11-11T21:48:40.0507805Z Found 485 error codes
2019-11-11T21:48:40.0508042Z Found 0 error codes with no tests
2019-11-11T21:48:40.0508311Z Done!
2019-11-11T21:48:40.0508311Z Done!
2019-11-11T21:48:40.0508558Z 
2019-11-11T21:48:40.0508739Z 
2019-11-11T21:48:40.0509702Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-11T21:48:40.0512545Z 
2019-11-11T21:48:40.0512993Z 
2019-11-11T21:48:40.0513306Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-11T21:48:40.0513554Z Build completed unsuccessfully in 0:01:27
2019-11-11T21:48:40.0513554Z Build completed unsuccessfully in 0:01:27
2019-11-11T21:48:40.0561225Z == clock drift check ==
2019-11-11T21:48:40.0591030Z   local time: Mon Nov 11 21:48:40 UTC 2019
2019-11-11T21:48:40.3453468Z   network time: Mon, 11 Nov 2019 21:48:40 GMT
2019-11-11T21:48:40.3453594Z == end clock drift check ==
2019-11-11T21:48:41.7049385Z 
2019-11-11T21:48:41.7155884Z ##[error]Bash exited with code '1'.
2019-11-11T21:48:41.7185743Z ##[section]Starting: Checkout
2019-11-11T21:48:41.7187184Z ==============================================================================
2019-11-11T21:48:41.7187230Z Task         : Get sources
2019-11-11T21:48:41.7187286Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
