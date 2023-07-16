plain
2019-08-10T07:37:36.6840614Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-10T07:37:36.7033518Z ##[command]git config gc.auto 0
2019-08-10T07:37:36.7114488Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-10T07:37:36.7171949Z ##[command]git config --get-all http.proxy
2019-08-10T07:37:36.7301904Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63343/merge:refs/remotes/pull/63343/merge
---
2019-08-10T07:38:11.7237322Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-10T07:38:11.7237392Z 
2019-08-10T07:38:11.7237560Z   git checkout -b <new-branch-name>
2019-08-10T07:38:11.7237582Z 
2019-08-10T07:38:11.7237617Z HEAD is now at e5da8a16d Merge 633548dd9b8d5aeab4d99e655f0559313df9aa4f into be8bbb06976c8065425b18e9cbe24a6d1d4e7515
2019-08-10T07:38:11.7366214Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-10T07:38:11.7369127Z ==============================================================================
2019-08-10T07:38:11.7369187Z Task         : Bash
2019-08-10T07:38:11.7369251Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-10T07:42:58.4353323Z    |
2019-08-10T07:42:58.4353526Z 76 | #![feature(const_generics)]
2019-08-10T07:42:58.4353715Z    |            ^^^^^^^^^^^^^^
2019-08-10T07:42:58.4353743Z 
2019-08-10T07:43:01.4057831Z error[E0549]: rustc_deprecated attribute must be paired with either stable or unstable attribute
2019-08-10T07:43:01.4058587Z     |
2019-08-10T07:43:01.4058587Z     |
2019-08-10T07:43:01.4058879Z 707 |     pub fn init<T>() -> T;
2019-08-10T07:43:01.4059185Z 
2019-08-10T07:43:01.4059185Z 
2019-08-10T07:43:01.4773101Z error: use of item 'intrinsics::uninit' that will be deprecated in future version 1.38.0: superseded by MaybeUninit, removal planned
2019-08-10T07:43:01.4773440Z    --> src/libcore/mem/mod.rs:485:5
2019-08-10T07:43:01.4773870Z 485 |     intrinsics::uninit()
2019-08-10T07:43:01.4774075Z     |     ^^^^^^^^^^^^^^^^^^
2019-08-10T07:43:01.4774259Z     |
2019-08-10T07:43:01.4774479Z     = note: `-D deprecated-in-future` implied by `-D warnings`
---
2019-08-10T07:43:08.6014158Z    Compiling unwind v0.0.0 (/checkout/src/libunwind)
2019-08-10T07:43:08.6372602Z error: Could not compile `core`.
2019-08-10T07:43:08.6391797Z warning: build failed, waiting for other jobs to finish...
2019-08-10T07:43:08.9934024Z error: build failed
2019-08-10T07:43:08.9959148Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
2019-08-10T07:43:08.9970275Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-08-10T07:43:08.9970658Z Build completed unsuccessfully in 0:02:09
2019-08-10T07:43:08.9970658Z Build completed unsuccessfully in 0:02:09
2019-08-10T07:43:22.9696018Z ##[error]Bash exited with code '1'.
2019-08-10T07:43:22.9732337Z ##[section]Starting: Checkout
2019-08-10T07:43:22.9733915Z ==============================================================================
2019-08-10T07:43:22.9733984Z Task         : Get sources
2019-08-10T07:43:22.9734027Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
