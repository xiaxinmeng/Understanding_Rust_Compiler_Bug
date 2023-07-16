plain
2019-08-08T14:57:09.7998733Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-08T14:57:09.8183853Z ##[command]git config gc.auto 0
2019-08-08T14:57:09.8250106Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-08T14:57:09.8303105Z ##[command]git config --get-all http.proxy
2019-08-08T14:57:09.8437893Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63008/merge:refs/remotes/pull/63008/merge
---
2019-08-08T14:57:44.3532239Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-08T14:57:44.3533600Z 
2019-08-08T14:57:44.3535584Z   git checkout -b <new-branch-name>
2019-08-08T14:57:44.3536215Z 
2019-08-08T14:57:44.3536743Z HEAD is now at 59290648e Merge b2f919985b365c912aafdbe37dce920e25f38cd6 into 2628f579f6246df385acf9203bf2ffb6aedf5ccc
2019-08-08T14:57:44.3657989Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-08T14:57:44.3660819Z ==============================================================================
2019-08-08T14:57:44.3660880Z Task         : Bash
2019-08-08T14:57:44.3660946Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-08T15:03:43.3011301Z    Compiling serde_json v1.0.40
2019-08-08T15:03:47.5124052Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-08-08T15:03:55.9110832Z     Finished release [optimized] target(s) in 1m 27s
2019-08-08T15:03:55.9174980Z tidy check
2019-08-08T15:03:56.1193234Z tidy error: /checkout/src/test/ui/consts/const-eval/match-test-ptr-null.rs:6: trailing whitespace
2019-08-08T15:03:57.7467810Z some tidy checks failed
2019-08-08T15:03:57.7468336Z 
2019-08-08T15:03:57.7468336Z 
2019-08-08T15:03:57.7469585Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-08T15:03:57.7469728Z 
2019-08-08T15:03:57.7469754Z 
2019-08-08T15:03:57.7469825Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-08T15:03:57.7469878Z Build completed unsuccessfully in 0:01:30
2019-08-08T15:03:57.7469878Z Build completed unsuccessfully in 0:01:30
2019-08-08T15:03:59.0351150Z ##[error]Bash exited with code '1'.
2019-08-08T15:03:59.0383050Z ##[section]Starting: Checkout
2019-08-08T15:03:59.0384751Z ==============================================================================
2019-08-08T15:03:59.0384820Z Task         : Get sources
2019-08-08T15:03:59.0384864Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
