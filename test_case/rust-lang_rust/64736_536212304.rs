plain
2019-09-28T17:50:31.1960629Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-28T17:50:31.2208002Z ##[command]git config gc.auto 0
2019-09-28T17:50:31.2303218Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-28T17:50:31.2369092Z ##[command]git config --get-all http.proxy
2019-09-28T17:50:31.2534629Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64736/merge:refs/remotes/pull/64736/merge
---
2019-09-28T17:57:36.7694095Z    Compiling serde_json v1.0.40
2019-09-28T17:57:38.6574426Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-09-28T17:57:50.1401209Z     Finished release [optimized] target(s) in 1m 31s
2019-09-28T17:57:50.1500237Z tidy check
2019-09-28T17:57:50.6418353Z tidy error: /checkout/src/librustc_mir/transform/simplify.rs:280: line longer than 100 chars
2019-09-28T17:57:50.9335490Z tidy error: /checkout/src/librustc/mir/mod.rs:216: line longer than 100 chars
2019-09-28T17:57:50.9335672Z tidy error: /checkout/src/librustc/mir/mod.rs:266: TODO is deprecated; use FIXME
2019-09-28T17:57:52.2541490Z some tidy checks failed
2019-09-28T17:57:52.2547453Z 
2019-09-28T17:57:52.2547453Z 
2019-09-28T17:57:52.2548768Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-28T17:57:52.2548921Z 
2019-09-28T17:57:52.2548946Z 
2019-09-28T17:57:52.2562467Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-28T17:57:52.2562554Z Build completed unsuccessfully in 0:01:34
2019-09-28T17:57:52.2562554Z Build completed unsuccessfully in 0:01:34
2019-09-28T17:57:52.2619295Z == clock drift check ==
2019-09-28T17:57:52.2634247Z   local time: Sat Sep 28 17:57:52 UTC 2019
2019-09-28T17:57:52.4165715Z   network time: Sat, 28 Sep 2019 17:57:52 GMT
2019-09-28T17:57:52.4171278Z == end clock drift check ==
2019-09-28T17:57:53.7891128Z ##[error]Bash exited with code '1'.
2019-09-28T17:57:53.7934730Z ##[section]Starting: Checkout
2019-09-28T17:57:53.7936711Z ==============================================================================
2019-09-28T17:57:53.7936768Z Task         : Get sources
2019-09-28T17:57:53.7936814Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
