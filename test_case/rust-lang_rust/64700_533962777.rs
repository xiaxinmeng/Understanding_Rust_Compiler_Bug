plain
2019-09-23T05:00:55.2410397Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-23T05:00:55.2630450Z ##[command]git config gc.auto 0
2019-09-23T05:00:55.2688470Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-23T05:00:55.2741039Z ##[command]git config --get-all http.proxy
2019-09-23T05:00:55.2907760Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64700/merge:refs/remotes/pull/64700/merge
---
2019-09-23T05:08:11.6730018Z    Compiling serde_json v1.0.40
2019-09-23T05:08:13.3096165Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-09-23T05:08:23.7508251Z     Finished release [optimized] target(s) in 1m 25s
2019-09-23T05:08:23.7604542Z tidy check
2019-09-23T05:08:23.8882644Z tidy error: /checkout/src/libsyntax/parse/parser/path.rs:457: trailing whitespace
2019-09-23T05:08:25.7125757Z some tidy checks failed
2019-09-23T05:08:25.7129443Z 
2019-09-23T05:08:25.7129443Z 
2019-09-23T05:08:25.7130359Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-23T05:08:25.7130519Z 
2019-09-23T05:08:25.7130540Z 
2019-09-23T05:08:25.7144944Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-23T05:08:25.7145041Z Build completed unsuccessfully in 0:01:29
2019-09-23T05:08:25.7145041Z Build completed unsuccessfully in 0:01:29
2019-09-23T05:08:25.7191124Z == clock drift check ==
2019-09-23T05:08:25.7207906Z   local time: Mon Sep 23 05:08:25 UTC 2019
2019-09-23T05:08:25.9980315Z   network time: Mon, 23 Sep 2019 05:08:25 GMT
2019-09-23T05:08:25.9980481Z == end clock drift check ==
2019-09-23T05:08:27.3779728Z ##[error]Bash exited with code '1'.
2019-09-23T05:08:27.3811646Z ##[section]Starting: Checkout
2019-09-23T05:08:27.3814197Z ==============================================================================
2019-09-23T05:08:27.3814260Z Task         : Get sources
2019-09-23T05:08:27.3814338Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
