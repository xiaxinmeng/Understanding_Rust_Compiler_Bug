plain
2019-11-16T04:24:03.9077431Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-16T04:24:03.9270705Z ##[command]git config gc.auto 0
2019-11-16T04:24:03.9341344Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-16T04:24:03.9401518Z ##[command]git config --get-all http.proxy
2019-11-16T04:24:03.9554084Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66215/merge:refs/remotes/pull/66215/merge
---
2019-11-16T04:30:08.3585111Z    Compiling serde_json v1.0.40
2019-11-16T04:30:10.1369296Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-11-16T04:30:21.3633653Z     Finished release [optimized] target(s) in 1m 29s
2019-11-16T04:30:21.3732727Z tidy check
2019-11-16T04:30:21.5251037Z tidy error: /checkout/src/liballoc/string.rs:2003: line longer than 100 chars
2019-11-16T04:30:24.1085849Z some tidy checks failed
2019-11-16T04:30:24.1089764Z Found 441 error codes
2019-11-16T04:30:24.1143818Z Found 0 error codes with no tests
2019-11-16T04:30:24.1143903Z Done!
2019-11-16T04:30:24.1143903Z Done!
2019-11-16T04:30:24.1143931Z 
2019-11-16T04:30:24.1143954Z 
2019-11-16T04:30:24.1144740Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-16T04:30:24.1144856Z 
2019-11-16T04:30:24.1144878Z 
2019-11-16T04:30:24.1144919Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-16T04:30:24.1144980Z Build completed unsuccessfully in 0:01:33
2019-11-16T04:30:24.1144980Z Build completed unsuccessfully in 0:01:33
2019-11-16T04:30:24.1147344Z == clock drift check ==
2019-11-16T04:30:24.1210501Z   local time: Sat Nov 16 04:30:24 UTC 2019
2019-11-16T04:30:24.3952422Z   network time: Sat, 16 Nov 2019 04:30:24 GMT
2019-11-16T04:30:24.3953935Z == end clock drift check ==
2019-11-16T04:30:25.7737916Z 
2019-11-16T04:30:25.7838933Z ##[error]Bash exited with code '1'.
2019-11-16T04:30:25.7873144Z ##[section]Starting: Checkout
2019-11-16T04:30:25.7875033Z ==============================================================================
2019-11-16T04:30:25.7875099Z Task         : Get sources
2019-11-16T04:30:25.7875142Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
