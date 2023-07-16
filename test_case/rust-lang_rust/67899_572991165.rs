plain
2020-01-10T10:53:34.9640843Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-10T10:53:34.9721076Z ##[command]git config gc.auto 0
2020-01-10T10:53:34.9796477Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-10T10:53:34.9867175Z ##[command]git config --get-all http.proxy
2020-01-10T10:53:35.0044695Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67899/merge:refs/remotes/pull/67899/merge
---
2020-01-10T11:00:11.7114932Z     Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
2020-01-10T11:00:15.5858140Z error: unused import: `Hasher`
2020-01-10T11:00:15.5859684Z  --> src/librustc/ty/view.rs:3:18
2020-01-10T11:00:15.5861016Z   |
2020-01-10T11:00:15.5862311Z 3 |     hash::{Hash, Hasher},
2020-01-10T11:00:15.5864743Z   |
2020-01-10T11:00:15.5865480Z   = note: `-D unused-imports` implied by `-D warnings`
2020-01-10T11:00:15.5865810Z 
2020-01-10T11:00:34.9664102Z error: aborting due to previous error
2020-01-10T11:00:34.9664102Z error: aborting due to previous error
2020-01-10T11:00:34.9664762Z 
2020-01-10T11:00:34.9992402Z error: could not compile `rustc`.
2020-01-10T11:00:34.9992967Z 
2020-01-10T11:00:34.9993494Z To learn more, run the command again with --verbose.
2020-01-10T11:00:35.0018997Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-01-10T11:00:35.0025319Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-01-10T11:00:35.0025608Z Build completed unsuccessfully in 0:04:39
2020-01-10T11:00:35.0075393Z == clock drift check ==
2020-01-10T11:00:35.0083107Z   local time: Fri Jan 10 11:00:35 UTC 2020
2020-01-10T11:00:35.0083107Z   local time: Fri Jan 10 11:00:35 UTC 2020
2020-01-10T11:00:35.2845038Z   network time: Fri, 10 Jan 2020 11:00:35 GMT
2020-01-10T11:00:35.7372568Z == end clock drift check ==
2020-01-10T11:00:35.7372797Z 
2020-01-10T11:00:35.7434141Z ##[error]Bash exited with code '1'.
2020-01-10T11:00:35.7465263Z ##[section]Starting: Checkout
2020-01-10T11:00:35.7467690Z ==============================================================================
2020-01-10T11:00:35.7467771Z Task         : Get sources
2020-01-10T11:00:35.7467824Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
