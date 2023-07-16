plain
2019-12-29T17:29:35.0134012Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-29T17:29:35.8256683Z ##[command]git config gc.auto 0
2019-12-29T17:29:35.8261962Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-29T17:29:35.8265590Z ##[command]git config --get-all http.proxy
2019-12-29T17:29:35.8269209Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67668/merge:refs/remotes/pull/67668/merge
---
2019-12-29T17:36:16.2366060Z Done!
2019-12-29T17:36:16.2366308Z some tidy checks failed
2019-12-29T17:36:16.2400631Z 
2019-12-29T17:36:16.2401243Z 
2019-12-29T17:36:16.2402414Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-29T17:36:16.2402845Z 
2019-12-29T17:36:16.2403017Z 
2019-12-29T17:36:16.2403254Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-29T17:36:16.2403433Z Build completed unsuccessfully in 0:01:35
2019-12-29T17:36:16.2403433Z Build completed unsuccessfully in 0:01:35
2019-12-29T17:36:16.2425298Z == clock drift check ==
2019-12-29T17:36:16.2433708Z   local time: Sun Dec 29 17:36:16 UTC 2019
2019-12-29T17:36:16.4240873Z   network time: Sun, 29 Dec 2019 17:36:16 GMT
2019-12-29T17:36:16.4242920Z == end clock drift check ==
2019-12-29T17:36:17.6282364Z 
2019-12-29T17:36:17.6411558Z ##[error]Bash exited with code '1'.
2019-12-29T17:36:17.6442398Z ##[section]Starting: Checkout
2019-12-29T17:36:17.6444141Z ==============================================================================
2019-12-29T17:36:17.6444201Z Task         : Get sources
2019-12-29T17:36:17.6444267Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
