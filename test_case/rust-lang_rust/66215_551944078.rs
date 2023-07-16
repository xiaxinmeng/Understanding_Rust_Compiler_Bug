plain
2019-11-08T18:25:08.3330577Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-08T18:25:08.3530501Z ##[command]git config gc.auto 0
2019-11-08T18:25:08.3603780Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-08T18:25:08.3665899Z ##[command]git config --get-all http.proxy
2019-11-08T18:25:08.3807412Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66215/merge:refs/remotes/pull/66215/merge
---
2019-11-08T18:31:41.4826085Z Found 0 error codes with no tests
2019-11-08T18:31:41.4826479Z Done!
2019-11-08T18:31:41.4826587Z 
2019-11-08T18:31:41.4826621Z 
2019-11-08T18:31:41.4827598Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-08T18:31:41.4827720Z 
2019-11-08T18:31:41.4827749Z 
2019-11-08T18:31:41.4836635Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-08T18:31:41.4836745Z Build completed unsuccessfully in 0:01:35
2019-11-08T18:31:41.4836745Z Build completed unsuccessfully in 0:01:35
2019-11-08T18:31:41.4891257Z == clock drift check ==
2019-11-08T18:31:41.4903430Z   local time: Fri Nov  8 18:31:41 UTC 2019
2019-11-08T18:31:41.5609387Z   network time: Fri, 08 Nov 2019 18:31:41 GMT
2019-11-08T18:31:41.5643360Z == end clock drift check ==
2019-11-08T18:31:42.8266573Z 
2019-11-08T18:31:42.8373886Z ##[error]Bash exited with code '1'.
2019-11-08T18:31:42.8405784Z ##[section]Starting: Checkout
2019-11-08T18:31:42.8408934Z ==============================================================================
2019-11-08T18:31:42.8408998Z Task         : Get sources
2019-11-08T18:31:42.8409070Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
