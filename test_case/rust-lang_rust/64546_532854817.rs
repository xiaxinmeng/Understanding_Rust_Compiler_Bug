plain
2019-09-18T20:15:28.8471982Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-18T20:15:28.8695843Z ##[command]git config gc.auto 0
2019-09-18T20:15:28.8792181Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-18T20:15:28.8852610Z ##[command]git config --get-all http.proxy
2019-09-18T20:15:28.9015124Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64546/merge:refs/remotes/pull/64546/merge
---
2019-09-18T20:22:46.5176252Z    Compiling serde_json v1.0.40
2019-09-18T20:22:48.4169798Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-09-18T20:22:59.8571410Z     Finished release [optimized] target(s) in 1m 35s
2019-09-18T20:22:59.8657771Z tidy check
2019-09-18T20:23:00.5314602Z tidy error: /checkout/src/test/ui/coherence/impl-foreign[foreign]-for-foreign.rs:13: line longer than 100 chars
2019-09-18T20:23:01.9382698Z some tidy checks failed
2019-09-18T20:23:01.9383726Z 
2019-09-18T20:23:01.9383726Z 
2019-09-18T20:23:01.9385329Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-18T20:23:01.9385609Z 
2019-09-18T20:23:01.9386208Z 
2019-09-18T20:23:01.9393547Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-18T20:23:01.9394074Z Build completed unsuccessfully in 0:01:38
2019-09-18T20:23:01.9394074Z Build completed unsuccessfully in 0:01:38
2019-09-18T20:23:01.9449624Z == clock drift check ==
2019-09-18T20:23:01.9470702Z   local time: Wed Sep 18 20:23:01 UTC 2019
2019-09-18T20:23:02.1118165Z   network time: Wed, 18 Sep 2019 20:23:02 GMT
2019-09-18T20:23:02.1123875Z == end clock drift check ==
2019-09-18T20:23:03.5036022Z ##[error]Bash exited with code '1'.
2019-09-18T20:23:03.5070634Z ##[section]Starting: Checkout
2019-09-18T20:23:03.5072947Z ==============================================================================
2019-09-18T20:23:03.5073027Z Task         : Get sources
2019-09-18T20:23:03.5073233Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
