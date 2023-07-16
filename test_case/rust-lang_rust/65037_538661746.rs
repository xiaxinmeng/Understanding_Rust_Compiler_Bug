plain
2019-10-05T15:29:24.5489802Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-05T15:29:24.5679948Z ##[command]git config gc.auto 0
2019-10-05T15:29:24.5750098Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-05T15:29:24.5797381Z ##[command]git config --get-all http.proxy
2019-10-05T15:29:24.5950998Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65037/merge:refs/remotes/pull/65037/merge
---
2019-10-05T15:37:09.1521930Z    Compiling serde_json v1.0.40
2019-10-05T15:37:11.0882597Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-05T15:37:22.9370629Z     Finished release [optimized] target(s) in 1m 35s
2019-10-05T15:37:22.9456860Z tidy check
2019-10-05T15:37:23.6775727Z tidy error: /checkout/src/librustc_typeck/check/wfcheck.rs:199: trailing whitespace
2019-10-05T15:37:24.9424353Z some tidy checks failed
2019-10-05T15:37:24.9424592Z 
2019-10-05T15:37:24.9424592Z 
2019-10-05T15:37:24.9425579Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-05T15:37:24.9425685Z 
2019-10-05T15:37:24.9425709Z 
2019-10-05T15:37:24.9436729Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-05T15:37:24.9436800Z Build completed unsuccessfully in 0:01:39
2019-10-05T15:37:24.9436800Z Build completed unsuccessfully in 0:01:39
2019-10-05T15:37:24.9488134Z == clock drift check ==
2019-10-05T15:37:24.9507741Z   local time: Sat Oct  5 15:37:24 UTC 2019
2019-10-05T15:37:25.3743770Z   network time: Sat, 05 Oct 2019 15:37:25 GMT
2019-10-05T15:37:25.3751427Z == end clock drift check ==
2019-10-05T15:37:26.6936018Z ##[error]Bash exited with code '1'.
2019-10-05T15:37:26.6968368Z ##[section]Starting: Checkout
2019-10-05T15:37:26.6970422Z ==============================================================================
2019-10-05T15:37:26.6970478Z Task         : Get sources
2019-10-05T15:37:26.6970548Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
