plain
2019-09-29T23:27:34.8597017Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-29T23:27:34.8798167Z ##[command]git config gc.auto 0
2019-09-29T23:27:34.8881372Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-29T23:27:34.8937742Z ##[command]git config --get-all http.proxy
2019-09-29T23:27:34.9089150Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64700/merge:refs/remotes/pull/64700/merge
---
2019-09-29T23:34:53.0632978Z    Compiling serde_json v1.0.40
2019-09-29T23:34:55.1200874Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-09-29T23:35:06.8895891Z     Finished release [optimized] target(s) in 1m 34s
2019-09-29T23:35:06.8973540Z tidy check
2019-09-29T23:35:07.1646076Z tidy error: /checkout/src/test/ui/const-generics/const-expression-parameter.rs:64: line longer than 100 chars
2019-09-29T23:35:07.1646821Z tidy error: /checkout/src/test/ui/const-generics/const-expression-parameter.rs:67: line longer than 100 chars
2019-09-29T23:35:07.1647134Z tidy error: /checkout/src/test/ui/const-generics/const-expression-parameter.rs: missing trailing newline
2019-09-29T23:35:08.9651956Z some tidy checks failed
2019-09-29T23:35:08.9653006Z 
2019-09-29T23:35:08.9653006Z 
2019-09-29T23:35:08.9654364Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-29T23:35:08.9654506Z 
2019-09-29T23:35:08.9654533Z 
2019-09-29T23:35:08.9665077Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-29T23:35:08.9665156Z Build completed unsuccessfully in 0:01:37
2019-09-29T23:35:08.9665156Z Build completed unsuccessfully in 0:01:37
2019-09-29T23:35:08.9722535Z == clock drift check ==
2019-09-29T23:35:08.9744679Z   local time: Sun Sep 29 23:35:08 UTC 2019
2019-09-29T23:35:09.2520271Z   network time: Sun, 29 Sep 2019 23:35:09 GMT
2019-09-29T23:35:09.2523425Z == end clock drift check ==
2019-09-29T23:35:10.6468280Z ##[error]Bash exited with code '1'.
2019-09-29T23:35:10.6510985Z ##[section]Starting: Checkout
2019-09-29T23:35:10.6513076Z ==============================================================================
2019-09-29T23:35:10.6513136Z Task         : Get sources
2019-09-29T23:35:10.6513186Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
