plain
2019-07-24T21:33:15.3095670Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-24T21:33:15.3317582Z ##[command]git config gc.auto 0
2019-07-24T21:33:15.3396130Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-24T21:33:15.3452942Z ##[command]git config --get-all http.proxy
2019-07-24T21:33:15.3586172Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62955/merge:refs/remotes/pull/62955/merge
---
2019-07-24T21:33:50.0958970Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-24T21:33:50.0959188Z 
2019-07-24T21:33:50.0959568Z   git checkout -b <new-branch-name>
2019-07-24T21:33:50.0959765Z 
2019-07-24T21:33:50.0959968Z HEAD is now at 66a2ddf15 Merge ef4c04eff79e32273a5b0f3deb61000fe9120eb9 into 03f19f7ff128a3b01eeab3f87f04cce22883f006
2019-07-24T21:33:50.1085624Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-24T21:33:50.1088084Z ==============================================================================
2019-07-24T21:33:50.1088136Z Task         : Bash
2019-07-24T21:33:50.1088193Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-24T21:43:08.3302143Z    Compiling parking_lot_core v0.4.0
2019-07-24T21:43:11.6511407Z     Checking tempfile v3.0.5
2019-07-24T21:43:11.9920674Z     Checking parking_lot v0.7.1
2019-07-24T21:43:12.2808951Z     Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
2019-07-24T21:43:12.5959894Z error[E0432]: unresolved import `crate::core::DocAccessLevels`
2019-07-24T21:43:12.5962541Z   --> src/librustdoc/clean/inline.rs:17:31
2019-07-24T21:43:12.5962817Z    |
2019-07-24T21:43:12.5963055Z 17 | use crate::core::{DocContext, DocAccessLevels};
2019-07-24T21:43:12.5963547Z    |                               |
2019-07-24T21:43:12.5963547Z    |                               |
2019-07-24T21:43:12.5963792Z    |                               no `DocAccessLevels` in `core`
2019-07-24T21:43:12.5964080Z    |                               help: a similar name exists in the module: `AccessLevels`
2019-07-24T21:43:12.5964230Z 
2019-07-24T21:43:12.5964476Z error[E0432]: unresolved import `crate::core::DocAccessLevels`
2019-07-24T21:43:12.5964722Z  --> src/librustdoc/clean/blanket_impl.rs:8:5
2019-07-24T21:43:12.5964905Z   |
2019-07-24T21:43:12.5965125Z 8 | use crate::core::DocAccessLevels;
2019-07-24T21:43:12.5965383Z   |     ^^^^^^^^^^^^^---------------
2019-07-24T21:43:12.5965617Z   |     |            |
2019-07-24T21:43:12.5965902Z   |     |            help: a similar name exists in the module: `AccessLevels`
2019-07-24T21:43:12.5966141Z   |     no `DocAccessLevels` in `core`
2019-07-24T21:43:12.5986913Z 
2019-07-24T21:43:12.5987917Z error[E0432]: unresolved import `crate::core::DocAccessLevels`
2019-07-24T21:43:12.5988210Z   --> src/librustdoc/html/format.rs:17:5
2019-07-24T21:43:12.5988448Z    |
2019-07-24T21:43:12.5988733Z 17 | use crate::core::DocAccessLevels;
2019-07-24T21:43:12.5989038Z    |     ^^^^^^^^^^^^^---------------
2019-07-24T21:43:12.5989347Z    |     |            |
2019-07-24T21:43:12.5989706Z    |     |            help: a similar name exists in the module: `AccessLevels`
2019-07-24T21:43:12.5990015Z    |     no `DocAccessLevels` in `core`
2019-07-24T21:43:12.5990069Z 
2019-07-24T21:43:12.5990351Z error[E0432]: unresolved import `crate::core::DocAccessLevels`
2019-07-24T21:43:12.5990933Z   --> src/librustdoc/passes/mod.rs:13:31
2019-07-24T21:43:12.5991307Z    |
2019-07-24T21:43:12.5991562Z 13 | use crate::core::{DocContext, DocAccessLevels};
2019-07-24T21:43:12.5992073Z    |                               |
2019-07-24T21:43:12.5992073Z    |                               |
2019-07-24T21:43:12.5992342Z    |                               no `DocAccessLevels` in `core`
2019-07-24T21:43:12.5992638Z    |                               help: a similar name exists in the module: `AccessLevels`
2019-07-24T21:43:15.2433417Z error: aborting due to 4 previous errors
2019-07-24T21:43:15.2434147Z 
2019-07-24T21:43:15.2434648Z For more information about this error, try `rustc --explain E0432`.
2019-07-24T21:43:15.2794858Z error: Could not compile `rustdoc`.
2019-07-24T21:43:15.2794858Z error: Could not compile `rustdoc`.
2019-07-24T21:43:15.2795554Z 
2019-07-24T21:43:15.2796258Z To learn more, run the command again with --verbose.
2019-07-24T21:43:15.2818399Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--color" "always" "--manifest-path" "/checkout/src/tools/rustdoc/Cargo.toml" "--message-format" "json"
2019-07-24T21:43:15.2818868Z expected success, got: exit code: 101
2019-07-24T21:43:15.2823174Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-07-24T21:43:15.2823685Z Build completed unsuccessfully in 0:06:13
2019-07-24T21:43:17.5667697Z ##[error]Bash exited with code '1'.
2019-07-24T21:43:17.5700779Z ##[section]Starting: Checkout
2019-07-24T21:43:17.5702159Z ==============================================================================
2019-07-24T21:43:17.5702204Z Task         : Get sources
2019-07-24T21:43:17.5702243Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
