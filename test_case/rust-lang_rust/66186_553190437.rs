plain
2019-11-13T00:50:50.7234935Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-13T00:50:50.7417405Z ##[command]git config gc.auto 0
2019-11-13T00:50:50.7495928Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-13T00:50:50.7552631Z ##[command]git config --get-all http.proxy
2019-11-13T00:50:51.4432363Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66186/merge:refs/remotes/pull/66186/merge
---
2019-11-13T00:57:00.4265400Z    Compiling serde_json v1.0.40
2019-11-13T00:57:02.3126582Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-11-13T00:57:13.9734603Z     Finished release [optimized] target(s) in 1m 31s
2019-11-13T00:57:13.9806620Z tidy check
2019-11-13T00:57:14.7871379Z tidy error: /checkout/src/librustc/error_codes.rs:1915: line longer than 80 chars
2019-11-13T00:57:16.7315449Z Found 485 error codes
2019-11-13T00:57:16.7322391Z Found 0 error codes with no tests
2019-11-13T00:57:16.7327540Z Done!
2019-11-13T00:57:16.7327597Z some tidy checks failed
2019-11-13T00:57:16.7327597Z some tidy checks failed
2019-11-13T00:57:16.7327629Z 
2019-11-13T00:57:16.7327656Z 
2019-11-13T00:57:16.7328922Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-13T00:57:16.7329080Z 
2019-11-13T00:57:16.7329121Z 
2019-11-13T00:57:16.7329172Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-13T00:57:16.7329224Z Build completed unsuccessfully in 0:01:35
2019-11-13T00:57:16.7329224Z Build completed unsuccessfully in 0:01:35
2019-11-13T00:57:16.7351711Z == clock drift check ==
2019-11-13T00:57:16.7363542Z   local time: Wed Nov 13 00:57:16 UTC 2019
2019-11-13T00:57:16.8843253Z   network time: Wed, 13 Nov 2019 00:57:16 GMT
2019-11-13T00:57:16.8845383Z == end clock drift check ==
2019-11-13T00:57:18.2286300Z 
2019-11-13T00:57:18.2389131Z ##[error]Bash exited with code '1'.
2019-11-13T00:57:18.2417113Z ##[section]Starting: Checkout
2019-11-13T00:57:18.2419201Z ==============================================================================
2019-11-13T00:57:18.2419258Z Task         : Get sources
2019-11-13T00:57:18.2419321Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
