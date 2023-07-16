plain
2019-11-12T02:22:21.0502107Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-12T02:22:21.0718543Z ##[command]git config gc.auto 0
2019-11-12T02:22:21.0806225Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-12T02:22:21.0863139Z ##[command]git config --get-all http.proxy
2019-11-12T02:22:21.1009080Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66321/merge:refs/remotes/pull/66321/merge
---
2019-11-12T02:28:48.3816682Z    Compiling serde_json v1.0.40
2019-11-12T02:28:50.4411822Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-11-12T02:29:02.2765116Z     Finished release [optimized] target(s) in 1m 32s
2019-11-12T02:29:02.2839485Z tidy check
2019-11-12T02:29:02.9052583Z tidy error: /checkout/src/librustc_mir/transform/generator.rs:1093: line longer than 100 chars
2019-11-12T02:29:02.9097390Z tidy error: /checkout/src/librustc_mir/transform/const_prop.rs:92: TODO is deprecated; use FIXME
2019-11-12T02:29:03.2062835Z tidy error: /checkout/src/librustc/mir/mod.rs:9: line longer than 100 chars
2019-11-12T02:29:03.2062968Z tidy error: /checkout/src/librustc/mir/mod.rs:9: TODO is deprecated; use FIXME
2019-11-12T02:29:03.2082001Z tidy error: /checkout/src/librustc/mir/mod.rs:3024: TODO is deprecated; use FIXME
2019-11-12T02:29:05.1586589Z Found 485 error codes
2019-11-12T02:29:05.1586731Z Found 0 error codes with no tests
2019-11-12T02:29:05.1586781Z Done!
2019-11-12T02:29:05.1586829Z some tidy checks failed
2019-11-12T02:29:05.1586829Z some tidy checks failed
2019-11-12T02:29:05.1586882Z 
2019-11-12T02:29:05.1586912Z 
2019-11-12T02:29:05.1587846Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-12T02:29:05.1588020Z 
2019-11-12T02:29:05.1588048Z 
2019-11-12T02:29:05.1588103Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-12T02:29:05.1588253Z Build completed unsuccessfully in 0:01:36
2019-11-12T02:29:05.1588253Z Build completed unsuccessfully in 0:01:36
2019-11-12T02:29:05.1635687Z == clock drift check ==
2019-11-12T02:29:05.1664909Z   local time: Tue Nov 12 02:29:05 UTC 2019
2019-11-12T02:29:05.2351591Z   network time: Tue, 12 Nov 2019 02:29:05 GMT
2019-11-12T02:29:05.2351779Z == end clock drift check ==
2019-11-12T02:29:06.6262033Z 
2019-11-12T02:29:06.6326284Z ##[error]Bash exited with code '1'.
2019-11-12T02:29:06.6359427Z ##[section]Starting: Checkout
2019-11-12T02:29:06.6361174Z ==============================================================================
2019-11-12T02:29:06.6361229Z Task         : Get sources
2019-11-12T02:29:06.6361276Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
