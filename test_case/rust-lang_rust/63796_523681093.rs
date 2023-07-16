plain
2019-08-21T22:38:25.5394372Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-21T22:38:25.5623951Z ##[command]git config gc.auto 0
2019-08-21T22:38:25.5715392Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-21T22:38:25.5780491Z ##[command]git config --get-all http.proxy
2019-08-21T22:38:25.5938553Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63796/merge:refs/remotes/pull/63796/merge
---
2019-08-21T22:39:00.4842542Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-21T22:39:00.4842591Z 
2019-08-21T22:39:00.4842808Z   git checkout -b <new-branch-name>
2019-08-21T22:39:00.4843044Z 
2019-08-21T22:39:00.4843096Z HEAD is now at 65111dcbb Merge 4889e2e1ab72d2038d8413a39bef664cd0387e2d into e44fdf97929d1315add3b76208adf99e8299252d
2019-08-21T22:39:00.5012993Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-21T22:39:00.5015905Z ==============================================================================
2019-08-21T22:39:00.5015963Z Task         : Bash
2019-08-21T22:39:00.5016027Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-21T22:45:29.7999693Z    Compiling serde_json v1.0.40
2019-08-21T22:45:31.7015796Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-08-21T22:45:42.9886342Z     Finished release [optimized] target(s) in 1m 33s
2019-08-21T22:45:42.9991945Z tidy check
2019-08-21T22:45:43.5747660Z tidy error: /checkout/src/test/ui/suggestions/opaque-type-error.rs:7: trailing whitespace
2019-08-21T22:45:43.5748038Z tidy error: /checkout/src/test/ui/suggestions/opaque-type-error.rs:15: trailing whitespace
2019-08-21T22:45:45.0487662Z some tidy checks failed
2019-08-21T22:45:45.0492823Z 
2019-08-21T22:45:45.0492823Z 
2019-08-21T22:45:45.0493950Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-21T22:45:45.0494897Z 
2019-08-21T22:45:45.0495834Z 
2019-08-21T22:45:45.0500268Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-21T22:45:45.0500778Z Build completed unsuccessfully in 0:01:36
2019-08-21T22:45:45.0500778Z Build completed unsuccessfully in 0:01:36
2019-08-21T22:45:45.0554008Z == clock drift check ==
2019-08-21T22:45:45.0572435Z   local time: Wed Aug 21 22:45:45 UTC 2019
2019-08-21T22:45:45.1415458Z   network time: Wed, 21 Aug 2019 22:45:45 GMT
2019-08-21T22:45:45.1417206Z == end clock drift check ==
2019-08-21T22:45:46.4686599Z ##[error]Bash exited with code '1'.
2019-08-21T22:45:46.4727713Z ##[section]Starting: Checkout
2019-08-21T22:45:46.4729426Z ==============================================================================
2019-08-21T22:45:46.4729483Z Task         : Get sources
2019-08-21T22:45:46.4729535Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
