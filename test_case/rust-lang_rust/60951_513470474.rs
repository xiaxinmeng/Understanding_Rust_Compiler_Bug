plain
2019-07-20T13:47:21.0576666Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-20T13:47:21.0810915Z ##[command]git config gc.auto 0
2019-07-20T13:47:21.0872732Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-20T13:47:21.0920801Z ##[command]git config --get-all http.proxy
2019-07-20T13:47:21.1050621Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/60951/merge:refs/remotes/pull/60951/merge
---
2019-07-20T13:47:53.8477634Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-20T13:47:53.8478003Z 
2019-07-20T13:47:53.8478228Z   git checkout -b <new-branch-name>
2019-07-20T13:47:53.8478252Z 
2019-07-20T13:47:53.8478310Z HEAD is now at f110b0373 Merge 61fc12eac0a66516886c3d4f539b137e117d4d65 into f69b07144a151f46aaee1b6230ba4160e9394562
2019-07-20T13:47:53.8613569Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-20T13:47:53.8616512Z ==============================================================================
2019-07-20T13:47:53.8616595Z Task         : Bash
2019-07-20T13:47:53.8616643Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-20T13:54:05.6226164Z    Compiling serde_json v1.0.40
2019-07-20T13:54:09.8876300Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-07-20T13:54:18.3802127Z     Finished release [optimized] target(s) in 1m 27s
2019-07-20T13:54:18.3873454Z tidy check
2019-07-20T13:54:18.7465169Z tidy error: /checkout/src/librustc/mir/mod.rs:3090: line longer than 100 chars
2019-07-20T13:54:18.7465444Z tidy error: /checkout/src/librustc/mir/mod.rs:3135: line longer than 100 chars
2019-07-20T13:54:20.1929163Z some tidy checks failed
2019-07-20T13:54:20.1935196Z 
2019-07-20T13:54:20.1935196Z 
2019-07-20T13:54:20.1936814Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-07-20T13:54:20.1936958Z 
2019-07-20T13:54:20.1936981Z 
2019-07-20T13:54:20.1948714Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-20T13:54:20.1948791Z Build completed unsuccessfully in 0:01:30
2019-07-20T13:54:20.1948791Z Build completed unsuccessfully in 0:01:30
2019-07-20T13:54:21.5463663Z ##[error]Bash exited with code '1'.
2019-07-20T13:54:21.5495146Z ##[section]Starting: Checkout
2019-07-20T13:54:21.5520755Z ==============================================================================
2019-07-20T13:54:21.5520813Z Task         : Get sources
2019-07-20T13:54:21.5520860Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
