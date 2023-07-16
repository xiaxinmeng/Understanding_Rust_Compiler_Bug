plain
2019-10-14T14:51:56.1422827Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-10-14T14:51:56.1531249Z ##[command]git config gc.auto 0
2019-10-14T14:51:56.1615446Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-10-14T14:51:56.1674274Z ##[command]git config --get-all http.proxy
2019-10-14T14:51:56.1849625Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/65307/merge:refs/remotes/pull/65307/merge
---
2019-10-14T14:58:09.7838720Z    Compiling serde_json v1.0.40
2019-10-14T14:58:11.7817399Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-10-14T14:58:24.3443050Z     Finished release [optimized] target(s) in 1m 36s
2019-10-14T14:58:24.3524722Z tidy check
2019-10-14T14:58:25.1209972Z tidy error: /checkout/src/librustc/hir/lowering.rs:3300: trailing whitespace
2019-10-14T14:58:26.7715847Z some tidy checks failed
2019-10-14T14:58:26.7716009Z Found 482 error codes
2019-10-14T14:58:26.7716058Z Found 0 error codes with no tests
2019-10-14T14:58:26.7716102Z Done!
2019-10-14T14:58:26.7716102Z Done!
2019-10-14T14:58:26.7716131Z 
2019-10-14T14:58:26.7716176Z 
2019-10-14T14:58:26.7718096Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-10-14T14:58:26.7718253Z 
2019-10-14T14:58:26.7718279Z 
2019-10-14T14:58:26.7718327Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-10-14T14:58:26.7718407Z Build completed unsuccessfully in 0:01:40
2019-10-14T14:58:26.7718407Z Build completed unsuccessfully in 0:01:40
2019-10-14T14:58:26.7763841Z == clock drift check ==
2019-10-14T14:58:26.7777766Z   local time: Mon Oct 14 14:58:26 UTC 2019
2019-10-14T14:58:26.9274879Z   network time: Mon, 14 Oct 2019 14:58:26 GMT
2019-10-14T14:58:26.9279791Z == end clock drift check ==
2019-10-14T14:58:27.7247259Z ##[error]Bash exited with code '1'.
2019-10-14T14:58:27.7294733Z ##[section]Starting: Checkout
2019-10-14T14:58:27.7296516Z ==============================================================================
2019-10-14T14:58:27.7296572Z Task         : Get sources
2019-10-14T14:58:27.7296620Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
