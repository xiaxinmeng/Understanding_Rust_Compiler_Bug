plain
2020-01-07T00:02:33.5434153Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-07T00:02:33.5721160Z ##[command]git config gc.auto 0
2020-01-07T00:02:33.5814167Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-07T00:02:33.5847412Z ##[command]git config --get-all http.proxy
2020-01-07T00:02:33.5991006Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67956/merge:refs/remotes/pull/67956/merge
---
2020-01-07T00:09:56.2645455Z     Checking rustc_typeck v0.0.0 (/checkout/src/librustc_typeck)
2020-01-07T00:09:56.8951430Z error[E0422]: cannot find struct, variant or union type `Item` in module `rustc::hir`
2020-01-07T00:09:56.8952502Z     --> src/librustc_typeck/check/mod.rs:2309:51
2020-01-07T00:09:56.8952741Z      |
2020-01-07T00:09:56.8953356Z 2309 |                     if let Node::Item(rustc::hir::Item { ident, .. }) = hir.get(hir_id) {
2020-01-07T00:09:56.8953833Z      |                                                   ^^^^ not found in `rustc::hir`
2020-01-07T00:09:56.8954581Z help: possible candidates are found in other modules, you can import them into scope
2020-01-07T00:09:56.8954905Z      |
2020-01-07T00:09:56.8955304Z 90   | use rustc::ty::InstanceDef::Item;
2020-01-07T00:09:56.8955637Z      |
---
2020-01-07T00:09:56.8958444Z 
2020-01-07T00:09:56.9066147Z error[E0422]: cannot find struct, variant or union type `Item` in module `rustc::hir`
2020-01-07T00:09:56.9066597Z     --> src/librustc_typeck/check/mod.rs:2321:59
2020-01-07T00:09:56.9070986Z      |
2020-01-07T00:09:56.9071707Z 2321 | ...                   if let Node::Item(rustc::hir::Item { ident, .. }) = hir.get(hir_id) {
2020-01-07T00:09:56.9072940Z      |                                                     ^^^^ not found in `rustc::hir`
2020-01-07T00:09:56.9073606Z help: possible candidates are found in other modules, you can import them into scope
2020-01-07T00:09:56.9073839Z      |
2020-01-07T00:09:56.9074121Z 90   | use rustc::ty::InstanceDef::Item;
2020-01-07T00:09:56.9074358Z      |
---
2020-01-07T00:10:04.1971879Z For more information about this error, try `rustc --explain E0422`.
2020-01-07T00:10:04.2038563Z error: could not compile `rustc_typeck`.
2020-01-07T00:10:04.2038942Z warning: build failed, waiting for other jobs to finish...
2020-01-07T00:10:12.5449006Z error: build failed
2020-01-07T00:10:12.5480431Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-01-07T00:10:12.5493323Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-01-07T00:10:12.5493732Z Build completed unsuccessfully in 0:04:35
2020-01-07T00:10:12.5546883Z == clock drift check ==
2020-01-07T00:10:12.5558188Z   local time: Tue Jan  7 00:10:12 UTC 2020
2020-01-07T00:10:12.5558188Z   local time: Tue Jan  7 00:10:12 UTC 2020
2020-01-07T00:10:12.6975127Z   network time: Tue, 07 Jan 2020 00:10:12 GMT
2020-01-07T00:10:12.6979342Z == end clock drift check ==
2020-01-07T00:10:13.3040207Z 
2020-01-07T00:10:13.3131061Z ##[error]Bash exited with code '1'.
2020-01-07T00:10:13.3164143Z ##[section]Starting: Checkout
2020-01-07T00:10:13.3166031Z ==============================================================================
2020-01-07T00:10:13.3166088Z Task         : Get sources
2020-01-07T00:10:13.3166156Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
