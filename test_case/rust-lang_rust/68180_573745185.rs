plain
2020-01-13T15:57:32.7350355Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-13T15:57:32.7360960Z ##[command]git config gc.auto 0
2020-01-13T15:57:32.7363750Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-13T15:57:32.7365883Z ##[command]git config --get-all http.proxy
2020-01-13T15:57:32.7368451Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68180/merge:refs/remotes/pull/68180/merge
---
2020-01-13T16:04:39.8690992Z     Checking rustc_resolve v0.0.0 (/checkout/src/librustc_resolve)
2020-01-13T16:04:42.7249528Z     Checking rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2020-01-13T16:04:43.1766749Z     Checking rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
2020-01-13T16:04:43.3331896Z     Checking rustc_codegen_ssa v0.0.0 (/checkout/src/librustc_codegen_ssa)
2020-01-13T16:04:44.1411270Z error[E0609]: no field `control_flow_guard` on type `rustc::rustc_session::config::CodegenOptions`
2020-01-13T16:04:44.1412373Z     --> src/librustc_codegen_ssa/back/link.rs:1297:21
2020-01-13T16:04:44.1412850Z      |
2020-01-13T16:04:44.1413429Z 1297 |     if sess.opts.cg.control_flow_guard.is_some() {
2020-01-13T16:04:44.1414388Z      |
2020-01-13T16:04:44.1414388Z      |
2020-01-13T16:04:44.1415118Z      = note: available fields are: `ar`, `linker`, `link_arg`, `link_args`, `link_dead_code` ... and 34 others
2020-01-13T16:04:50.3124930Z error: aborting due to previous error
2020-01-13T16:04:50.3128561Z 
2020-01-13T16:04:50.3136682Z For more information about this error, try `rustc --explain E0609`.
2020-01-13T16:04:50.3211485Z error: could not compile `rustc_codegen_ssa`.
2020-01-13T16:04:50.3211485Z error: could not compile `rustc_codegen_ssa`.
2020-01-13T16:04:50.3240390Z warning: build failed, waiting for other jobs to finish...
2020-01-13T16:05:06.2488826Z error: build failed
2020-01-13T16:05:06.2509124Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "-Zconfig-profile" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-01-13T16:05:06.2522441Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-01-13T16:05:06.2523120Z Build completed unsuccessfully in 0:05:21
2020-01-13T16:05:06.2574211Z == clock drift check ==
2020-01-13T16:05:06.2587231Z   local time: Mon Jan 13 16:05:06 UTC 2020
2020-01-13T16:05:06.2587231Z   local time: Mon Jan 13 16:05:06 UTC 2020
2020-01-13T16:05:06.5233919Z   network time: Mon, 13 Jan 2020 16:05:06 GMT
2020-01-13T16:05:06.5238393Z == end clock drift check ==
2020-01-13T16:05:06.8740093Z 
2020-01-13T16:05:06.8834248Z ##[error]Bash exited with code '1'.
2020-01-13T16:05:06.8878701Z ##[section]Starting: Checkout
2020-01-13T16:05:06.8880294Z ==============================================================================
2020-01-13T16:05:06.8880342Z Task         : Get sources
2020-01-13T16:05:06.8880397Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
