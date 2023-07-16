plain
2019-12-08T01:34:36.8484935Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-08T01:34:36.8495082Z ##[command]git config gc.auto 0
2019-12-08T01:34:36.8497185Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-08T01:34:36.8498929Z ##[command]git config --get-all http.proxy
2019-12-08T01:34:36.8501424Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67136/merge:refs/remotes/pull/67136/merge
---
2019-12-08T01:40:24.5454763Z Error code E0630 needs to have at least one UI test!
2019-12-08T01:40:24.5455420Z some tidy checks failed
2019-12-08T01:40:24.5455821Z 
2019-12-08T01:40:24.5456001Z 
2019-12-08T01:40:24.5456927Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-08T01:40:24.5457468Z 
2019-12-08T01:40:24.5457641Z 
2019-12-08T01:40:24.5459874Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-08T01:40:24.5460165Z Build completed unsuccessfully in 0:01:19
2019-12-08T01:40:24.5460165Z Build completed unsuccessfully in 0:01:19
2019-12-08T01:40:24.5506462Z == clock drift check ==
2019-12-08T01:40:24.5515047Z   local time: Sun Dec  8 01:40:24 UTC 2019
2019-12-08T01:40:24.6946681Z   network time: Sun, 08 Dec 2019 01:40:24 GMT
2019-12-08T01:40:24.6947385Z == end clock drift check ==
2019-12-08T01:40:26.0166874Z 
2019-12-08T01:40:26.0250301Z ##[error]Bash exited with code '1'.
2019-12-08T01:40:26.0275572Z ##[section]Starting: Checkout
2019-12-08T01:40:26.0276930Z ==============================================================================
2019-12-08T01:40:26.0276972Z Task         : Get sources
2019-12-08T01:40:26.0277024Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
