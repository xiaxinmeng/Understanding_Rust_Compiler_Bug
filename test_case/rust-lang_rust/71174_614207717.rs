plain
2020-04-15T18:18:54.2651901Z ========================== Starting Command Output ===========================
2020-04-15T18:18:54.2655402Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/b59a9b21-c200-4a77-99fd-c617141f9525.sh
2020-04-15T18:18:54.2655626Z 
2020-04-15T18:18:54.2662359Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-15T18:18:54.2684591Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71174/merge to s
2020-04-15T18:18:54.2688872Z Task         : Get sources
2020-04-15T18:18:54.2689111Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-15T18:18:54.2689446Z Version      : 1.0.0
2020-04-15T18:18:54.2689607Z Author       : Microsoft
---
2020-04-15T18:18:55.2609032Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-15T18:18:55.2619324Z ##[command]git config gc.auto 0
2020-04-15T18:18:55.2626599Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-15T18:18:55.2632474Z ##[command]git config --get-all http.proxy
2020-04-15T18:18:55.2641227Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71174/merge:refs/remotes/pull/71174/merge
---
2020-04-15T18:22:16.8226473Z  ---> 78ad2f4d4aca
2020-04-15T18:22:16.8228962Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-15T18:22:16.8234194Z  ---> Using cache
2020-04-15T18:22:16.8235165Z  ---> 4d2dc61c4d00
2020-04-15T18:22:16.8236752Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-15T18:22:16.8239580Z  ---> 776b6266a8b7
2020-04-15T18:22:16.8286649Z Successfully built 776b6266a8b7
2020-04-15T18:22:16.8317386Z Successfully tagged rust-ci:latest
2020-04-15T18:22:17.1438137Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-15T18:22:17.1438137Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-15T18:22:17.1451744Z Looks like docker image is the same as before, not uploading
2020-04-15T18:22:17.9241809Z [CI_JOB_NAME=mingw-check]
2020-04-15T18:22:17.9471407Z [CI_JOB_NAME=mingw-check]
2020-04-15T18:22:17.9500342Z == clock drift check ==
2020-04-15T18:22:17.9509676Z   local time: Wed Apr 15 18:22:17 UTC 2020
2020-04-15T18:22:18.0400842Z   network time: Wed, 15 Apr 2020 18:22:18 GMT
2020-04-15T18:22:18.0422063Z Starting sccache server...
2020-04-15T18:22:18.1491594Z configure: processing command line
2020-04-15T18:22:18.1492251Z configure: 
2020-04-15T18:22:18.1493428Z configure: rust.parallel-compiler := True
---
2020-04-15T18:26:05.5854312Z     Checking rustc_span v0.0.0 (/checkout/src/librustc_span)
2020-04-15T18:26:10.2625197Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2020-04-15T18:26:11.5010243Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-15T18:26:11.5353369Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-15T18:26:11.7392863Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-15T18:26:12.5658133Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-15T18:26:12.5996541Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-15T18:26:14.1551505Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-15T18:26:14.6524314Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
---
2020-04-15T18:26:43.1034361Z      |
2020-04-15T18:26:43.1034825Z 1311 |     ) {
2020-04-15T18:26:43.1037690Z      |       - closing delimiter possibly meant for this
2020-04-15T18:26:43.1038465Z ...
2020-04-15T18:26:43.1039383Z 1316 |             .map(|generator_kind| matches!(generator_kind, hir::GeneratorKind::Async(..))
2020-04-15T18:26:43.1040646Z ...
2020-04-15T18:26:43.1041062Z 1472 |     }
2020-04-15T18:26:43.1041679Z      |     ^ mismatched closing delimiter
2020-04-15T18:26:43.1041933Z 
2020-04-15T18:26:43.1041933Z 
2020-04-15T18:26:43.1080887Z error: expected one of `)`, `,`, `.`, `?`, or an operator, found `;`
2020-04-15T18:26:43.1082428Z      |
2020-04-15T18:26:43.1082428Z      |
2020-04-15T18:26:43.1083255Z 1316 |             .map(|generator_kind| matches!(generator_kind, hir::GeneratorKind::Async(..))
2020-04-15T18:26:43.1084940Z 1317 |             .unwrap_or(false);
2020-04-15T18:26:43.1084940Z 1317 |             .unwrap_or(false);
2020-04-15T18:26:43.1085646Z      |                              ^ help: `)` may belong here
2020-04-15T18:26:43.1125465Z error: expected expression, found `)`
2020-04-15T18:26:43.1126337Z     --> src/librustc_trait_selection/traits/error_reporting/suggestions.rs:1472:5
2020-04-15T18:26:43.1127177Z      |
2020-04-15T18:26:43.1127597Z 1472 |     }
---
2020-04-15T18:26:44.9992475Z 
2020-04-15T18:26:44.9992860Z To learn more, run the command again with --verbose.
2020-04-15T18:26:44.9993363Z warning: build failed, waiting for other jobs to finish...
2020-04-15T18:26:45.4405328Z error: build failed
2020-04-15T18:26:45.4443201Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-15T18:26:45.4447055Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-04-15T18:26:45.4447536Z Build completed unsuccessfully in 0:04:27
2020-04-15T18:26:45.4553060Z == clock drift check ==
2020-04-15T18:26:45.4569294Z   local time: Wed Apr 15 18:26:45 UTC 2020
2020-04-15T18:26:45.4569294Z   local time: Wed Apr 15 18:26:45 UTC 2020
2020-04-15T18:26:45.5856918Z   network time: Wed, 15 Apr 2020 18:26:45 GMT
2020-04-15T18:26:46.2564274Z 
2020-04-15T18:26:46.2564274Z 
2020-04-15T18:26:46.2630452Z ##[error]Bash exited with code '1'.
2020-04-15T18:26:46.2645737Z ##[section]Finishing: Run build
2020-04-15T18:26:46.2691349Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71174/merge to s
2020-04-15T18:26:46.2695861Z Task         : Get sources
2020-04-15T18:26:46.2696144Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-15T18:26:46.2696405Z Version      : 1.0.0
2020-04-15T18:26:46.2696580Z Author       : Microsoft
2020-04-15T18:26:46.2696580Z Author       : Microsoft
2020-04-15T18:26:46.2696856Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-15T18:26:46.2697183Z ==============================================================================
2020-04-15T18:26:46.6010618Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-15T18:26:46.6051980Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71174/merge to s
2020-04-15T18:26:46.6150084Z Cleaning up task key
2020-04-15T18:26:46.6151145Z Start cleaning up orphan processes.
2020-04-15T18:26:46.6329868Z Terminate orphan process: pid (3543) (python)
2020-04-15T18:26:46.7369791Z ##[section]Finishing: Finalize Job
