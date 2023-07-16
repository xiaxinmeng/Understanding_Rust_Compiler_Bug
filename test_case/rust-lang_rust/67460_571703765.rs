plain
2020-01-07T17:56:30.7773474Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-07T17:56:30.7784052Z ##[command]git config gc.auto 0
2020-01-07T17:56:30.7786536Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-07T17:56:30.7788229Z ##[command]git config --get-all http.proxy
2020-01-07T17:56:30.7790451Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67460/merge:refs/remotes/pull/67460/merge
---
2020-01-07T18:03:00.8764121Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-01-07T18:03:00.8853696Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-01-07T18:03:02.8368752Z     Checking rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-01-07T18:03:04.8773433Z     Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
2020-01-07T18:03:06.4744410Z error[E0432]: unresolved import `crate::hir::def_id`
2020-01-07T18:03:06.4745388Z  --> src/librustc/infer/error_reporting/nice_region_error/trait_impl_difference.rs:4:17
2020-01-07T18:03:06.4747701Z 4 | use crate::hir::def_id::DefId;
2020-01-07T18:03:06.4748590Z   |                 ^^^^^^ could not find `def_id` in `hir`
2020-01-07T18:03:06.4749101Z 
2020-01-07T18:03:06.6578106Z error[E0433]: failed to resolve: could not find `ItemKind` in `hir`
2020-01-07T18:03:06.6578106Z error[E0433]: failed to resolve: could not find `ItemKind` in `hir`
2020-01-07T18:03:06.6579440Z   --> src/librustc/infer/error_reporting/nice_region_error/trait_impl_difference.rs:89:25
2020-01-07T18:03:06.6580450Z 89 |             if let hir::ItemKind::Trait(_, _, generics, _, _) = &trait_item.kind {
2020-01-07T18:03:06.6580982Z    |                         ^^^^^^^^ could not find `ItemKind` in `hir`
2020-01-07T18:03:06.6581450Z 
2020-01-07T18:03:07.8346722Z error: unused import: `FxHashSet`
2020-01-07T18:03:07.8346722Z error: unused import: `FxHashSet`
2020-01-07T18:03:07.8347075Z   --> src/librustc/infer/error_reporting/nice_region_error/trait_impl_difference.rs:13:33
2020-01-07T18:03:07.8347268Z    |
2020-01-07T18:03:07.8347577Z 13 | use rustc_data_structures::fx::{FxHashSet, FxIndexSet};
2020-01-07T18:03:07.8348007Z    |
2020-01-07T18:03:07.8348236Z    = note: `-D unused-imports` implied by `-D warnings`
2020-01-07T18:03:07.8348268Z 
2020-01-07T18:03:25.2535679Z error: aborting due to 3 previous errors
2020-01-07T18:03:25.2535679Z error: aborting due to 3 previous errors
2020-01-07T18:03:25.2536690Z 
2020-01-07T18:03:25.2538986Z Some errors have detailed explanations: E0432, E0433.
2020-01-07T18:03:25.2539609Z For more information about an error, try `rustc --explain E0432`.
2020-01-07T18:03:25.2794658Z error: could not compile `rustc`.
2020-01-07T18:03:25.2795143Z 
2020-01-07T18:03:25.2795817Z To learn more, run the command again with --verbose.
2020-01-07T18:03:25.2822504Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-01-07T18:03:25.2827237Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-01-07T18:03:25.2827851Z Build completed unsuccessfully in 0:04:34
2020-01-07T18:03:25.2875736Z == clock drift check ==
2020-01-07T18:03:25.2882289Z   local time: Tue Jan  7 18:03:25 UTC 2020
2020-01-07T18:03:25.2882289Z   local time: Tue Jan  7 18:03:25 UTC 2020
2020-01-07T18:03:25.5771970Z   network time: Tue, 07 Jan 2020 18:03:25 GMT
2020-01-07T18:03:25.5775485Z == end clock drift check ==
2020-01-07T18:03:25.9821745Z 
2020-01-07T18:03:25.9882867Z ##[error]Bash exited with code '1'.
2020-01-07T18:03:25.9908992Z ##[section]Starting: Checkout
2020-01-07T18:03:25.9910746Z ==============================================================================
2020-01-07T18:03:25.9910823Z Task         : Get sources
2020-01-07T18:03:25.9910872Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
