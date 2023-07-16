plain
2020-03-15T23:57:26.2349212Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-15T23:57:28.5509707Z     Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-03-15T23:57:29.2342262Z     Checking rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-03-15T23:57:30.6355790Z     Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
2020-03-15T23:57:33.8973180Z error[E0599]: no function or associated item named `from_u32_const` found for struct `rustc_hir::hir_id::ItemLocalId` in the current scope
2020-03-15T23:57:33.8975017Z    --> src/librustc/hir/map/mod.rs:348:40
2020-03-15T23:57:33.8976044Z     |
2020-03-15T23:57:33.8977419Z 348 |         if id.local_id == ItemLocalId::from_u32_const(0) {
2020-03-15T23:57:33.8979807Z     |                                        ^^^^^^^^^^^^^^ function or associated item not found in `rustc_hir::hir_id::ItemLocalId`
2020-03-15T23:57:40.7513655Z error: aborting due to previous error
2020-03-15T23:57:40.7514830Z 
2020-03-15T23:57:40.7515825Z For more information about this error, try `rustc --explain E0599`.
2020-03-15T23:57:40.7757183Z error: could not compile `rustc`.
2020-03-15T23:57:40.7757183Z error: could not compile `rustc`.
2020-03-15T23:57:40.7758036Z 
2020-03-15T23:57:40.7759194Z To learn more, run the command again with --verbose.
2020-03-15T23:57:40.7788472Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-03-15T23:57:40.7793306Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-03-15T23:57:40.7794002Z Build completed unsuccessfully in 0:03:57
2020-03-15T23:57:40.7851731Z == clock drift check ==
2020-03-15T23:57:40.7867815Z   local time: Sun Mar 15 23:57:40 UTC 2020
2020-03-15T23:57:40.7867815Z   local time: Sun Mar 15 23:57:40 UTC 2020
2020-03-15T23:57:41.0576523Z   network time: Sun, 15 Mar 2020 23:57:41 GMT
2020-03-15T23:57:41.0584591Z == end clock drift check ==
2020-03-15T23:57:41.9910900Z 
2020-03-15T23:57:41.9978794Z ##[error]Bash exited with code '1'.
2020-03-15T23:57:42.0047252Z ##[section]Starting: Checkout rust-lang/rust@auto to s
2020-03-15T23:57:42.0051901Z ==============================================================================
2020-03-15T23:57:42.0052561Z Task         : Get sources
2020-03-15T23:57:42.0052959Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
