plain
2020-04-26T16:01:13.1041829Z ========================== Starting Command Output ===========================
2020-04-26T16:01:13.1044154Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/a8440ee4-ef53-4703-9b72-e1e59a065c1b.sh
2020-04-26T16:01:13.1044408Z 
2020-04-26T16:01:13.1048148Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-26T16:01:13.1066531Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71587/merge to s
2020-04-26T16:01:13.1070262Z Task         : Get sources
2020-04-26T16:01:13.1070536Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-26T16:01:13.1070822Z Version      : 1.0.0
2020-04-26T16:01:13.1071002Z Author       : Microsoft
---
2020-04-26T16:01:14.3482923Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-26T16:01:14.3491116Z ##[command]git config gc.auto 0
2020-04-26T16:01:14.3498024Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-26T16:01:14.3503237Z ##[command]git config --get-all http.proxy
2020-04-26T16:01:14.3513754Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71587/merge:refs/remotes/pull/71587/merge
---
2020-04-26T16:03:38.9111321Z  ---> f7353ccad5b1
2020-04-26T16:03:38.9112191Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-26T16:03:38.9116634Z  ---> Using cache
2020-04-26T16:03:38.9117378Z  ---> ed38efbaa060
2020-04-26T16:03:38.9118856Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-26T16:03:38.9121531Z  ---> c5008ef7ae8e
2020-04-26T16:03:38.9153537Z Successfully built c5008ef7ae8e
2020-04-26T16:03:38.9191050Z Successfully tagged rust-ci:latest
2020-04-26T16:03:38.9445257Z Built container sha256:c5008ef7ae8e94d7ef502e3cef26e61208e14ebdb36913f3a8bb86291bd6430b
2020-04-26T16:03:38.9445257Z Built container sha256:c5008ef7ae8e94d7ef502e3cef26e61208e14ebdb36913f3a8bb86291bd6430b
2020-04-26T16:03:38.9491204Z Looks like docker image is the same as before, not uploading
2020-04-26T16:03:46.1357229Z [CI_JOB_NAME=mingw-check]
2020-04-26T16:03:46.1673022Z [CI_JOB_NAME=mingw-check]
2020-04-26T16:03:46.1705981Z == clock drift check ==
2020-04-26T16:03:46.1715714Z   local time: Sun Apr 26 16:03:46 UTC 2020
2020-04-26T16:03:46.8918265Z   network time: Sun, 26 Apr 2020 16:03:46 GMT
2020-04-26T16:03:46.8919604Z Starting sccache server...
2020-04-26T16:03:46.8919934Z configure: processing command line
2020-04-26T16:03:46.8920269Z configure: 
2020-04-26T16:03:46.8921303Z configure: rust.parallel-compiler := True
---
2020-04-26T16:05:56.0753331Z     Checking term v0.0.0 (/checkout/src/libterm)
2020-04-26T16:05:56.3543201Z     Checking unicode-width v0.1.6
2020-04-26T16:05:56.4350011Z     Checking getopts v0.2.21
2020-04-26T16:05:58.4717537Z     Checking test v0.0.0 (/checkout/src/libtest)
2020-04-26T16:05:59.1646565Z {"reason":"build-finished","success":true}
2020-04-26T16:05:59.1801840Z Checking compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2020-04-26T16:05:59.7885983Z     Checking cfg-if v0.1.10
2020-04-26T16:05:59.7886527Z    Compiling libc v0.2.69
2020-04-26T16:05:59.8233978Z    Compiling semver-parser v0.7.0
---
2020-04-26T16:07:35.8129018Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-26T16:07:35.8979646Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-26T16:07:36.0808061Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-26T16:07:36.2618121Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-26T16:07:36.6757962Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-26T16:07:39.0110642Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-26T16:07:39.4877050Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-26T16:07:41.4568752Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-26T16:07:41.8910660Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-26T16:08:21.5925567Z     Checking rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2020-04-26T16:08:24.1649516Z error: aborting due to 2 previous errors
2020-04-26T16:08:24.1649785Z 
2020-04-26T16:08:24.1650267Z For more information about this error, try `rustc --explain E0308`.
2020-04-26T16:08:24.1732862Z {"reason":"build-finished","success":false}
2020-04-26T16:08:24.1847359Z error: could not compile `rustc_mir`.
2020-04-26T16:08:24.1848024Z To learn more, run the command again with --verbose.
2020-04-26T16:08:24.1848024Z To learn more, run the command again with --verbose.
2020-04-26T16:08:24.1872653Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-26T16:08:24.1885629Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-04-26T16:08:24.1885971Z Build completed unsuccessfully in 0:04:37
2020-04-26T16:08:24.2003287Z == clock drift check ==
2020-04-26T16:08:24.2016875Z   local time: Sun Apr 26 16:08:24 UTC 2020
2020-04-26T16:08:24.2016875Z   local time: Sun Apr 26 16:08:24 UTC 2020
2020-04-26T16:08:24.3382508Z   network time: Sun, 26 Apr 2020 16:08:24 GMT
2020-04-26T16:08:25.0660310Z 
2020-04-26T16:08:25.0660310Z 
2020-04-26T16:08:25.0725419Z ##[error]Bash exited with code '1'.
2020-04-26T16:08:25.0739307Z ##[section]Finishing: Run build
2020-04-26T16:08:25.0803869Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71587/merge to s
2020-04-26T16:08:25.0808781Z Task         : Get sources
2020-04-26T16:08:25.0809103Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-26T16:08:25.0809383Z Version      : 1.0.0
2020-04-26T16:08:25.0809586Z Author       : Microsoft
2020-04-26T16:08:25.0809586Z Author       : Microsoft
2020-04-26T16:08:25.0809920Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-26T16:08:25.0810276Z ==============================================================================
2020-04-26T16:08:25.4401566Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-26T16:08:25.4447461Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71587/merge to s
2020-04-26T16:08:25.4539963Z Cleaning up task key
2020-04-26T16:08:25.4541282Z Start cleaning up orphan processes.
2020-04-26T16:08:25.4722436Z Terminate orphan process: pid (4256) (python)
2020-04-26T16:08:25.4880700Z ##[section]Finishing: Finalize Job
