plain
2019-11-29T22:36:03.0272189Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-29T22:36:03.0455235Z ##[command]git config gc.auto 0
2019-11-29T22:36:03.0543804Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-29T22:36:03.0613388Z ##[command]git config --get-all http.proxy
2019-11-29T22:36:03.0743604Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66881/merge:refs/remotes/pull/66881/merge
---
2019-11-29T22:41:28.4914510Z    Compiling serde_json v1.0.40
2019-11-29T22:41:29.9993582Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-11-29T22:41:40.2863524Z     Finished release [optimized] target(s) in 1m 21s
2019-11-29T22:41:40.2973192Z tidy check
2019-11-29T22:41:41.1372764Z tidy error: /checkout/src/libcore/cmp.rs:1137: undocumented unsafe
2019-11-29T22:41:42.7887837Z some tidy checks failed
2019-11-29T22:41:42.7888011Z Found 486 error codes
2019-11-29T22:41:42.7888051Z Found 0 error codes with no tests
2019-11-29T22:41:42.7888090Z Done!
2019-11-29T22:41:42.7888090Z Done!
2019-11-29T22:41:42.7888113Z 
2019-11-29T22:41:42.7888134Z 
2019-11-29T22:41:42.7891452Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-29T22:41:42.7891578Z 
2019-11-29T22:41:42.7891625Z 
2019-11-29T22:41:42.7896350Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-29T22:41:42.7896803Z Build completed unsuccessfully in 0:01:24
2019-11-29T22:41:42.7896803Z Build completed unsuccessfully in 0:01:24
2019-11-29T22:41:42.7959975Z == clock drift check ==
2019-11-29T22:41:42.7965124Z   local time: Fri Nov 29 22:41:42 UTC 2019
2019-11-29T22:41:42.8806777Z   network time: Fri, 29 Nov 2019 22:41:42 GMT
2019-11-29T22:41:42.8811448Z == end clock drift check ==
2019-11-29T22:41:44.2093575Z 
2019-11-29T22:41:44.2191949Z ##[error]Bash exited with code '1'.
2019-11-29T22:41:44.2216266Z ##[section]Starting: Checkout
2019-11-29T22:41:44.2217676Z ==============================================================================
2019-11-29T22:41:44.2217719Z Task         : Get sources
2019-11-29T22:41:44.2217771Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
