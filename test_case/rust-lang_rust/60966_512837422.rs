plain
2019-07-18T13:51:58.5510280Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-18T13:51:58.5684326Z ##[command]git config gc.auto 0
2019-07-18T13:51:58.5757815Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-18T13:51:58.5809117Z ##[command]git config --get-all http.proxy
2019-07-18T13:51:59.1064177Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/60966/merge:refs/remotes/pull/60966/merge
---
2019-07-18T13:52:33.5602694Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-18T13:52:33.5602726Z 
2019-07-18T13:52:33.5602956Z   git checkout -b <new-branch-name>
2019-07-18T13:52:33.5602986Z 
2019-07-18T13:52:33.5603052Z HEAD is now at c32f59ad6 Merge be212df8c899eda5a45da1fdcccff68f65bf38ba into 21d5b8bf0c26e3ee4c270ce5527df66b1af56513
2019-07-18T13:52:33.5747674Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-18T13:52:33.5750885Z ==============================================================================
2019-07-18T13:52:33.5750946Z Task         : Bash
2019-07-18T13:52:33.5750994Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-18T14:22:07.4004626Z    |
2019-07-18T14:22:07.4012420Z 77 | #![cfg_attr(not(bootstrap), feature(const_generics))]
2019-07-18T14:22:07.4019945Z    |                                     ^^^^^^^^^^^^^^
2019-07-18T14:22:07.4057273Z 
2019-07-18T14:22:08.7912428Z error[E0658]: The attribute `rustc_diagnostic_item` is currently unknown to the compiler and may have meaning added to it in the future
2019-07-18T14:22:08.7913625Z    --> src/libcore/fmt/mod.rs:522:28
2019-07-18T14:22:08.7914712Z     |
2019-07-18T14:22:08.7915917Z 522 | #[cfg_attr(not(bootstrap), rustc_diagnostic_item = "debug_trait")]
2019-07-18T14:22:08.7923284Z     |
2019-07-18T14:22:08.7923284Z     |
2019-07-18T14:22:08.7924280Z     = note: for more information, see ***/issues/29642
2019-07-18T14:22:08.7925071Z     = help: add `#![feature(custom_attribute)]` to the crate attributes to enable
2019-07-18T14:22:08.7925362Z 
2019-07-18T14:22:08.9394249Z error: cannot determine resolution for the derive macro `Debug`
2019-07-18T14:22:08.9395358Z   --> src/libcore/num/dec2flt/rawfp.rs:30:23
2019-07-18T14:22:08.9397070Z    |
2019-07-18T14:22:08.9397723Z 30 | #[derive(Copy, Clone, Debug)]
2019-07-18T14:22:08.9398994Z    |
2019-07-18T14:22:08.9398994Z    |
2019-07-18T14:22:08.9399756Z    = note: import resolution is stuck, try simplifying macro imports
2019-07-18T14:22:15.7296095Z    Compiling libc v0.2.54
2019-07-18T14:22:17.0799587Z    Compiling autocfg v0.1.4
2019-07-18T14:22:18.2733567Z    Compiling unwind v0.0.0 (/checkout/src/libunwind)
2019-07-18T14:22:18.8107789Z    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
2019-07-18T14:22:18.8107789Z    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
2019-07-18T14:22:19.7582462Z error: aborting due to 2 previous errors
2019-07-18T14:22:19.7584084Z 
2019-07-18T14:22:19.7587889Z For more information about this error, try `rustc --explain E0658`.
2019-07-18T14:22:19.9188561Z error: Could not compile `core`.
2019-07-18T14:22:19.9188974Z warning: build failed, waiting for other jobs to finish...
2019-07-18T14:22:20.1959901Z error: build failed
2019-07-18T14:22:20.1985865Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
2019-07-18T14:22:20.1998919Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
2019-07-18T14:22:20.1999003Z Build completed unsuccessfully in 0:23:39
2019-07-18T14:22:20.1999003Z Build completed unsuccessfully in 0:23:39
2019-07-18T14:22:21.0679732Z ##[error]Bash exited with code '1'.
2019-07-18T14:22:21.0721160Z ##[section]Starting: Checkout
2019-07-18T14:22:21.0722907Z ==============================================================================
2019-07-18T14:22:21.0722981Z Task         : Get sources
2019-07-18T14:22:21.0723027Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
