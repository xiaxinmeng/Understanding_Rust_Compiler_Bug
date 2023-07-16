plain
2019-11-03T22:26:09.5123970Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-03T22:26:09.5367901Z ##[command]git config gc.auto 0
2019-11-03T22:26:09.5434937Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-03T22:26:09.5820730Z ##[command]git config --get-all http.proxy
2019-11-03T22:26:09.5962311Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66069/merge:refs/remotes/pull/66069/merge
---
2019-11-03T22:32:28.4991500Z    Compiling serde_json v1.0.40
2019-11-03T22:32:30.1682336Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-11-03T22:32:41.0866804Z     Finished release [optimized] target(s) in 1m 24s
2019-11-03T22:32:41.0938739Z tidy check
2019-11-03T22:32:41.2182546Z tidy error: /checkout/src/liballoc/tests/vec.rs:1267: line longer than 100 chars
2019-11-03T22:32:41.2234387Z tidy error: /checkout/src/liballoc/vec.rs:650: line longer than 100 chars
2019-11-03T22:32:41.2234737Z tidy error: /checkout/src/liballoc/vec.rs:683: TODO is deprecated; use FIXME
2019-11-03T22:32:41.2242701Z tidy error: /checkout/src/liballoc/vec.rs: too many lines (3029) (add `// ignore-tidy-filelength` to the file to suppress this error)
2019-11-03T22:32:43.3224762Z Found 485 error codes
2019-11-03T22:32:43.3226018Z Found 0 error codes with no tests
2019-11-03T22:32:43.3226251Z Done!
2019-11-03T22:32:43.3226677Z some tidy checks failed
2019-11-03T22:32:43.3226677Z some tidy checks failed
2019-11-03T22:32:43.3226710Z 
2019-11-03T22:32:43.3226738Z 
2019-11-03T22:32:43.3227857Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-03T22:32:43.3229668Z 
2019-11-03T22:32:43.3229729Z 
2019-11-03T22:32:43.3257776Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-03T22:32:43.3257854Z Build completed unsuccessfully in 0:01:27
2019-11-03T22:32:43.3257854Z Build completed unsuccessfully in 0:01:27
2019-11-03T22:32:43.3289829Z == clock drift check ==
2019-11-03T22:32:43.3300593Z   local time: Sun Nov  3 22:32:43 UTC 2019
2019-11-03T22:32:43.4783496Z   network time: Sun, 03 Nov 2019 22:32:43 GMT
2019-11-03T22:32:43.4787278Z == end clock drift check ==
2019-11-03T22:32:44.8057611Z 
2019-11-03T22:32:44.8136499Z ##[error]Bash exited with code '1'.
2019-11-03T22:32:44.8173128Z ##[section]Starting: Checkout
2019-11-03T22:32:44.8174609Z ==============================================================================
2019-11-03T22:32:44.8174679Z Task         : Get sources
2019-11-03T22:32:44.8174723Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
