plain
2019-07-26T13:10:07.6796398Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-26T13:10:07.7002660Z ##[command]git config gc.auto 0
2019-07-26T13:10:07.7082213Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-26T13:10:07.7130258Z ##[command]git config --get-all http.proxy
2019-07-26T13:10:07.7297228Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62959/merge:refs/remotes/pull/62959/merge
---
2019-07-26T13:10:42.7364386Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-26T13:10:42.7364609Z 
2019-07-26T13:10:42.7365008Z   git checkout -b <new-branch-name>
2019-07-26T13:10:42.7365181Z 
2019-07-26T13:10:42.7365347Z HEAD is now at 90b1bbd77 Merge 6f72750669adc9b1021a0765589cebbd4e1ca712 into 1a563362865e6051d4c350544131228e8eff5138
2019-07-26T13:10:42.7531656Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-26T13:10:42.7535100Z ==============================================================================
2019-07-26T13:10:42.7535166Z Task         : Bash
2019-07-26T13:10:42.7535217Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-26T13:16:33.5836300Z    Compiling serde_json v1.0.40
2019-07-26T13:16:37.5713207Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-07-26T13:16:46.4366232Z     Finished release [optimized] target(s) in 1m 30s
2019-07-26T13:16:46.4444747Z tidy check
2019-07-26T13:16:46.6484834Z tidy error: /checkout/src/libcore/array.rs:151: TODO is deprecated; use FIXME
2019-07-26T13:16:48.2491644Z some tidy checks failed
2019-07-26T13:16:48.2492440Z 
2019-07-26T13:16:48.2492440Z 
2019-07-26T13:16:48.2496982Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-07-26T13:16:48.2497444Z 
2019-07-26T13:16:48.2497475Z 
2019-07-26T13:16:48.2501093Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-26T13:16:48.2502498Z Build completed unsuccessfully in 0:01:33
2019-07-26T13:16:48.2502498Z Build completed unsuccessfully in 0:01:33
2019-07-26T13:16:49.5641789Z ##[error]Bash exited with code '1'.
2019-07-26T13:16:49.5676291Z ##[section]Starting: Checkout
2019-07-26T13:16:49.5678055Z ==============================================================================
2019-07-26T13:16:49.5678107Z Task         : Get sources
2019-07-26T13:16:49.5678148Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
