plain
2019-07-30T15:57:33.4391189Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-30T15:57:33.4566415Z ##[command]git config gc.auto 0
2019-07-30T15:57:33.4625121Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-30T15:57:33.4669053Z ##[command]git config --get-all http.proxy
2019-07-30T15:57:33.4783449Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63127/merge:refs/remotes/pull/63127/merge
---
2019-07-30T15:58:08.3198062Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-30T15:58:08.3198094Z 
2019-07-30T15:58:08.3198322Z   git checkout -b <new-branch-name>
2019-07-30T15:58:08.3198352Z 
2019-07-30T15:58:08.3198420Z HEAD is now at d9aa6faa3 Merge 5f155f103f1b6ce1aef11a22f2f34450ff40558a into f690098e6d65ad7b33dc7fdefccc387806782027
2019-07-30T15:58:08.3320254Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-30T15:58:08.3323001Z ==============================================================================
2019-07-30T15:58:08.3323044Z Task         : Bash
2019-07-30T15:58:08.3323077Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-30T16:03:50.0321046Z    Compiling serde_json v1.0.40
2019-07-30T16:03:53.8187490Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-07-30T16:04:01.6267000Z     Finished release [optimized] target(s) in 1m 21s
2019-07-30T16:04:01.6355934Z tidy check
2019-07-30T16:04:01.7562835Z tidy error: /checkout/src/librustc_typeck/astconv.rs:254: trailing whitespace
2019-07-30T16:04:01.8794900Z tidy error: /checkout/src/librustc_mir/borrow_check/nll/region_infer/error_reporting/region_name.rs:567: line longer than 100 chars
2019-07-30T16:04:01.9115067Z tidy error: /checkout/src/librustc/middle/resolve_lifetime.rs:1913: line longer than 100 chars
2019-07-30T16:04:03.2999766Z some tidy checks failed
2019-07-30T16:04:03.2999885Z 
2019-07-30T16:04:03.2999885Z 
2019-07-30T16:04:03.3007046Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-07-30T16:04:03.3007555Z 
2019-07-30T16:04:03.3007655Z 
2019-07-30T16:04:03.3007820Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-30T16:04:03.3007965Z Build completed unsuccessfully in 0:01:24
2019-07-30T16:04:03.3007965Z Build completed unsuccessfully in 0:01:24
2019-07-30T16:04:04.8028651Z ##[error]Bash exited with code '1'.
2019-07-30T16:04:04.8058059Z ##[section]Starting: Checkout
2019-07-30T16:04:04.8059434Z ==============================================================================
2019-07-30T16:04:04.8059488Z Task         : Get sources
2019-07-30T16:04:04.8059525Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
