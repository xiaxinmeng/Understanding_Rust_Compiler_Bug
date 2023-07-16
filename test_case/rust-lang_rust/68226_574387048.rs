plain
2020-01-14T21:29:58.7735884Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-14T21:29:58.7821073Z ##[command]git config gc.auto 0
2020-01-14T21:29:58.7902516Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-14T21:29:58.7959300Z ##[command]git config --get-all http.proxy
2020-01-14T21:29:58.8143184Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68226/merge:refs/remotes/pull/68226/merge
---
2020-01-14T21:36:13.8068025Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-01-14T21:36:16.2020110Z     Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-01-14T21:36:16.8883933Z     Checking rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-01-14T21:36:18.4108973Z     Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
2020-01-14T21:36:18.7799356Z error: expected one of `,`, `.`, `?`, or an operator, found `tcx`
2020-01-14T21:36:18.7802908Z     --> src/librustc/infer/opaque_types/mod.rs:1223:9
2020-01-14T21:36:18.7803479Z 1222 |         tcx.hir().find(hir_id)
2020-01-14T21:36:18.7803479Z 1222 |         tcx.hir().find(hir_id)
2020-01-14T21:36:18.7803883Z      |                               - expected one of `,`, `.`, `?`, or an operator
2020-01-14T21:36:18.7804243Z 1223 |         tcx.hir().get(opaque_hir_id),
2020-01-14T21:36:18.7804664Z 
2020-01-14T21:36:40.1433952Z error: aborting due to previous error
2020-01-14T21:36:40.1434283Z 
2020-01-14T21:36:40.1435374Z error: could not compile `rustc`.
2020-01-14T21:36:40.1435374Z error: could not compile `rustc`.
2020-01-14T21:36:40.1435600Z 
2020-01-14T21:36:40.1435813Z To learn more, run the command again with --verbose.
2020-01-14T21:36:40.1437116Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-01-14T21:36:40.1437271Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-01-14T21:36:40.1437449Z Build completed unsuccessfully in 0:04:21
2020-01-14T21:36:40.1437494Z == clock drift check ==
2020-01-14T21:36:40.1437537Z   local time: Tue Jan 14 21:36:38 UTC 2020
2020-01-14T21:36:40.1437537Z   local time: Tue Jan 14 21:36:38 UTC 2020
2020-01-14T21:36:40.1437599Z   network time: Tue, 14 Jan 2020 21:36:39 GMT
2020-01-14T21:36:40.1437641Z == end clock drift check ==
2020-01-14T21:36:40.1437671Z 
2020-01-14T21:36:40.1502187Z ##[error]Bash exited with code '1'.
2020-01-14T21:36:40.1530112Z ##[section]Starting: Checkout
2020-01-14T21:36:40.1532619Z ==============================================================================
2020-01-14T21:36:40.1532678Z Task         : Get sources
2020-01-14T21:36:40.1532746Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
