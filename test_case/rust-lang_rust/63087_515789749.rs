plain
2019-07-28T19:28:42.9673848Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-28T19:28:42.9879767Z ##[command]git config gc.auto 0
2019-07-28T19:28:42.9946565Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-28T19:28:42.9996474Z ##[command]git config --get-all http.proxy
2019-07-28T19:28:43.0142717Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63087/merge:refs/remotes/pull/63087/merge
---
2019-07-28T19:29:16.5489786Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-28T19:29:16.5489821Z 
2019-07-28T19:29:16.5490070Z   git checkout -b <new-branch-name>
2019-07-28T19:29:16.5490104Z 
2019-07-28T19:29:16.5490170Z HEAD is now at dda57795a Merge 7df07886c36f930be8ba3c970434bfbbacc06d9a into 023525dbda35748a10713471b948974b68a1c2cc
2019-07-28T19:29:16.5626633Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-28T19:29:16.5630449Z ==============================================================================
2019-07-28T19:29:16.5630515Z Task         : Bash
2019-07-28T19:29:16.5630567Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-28T19:35:21.3409978Z     Finished release [optimized] target(s) in 1m 27s
2019-07-28T19:35:21.3504385Z tidy check
2019-07-28T19:35:22.3034923Z * 577 error codes
2019-07-28T19:35:22.3035876Z * highest error code: E0732
2019-07-28T19:35:22.3062991Z tidy error: /checkout/src/tools/rustc-std-workspace-alloc/Cargo.toml doesn't have `edition = "2018"` on a separate line
2019-07-28T19:35:22.3241890Z tidy error: /checkout/src/test/run-make/thumb-none-qemu/example/Cargo.toml doesn't have `edition = "2018"` on a separate line
2019-07-28T19:35:22.3294076Z tidy error: /checkout/src/doc/book/2018-edition/Cargo.toml doesn't have `edition = "2018"` on a separate line
2019-07-28T19:35:22.3295234Z tidy error: /checkout/src/doc/book/ci/stable-check/Cargo.toml doesn't have `edition = "2018"` on a separate line
2019-07-28T19:35:22.3296006Z tidy error: /checkout/src/doc/book/second-edition/Cargo.toml doesn't have `edition = "2018"` on a separate line
2019-07-28T19:35:22.3303938Z tidy error: /checkout/src/doc/reference/stable-check/Cargo.toml doesn't have `edition = "2018"` on a separate line
2019-07-28T19:35:23.3171187Z some tidy checks failed
2019-07-28T19:35:23.3175898Z 
2019-07-28T19:35:23.3175898Z 
2019-07-28T19:35:23.3179400Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-07-28T19:35:23.3179586Z 
2019-07-28T19:35:23.3179863Z 
2019-07-28T19:35:23.3194171Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-28T19:35:23.3194274Z Build completed unsuccessfully in 0:01:31
2019-07-28T19:35:23.3194274Z Build completed unsuccessfully in 0:01:31
2019-07-28T19:35:24.6869602Z ##[error]Bash exited with code '1'.
2019-07-28T19:35:24.6911463Z ##[section]Starting: Checkout
2019-07-28T19:35:24.6913587Z ==============================================================================
2019-07-28T19:35:24.6913672Z Task         : Get sources
2019-07-28T19:35:24.6913727Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
