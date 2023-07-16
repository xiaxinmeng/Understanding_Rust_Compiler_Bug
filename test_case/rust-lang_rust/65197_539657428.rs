plain
2019-10-08T18:46:01.1269711Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-08T18:46:01.1491341Z ##[command]git config gc.auto 0
2019-10-08T18:46:01.1583902Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-08T18:46:01.1664782Z ##[command]git config --get-all http.proxy
2019-10-08T18:46:01.1811414Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65197/merge:refs/remotes/pull/65197/merge
---
2019-10-08T18:53:30.9124041Z    Compiling serde_json v1.0.40
2019-10-08T18:53:33.0998357Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-08T18:53:45.1501718Z     Finished release [optimized] target(s) in 1m 37s
2019-10-08T18:53:45.1637059Z tidy check
2019-10-08T18:53:45.8784674Z tidy error: /checkout/src/librustc/mir/visit.rs:798: line longer than 100 chars
2019-10-08T18:53:47.2235647Z some tidy checks failed
2019-10-08T18:53:47.2238966Z 
2019-10-08T18:53:47.2238966Z 
2019-10-08T18:53:47.2249711Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-08T18:53:47.2251227Z 
2019-10-08T18:53:47.2251263Z 
2019-10-08T18:53:47.2259650Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-08T18:53:47.2260089Z Build completed unsuccessfully in 0:01:40
2019-10-08T18:53:47.2260089Z Build completed unsuccessfully in 0:01:40
2019-10-08T18:53:47.2314515Z == clock drift check ==
2019-10-08T18:53:47.2329962Z   local time: Tue Oct  8 18:53:47 UTC 2019
2019-10-08T18:53:47.3957172Z   network time: Tue, 08 Oct 2019 18:53:47 GMT
2019-10-08T18:53:47.3960811Z == end clock drift check ==
2019-10-08T18:53:48.7723724Z ##[error]Bash exited with code '1'.
2019-10-08T18:53:48.7781085Z ##[section]Starting: Checkout
2019-10-08T18:53:48.7782874Z ==============================================================================
2019-10-08T18:53:48.7782950Z Task         : Get sources
2019-10-08T18:53:48.7783001Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
