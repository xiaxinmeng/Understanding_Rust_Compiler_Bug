plain
2020-04-22T14:29:02.2361941Z ========================== Starting Command Output ===========================
2020-04-22T14:29:02.2365926Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/6f4f27aa-e225-4112-825a-6219142c2a97.sh
2020-04-22T14:29:02.2366192Z 
2020-04-22T14:29:02.2374623Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-22T14:29:02.2392157Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70820/merge to s
2020-04-22T14:29:02.2395311Z Task         : Get sources
2020-04-22T14:29:02.2395584Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-22T14:29:02.2395851Z Version      : 1.0.0
2020-04-22T14:29:02.2396037Z Author       : Microsoft
---
2020-04-22T14:29:03.2487105Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-22T14:29:03.2491972Z ##[command]git config gc.auto 0
2020-04-22T14:29:03.2495068Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-22T14:29:03.2498071Z ##[command]git config --get-all http.proxy
2020-04-22T14:29:03.2502920Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70820/merge:refs/remotes/pull/70820/merge
---
2020-04-22T14:31:13.9099876Z  ---> 78ad2f4d4aca
2020-04-22T14:31:13.9100103Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-22T14:31:13.9156841Z  ---> Using cache
2020-04-22T14:31:13.9157205Z  ---> 4d2dc61c4d00
2020-04-22T14:31:13.9158477Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-22T14:31:13.9159740Z  ---> 776b6266a8b7
2020-04-22T14:31:13.9159939Z Successfully built 776b6266a8b7
2020-04-22T14:31:13.9192073Z Successfully tagged rust-ci:latest
2020-04-22T14:31:13.9474356Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-22T14:31:13.9474356Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-22T14:31:13.9489842Z Looks like docker image is the same as before, not uploading
2020-04-22T14:31:22.2699924Z [CI_JOB_NAME=mingw-check]
2020-04-22T14:31:22.2886580Z [CI_JOB_NAME=mingw-check]
2020-04-22T14:31:22.2910855Z == clock drift check ==
2020-04-22T14:31:22.2919943Z   local time: Wed Apr 22 14:31:22 UTC 2020
2020-04-22T14:31:22.4510419Z   network time: Wed, 22 Apr 2020 14:31:22 GMT
2020-04-22T14:31:22.4535989Z Starting sccache server...
2020-04-22T14:31:22.5454763Z configure: processing command line
2020-04-22T14:31:22.5455197Z configure: 
2020-04-22T14:31:22.5456132Z configure: rust.parallel-compiler := True
---
2020-04-22T14:34:35.6868399Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-22T14:34:35.7402864Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-22T14:34:35.8916157Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-22T14:34:36.0196853Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-22T14:34:36.3947844Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-22T14:34:38.2527193Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-22T14:34:38.6169771Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-22T14:34:40.2356295Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-22T14:34:40.6037036Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-22T14:35:09.7853539Z     Checking rustc_passes v0.0.0 (/checkout/src/librustc_passes)
2020-04-22T14:35:10.0646524Z error[E0609]: no field `uneval_consts` on type `rustc_middle::mir::BodyAndCache<'_>`
2020-04-22T14:35:10.0647833Z    --> src/librustc_mir/transform/mod.rs:255:10
2020-04-22T14:35:10.0648420Z     |
2020-04-22T14:35:10.0649075Z 255 |     body.uneval_consts = uneval_consts;
2020-04-22T14:35:10.0650268Z 
2020-04-22T14:35:10.8388619Z error[E0609]: no field `uneval_consts` on type `&mut rustc_middle::mir::BodyAndCache<'tcx>`
2020-04-22T14:35:10.8390020Z    --> src/librustc_mir/transform/inline.rs:127:29
2020-04-22T14:35:10.8390697Z     |
2020-04-22T14:35:10.8390697Z     |
2020-04-22T14:35:10.8391552Z 127 |                 caller_body.uneval_consts.extend(callee_body.uneval_consts.iter().copied().filter(
2020-04-22T14:35:10.8393078Z 
2020-04-22T14:35:10.8393897Z error[E0609]: no field `uneval_consts` on type `rustc_middle::mir::BodyAndCache<'_>`
2020-04-22T14:35:10.8394796Z    --> src/librustc_mir/transform/inline.rs:127:62
2020-04-22T14:35:10.8395428Z     |
2020-04-22T14:35:10.8395428Z     |
2020-04-22T14:35:10.8396318Z 127 |                 caller_body.uneval_consts.extend(callee_body.uneval_consts.iter().copied().filter(
2020-04-22T14:35:10.8398033Z 
2020-04-22T14:35:10.9623044Z error: aborting due to 3 previous errors
2020-04-22T14:35:10.9623280Z 
2020-04-22T14:35:10.9623684Z For more information about this error, try `rustc --explain E0609`.
2020-04-22T14:35:10.9623684Z For more information about this error, try `rustc --explain E0609`.
2020-04-22T14:35:10.9708885Z error: could not compile `rustc_mir`.
2020-04-22T14:35:10.9709122Z 
2020-04-22T14:35:10.9709485Z To learn more, run the command again with --verbose.
2020-04-22T14:35:10.9710017Z warning: build failed, waiting for other jobs to finish...
2020-04-22T14:35:11.0121988Z error: build failed
2020-04-22T14:35:11.0171331Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-22T14:35:11.0181645Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-04-22T14:35:11.0181911Z Build completed unsuccessfully in 0:03:48
2020-04-22T14:35:11.0276633Z == clock drift check ==
2020-04-22T14:35:11.0298137Z   local time: Wed Apr 22 14:35:11 UTC 2020
2020-04-22T14:35:11.0298137Z   local time: Wed Apr 22 14:35:11 UTC 2020
2020-04-22T14:35:11.3212438Z   network time: Wed, 22 Apr 2020 14:35:11 GMT
2020-04-22T14:35:12.0789076Z 
2020-04-22T14:35:12.0789076Z 
2020-04-22T14:35:12.0839473Z ##[error]Bash exited with code '1'.
2020-04-22T14:35:12.0848990Z ##[section]Finishing: Run build
2020-04-22T14:35:12.0887318Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70820/merge to s
2020-04-22T14:35:12.0890867Z Task         : Get sources
2020-04-22T14:35:12.0891105Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-22T14:35:12.0891311Z Version      : 1.0.0
2020-04-22T14:35:12.0891458Z Author       : Microsoft
2020-04-22T14:35:12.0891458Z Author       : Microsoft
2020-04-22T14:35:12.0891705Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-22T14:35:12.0891965Z ==============================================================================
2020-04-22T14:35:12.3661157Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-22T14:35:12.3706580Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70820/merge to s
2020-04-22T14:35:12.3792639Z Cleaning up task key
2020-04-22T14:35:12.3793725Z Start cleaning up orphan processes.
2020-04-22T14:35:12.3960224Z Terminate orphan process: pid (3609) (python)
2020-04-22T14:35:12.4171239Z ##[section]Finishing: Finalize Job
