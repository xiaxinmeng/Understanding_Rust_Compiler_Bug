plain
2019-07-25T21:53:50.5074660Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-25T21:53:50.5258485Z ##[command]git config gc.auto 0
2019-07-25T21:53:50.5321878Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-25T21:53:50.5371285Z ##[command]git config --get-all http.proxy
2019-07-25T21:53:50.5497424Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62982/merge:refs/remotes/pull/62982/merge
---
2019-07-25T21:54:25.0970371Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-25T21:54:25.0970610Z 
2019-07-25T21:54:25.0970976Z   git checkout -b <new-branch-name>
2019-07-25T21:54:25.0971201Z 
2019-07-25T21:54:25.0971533Z HEAD is now at 3c326a6c6 Merge d9ac0c67ed5a3ea7d708894a4636a6e83c5aec49 into eedf6ce4ef54bb03818ab21d714f1b9f13a6b31c
2019-07-25T21:54:25.1095623Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-25T21:54:25.1098416Z ==============================================================================
2019-07-25T21:54:25.1098459Z Task         : Bash
2019-07-25T21:54:25.1098494Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-25T22:00:21.9746089Z    Compiling serde_json v1.0.40
2019-07-25T22:00:25.8346832Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2019-07-25T22:00:33.6614712Z     Finished release [optimized] target(s) in 1m 22s
2019-07-25T22:00:33.6675842Z tidy check
2019-07-25T22:00:33.9147658Z tidy error: /checkout/src/librustc_mir/interpret/memory.rs:538: line longer than 100 chars
2019-07-25T22:00:35.4949603Z some tidy checks failed
2019-07-25T22:00:35.4949722Z 
2019-07-25T22:00:35.4949722Z 
2019-07-25T22:00:35.4953435Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-07-25T22:00:35.4953538Z 
2019-07-25T22:00:35.4953560Z 
2019-07-25T22:00:35.4958173Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-07-25T22:00:35.4958226Z Build completed unsuccessfully in 0:01:25
2019-07-25T22:00:35.4958226Z Build completed unsuccessfully in 0:01:25
2019-07-25T22:00:36.9459161Z ##[error]Bash exited with code '1'.
2019-07-25T22:00:36.9513858Z ##[section]Starting: Checkout
2019-07-25T22:00:36.9515664Z ==============================================================================
2019-07-25T22:00:36.9515727Z Task         : Get sources
2019-07-25T22:00:36.9515785Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
