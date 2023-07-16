plain
2019-10-14T13:55:00.0675543Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-14T13:55:00.1169025Z ##[command]git config gc.auto 0
2019-10-14T13:55:00.1203668Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-14T13:55:00.1255141Z ##[command]git config --get-all http.proxy
2019-10-14T13:55:00.1389624Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65307/merge:refs/remotes/pull/65307/merge
---
2019-10-14T14:01:29.7202354Z    Compiling serde_json v1.0.40
2019-10-14T14:01:31.4379471Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-14T14:01:42.6096955Z     Finished release [optimized] target(s) in 1m 27s
2019-10-14T14:01:42.6171282Z tidy check
2019-10-14T14:01:43.3193873Z tidy error: /checkout/src/librustc/hir/lowering.rs:3297: line longer than 100 chars
2019-10-14T14:01:43.3194075Z tidy error: /checkout/src/librustc/hir/lowering.rs:3298: line longer than 100 chars
2019-10-14T14:01:43.3194121Z tidy error: /checkout/src/librustc/hir/lowering.rs:3299: line longer than 100 chars
2019-10-14T14:01:43.3194181Z tidy error: /checkout/src/librustc/hir/lowering.rs:3300: line longer than 100 chars
2019-10-14T14:01:44.6577604Z Found 482 error codes
2019-10-14T14:01:44.6577900Z Found 0 error codes with no tests
2019-10-14T14:01:44.6581925Z Done!
2019-10-14T14:01:44.6582206Z some tidy checks failed
2019-10-14T14:01:44.6582206Z some tidy checks failed
2019-10-14T14:01:44.6582252Z 
2019-10-14T14:01:44.6582274Z 
2019-10-14T14:01:44.6583465Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-14T14:01:44.6583695Z 
2019-10-14T14:01:44.6583716Z 
2019-10-14T14:01:44.6583805Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-14T14:01:44.6583846Z Build completed unsuccessfully in 0:01:31
2019-10-14T14:01:44.6583846Z Build completed unsuccessfully in 0:01:31
2019-10-14T14:01:44.6627303Z == clock drift check ==
2019-10-14T14:01:44.6644847Z   local time: Mon Oct 14 14:01:44 UTC 2019
2019-10-14T14:01:44.7608054Z   network time: Mon, 14 Oct 2019 14:01:44 GMT
2019-10-14T14:01:44.7615251Z == end clock drift check ==
2019-10-14T14:01:45.6525208Z ##[error]Bash exited with code '1'.
2019-10-14T14:01:45.6677724Z ##[section]Starting: Checkout
2019-10-14T14:01:45.6679480Z ==============================================================================
2019-10-14T14:01:45.6679540Z Task         : Get sources
2019-10-14T14:01:45.6679580Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
