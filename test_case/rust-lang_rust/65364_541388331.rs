plain
2019-10-13T05:18:36.9479649Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-13T05:18:36.9582789Z ##[command]git config gc.auto 0
2019-10-13T05:18:36.9654446Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-13T05:18:36.9743844Z ##[command]git config --get-all http.proxy
2019-10-13T05:18:36.9890730Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65364/merge:refs/remotes/pull/65364/merge
---
2019-10-13T05:25:46.4276847Z    Compiling serde_json v1.0.40
2019-10-13T05:25:48.4225871Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-13T05:26:01.1904851Z     Finished release [optimized] target(s) in 1m 38s
2019-10-13T05:26:01.2009956Z tidy check
2019-10-13T05:26:02.2158910Z tidy error: /checkout/src/libsyntax/parse/lexer/tokentrees.rs:215: line longer than 100 chars
2019-10-13T05:26:03.6395621Z Found 482 error codes
2019-10-13T05:26:03.6398512Z Found 0 error codes with no tests
2019-10-13T05:26:03.6405124Z Done!
2019-10-13T05:26:03.6405454Z some tidy checks failed
2019-10-13T05:26:03.6405454Z some tidy checks failed
2019-10-13T05:26:03.6405506Z 
2019-10-13T05:26:03.6405581Z 
2019-10-13T05:26:03.6406726Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-13T05:26:03.6406830Z 
2019-10-13T05:26:03.6406878Z 
2019-10-13T05:26:03.6411619Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-13T05:26:03.6412090Z Build completed unsuccessfully in 0:01:42
2019-10-13T05:26:03.6412090Z Build completed unsuccessfully in 0:01:42
2019-10-13T05:26:03.6461874Z == clock drift check ==
2019-10-13T05:26:03.6483988Z   local time: Sun Oct 13 05:26:03 UTC 2019
2019-10-13T05:26:03.7333743Z   network time: Sun, 13 Oct 2019 05:26:03 GMT
2019-10-13T05:26:03.7334557Z == end clock drift check ==
2019-10-13T05:26:04.5080978Z ##[error]Bash exited with code '1'.
2019-10-13T05:26:04.5127524Z ##[section]Starting: Checkout
2019-10-13T05:26:04.5129316Z ==============================================================================
2019-10-13T05:26:04.5129394Z Task         : Get sources
2019-10-13T05:26:04.5129442Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
