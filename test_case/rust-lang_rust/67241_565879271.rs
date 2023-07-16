plain
2019-12-16T02:16:49.9643403Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-16T02:16:50.9809493Z ##[command]git config gc.auto 0
2019-12-16T02:16:50.9815290Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-16T02:16:50.9818660Z ##[command]git config --get-all http.proxy
2019-12-16T02:16:50.9820840Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67241/merge:refs/remotes/pull/67241/merge
---
2019-12-16T02:24:41.2310846Z     Checking rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2019-12-16T02:24:44.8391518Z error[E0624]: method `universal_upper_bound` is private
2019-12-16T02:24:44.8393370Z    --> src/librustc_mir/borrow_check/diagnostics/region_errors.rs:167:36
2019-12-16T02:24:44.8393928Z     |
2019-12-16T02:24:44.8394496Z 167 |             let upper_bound = self.universal_upper_bound(r);
2019-12-16T02:24:44.8395234Z 
2019-12-16T02:24:46.1294300Z error[E0308]: mismatched types
2019-12-16T02:24:46.1301464Z     --> src/librustc_mir/borrow_check/region_infer/mod.rs:1467:16
2019-12-16T02:24:46.1302544Z      |
2019-12-16T02:24:46.1302544Z      |
2019-12-16T02:24:46.1302776Z 1467 |             if propagated {
2019-12-16T02:24:46.1303100Z      |                ^^^^^^^^^^ expected bool, found enum `std::option::Option`
2019-12-16T02:24:46.1303315Z      |
2019-12-16T02:24:46.1303731Z      = note: expected type `bool`
2019-12-16T02:24:46.1303966Z                 found type `std::option::Option<syntax::rustc_errors::ErrorReported>`
2019-12-16T02:24:46.1304358Z error[E0308]: mismatched types
2019-12-16T02:24:46.1304593Z     --> src/librustc_mir/borrow_check/region_infer/mod.rs:1468:24
2019-12-16T02:24:46.1304767Z      |
2019-12-16T02:24:46.1304975Z 1468 |                 return None;
---
2019-12-16T02:24:46.1306808Z      |
2019-12-16T02:24:46.1307048Z 1485 |                 longer_fr,
2019-12-16T02:24:46.1307295Z      |                 ^^^^^^^^^
2019-12-16T02:24:46.1307526Z      |                 |
2019-12-16T02:24:46.1307795Z      |                 expected struct `rustc::ty::RegionVid`, found reference
2019-12-16T02:24:46.1308083Z      |                 help: consider dereferencing the borrow: `*longer_fr`
2019-12-16T02:24:46.1309102Z      = note: expected type `rustc::ty::RegionVid`
2019-12-16T02:24:46.1309381Z                 found type `&rustc::ty::RegionVid`
2019-12-16T02:24:46.1309419Z 
2019-12-16T02:24:46.1309668Z error[E0308]: mismatched types
2019-12-16T02:24:46.1309668Z error[E0308]: mismatched types
2019-12-16T02:24:46.1309969Z     --> src/librustc_mir/borrow_check/region_infer/mod.rs:1487:17
2019-12-16T02:24:46.1310188Z      |
2019-12-16T02:24:46.1310598Z 1487 |                 shorter_fr,
2019-12-16T02:24:46.1310941Z      |                 ^^^^^^^^^^
2019-12-16T02:24:46.1311211Z      |                 |
2019-12-16T02:24:46.1311529Z      |                 expected struct `rustc::ty::RegionVid`, found reference
2019-12-16T02:24:46.1312086Z      |                 help: consider dereferencing the borrow: `*shorter_fr`
2019-12-16T02:24:46.1312755Z      = note: expected type `rustc::ty::RegionVid`
2019-12-16T02:24:46.1313146Z                 found type `&rustc::ty::RegionVid`
2019-12-16T02:24:46.1313175Z 
2019-12-16T02:24:46.1313371Z error[E0308]: mismatched types
2019-12-16T02:24:46.1313371Z error[E0308]: mismatched types
2019-12-16T02:24:46.1313616Z     --> src/librustc_mir/borrow_check/region_infer/mod.rs:1488:17
2019-12-16T02:24:46.1313789Z      |
2019-12-16T02:24:46.1314010Z 1488 |                 outlives_suggestion,
2019-12-16T02:24:46.1314248Z      |                 ^^^^^^^^^^^^^^^^^^^
2019-12-16T02:24:46.1314461Z      |                 |
2019-12-16T02:24:46.1314928Z      |                 expected mutable reference, found struct `borrow_check::diagnostics::outlives_suggestion::OutlivesSuggestionBuilder`
2019-12-16T02:24:46.1315389Z      |
2019-12-16T02:24:46.1315389Z      |
2019-12-16T02:24:46.1315661Z      = note: expected type `&mut borrow_check::diagnostics::outlives_suggestion::OutlivesSuggestionBuilder<'_>`
2019-12-16T02:24:46.1315928Z                 found type `borrow_check::diagnostics::outlives_suggestion::OutlivesSuggestionBuilder<'_>`
2019-12-16T02:24:46.1316139Z error[E0308]: mismatched types
2019-12-16T02:24:46.1316375Z     --> src/librustc_mir/borrow_check/region_infer/mod.rs:1492:23
2019-12-16T02:24:46.1316548Z      |
2019-12-16T02:24:46.1316548Z      |
2019-12-16T02:24:46.1316761Z 1492 |             db.buffer(errors_buffer);
2019-12-16T02:24:46.1317106Z      |                       ^^^^^^^^^^^^^ expected struct `std::vec::Vec`, found struct `borrow_check::diagnostics::region_errors::RegionErrors`
2019-12-16T02:24:46.1317286Z      |
2019-12-16T02:24:46.1317554Z      = note: expected type `&mut std::vec::Vec<syntax::rustc_errors::Diagnostic>`
2019-12-16T02:24:46.1318216Z                 found type `&mut borrow_check::diagnostics::region_errors::RegionErrors<'tcx>`
2019-12-16T02:24:46.2438391Z error[E0308]: mismatched types
2019-12-16T02:24:46.2438877Z     --> src/librustc_mir/borrow_check/region_infer/mod.rs:1634:12
2019-12-16T02:24:46.2439201Z      |
2019-12-16T02:24:46.2439509Z 1634 |         if propagated {
2019-12-16T02:24:46.2439509Z 1634 |         if propagated {
2019-12-16T02:24:46.2440102Z      |            ^^^^^^^^^^ expected bool, found enum `std::option::Option`
2019-12-16T02:24:46.2440416Z      |
2019-12-16T02:24:46.2440751Z      = note: expected type `bool`
2019-12-16T02:24:46.2441072Z                 found type `std::option::Option<syntax::rustc_errors::ErrorReported>`
2019-12-16T02:24:46.2569005Z     Checking rustc_lint v0.0.0 (/checkout/src/librustc_lint)
2019-12-16T02:24:46.3416429Z error[E0308]: mismatched types
2019-12-16T02:24:46.3417103Z     --> src/librustc_mir/borrow_check/region_infer/mod.rs:1660:19
2019-12-16T02:24:46.3417528Z      |
2019-12-16T02:24:46.3417528Z      |
2019-12-16T02:24:46.3418410Z 1660 |         db.buffer(errors_buffer);
2019-12-16T02:24:46.3418917Z      |                   ^^^^^^^^^^^^^ expected struct `std::vec::Vec`, found struct `borrow_check::diagnostics::region_errors::RegionErrors`
2019-12-16T02:24:46.3419159Z      |
2019-12-16T02:24:46.3419499Z      = note: expected type `&mut std::vec::Vec<syntax::rustc_errors::Diagnostic>`
2019-12-16T02:24:46.3419830Z                 found type `&mut borrow_check::diagnostics::region_errors::RegionErrors<'tcx>`
2019-12-16T02:24:46.3492610Z error[E0308]: mismatched types
2019-12-16T02:24:46.3492895Z     --> src/librustc_mir/borrow_check/region_infer/mod.rs:1712:24
2019-12-16T02:24:46.3493081Z      |
2019-12-16T02:24:46.3493081Z      |
2019-12-16T02:24:46.3493299Z 1680 |     ) -> Option<ErrorReported> {
2019-12-16T02:24:46.3493633Z      |          --------------------- expected `std::option::Option<syntax::rustc_errors::ErrorReported>` because of return type
2019-12-16T02:24:46.3494023Z 1712 |                 return true;
2019-12-16T02:24:46.3494300Z      |                        ^^^^ expected enum `std::option::Option`, found bool
2019-12-16T02:24:46.3494481Z      |
2019-12-16T02:24:46.3494481Z      |
2019-12-16T02:24:46.3494751Z      = note: expected type `std::option::Option<syntax::rustc_errors::ErrorReported>`
2019-12-16T02:24:46.3499782Z 
2019-12-16T02:24:46.4464500Z error[E0308]: mismatched types
2019-12-16T02:24:46.4464806Z     --> src/librustc_mir/borrow_check/region_infer/mod.rs:1716:9
2019-12-16T02:24:46.4464981Z      |
2019-12-16T02:24:46.4464981Z      |
2019-12-16T02:24:46.4465221Z 1680 |     ) -> Option<ErrorReported> {
2019-12-16T02:24:46.4465547Z      |          --------------------- expected `std::option::Option<syntax::rustc_errors::ErrorReported>` because of return type
2019-12-16T02:24:46.4465970Z 1716 |         false
2019-12-16T02:24:46.4466386Z      |         ^^^^^ expected enum `std::option::Option`, found bool
2019-12-16T02:24:46.4466780Z      |
2019-12-16T02:24:46.4466780Z      |
2019-12-16T02:24:46.4467043Z      = note: expected type `std::option::Option<syntax::rustc_errors::ErrorReported>`
2019-12-16T02:24:46.4473313Z 
2019-12-16T02:24:47.7752752Z     Checking rustc_passes v0.0.0 (/checkout/src/librustc_passes)
2019-12-16T02:24:48.7903593Z     Checking rustc_codegen_utils v0.0.0 (/checkout/src/librustc_codegen_utils)
2019-12-16T02:24:49.3149961Z     Checking rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
---
2019-12-16T02:24:51.6923627Z For more information about an error, try `rustc --explain E0308`.
2019-12-16T02:24:51.7589509Z error: could not compile `rustc_mir`.
2019-12-16T02:24:51.7589950Z warning: build failed, waiting for other jobs to finish...
2019-12-16T02:24:52.0606744Z error: build failed
2019-12-16T02:24:52.0643684Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2019-12-16T02:24:52.0663247Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2019-12-16T02:24:52.0664286Z Build completed unsuccessfully in 0:04:47
2019-12-16T02:24:52.0718832Z == clock drift check ==
2019-12-16T02:24:52.0733472Z   local time: Mon Dec 16 02:24:52 UTC 2019
2019-12-16T02:24:52.0733472Z   local time: Mon Dec 16 02:24:52 UTC 2019
2019-12-16T02:24:52.3574353Z   network time: Mon, 16 Dec 2019 02:24:52 GMT
2019-12-16T02:24:52.3575872Z == end clock drift check ==
2019-12-16T02:24:53.5785271Z 
2019-12-16T02:24:53.5877705Z ##[error]Bash exited with code '1'.
2019-12-16T02:24:53.5903766Z ##[section]Starting: Checkout
2019-12-16T02:24:53.5905726Z ==============================================================================
2019-12-16T02:24:53.5905806Z Task         : Get sources
2019-12-16T02:24:53.5905846Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
