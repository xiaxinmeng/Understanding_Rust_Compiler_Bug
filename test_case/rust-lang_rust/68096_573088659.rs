plain
2020-01-10T15:25:56.2336302Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-10T15:25:56.2352539Z ##[command]git config gc.auto 0
2020-01-10T15:25:56.2358688Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-10T15:25:56.2360843Z ##[command]git config --get-all http.proxy
2020-01-10T15:25:56.2365207Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68096/merge:refs/remotes/pull/68096/merge
---
2020-01-10T15:31:12.5989010Z    Compiling serde_json v1.0.40
2020-01-10T15:31:14.3671490Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-01-10T15:31:24.4026689Z     Finished release [optimized] target(s) in 1m 23s
2020-01-10T15:31:24.4140350Z tidy check
2020-01-10T15:31:24.9416165Z tidy error: /checkout/src/test/ui/resolve/impl-items-vis-unresolved.rs:21: line longer than 100 chars
2020-01-10T15:31:27.3513628Z some tidy checks failed
2020-01-10T15:31:27.3513773Z Found 486 error codes
2020-01-10T15:31:27.3513821Z Found 0 error codes with no tests
2020-01-10T15:31:27.3513864Z Done!
2020-01-10T15:31:27.3513864Z Done!
2020-01-10T15:31:27.3521449Z 
2020-01-10T15:31:27.3521609Z 
2020-01-10T15:31:27.3522638Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-01-10T15:31:27.3522784Z 
2020-01-10T15:31:27.3522811Z 
2020-01-10T15:31:27.3530123Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-01-10T15:31:27.3530551Z Build completed unsuccessfully in 0:01:34
2020-01-10T15:31:27.3530551Z Build completed unsuccessfully in 0:01:34
2020-01-10T15:31:27.3583637Z == clock drift check ==
2020-01-10T15:31:27.3591590Z   local time: Fri Jan 10 15:31:27 UTC 2020
2020-01-10T15:31:27.6465332Z   network time: Fri, 10 Jan 2020 15:31:27 GMT
2020-01-10T15:31:27.6472327Z == end clock drift check ==
2020-01-10T15:31:28.3916872Z 
2020-01-10T15:31:28.4024038Z ##[error]Bash exited with code '1'.
2020-01-10T15:31:28.4053221Z ##[section]Starting: Checkout
2020-01-10T15:31:28.4055349Z ==============================================================================
2020-01-10T15:31:28.4055431Z Task         : Get sources
2020-01-10T15:31:28.4055481Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
