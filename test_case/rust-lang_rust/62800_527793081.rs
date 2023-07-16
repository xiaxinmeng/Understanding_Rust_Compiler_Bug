plain
2019-09-04T07:50:13.8472877Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-04T07:50:13.8701995Z ##[command]git config gc.auto 0
2019-09-04T07:50:13.8794831Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-04T07:50:13.8876225Z ##[command]git config --get-all http.proxy
2019-09-04T07:50:13.9044333Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62800/merge:refs/remotes/pull/62800/merge
---
2019-09-04T07:57:11.7499500Z    Compiling serde_json v1.0.40
2019-09-04T07:57:13.6581696Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-09-04T07:57:25.2471909Z     Finished release [optimized] target(s) in 1m 34s
2019-09-04T07:57:25.2558698Z tidy check
2019-09-04T07:57:25.7264080Z tidy error: /checkout/src/librustc_mir/borrow_check/nll/mod.rs:14: line longer than 100 chars
2019-09-04T07:57:25.7264219Z tidy error: /checkout/src/librustc_mir/borrow_check/nll/mod.rs:75: line longer than 100 chars
2019-09-04T07:57:25.7264335Z tidy error: /checkout/src/librustc_mir/borrow_check/nll/mod.rs:76: line longer than 100 chars
2019-09-04T07:57:25.7264392Z tidy error: /checkout/src/librustc_mir/borrow_check/nll/mod.rs:79: line longer than 100 chars
2019-09-04T07:57:25.7264480Z tidy error: /checkout/src/librustc_mir/borrow_check/nll/mod.rs:102: line longer than 100 chars
2019-09-04T07:57:25.7264552Z tidy error: /checkout/src/librustc_mir/borrow_check/nll/mod.rs:124: line longer than 100 chars
2019-09-04T07:57:28.2558774Z some tidy checks failed
2019-09-04T07:57:28.2560058Z 
2019-09-04T07:57:28.2560058Z 
2019-09-04T07:57:28.2561285Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-04T07:57:28.2561499Z 
2019-09-04T07:57:28.2561525Z 
2019-09-04T07:57:28.2561573Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-04T07:57:28.2561640Z Build completed unsuccessfully in 0:01:38
2019-09-04T07:57:28.2561640Z Build completed unsuccessfully in 0:01:38
2019-09-04T07:57:28.2561684Z == clock drift check ==
2019-09-04T07:57:28.2561728Z   local time: Wed Sep  4 07:57:27 UTC 2019
2019-09-04T07:57:28.2561788Z   network time: Wed, 04 Sep 2019 07:57:27 GMT
2019-09-04T07:57:28.2562314Z == end clock drift check ==
2019-09-04T07:57:29.0135619Z ##[error]Bash exited with code '1'.
2019-09-04T07:57:29.0173213Z ##[section]Starting: Checkout
2019-09-04T07:57:29.0175472Z ==============================================================================
2019-09-04T07:57:29.0175544Z Task         : Get sources
2019-09-04T07:57:29.0175593Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
