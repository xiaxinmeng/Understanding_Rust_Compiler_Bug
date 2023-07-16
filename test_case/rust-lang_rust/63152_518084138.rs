plain
2019-08-05T04:27:45.8840036Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-05T04:27:45.9043104Z ##[command]git config gc.auto 0
2019-08-05T04:27:45.9126991Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-05T04:27:45.9207216Z ##[command]git config --get-all http.proxy
2019-08-05T04:27:45.9371209Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63152/merge:refs/remotes/pull/63152/merge
---
2019-08-05T04:28:20.6578450Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-05T04:28:20.6578484Z 
2019-08-05T04:28:20.6578800Z   git checkout -b <new-branch-name>
2019-08-05T04:28:20.6578833Z 
2019-08-05T04:28:20.6578910Z HEAD is now at 3d21b04b5 Merge a7b7f0724d9140175ae6dacfed16106ddc52978e into d3f8a0b5dfddfe443d9db1f1da18348dbceb0e47
2019-08-05T04:28:20.6745053Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-05T04:28:20.6748201Z ==============================================================================
2019-08-05T04:28:20.6748268Z Task         : Bash
2019-08-05T04:28:20.6748347Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-05T04:34:12.9305041Z tidy check
2019-08-05T04:34:13.9921104Z * 578 error codes
2019-08-05T04:34:13.9921224Z * highest error code: E0733
2019-08-05T04:34:14.3488000Z * 262 features
2019-08-05T04:34:14.9697496Z Stray file with UI testing output: "/checkout/src/test/ui/huge-array-simple.stderr"
2019-08-05T04:34:15.0129662Z some tidy checks failed
2019-08-05T04:34:15.0134860Z 
2019-08-05T04:34:15.0134860Z 
2019-08-05T04:34:15.0135850Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-08-05T04:34:15.0135974Z 
2019-08-05T04:34:15.0135999Z 
2019-08-05T04:34:15.0143105Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-08-05T04:34:15.0143475Z Build completed unsuccessfully in 0:01:33
2019-08-05T04:34:15.0143475Z Build completed unsuccessfully in 0:01:33
2019-08-05T04:34:16.3697271Z ##[error]Bash exited with code '1'.
2019-08-05T04:34:16.3734864Z ##[section]Starting: Checkout
2019-08-05T04:34:16.3736626Z ==============================================================================
2019-08-05T04:34:16.3737016Z Task         : Get sources
2019-08-05T04:34:16.3737091Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
