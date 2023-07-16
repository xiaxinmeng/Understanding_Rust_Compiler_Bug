plain
2019-08-13T08:00:52.4676579Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-13T08:00:52.4846095Z ##[command]git config gc.auto 0
2019-08-13T08:00:52.4917683Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-13T08:00:52.4972812Z ##[command]git config --get-all http.proxy
2019-08-13T08:00:52.5111170Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63512/merge:refs/remotes/pull/63512/merge
---
2019-08-13T08:01:30.2375865Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-13T08:01:30.2375904Z 
2019-08-13T08:01:30.2376066Z   git checkout -b <new-branch-name>
2019-08-13T08:01:30.2376088Z 
2019-08-13T08:01:30.2376125Z HEAD is now at de4c4c165 Merge 44e58f748299111c0de9f7f1f6dda472bf0fe88f into 60960a260f7b5c695fd0717311d72ce62dd4eb43
2019-08-13T08:01:30.2520840Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-08-13T08:01:30.2524000Z ==============================================================================
2019-08-13T08:01:30.2524058Z Task         : Bash
2019-08-13T08:01:30.2524105Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-08-13T08:06:08.3385040Z    Compiling unwind v0.0.0 (/checkout/src/libunwind)
2019-08-13T08:06:08.8127640Z    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
2019-08-13T08:06:10.3431331Z    Compiling compiler_builtins v0.1.18
2019-08-13T08:06:12.6545570Z    Compiling cmake v0.1.38
2019-08-13T08:06:14.5123877Z error: item has missing stability attribute
2019-08-13T08:06:14.5133207Z   --> src/libcore/task/poll.rs:86:5
2019-08-13T08:06:14.5140954Z    |
2019-08-13T08:06:14.5147001Z 86 | /     pub fn map_ok<U, F>(self, f: F) -> Poll<Option<Result<U, E>>>
2019-08-13T08:06:14.5173480Z 87 | |         where F: FnOnce(T) -> U
2019-08-13T08:06:14.5174189Z 89 | |         match self {
2019-08-13T08:06:14.5174418Z ...  |
2019-08-13T08:06:14.5175306Z 94 | |         }
2019-08-13T08:06:14.5175687Z 95 | |     }
2019-08-13T08:06:14.5175687Z 95 | |     }
2019-08-13T08:06:14.5175946Z    | |_____^
2019-08-13T08:06:14.5175983Z 
2019-08-13T08:06:14.5176232Z error: item has missing stability attribute
2019-08-13T08:06:14.5176536Z    --> src/libcore/task/poll.rs:98:5
2019-08-13T08:06:14.5176756Z     |
2019-08-13T08:06:14.5177113Z 98  | /     pub fn map_err<U, F>(self, f: F) -> Poll<Option<Result<T, U>>>
2019-08-13T08:06:14.5177476Z 99  | |         where F: FnOnce(E) -> U
2019-08-13T08:06:14.5178297Z 101 | |         match self {
2019-08-13T08:06:14.5178713Z ...   |
2019-08-13T08:06:14.5178938Z 106 | |         }
2019-08-13T08:06:14.5179170Z 107 | |     }
2019-08-13T08:06:14.5179170Z 107 | |     }
2019-08-13T08:06:14.5249680Z     | |_____^
2019-08-13T08:06:14.5249912Z 
2019-08-13T08:06:15.0062259Z error: aborting due to 2 previous errors
2019-08-13T08:06:15.0067143Z 
2019-08-13T08:06:15.1363055Z error: Could not compile `core`.
2019-08-13T08:06:15.1363336Z warning: build failed, waiting for other jobs to finish...
2019-08-13T08:06:15.4466575Z error: build failed
2019-08-13T08:06:15.4483822Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
2019-08-13T08:06:15.4498526Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-08-13T08:06:15.4498759Z Build completed unsuccessfully in 0:02:10
2019-08-13T08:06:15.4498759Z Build completed unsuccessfully in 0:02:10
2019-08-13T08:06:27.9929843Z ##[error]Bash exited with code '1'.
2019-08-13T08:06:27.9961442Z ##[section]Starting: Checkout
2019-08-13T08:06:27.9963767Z ==============================================================================
2019-08-13T08:06:27.9963823Z Task         : Get sources
2019-08-13T08:06:27.9963888Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
