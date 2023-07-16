plain
2019-09-12T19:35:42.9378034Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-09-12T19:35:42.9643729Z ##[command]git config gc.auto 0
2019-09-12T19:35:42.9727403Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-09-12T19:35:42.9798495Z ##[command]git config --get-all http.proxy
2019-09-12T19:35:42.9981277Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/64358/merge:refs/remotes/pull/64358/merge
---
2019-09-12T19:39:23.7782859Z ###############################                                           43.5%
2019-09-12T19:39:23.8038159Z #############################################################             85.6%
2019-09-12T19:39:23.8039019Z ######################################################################## 100.0%
2019-09-12T19:39:24.3694008Z extracting /checkout/obj/build/cache/2019-08-13/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-09-12T19:39:24.4366313Z     Updating git repository `https://github.com/cuviper/rayon`
---
2019-09-12T19:42:57.7267748Z * highest error code: E0733
2019-09-12T19:42:58.0903070Z * 263 features
2019-09-12T19:42:58.7504238Z Dependencies not on the whitelist:
2019-09-12T19:42:58.7505134Z * crossbeam-queue 
2019-09-12T19:42:58.7505212Z invalid source: "git+https://github.com/cuviper/rayon?branch=rustc#7ffde12e0cc7b5ab2ed8bea8c6c08c3005ac4db7"
2019-09-12T19:42:58.7505295Z invalid source: "git+https://github.com/cuviper/rayon?branch=rustc#7ffde12e0cc7b5ab2ed8bea8c6c08c3005ac4db7"
2019-09-12T19:42:58.8026661Z some tidy checks failed
2019-09-12T19:42:58.8026768Z 
2019-09-12T19:42:58.8026768Z 
2019-09-12T19:42:58.8027586Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-09-12T19:42:58.8027686Z 
2019-09-12T19:42:58.8027729Z 
2019-09-12T19:42:58.8036443Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-09-12T19:42:58.8036519Z Build completed unsuccessfully in 0:01:33
2019-09-12T19:42:58.8036519Z Build completed unsuccessfully in 0:01:33
2019-09-12T19:42:58.8102601Z == clock drift check ==
2019-09-12T19:42:58.8132132Z   local time: Thu Sep 12 19:42:58 UTC 2019
2019-09-12T19:42:58.8904919Z   network time: Thu, 12 Sep 2019 19:42:58 GMT
2019-09-12T19:42:58.8905484Z == end clock drift check ==
2019-09-12T19:43:00.3323363Z ##[error]Bash exited with code '1'.
2019-09-12T19:43:00.3383753Z ##[section]Starting: Checkout
2019-09-12T19:43:00.3386141Z ==============================================================================
2019-09-12T19:43:00.3386206Z Task         : Get sources
2019-09-12T19:43:00.3386262Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
