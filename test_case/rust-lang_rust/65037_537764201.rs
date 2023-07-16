plain
2019-10-03T02:41:07.1585591Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-03T02:41:07.1797212Z ##[command]git config gc.auto 0
2019-10-03T02:41:07.1886785Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-03T02:41:07.1954938Z ##[command]git config --get-all http.proxy
2019-10-03T02:41:07.2105548Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65037/merge:refs/remotes/pull/65037/merge
---
2019-10-03T02:48:27.1741353Z tidy error: /checkout/src/test/ui/feature-gates/feature-gate-track_caller.rs: leading newline
2019-10-03T02:48:28.9600229Z some tidy checks failed
2019-10-03T02:48:28.9604329Z 
2019-10-03T02:48:28.9604822Z 
2019-10-03T02:48:28.9605914Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-03T02:48:28.9606527Z 
2019-10-03T02:48:28.9606617Z 
2019-10-03T02:48:28.9617832Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-03T02:48:28.9618081Z Build completed unsuccessfully in 0:01:36
2019-10-03T02:48:28.9618081Z Build completed unsuccessfully in 0:01:36
2019-10-03T02:48:28.9669758Z == clock drift check ==
2019-10-03T02:48:28.9687437Z   local time: Thu Oct  3 02:48:28 UTC 2019
2019-10-03T02:48:29.0631137Z   network time: Thu, 03 Oct 2019 02:48:29 GMT
2019-10-03T02:48:29.0632857Z == end clock drift check ==
2019-10-03T02:48:30.4119019Z ##[error]Bash exited with code '1'.
2019-10-03T02:48:30.4150627Z ##[section]Starting: Checkout
2019-10-03T02:48:30.4152186Z ==============================================================================
2019-10-03T02:48:30.4152235Z Task         : Get sources
2019-10-03T02:48:30.4152295Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
