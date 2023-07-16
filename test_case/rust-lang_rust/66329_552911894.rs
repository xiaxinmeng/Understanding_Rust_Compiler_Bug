plain
2019-11-12T13:57:31.3960974Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-11-12T13:57:31.9687263Z ##[command]git config gc.auto 0
2019-11-12T13:57:31.9693918Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-11-12T13:57:31.9698046Z ##[command]git config --get-all http.proxy
2019-11-12T13:57:31.9702250Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/66329/merge:refs/remotes/pull/66329/merge
---
2019-11-12T14:04:34.1801734Z Done!
2019-11-12T14:04:34.1801990Z some tidy checks failed
2019-11-12T14:04:34.1802028Z 
2019-11-12T14:04:34.1805364Z 
2019-11-12T14:04:34.1806159Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-11-12T14:04:34.1806269Z 
2019-11-12T14:04:34.1806290Z 
2019-11-12T14:04:34.1806329Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-11-12T14:04:34.1806401Z Build completed unsuccessfully in 0:01:37
2019-11-12T14:04:34.1806401Z Build completed unsuccessfully in 0:01:37
2019-11-12T14:04:34.1806437Z == clock drift check ==
2019-11-12T14:04:34.1806473Z   local time: Tue Nov 12 14:04:34 UTC 2019
2019-11-12T14:04:34.2611892Z   network time: Tue, 12 Nov 2019 14:04:34 GMT
2019-11-12T14:04:34.2612011Z == end clock drift check ==
2019-11-12T14:04:35.6533061Z 
2019-11-12T14:04:35.6648141Z ##[error]Bash exited with code '1'.
2019-11-12T14:04:35.6702771Z ##[section]Starting: Checkout
2019-11-12T14:04:35.6704585Z ==============================================================================
2019-11-12T14:04:35.6704815Z Task         : Get sources
2019-11-12T14:04:35.6705040Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
