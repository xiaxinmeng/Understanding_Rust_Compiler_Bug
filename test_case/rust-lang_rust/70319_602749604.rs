plain
2020-03-23T17:28:18.3530372Z ========================== Starting Command Output ===========================
2020-03-23T17:28:18.3534721Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/db39795a-0bf8-4cb4-a8b5-315aabe34469.sh
2020-03-23T17:28:18.3535195Z 
2020-03-23T17:28:18.3540612Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-23T17:28:18.3563589Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70319/merge to s
2020-03-23T17:28:18.3566844Z Task         : Get sources
2020-03-23T17:28:18.3567152Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-23T17:28:18.3567451Z Version      : 1.0.0
2020-03-23T17:28:18.3567669Z Author       : Microsoft
---
2020-03-23T17:28:19.5761496Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-23T17:28:19.5770357Z ##[command]git config gc.auto 0
2020-03-23T17:28:19.5776926Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-23T17:28:19.5783400Z ##[command]git config --get-all http.proxy
2020-03-23T17:28:19.5792933Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70319/merge:refs/remotes/pull/70319/merge
---
2020-03-23T17:34:37.6160199Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-03-23T17:34:38.3614684Z error[E0425]: cannot find value `bound_ty` in this scope
2020-03-23T17:34:38.3615487Z    --> src/librustc/traits/structural_impls.rs:280:21
2020-03-23T17:34:38.3616014Z     |
2020-03-23T17:34:38.3616621Z 280 |                     bound_ty.var.as_u32(),
2020-03-23T17:34:38.3622208Z 
2020-03-23T17:34:38.3667323Z error[E0425]: cannot find value `bound_ty` in this scope
2020-03-23T17:34:38.3668300Z    --> src/librustc/traits/structural_impls.rs:281:52
2020-03-23T17:34:38.3669215Z     |
2020-03-23T17:34:38.3669215Z     |
2020-03-23T17:34:38.3670043Z 281 |                     Symbol::intern(&format!("^{}", bound_ty.var.as_u32())),
2020-03-23T17:34:38.3671626Z 
2020-03-23T17:34:38.7718115Z error: unused import: `GenericArg`
2020-03-23T17:34:38.7718846Z   --> src/librustc/ty/normalize_erasing_regions.rs:11:24
2020-03-23T17:34:38.7719425Z    |
2020-03-23T17:34:38.7719425Z    |
2020-03-23T17:34:38.7720100Z 11 | use crate::ty::subst::{GenericArg, Subst, SubstsRef};
2020-03-23T17:34:38.7721651Z    |
2020-03-23T17:34:38.7722349Z    = note: `-D unused-imports` implied by `-D warnings`
2020-03-23T17:34:38.7722634Z 
2020-03-23T17:34:38.8052194Z     Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
2020-03-23T17:34:38.8052194Z     Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
2020-03-23T17:34:43.8818921Z error[E0308]: mismatched types
2020-03-23T17:34:43.8821260Z    --> src/librustc/traits/structural_impls.rs:279:17
2020-03-23T17:34:43.8822669Z     |
2020-03-23T17:34:43.8824165Z 277 |  /         match c.val {
2020-03-23T17:34:43.8826323Z 278 |  |             ty::ConstKind::Bound(debruijn, bound_var) if debruijn == self.binder_index => {
2020-03-23T17:34:43.8831544Z 279 |  |                 self.types.insert(
2020-03-23T17:34:43.8832347Z     |  |_________________^
2020-03-23T17:34:43.8833241Z 280 | ||                     bound_ty.var.as_u32(),
2020-03-23T17:34:43.8834359Z 281 | ||                     Symbol::intern(&format!("^{}", bound_ty.var.as_u32())),
2020-03-23T17:34:43.8835400Z 282 | ||                 )
2020-03-23T17:34:43.8836498Z     | ||_________________^ expected `()`, found enum `std::option::Option`
2020-03-23T17:34:43.8838326Z 284 |  |             _ => (),
2020-03-23T17:34:43.8839111Z 285 |  |         }
2020-03-23T17:34:43.8839922Z     |  |_________- expected this to be `()`
2020-03-23T17:34:43.8840575Z     |
---
2020-03-23T17:34:48.4042302Z For more information about an error, try `rustc --explain E0308`.
2020-03-23T17:34:48.4326095Z error: could not compile `rustc`.
2020-03-23T17:34:48.4327048Z 
2020-03-23T17:34:48.4328153Z To learn more, run the command again with --verbose.
2020-03-23T17:34:48.4370485Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-03-23T17:34:48.4384316Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-03-23T17:34:48.4384665Z Build completed unsuccessfully in 0:04:01
2020-03-23T17:34:48.4439747Z == clock drift check ==
2020-03-23T17:34:48.4456890Z   local time: Mon Mar 23 17:34:48 UTC 2020
2020-03-23T17:34:48.4456890Z   local time: Mon Mar 23 17:34:48 UTC 2020
2020-03-23T17:34:48.6074758Z   network time: Mon, 23 Mar 2020 17:34:48 GMT
2020-03-23T17:34:48.6081872Z == end clock drift check ==
2020-03-23T17:34:49.2517558Z 
2020-03-23T17:34:49.2585618Z ##[error]Bash exited with code '1'.
2020-03-23T17:34:49.2599129Z ##[section]Finishing: Run build
2020-03-23T17:34:49.2653079Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70319/merge to s
2020-03-23T17:34:49.2658096Z Task         : Get sources
2020-03-23T17:34:49.2658456Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-23T17:34:49.2658769Z Version      : 1.0.0
2020-03-23T17:34:49.2658993Z Author       : Microsoft
2020-03-23T17:34:49.2658993Z Author       : Microsoft
2020-03-23T17:34:49.2659360Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-23T17:34:49.2659764Z ==============================================================================
2020-03-23T17:34:49.6001971Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-23T17:34:49.6049698Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70319/merge to s
2020-03-23T17:34:49.6141821Z Cleaning up task key
2020-03-23T17:34:49.6143140Z Start cleaning up orphan processes.
2020-03-23T17:34:49.6325940Z Terminate orphan process: pid (3447) (python)
2020-03-23T17:34:49.6515453Z ##[section]Finishing: Finalize Job
