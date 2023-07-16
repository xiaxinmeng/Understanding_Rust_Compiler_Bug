plain
2019-10-17T04:34:42.3205180Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-17T04:34:42.3398749Z ##[command]git config gc.auto 0
2019-10-17T04:34:42.3491506Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-17T04:34:42.3549863Z ##[command]git config --get-all http.proxy
2019-10-17T04:34:42.3697457Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65421/merge:refs/remotes/pull/65421/merge
---
2019-10-17T04:41:54.7546335Z    Compiling serde_json v1.0.40
2019-10-17T04:41:56.6831903Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-17T04:42:08.9875917Z     Finished release [optimized] target(s) in 1m 34s
2019-10-17T04:42:08.9969762Z tidy check
2019-10-17T04:42:09.2499650Z tidy error: /checkout/src/test/ui/issues/issue-pr29383.rs:9: line longer than 100 chars
2019-10-17T04:42:09.2500213Z tidy error: /checkout/src/test/ui/issues/issue-pr29383.rs:10: line longer than 100 chars
2019-10-17T04:42:09.2513268Z tidy error: /checkout/src/test/ui/issues/issue-42944.rs:15: line longer than 100 chars
2019-10-17T04:42:09.2513781Z tidy error: /checkout/src/test/ui/issues/issue-63983.rs:9: line longer than 100 chars
2019-10-17T04:42:09.2514512Z tidy error: /checkout/src/test/ui/issues/issue-63983.rs:11: line longer than 100 chars
2019-10-17T04:42:09.2603284Z tidy error: /checkout/src/test/ui/issues/issue-12863.rs:5: line longer than 100 chars
2019-10-17T04:42:09.2655033Z tidy error: /checkout/src/test/ui/issues/issue-58022.rs:14: line longer than 100 chars
2019-10-17T04:42:09.2790737Z tidy error: /checkout/src/test/ui/issues/issue-4366.rs:18: line longer than 100 chars
2019-10-17T04:42:09.3078289Z tidy error: /checkout/src/test/ui/no-implicit-prelude-nested.rs:18: line longer than 100 chars
2019-10-17T04:42:09.3078639Z tidy error: /checkout/src/test/ui/no-implicit-prelude-nested.rs:45: line longer than 100 chars
2019-10-17T04:42:09.3243401Z tidy error: /checkout/src/test/ui/empty/empty-struct-braces-pat-1.rs:32: line longer than 100 chars
2019-10-17T04:42:09.3243762Z tidy error: /checkout/src/test/ui/empty/empty-struct-braces-expr.rs:16: line longer than 100 chars
2019-10-17T04:42:09.3244083Z tidy error: /checkout/src/test/ui/empty/empty-struct-braces-expr.rs:18: line longer than 100 chars
2019-10-17T04:42:09.3244406Z tidy error: /checkout/src/test/ui/empty/empty-struct-braces-expr.rs:21: line longer than 100 chars
2019-10-17T04:42:09.3244691Z tidy error: /checkout/src/test/ui/empty/empty-struct-unit-pat.rs:24: line longer than 100 chars
2019-10-17T04:42:09.3244988Z tidy error: /checkout/src/test/ui/empty/empty-struct-unit-pat.rs:27: line longer than 100 chars
2019-10-17T04:42:09.3245283Z tidy error: /checkout/src/test/ui/empty/empty-struct-unit-pat.rs:30: line longer than 100 chars
2019-10-17T04:42:09.3245559Z tidy error: /checkout/src/test/ui/empty/empty-struct-unit-pat.rs:34: line longer than 100 chars
2019-10-17T04:42:09.3246007Z tidy error: /checkout/src/test/ui/empty/empty-struct-unit-pat.rs:42: line longer than 100 chars
2019-10-17T04:42:09.3307487Z tidy error: /checkout/src/test/ui/qualified/qualified-path-params.rs:21: line longer than 100 chars
2019-10-17T04:42:09.3335696Z tidy error: /checkout/src/test/ui/parser-recovery-2.rs:7: line longer than 100 chars
2019-10-17T04:42:09.3467147Z tidy error: /checkout/src/test/ui/pattern/pattern-error-continue.rs:18: line longer than 100 chars
2019-10-17T04:42:09.3826808Z tidy error: /checkout/src/test/ui/derived-errors/issue-31997.rs:14: line longer than 100 chars
2019-10-17T04:42:09.3895072Z tidy error: /checkout/src/test/ui/resolve/issue-6702.rs:7: line longer than 100 chars
2019-10-17T04:42:09.4147247Z tidy error: /checkout/src/test/ui/imports/import-glob-circular.rs:16: line longer than 100 chars
2019-10-17T04:42:09.4147597Z tidy error: /checkout/src/test/ui/imports/glob-conflict-cross-crate.rs:6: line longer than 100 chars
2019-10-17T04:42:09.4147919Z tidy error: /checkout/src/test/ui/imports/glob-conflict-cross-crate.rs:7: line longer than 100 chars
2019-10-17T04:42:09.4704977Z tidy error: /checkout/src/librustc_resolve/error_codes.rs:995: line longer than 80 chars
2019-10-17T04:42:09.9449230Z tidy error: /checkout/src/librustc_typeck/error_codes.rs:4322: line longer than 80 chars
2019-10-17T04:42:11.4739125Z Found 482 error codes
2019-10-17T04:42:11.4740028Z Found 0 error codes with no tests
2019-10-17T04:42:11.4740324Z Done!
2019-10-17T04:42:11.4740684Z some tidy checks failed
2019-10-17T04:42:11.4740684Z some tidy checks failed
2019-10-17T04:42:11.4740814Z 
2019-10-17T04:42:11.4740967Z 
2019-10-17T04:42:11.4742117Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-17T04:42:11.4742927Z 
2019-10-17T04:42:11.4743056Z 
2019-10-17T04:42:11.4747467Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-17T04:42:11.4747706Z Build completed unsuccessfully in 0:01:38
2019-10-17T04:42:11.4747706Z Build completed unsuccessfully in 0:01:38
2019-10-17T04:42:11.4798541Z == clock drift check ==
2019-10-17T04:42:11.4818449Z   local time: Thu Oct 17 04:42:11 UTC 2019
2019-10-17T04:42:11.6437027Z   network time: Thu, 17 Oct 2019 04:42:11 GMT
2019-10-17T04:42:11.6439197Z == end clock drift check ==
2019-10-17T04:42:13.4753128Z ##[error]Bash exited with code '1'.
2019-10-17T04:42:13.4836254Z ##[section]Starting: Checkout
2019-10-17T04:42:13.4838090Z ==============================================================================
2019-10-17T04:42:13.4838162Z Task         : Get sources
2019-10-17T04:42:13.4838212Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
