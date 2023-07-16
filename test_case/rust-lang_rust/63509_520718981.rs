plain
2019-08-13T06:57:28.5965569Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-13T06:57:28.6175673Z ##[command]git config gc.auto 0
2019-08-13T06:57:28.6245462Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-13T06:57:28.6306801Z ##[command]git config --get-all http.proxy
2019-08-13T06:57:28.6445442Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63509/merge:refs/remotes/pull/63509/merge
---
2019-08-13T06:58:04.1622956Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-13T06:58:04.1623289Z 
2019-08-13T06:58:04.1623827Z   git checkout -b <new-branch-name>
2019-08-13T06:58:04.1624117Z 
2019-08-13T06:58:04.1624409Z HEAD is now at 9a7ec9d73 Merge a9c7081fe93ada5478e6725b991e81f16e614ca1 into 60960a260f7b5c695fd0717311d72ce62dd4eb43
2019-08-13T06:58:04.1789962Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-13T06:58:04.1793556Z ==============================================================================
2019-08-13T06:58:04.1793642Z Task         : Bash
2019-08-13T06:58:04.1793694Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-13T07:03:57.3420662Z    Compiling serde_json v1.0.40
2019-08-13T07:04:01.8135858Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-08-13T07:04:10.6612073Z     Finished release [optimized] target(s) in 1m 31s
2019-08-13T07:04:10.6693373Z tidy check
2019-08-13T07:04:11.3960790Z tidy error: /checkout/src/test/ui/issues/issue-63398.rs:10: line longer than 100 chars
2019-08-13T07:04:12.7285351Z some tidy checks failed
2019-08-13T07:04:12.7290901Z 
2019-08-13T07:04:12.7290901Z 
2019-08-13T07:04:12.7295814Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-13T07:04:12.7295993Z 
2019-08-13T07:04:12.7296024Z 
2019-08-13T07:04:12.7320106Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-13T07:04:12.7320306Z Build completed unsuccessfully in 0:01:34
2019-08-13T07:04:12.7320306Z Build completed unsuccessfully in 0:01:34
2019-08-13T07:04:14.1101052Z ##[error]Bash exited with code '1'.
2019-08-13T07:04:14.1137010Z ##[section]Starting: Checkout
2019-08-13T07:04:14.1138837Z ==============================================================================
2019-08-13T07:04:14.1138917Z Task         : Get sources
2019-08-13T07:04:14.1138972Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
