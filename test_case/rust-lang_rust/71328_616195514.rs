plain
2020-04-19T16:00:51.0304670Z ========================== Starting Command Output ===========================
2020-04-19T16:00:51.0307051Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/e3d64898-5ec8-40fe-ad3c-ed48e75b5465.sh
2020-04-19T16:00:51.0307270Z 
2020-04-19T16:00:51.0311341Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-19T16:00:51.0328691Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71328/merge to s
2020-04-19T16:00:51.0332091Z Task         : Get sources
2020-04-19T16:00:51.0332372Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-19T16:00:51.0332646Z Version      : 1.0.0
2020-04-19T16:00:51.0332846Z Author       : Microsoft
---
2020-04-19T16:00:52.3014233Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-19T16:00:52.3023882Z ##[command]git config gc.auto 0
2020-04-19T16:00:52.3030612Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-19T16:00:52.3036729Z ##[command]git config --get-all http.proxy
2020-04-19T16:00:52.3046289Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71328/merge:refs/remotes/pull/71328/merge
---
2020-04-19T16:03:57.4649365Z  ---> 78ad2f4d4aca
2020-04-19T16:03:57.4651041Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-19T16:03:57.4654893Z  ---> Using cache
2020-04-19T16:03:57.4670413Z  ---> 4d2dc61c4d00
2020-04-19T16:03:57.4675288Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-19T16:03:57.4679679Z  ---> 776b6266a8b7
2020-04-19T16:03:57.4712310Z Successfully built 776b6266a8b7
2020-04-19T16:03:57.4770245Z Successfully tagged rust-ci:latest
2020-04-19T16:03:57.5024264Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-19T16:03:57.5024264Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-19T16:03:57.5045654Z Looks like docker image is the same as before, not uploading
2020-04-19T16:04:05.4758367Z [CI_JOB_NAME=mingw-check]
2020-04-19T16:04:05.5179536Z [CI_JOB_NAME=mingw-check]
2020-04-19T16:04:05.5210994Z == clock drift check ==
2020-04-19T16:04:05.5218009Z   local time: Sun Apr 19 16:04:05 UTC 2020
2020-04-19T16:04:05.5803344Z   network time: Sun, 19 Apr 2020 16:04:05 GMT
2020-04-19T16:04:05.5828273Z Starting sccache server...
2020-04-19T16:04:05.6861006Z configure: processing command line
2020-04-19T16:04:05.6861242Z configure: 
2020-04-19T16:04:05.6862054Z configure: rust.parallel-compiler := True
---
2020-04-19T16:07:33.8257000Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-19T16:07:33.9550906Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-19T16:07:34.1332544Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-19T16:07:34.2338822Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-19T16:07:34.6963853Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-19T16:07:36.6986313Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-19T16:07:37.1297356Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-19T16:07:38.8744011Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-19T16:07:39.2671564Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-19T16:09:12.4600052Z configure: rust.debug-assertions := True
2020-04-19T16:09:12.4600501Z configure: rust.channel         := nightly
2020-04-19T16:09:12.4601100Z configure: build.locked-deps    := True
2020-04-19T16:09:12.4602254Z configure: dist.missing-tools   := True
2020-04-19T16:09:12.4603066Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-19T16:09:12.4603790Z configure: writing `config.toml` in current directory
2020-04-19T16:09:12.4604097Z configure: 
2020-04-19T16:09:12.4604608Z configure: run `python /checkout/x.py --help`
2020-04-19T16:09:12.4604917Z configure: 
---
2020-04-19T16:10:36.9606995Z Hugepagesize:       2048 kB
2020-04-19T16:10:36.9607161Z DirectMap4k:      133056 kB
2020-04-19T16:10:36.9607337Z DirectMap2M:     4061184 kB
2020-04-19T16:10:36.9607498Z DirectMap1G:     5242880 kB
2020-04-19T16:10:36.9608267Z + python3 ../x.py test src/tools/expand-yaml-anchors
2020-04-19T16:10:37.6270714Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-19T16:10:37.6270714Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-19T16:10:37.6279937Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-19T16:10:37.8351792Z    Compiling unicode-xid v0.2.0
2020-04-19T16:10:37.9507543Z    Compiling syn v1.0.11
2020-04-19T16:10:38.6909812Z    Compiling linked-hash-map v0.5.2
2020-04-19T16:10:38.7068874Z    Compiling lazy_static v1.4.0
2020-04-19T16:10:38.7068874Z    Compiling lazy_static v1.4.0
2020-04-19T16:10:38.9010545Z    Compiling yaml-rust v0.4.3
2020-04-19T16:10:42.7099260Z    Compiling quote v1.0.2
2020-04-19T16:10:55.2839216Z    Compiling thiserror-impl v1.0.5
2020-04-19T16:10:59.2926613Z    Compiling thiserror v1.0.5
2020-04-19T16:10:59.3459671Z    Compiling yaml-merge-keys v0.4.0
2020-04-19T16:11:00.3736444Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-19T16:11:03.4965128Z Build completed successfully in 0:00:27
2020-04-19T16:11:03.5044159Z + python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-19T16:11:03.7483340Z     Finished dev [unoptimized] target(s) in 0.15s
2020-04-19T16:11:04.7347481Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-19T16:12:56.8107925Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-19T16:12:57.1693209Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-19T16:12:57.1788534Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-19T16:12:57.3661627Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-19T16:12:57.7297282Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-19T16:12:59.7683331Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-19T16:13:00.2193155Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-19T16:13:02.1117730Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-19T16:13:02.5121950Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-19T16:16:25.7503444Z     Finished release [optimized] target(s) in 21.98s
2020-04-19T16:16:25.7606393Z tidy check
2020-04-19T16:16:36.9118062Z * 594 error codes
2020-04-19T16:16:36.9118793Z * highest error code: E0751
2020-04-19T16:16:37.0245841Z tidy error: /checkout/src/libstd/path.rs:1132: malformed stability attribute: can't parse `since` key
2020-04-19T16:16:37.0247893Z tidy error: /checkout/src/libstd/path.rs:1376: malformed stability attribute: can't parse `since` key
2020-04-19T16:16:37.0249455Z tidy error: /checkout/src/libstd/path.rs:1385: malformed stability attribute: can't parse `since` key
2020-04-19T16:16:37.0250965Z tidy error: /checkout/src/libstd/path.rs:1394: malformed stability attribute: can't parse `since` key
2020-04-19T16:16:37.0252997Z tidy error: /checkout/src/libstd/path.rs:1403: malformed stability attribute: can't parse `since` key
2020-04-19T16:16:37.0254486Z tidy error: /checkout/src/libstd/path.rs:1412: malformed stability attribute: can't parse `since` key
2020-04-19T16:16:39.4333003Z Found 490 error codes
2020-04-19T16:16:39.4333984Z Found 0 error codes with no tests
2020-04-19T16:16:39.4334714Z Done!
2020-04-19T16:16:39.4335225Z some tidy checks failed
2020-04-19T16:16:39.4335225Z some tidy checks failed
2020-04-19T16:16:39.4340121Z 
2020-04-19T16:16:39.4340652Z 
2020-04-19T16:16:39.4342516Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"
2020-04-19T16:16:39.4344747Z 
2020-04-19T16:16:39.4345039Z 
2020-04-19T16:16:39.4347555Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-04-19T16:16:39.4348012Z Build completed unsuccessfully in 0:00:36
2020-04-19T16:16:39.4348012Z Build completed unsuccessfully in 0:00:36
2020-04-19T16:16:39.4443758Z == clock drift check ==
2020-04-19T16:16:39.4462384Z   local time: Sun Apr 19 16:16:39 UTC 2020
2020-04-19T16:16:39.4965879Z   network time: Sun, 19 Apr 2020 16:16:39 GMT
2020-04-19T16:16:41.0489213Z 
2020-04-19T16:16:41.0489213Z 
2020-04-19T16:16:41.0557700Z ##[error]Bash exited with code '1'.
2020-04-19T16:16:41.0570849Z ##[section]Finishing: Run build
2020-04-19T16:16:41.0617032Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71328/merge to s
2020-04-19T16:16:41.0621763Z Task         : Get sources
2020-04-19T16:16:41.0622080Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-19T16:16:41.0622393Z Version      : 1.0.0
2020-04-19T16:16:41.0622604Z Author       : Microsoft
2020-04-19T16:16:41.0622604Z Author       : Microsoft
2020-04-19T16:16:41.0622927Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-19T16:16:41.0623306Z ==============================================================================
2020-04-19T16:16:41.3680254Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-19T16:16:41.3735409Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71328/merge to s
2020-04-19T16:16:41.3819308Z Cleaning up task key
2020-04-19T16:16:41.3820562Z Start cleaning up orphan processes.
2020-04-19T16:16:41.3985276Z Terminate orphan process: pid (4669) (python)
2020-04-19T16:16:41.4138328Z ##[section]Finishing: Finalize Job
