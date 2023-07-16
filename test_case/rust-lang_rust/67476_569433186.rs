plain
2019-12-28T16:43:38.3266804Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-28T16:43:39.0508640Z ##[command]git config gc.auto 0
2019-12-28T16:43:39.0514432Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-28T16:43:39.0529823Z ##[command]git config --get-all http.proxy
2019-12-28T16:43:39.0534806Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67476/merge:refs/remotes/pull/67476/merge
---
2019-12-28T16:49:46.2881736Z    Compiling serde_json v1.0.40
2019-12-28T16:49:47.8083703Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-12-28T16:49:57.5711275Z     Finished release [optimized] target(s) in 1m 18s
2019-12-28T16:49:57.5805159Z tidy check
2019-12-28T16:49:58.3877548Z tidy error: /checkout/src/librustc_mir/borrow_check/mod.rs:1483: TODO is deprecated; use FIXME
2019-12-28T16:49:58.3939098Z tidy error: /checkout/src/librustc_mir/borrow_check/diagnostics/region_name.rs:164: TODO is deprecated; use FIXME
2019-12-28T16:49:58.3959462Z tidy error: /checkout/src/librustc_mir/borrow_check/diagnostics/region_errors.rs:778: TODO is deprecated; use FIXME
2019-12-28T16:50:00.0985038Z Found 486 error codes
2019-12-28T16:50:00.0985297Z Found 0 error codes with no tests
2019-12-28T16:50:00.0985333Z Done!
2019-12-28T16:50:00.0985412Z some tidy checks failed
2019-12-28T16:50:00.0985412Z some tidy checks failed
2019-12-28T16:50:00.0994092Z 
2019-12-28T16:50:00.0994591Z 
2019-12-28T16:50:00.0996095Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-28T16:50:00.0996433Z 
2019-12-28T16:50:00.0996546Z 
2019-12-28T16:50:00.0996666Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-28T16:50:00.0996782Z Build completed unsuccessfully in 0:01:28
2019-12-28T16:50:00.0996782Z Build completed unsuccessfully in 0:01:28
2019-12-28T16:50:00.1050548Z == clock drift check ==
2019-12-28T16:50:00.1059279Z   local time: Sat Dec 28 16:50:00 UTC 2019
2019-12-28T16:50:01.1637271Z   network time: Sat, 28 Dec 2019 16:50:00 GMT
2019-12-28T16:50:01.1637927Z == end clock drift check ==
2019-12-28T16:50:01.6155701Z 
2019-12-28T16:50:01.6221843Z ##[error]Bash exited with code '1'.
2019-12-28T16:50:01.6248700Z ##[section]Starting: Checkout
2019-12-28T16:50:01.6250999Z ==============================================================================
2019-12-28T16:50:01.6251077Z Task         : Get sources
2019-12-28T16:50:01.6251127Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
