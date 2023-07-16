plain
2020-04-24T19:06:05.4491387Z ========================== Starting Command Output ===========================
2020-04-24T19:06:05.4493573Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/4a647c4e-6547-482b-92cf-8b80bfd37813.sh
2020-04-24T19:06:05.4493804Z 
2020-04-24T19:06:05.4497193Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-24T19:06:05.4514101Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71526/merge to s
2020-04-24T19:06:05.4517201Z Task         : Get sources
2020-04-24T19:06:05.4517477Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-24T19:06:05.4517764Z Version      : 1.0.0
2020-04-24T19:06:05.4517947Z Author       : Microsoft
---
2020-04-24T19:06:06.4427403Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-24T19:06:06.4432829Z ##[command]git config gc.auto 0
2020-04-24T19:06:06.4436364Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-24T19:06:06.4439620Z ##[command]git config --get-all http.proxy
2020-04-24T19:06:06.4445646Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71526/merge:refs/remotes/pull/71526/merge
---
2020-04-24T19:08:23.2453345Z  ---> f7353ccad5b1
2020-04-24T19:08:23.2453892Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-24T19:08:23.2455814Z  ---> Using cache
2020-04-24T19:08:23.2456653Z  ---> ed38efbaa060
2020-04-24T19:08:23.2458493Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-24T19:08:23.2461934Z  ---> c5008ef7ae8e
2020-04-24T19:08:23.2516956Z Successfully built c5008ef7ae8e
2020-04-24T19:08:23.2543586Z Successfully tagged rust-ci:latest
2020-04-24T19:08:23.2822297Z Built container sha256:c5008ef7ae8e94d7ef502e3cef26e61208e14ebdb36913f3a8bb86291bd6430b
2020-04-24T19:08:23.2822297Z Built container sha256:c5008ef7ae8e94d7ef502e3cef26e61208e14ebdb36913f3a8bb86291bd6430b
2020-04-24T19:08:23.2838073Z Looks like docker image is the same as before, not uploading
2020-04-24T19:08:31.2544984Z [CI_JOB_NAME=mingw-check]
2020-04-24T19:08:31.2747239Z [CI_JOB_NAME=mingw-check]
2020-04-24T19:08:31.2779970Z == clock drift check ==
2020-04-24T19:08:31.2790529Z   local time: Fri Apr 24 19:08:31 UTC 2020
2020-04-24T19:08:31.5690743Z   network time: Fri, 24 Apr 2020 19:08:31 GMT
2020-04-24T19:08:31.5727723Z Starting sccache server...
2020-04-24T19:08:31.6797550Z configure: processing command line
2020-04-24T19:08:31.6797793Z configure: 
2020-04-24T19:08:31.6798669Z configure: rust.parallel-compiler := True
---
2020-04-24T19:12:03.2158690Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-24T19:12:03.4442812Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-24T19:12:03.5769474Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-24T19:12:03.6285932Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-24T19:12:04.1161517Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-24T19:12:06.1718887Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-24T19:12:06.6065176Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-24T19:12:08.4034005Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-24T19:12:08.7949684Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-24T19:12:45.0323771Z     Checking rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2020-04-24T19:12:45.8363668Z error[E0308]: `if` and `else` have incompatible types
2020-04-24T19:12:45.8365148Z    --> src/librustc_mir/transform/check_consts/validation.rs:423:17
2020-04-24T19:12:45.8366084Z     |
2020-04-24T19:12:45.8367258Z 416 |               } else if let ty::ConstKind::Unevaluated(def_id, _, promoted) = c.literal.val {
2020-04-24T19:12:45.8369415Z 417 | |                 assert!(promoted.is_none(), "Const-checking should run before promotion");
2020-04-24T19:12:45.8370626Z 418 | |
2020-04-24T19:12:45.8371792Z 419 | |                 // FIXME: This means we don't look for cycles involving associated constants, but
2020-04-24T19:12:45.8387151Z 420 | |                 // we should handle fully monomorphized ones here at least.
2020-04-24T19:12:45.8387151Z 420 | |                 // we should handle fully monomorphized ones here at least.
2020-04-24T19:12:45.8398036Z 421 | |                 self.tcx.trait_of_item(def_id).is_none().then_some(|| def_id);
2020-04-24T19:12:45.8400033Z     | |                 |                                                            |
2020-04-24T19:12:45.8401246Z     | |                 |                                                            help: consider removing this semicolon
2020-04-24T19:12:45.8402257Z     | |                 expected because of this
2020-04-24T19:12:45.8403001Z 422 | |             } else {
---
2020-04-24T19:12:46.6937433Z For more information about this error, try `rustc --explain E0308`.
2020-04-24T19:12:46.7128307Z error: could not compile `rustc_mir`.
2020-04-24T19:12:46.7128618Z 
2020-04-24T19:12:46.7129029Z To learn more, run the command again with --verbose.
2020-04-24T19:12:46.7194656Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-24T19:12:46.7204395Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-04-24T19:12:46.7204759Z Build completed unsuccessfully in 0:04:14
2020-04-24T19:12:46.7310134Z == clock drift check ==
2020-04-24T19:12:46.7310134Z == clock drift check ==
2020-04-24T19:12:46.7334364Z   local time: Fri Apr 24 19:12:46 UTC 2020
2020-04-24T19:12:47.0215083Z   network time: Fri, 24 Apr 2020 19:12:47 GMT
2020-04-24T19:12:47.7508250Z 
2020-04-24T19:12:47.7508250Z 
2020-04-24T19:12:47.7569888Z ##[error]Bash exited with code '1'.
2020-04-24T19:12:47.7583199Z ##[section]Finishing: Run build
2020-04-24T19:12:47.7631242Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71526/merge to s
2020-04-24T19:12:47.7636017Z Task         : Get sources
2020-04-24T19:12:47.7636345Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-24T19:12:47.7636631Z Version      : 1.0.0
2020-04-24T19:12:47.7636836Z Author       : Microsoft
2020-04-24T19:12:47.7636836Z Author       : Microsoft
2020-04-24T19:12:47.7637176Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-24T19:12:47.7637551Z ==============================================================================
2020-04-24T19:12:48.0785112Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-24T19:12:48.0834583Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71526/merge to s
2020-04-24T19:12:48.0930516Z Cleaning up task key
2020-04-24T19:12:48.0931686Z Start cleaning up orphan processes.
2020-04-24T19:12:48.1109802Z Terminate orphan process: pid (3671) (python)
2020-04-24T19:12:48.1235713Z ##[section]Finishing: Finalize Job
