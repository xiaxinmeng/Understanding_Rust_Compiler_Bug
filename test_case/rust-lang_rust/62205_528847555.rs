plain
2019-09-06T12:47:44.8896981Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-06T12:47:44.9082508Z ##[command]git config gc.auto 0
2019-09-06T12:47:44.9167032Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-06T12:47:44.9228962Z ##[command]git config --get-all http.proxy
2019-09-06T12:47:44.9374128Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62205/merge:refs/remotes/pull/62205/merge
---
2019-09-06T12:54:38.6407606Z    Compiling serde_json v1.0.40
2019-09-06T12:54:40.5923386Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-09-06T12:54:52.1793364Z     Finished release [optimized] target(s) in 1m 35s
2019-09-06T12:54:52.1876304Z tidy check
2019-09-06T12:54:52.6987477Z tidy error: /checkout/src/libcore/iter/traits/iterator.rs: ignoring file length unnecessarily
2019-09-06T12:54:54.2959219Z some tidy checks failed
2019-09-06T12:54:54.2964256Z 
2019-09-06T12:54:54.2964256Z 
2019-09-06T12:54:54.2965966Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-06T12:54:54.2971572Z 
2019-09-06T12:54:54.2971871Z 
2019-09-06T12:54:54.2982870Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-06T12:54:54.2983388Z Build completed unsuccessfully in 0:01:39
2019-09-06T12:54:54.2983388Z Build completed unsuccessfully in 0:01:39
2019-09-06T12:54:54.3083686Z == clock drift check ==
2019-09-06T12:54:54.3139945Z   local time: Fri Sep  6 12:54:54 UTC 2019
2019-09-06T12:54:54.4646759Z   network time: Fri, 06 Sep 2019 12:54:54 GMT
2019-09-06T12:54:54.4647152Z == end clock drift check ==
2019-09-06T12:54:55.7773425Z ##[error]Bash exited with code '1'.
2019-09-06T12:54:55.7807176Z ##[section]Starting: Checkout
2019-09-06T12:54:55.7808947Z ==============================================================================
2019-09-06T12:54:55.7809025Z Task         : Get sources
2019-09-06T12:54:55.7809073Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
