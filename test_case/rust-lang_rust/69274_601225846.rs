plain
2020-03-19T14:37:10.3095613Z ========================== Starting Command Output ===========================
2020-03-19T14:37:10.3100081Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/250d7f86-39db-4e2e-b37a-67eeebb05876.sh
2020-03-19T14:37:10.3100447Z 
2020-03-19T14:37:10.3105226Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-19T14:37:10.3127629Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69274/merge to s
2020-03-19T14:37:10.3131237Z Task         : Get sources
2020-03-19T14:37:10.3131502Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-19T14:37:10.3131756Z Version      : 1.0.0
2020-03-19T14:37:10.3131930Z Author       : Microsoft
---
2020-03-19T14:37:11.3423917Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-19T14:37:11.3430315Z ##[command]git config gc.auto 0
2020-03-19T14:37:11.3434560Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-19T14:37:11.3438436Z ##[command]git config --get-all http.proxy
2020-03-19T14:37:11.3445983Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69274/merge:refs/remotes/pull/69274/merge
---
2020-03-19T14:46:15.4590579Z     Checking rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
2020-03-19T14:46:16.4143797Z error: unused import: `Unsafety`
2020-03-19T14:46:16.4144524Z   --> src/librustc_typeck/collect.rs:42:41
2020-03-19T14:46:16.4145077Z    |
2020-03-19T14:46:16.4145753Z 42 | use rustc_hir::{GenericParamKind, Node, Unsafety};
2020-03-19T14:46:16.4147136Z    |
2020-03-19T14:46:16.4147766Z    = note: `-D unused-imports` implied by `-D warnings`
2020-03-19T14:46:16.4148088Z 
2020-03-19T14:46:17.1915765Z     Checking rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2020-03-19T14:46:17.1915765Z     Checking rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2020-03-19T14:46:20.0936462Z error[E0599]: no variant or associated item named `Method` found for enum `rustc_hir::hir::ImplItemKind<'_>` in the current scope
2020-03-19T14:46:20.0937417Z     --> src/librustc_typeck/collect.rs:2630:68
2020-03-19T14:46:20.0938047Z      |
2020-03-19T14:46:20.0938881Z 2630 |     if let Node::ImplItem(hir::ImplItem { kind: hir::ImplItemKind::Method(..), .. }) = node {
2020-03-19T14:46:20.0941121Z 
2020-03-19T14:46:20.4143174Z error: aborting due to 2 previous errors
2020-03-19T14:46:20.4143484Z 
2020-03-19T14:46:20.4144009Z For more information about this error, try `rustc --explain E0599`.
2020-03-19T14:46:20.4144009Z For more information about this error, try `rustc --explain E0599`.
2020-03-19T14:46:20.4212096Z error: could not compile `rustc_typeck`.
2020-03-19T14:46:20.4212343Z 
2020-03-19T14:46:20.4212746Z To learn more, run the command again with --verbose.
2020-03-19T14:46:20.4213342Z warning: build failed, waiting for other jobs to finish...
2020-03-19T14:46:31.3807623Z error: build failed
2020-03-19T14:46:31.3840902Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-03-19T14:46:31.3855063Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-03-19T14:46:31.3855780Z Build completed unsuccessfully in 0:05:38
2020-03-19T14:46:31.3911204Z == clock drift check ==
2020-03-19T14:46:31.3927299Z   local time: Thu Mar 19 14:46:31 UTC 2020
2020-03-19T14:46:31.3927299Z   local time: Thu Mar 19 14:46:31 UTC 2020
2020-03-19T14:46:31.6881913Z   network time: Thu, 19 Mar 2020 14:46:31 GMT
2020-03-19T14:46:31.6882251Z == end clock drift check ==
2020-03-19T14:46:32.0909645Z 
2020-03-19T14:46:32.0981761Z ##[error]Bash exited with code '1'.
2020-03-19T14:46:32.0997142Z ##[section]Finishing: Run build
2020-03-19T14:46:32.1051950Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69274/merge to s
2020-03-19T14:46:32.1061411Z Task         : Get sources
2020-03-19T14:46:32.1061766Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-19T14:46:32.1062112Z Version      : 1.0.0
2020-03-19T14:46:32.1062362Z Author       : Microsoft
2020-03-19T14:46:32.1062362Z Author       : Microsoft
2020-03-19T14:46:32.1062722Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-19T14:46:32.1063334Z ==============================================================================
2020-03-19T14:46:32.4698733Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-19T14:46:32.4747715Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69274/merge to s
2020-03-19T14:46:32.4842951Z Cleaning up task key
2020-03-19T14:46:32.4844460Z Start cleaning up orphan processes.
2020-03-19T14:46:32.5078380Z Terminate orphan process: pid (3481) (python)
2020-03-19T14:46:32.5220256Z ##[section]Finishing: Finalize Job
