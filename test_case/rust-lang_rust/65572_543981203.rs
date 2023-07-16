plain
2019-10-18T22:05:09.3314751Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-18T22:05:09.3501902Z ##[command]git config gc.auto 0
2019-10-18T22:05:09.3589323Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-18T22:05:09.3647293Z ##[command]git config --get-all http.proxy
2019-10-18T22:05:09.3785916Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65572/merge:refs/remotes/pull/65572/merge
---
2019-10-18T22:11:44.7281689Z    Compiling serde_json v1.0.40
2019-10-18T22:11:46.7670384Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-18T22:11:57.9595810Z     Finished release [optimized] target(s) in 1m 29s
2019-10-18T22:11:57.9671957Z tidy check
2019-10-18T22:11:58.4047563Z tidy error: /checkout/src/librustc_mir/transform/qualify_min_const_fn.rs:169: line longer than 100 chars
2019-10-18T22:11:58.4047698Z tidy error: /checkout/src/librustc_mir/transform/qualify_min_const_fn.rs:172: line longer than 100 chars
2019-10-18T22:11:58.4047829Z tidy error: /checkout/src/librustc_mir/transform/qualify_min_const_fn.rs:356: line longer than 100 chars
2019-10-18T22:11:58.6107204Z tidy error: /checkout/src/librustc/mir/visit.rs:870: line longer than 100 chars
2019-10-18T22:12:00.0208546Z Found 482 error codes
2019-10-18T22:12:00.0208659Z Found 0 error codes with no tests
2019-10-18T22:12:00.0208707Z Done!
2019-10-18T22:12:00.0208776Z some tidy checks failed
2019-10-18T22:12:00.0208776Z some tidy checks failed
2019-10-18T22:12:00.0208808Z 
2019-10-18T22:12:00.0208836Z 
2019-10-18T22:12:00.0209870Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-18T22:12:00.0209999Z 
2019-10-18T22:12:00.0210025Z 
2019-10-18T22:12:00.0217808Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-18T22:12:00.0217932Z Build completed unsuccessfully in 0:01:33
2019-10-18T22:12:00.0217932Z Build completed unsuccessfully in 0:01:33
2019-10-18T22:12:00.0265630Z == clock drift check ==
2019-10-18T22:12:00.0276867Z   local time: Fri Oct 18 22:12:00 UTC 2019
2019-10-18T22:12:00.0999806Z   network time: Fri, 18 Oct 2019 22:12:00 GMT
2019-10-18T22:12:00.1004462Z == end clock drift check ==
2019-10-18T22:12:01.5241031Z 
2019-10-18T22:12:01.5342470Z ##[error]Bash exited with code '1'.
2019-10-18T22:12:01.5376594Z ##[section]Starting: Checkout
2019-10-18T22:12:01.5378231Z ==============================================================================
2019-10-18T22:12:01.5378305Z Task         : Get sources
2019-10-18T22:12:01.5378356Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
