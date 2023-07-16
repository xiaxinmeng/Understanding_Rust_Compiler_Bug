plain
2020-01-02T08:54:07.8892166Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-02T08:54:07.9069249Z ##[command]git config gc.auto 0
2020-01-02T08:54:08.5196709Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-02T08:54:08.5212621Z ##[command]git config --get-all http.proxy
2020-01-02T08:54:08.5224534Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67803/merge:refs/remotes/pull/67803/merge
---
2020-01-02T09:01:40.8605192Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2020-01-02T09:01:42.1877365Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-01-02T09:01:42.6318625Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-01-02T09:01:44.1398085Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-01-02T09:01:52.1214517Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-01-02T09:01:54.8345630Z     Checking rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-01-02T09:01:56.5925829Z     Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
2020-01-02T09:02:57.5314700Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-01-02T09:02:58.9723714Z     Checking rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
2020-01-02T09:02:58.9723714Z     Checking rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
2020-01-02T09:03:03.0705424Z     Checking rustc_passes v0.0.0 (/checkout/src/librustc_passes)
2020-01-02T09:03:03.2077827Z error[E0432]: unresolved import `rustc::util::nodemap`
2020-01-02T09:03:03.2078270Z   --> src/librustc_passes/stability.rs:14:18
2020-01-02T09:03:03.2078517Z    |
2020-01-02T09:03:03.2078866Z 14 | use rustc::util::nodemap::{FxHashMap, FxHashSet};
2020-01-02T09:03:03.2079215Z    |                  ^^^^^^^ could not find `nodemap` in `util`
2020-01-02T09:03:03.9325434Z error: aborting due to previous error
2020-01-02T09:03:03.9325532Z 
2020-01-02T09:03:03.9325882Z For more information about this error, try `rustc --explain E0432`.
2020-01-02T09:03:03.9370215Z error: could not compile `rustc_passes`.
2020-01-02T09:03:03.9370215Z error: could not compile `rustc_passes`.
2020-01-02T09:03:03.9370597Z warning: build failed, waiting for other jobs to finish...
2020-01-02T09:03:22.6716610Z error: build failed
2020-01-02T09:03:22.6742907Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-01-02T09:03:22.6756126Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-01-02T09:03:22.6756567Z Build completed unsuccessfully in 0:05:50
2020-01-02T09:03:22.6814883Z == clock drift check ==
2020-01-02T09:03:22.6830433Z   local time: Thu Jan  2 09:03:22 UTC 2020
2020-01-02T09:03:22.6830433Z   local time: Thu Jan  2 09:03:22 UTC 2020
2020-01-02T09:03:22.8455126Z   network time: Thu, 02 Jan 2020 09:03:22 GMT
2020-01-02T09:03:22.8456424Z == end clock drift check ==
2020-01-02T09:03:24.0826790Z 
2020-01-02T09:03:24.0940602Z ##[error]Bash exited with code '1'.
2020-01-02T09:03:24.0970125Z ##[section]Starting: Checkout
2020-01-02T09:03:24.0971925Z ==============================================================================
2020-01-02T09:03:24.0972004Z Task         : Get sources
2020-01-02T09:03:24.0972057Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
