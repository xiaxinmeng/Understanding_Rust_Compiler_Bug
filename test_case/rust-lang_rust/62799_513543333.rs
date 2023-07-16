plain
2019-07-21T10:30:07.7350600Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-21T10:30:08.3137296Z ##[command]git config gc.auto 0
2019-07-21T10:30:08.3143749Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-21T10:30:08.3149746Z ##[command]git config --get-all http.proxy
2019-07-21T10:30:08.3152904Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62799/merge:refs/remotes/pull/62799/merge
---
2019-07-21T10:30:42.8072851Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-21T10:30:42.8072883Z 
2019-07-21T10:30:42.8073334Z   git checkout -b <new-branch-name>
2019-07-21T10:30:42.8073365Z 
2019-07-21T10:30:42.8073441Z HEAD is now at 1b45b5e3c Merge 4b47e78a16524ed4878bfffaaf60c32bb18d88ae into 1301422a6c2e8916560b8cc2f0564f38d8858a75
2019-07-21T10:30:42.8207521Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-21T10:30:42.8209720Z ==============================================================================
2019-07-21T10:30:42.8209764Z Task         : Bash
2019-07-21T10:30:42.8209814Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-21T10:36:59.3365617Z     Finished release [optimized] target(s) in 1m 29s
2019-07-21T10:36:59.3438336Z tidy check
2019-07-21T10:37:00.2715485Z * 577 error codes
2019-07-21T10:37:00.2715595Z * highest error code: E0732
2019-07-21T10:37:00.3128062Z tidy error: /checkout/src/libcore/mem/maybe_uninit.rs:256: malformed stability attribute: missing `feature` key
2019-07-21T10:37:01.1369702Z some tidy checks failed
2019-07-21T10:37:01.1371237Z 
2019-07-21T10:37:01.1371237Z 
2019-07-21T10:37:01.1372279Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-07-21T10:37:01.1372860Z 
2019-07-21T10:37:01.1373042Z 
2019-07-21T10:37:01.1376215Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-21T10:37:01.1376733Z Build completed unsuccessfully in 0:01:32
2019-07-21T10:37:01.1376733Z Build completed unsuccessfully in 0:01:32
2019-07-21T10:37:02.7082063Z ##[error]Bash exited with code '1'.
2019-07-21T10:37:02.7118602Z ##[section]Starting: Checkout
2019-07-21T10:37:02.7120699Z ==============================================================================
2019-07-21T10:37:02.7120757Z Task         : Get sources
2019-07-21T10:37:02.7120805Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
