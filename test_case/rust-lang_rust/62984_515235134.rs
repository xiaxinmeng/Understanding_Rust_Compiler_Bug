plain
2019-07-25T21:27:25.6526737Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-25T21:27:25.6724505Z ##[command]git config gc.auto 0
2019-07-25T21:27:25.6794394Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-25T21:27:25.6859051Z ##[command]git config --get-all http.proxy
2019-07-25T21:27:25.7006579Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62984/merge:refs/remotes/pull/62984/merge
---
2019-07-25T21:28:00.2186918Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-25T21:28:00.2186982Z 
2019-07-25T21:28:00.2187279Z   git checkout -b <new-branch-name>
2019-07-25T21:28:00.2187497Z 
2019-07-25T21:28:00.2187552Z HEAD is now at 8647f7372 Merge 2bfed94a09a3117c730f6fa4e66b3c61f8db4976 into eedf6ce4ef54bb03818ab21d714f1b9f13a6b31c
2019-07-25T21:28:00.2354694Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-25T21:28:00.2358193Z ==============================================================================
2019-07-25T21:28:00.2358263Z Task         : Bash
2019-07-25T21:28:00.2358316Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-25T22:14:01.2118204Z    Compiling rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
2019-07-25T22:14:01.9976973Z error: unnecessary trailing semicolons
2019-07-25T22:14:01.9991837Z   --> src/librustc_typeck/structured_errors.rs:51:9
2019-07-25T22:14:01.9996967Z    |
2019-07-25T22:14:02.0014596Z 51 |         __diagnostic_used!(E0617);
2019-07-25T22:14:02.0018302Z    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove these semicolons
2019-07-25T22:14:02.0019862Z    = note: `-D redundant-semicolon` implied by `-D warnings`
2019-07-25T22:14:02.0019937Z 
2019-07-25T22:14:02.0084024Z error: unnecessary trailing semicolons
2019-07-25T22:14:02.0085460Z    --> src/librustc_typeck/structured_errors.rs:107:9
2019-07-25T22:14:02.0085460Z    --> src/librustc_typeck/structured_errors.rs:107:9
2019-07-25T22:14:02.0088578Z     |
2019-07-25T22:14:02.0089347Z 107 |         __diagnostic_used!(E0607);
2019-07-25T22:14:02.0090077Z     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^ help: remove these semicolons
2019-07-25T22:14:05.6880456Z error: aborting due to 2 previous errors
2019-07-25T22:14:05.6880650Z 
2019-07-25T22:14:05.7468326Z error: Could not compile `rustc_typeck`.
2019-07-25T22:14:05.7479062Z warning: build failed, waiting for other jobs to finish...
2019-07-25T22:14:05.7479062Z warning: build failed, waiting for other jobs to finish...
2019-07-25T22:16:35.1530422Z error: build failed
2019-07-25T22:16:35.1569984Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
2019-07-25T22:16:35.1570622Z expected success, got: exit code: 101
2019-07-25T22:16:35.1574229Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-25T22:16:35.1574588Z Build completed unsuccessfully in 0:42:18
2019-07-25T22:16:36.9197219Z ##[error]Bash exited with code '1'.
2019-07-25T22:16:36.9230930Z ##[section]Starting: Checkout
2019-07-25T22:16:36.9233110Z ==============================================================================
2019-07-25T22:16:36.9233173Z Task         : Get sources
2019-07-25T22:16:36.9233240Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
