plain
2019-08-26T04:11:15.3229162Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-26T04:11:16.2175925Z ##[command]git config gc.auto 0
2019-08-26T04:11:16.2180610Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-26T04:11:16.2183097Z ##[command]git config --get-all http.proxy
2019-08-26T04:11:16.2187942Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63906/merge:refs/remotes/pull/63906/merge
---
2019-08-26T04:11:52.2661544Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-26T04:11:52.2661567Z 
2019-08-26T04:11:52.2661749Z   git checkout -b <new-branch-name>
2019-08-26T04:11:52.2662803Z 
2019-08-26T04:11:52.2663588Z HEAD is now at 019ba58cc Merge 4ab65c6a993e82a85c8ce50aa52a7215e53a5b41 into 4c58535d09d1261d21569df0036b974811544256
2019-08-26T04:11:52.2816432Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-26T04:11:52.2819474Z ==============================================================================
2019-08-26T04:11:52.2819696Z Task         : Bash
2019-08-26T04:11:52.2819865Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-26T04:42:46.3754255Z    Compiling semver v0.9.0
2019-08-26T04:42:47.8045566Z error: failed to run custom build command for `libc v0.2.61`
2019-08-26T04:42:47.8045677Z 
2019-08-26T04:42:47.8045727Z Caused by:
2019-08-26T04:42:47.8046283Z   process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/build/libc-b24c65ec356e6fe9/build-script-build` (signal: 11, SIGSEGV: invalid memory reference)
2019-08-26T04:42:48.1971967Z error: build failed
2019-08-26T04:42:48.2007703Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-08-26T04:42:48.2008046Z expected success, got: exit code: 101
2019-08-26T04:42:48.2017509Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-26T04:42:48.2017509Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-08-26T04:42:48.2017586Z Build completed unsuccessfully in 0:24:38
2019-08-26T04:42:48.2070109Z == clock drift check ==
2019-08-26T04:42:48.2085072Z   local time: Mon Aug 26 04:42:48 UTC 2019
2019-08-26T04:42:48.3578557Z   network time: Mon, 26 Aug 2019 04:42:48 GMT
2019-08-26T04:42:48.3581470Z == end clock drift check ==
2019-08-26T04:42:49.5777899Z ##[error]Bash exited with code '1'.
2019-08-26T04:42:49.5842012Z ##[section]Starting: Checkout
2019-08-26T04:42:49.5844336Z ==============================================================================
2019-08-26T04:42:49.5844408Z Task         : Get sources
2019-08-26T04:42:49.5844456Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
