plain
2019-11-12T16:33:04.9649557Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-12T16:33:04.9843691Z ##[command]git config gc.auto 0
2019-11-12T16:33:04.9940351Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-12T16:33:05.0017566Z ##[command]git config --get-all http.proxy
2019-11-12T16:33:05.0160465Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66321/merge:refs/remotes/pull/66321/merge
---
2019-11-12T16:39:36.6586527Z    Compiling serde_json v1.0.40
2019-11-12T16:39:38.5359901Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-11-12T16:39:50.6123575Z     Finished release [optimized] target(s) in 1m 33s
2019-11-12T16:39:50.6205709Z tidy check
2019-11-12T16:39:51.2871936Z tidy error: /checkout/src/librustc_mir/transform/generator.rs:1093: line longer than 100 chars
2019-11-12T16:39:51.2921572Z tidy error: /checkout/src/librustc_mir/transform/const_prop.rs:92: TODO is deprecated; use FIXME
2019-11-12T16:39:51.5867946Z tidy error: /checkout/src/librustc/mir/mod.rs:9: line longer than 100 chars
2019-11-12T16:39:51.5868795Z tidy error: /checkout/src/librustc/mir/mod.rs:9: TODO is deprecated; use FIXME
2019-11-12T16:39:51.5886699Z tidy error: /checkout/src/librustc/mir/mod.rs:3024: TODO is deprecated; use FIXME
2019-11-12T16:39:53.5033318Z Found 485 error codes
2019-11-12T16:39:53.5033580Z Found 0 error codes with no tests
2019-11-12T16:39:53.5033625Z Done!
2019-11-12T16:39:53.5033936Z some tidy checks failed
2019-11-12T16:39:53.5033936Z some tidy checks failed
2019-11-12T16:39:53.5033980Z 
2019-11-12T16:39:53.5034025Z 
2019-11-12T16:39:53.5034894Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-12T16:39:53.5035018Z 
2019-11-12T16:39:53.5035161Z 
2019-11-12T16:39:53.5038816Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-12T16:39:53.5039695Z Build completed unsuccessfully in 0:01:37
2019-11-12T16:39:53.5039695Z Build completed unsuccessfully in 0:01:37
2019-11-12T16:39:53.5084676Z == clock drift check ==
2019-11-12T16:39:53.5091289Z   local time: Tue Nov 12 16:39:53 UTC 2019
2019-11-12T16:39:53.6057998Z   network time: Tue, 12 Nov 2019 16:39:53 GMT
2019-11-12T16:39:53.6061508Z == end clock drift check ==
2019-11-12T16:39:54.9471259Z 
2019-11-12T16:39:54.9544162Z ##[error]Bash exited with code '1'.
2019-11-12T16:39:54.9578440Z ##[section]Starting: Checkout
2019-11-12T16:39:54.9580924Z ==============================================================================
2019-11-12T16:39:54.9581011Z Task         : Get sources
2019-11-12T16:39:54.9581065Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
