plain
2019-11-13T17:57:41.9335710Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-13T17:57:41.9529078Z ##[command]git config gc.auto 0
2019-11-13T17:57:41.9590200Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-13T17:57:42.5615464Z ##[command]git config --get-all http.proxy
2019-11-13T17:57:42.5623290Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66379/merge:refs/remotes/pull/66379/merge
---
2019-11-13T18:04:12.4229439Z Found 0 error codes with no tests
2019-11-13T18:04:12.4229488Z Done!
2019-11-13T18:04:12.4229546Z 
2019-11-13T18:04:12.4229593Z 
2019-11-13T18:04:12.4231667Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-13T18:04:12.4231833Z 
2019-11-13T18:04:12.4231861Z 
2019-11-13T18:04:12.4237773Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-13T18:04:12.4237868Z Build completed unsuccessfully in 0:01:39
2019-11-13T18:04:12.4237868Z Build completed unsuccessfully in 0:01:39
2019-11-13T18:04:12.4294484Z == clock drift check ==
2019-11-13T18:04:12.4315222Z   local time: Wed Nov 13 18:04:12 UTC 2019
2019-11-13T18:04:12.5821548Z   network time: Wed, 13 Nov 2019 18:04:12 GMT
2019-11-13T18:04:12.5826974Z == end clock drift check ==
2019-11-13T18:04:13.8882637Z 
2019-11-13T18:04:13.9008829Z ##[error]Bash exited with code '1'.
2019-11-13T18:04:13.9039347Z ##[section]Starting: Checkout
2019-11-13T18:04:13.9042240Z ==============================================================================
2019-11-13T18:04:13.9042319Z Task         : Get sources
2019-11-13T18:04:13.9042366Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
