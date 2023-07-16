plain
2020-04-22T08:23:46.2723151Z ========================== Starting Command Output ===========================
2020-04-22T08:23:46.2725531Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/d8074b08-1cff-4bb3-8bf0-dd12cc5ce6cc.sh
2020-04-22T08:23:46.2726091Z 
2020-04-22T08:23:46.2731022Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-22T08:23:46.2751520Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71420/merge to s
2020-04-22T08:23:46.2760778Z Task         : Get sources
2020-04-22T08:23:46.2762066Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-22T08:23:46.2762344Z Version      : 1.0.0
2020-04-22T08:23:46.2762527Z Author       : Microsoft
---
2020-04-22T08:23:47.2596591Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-22T08:23:47.2603311Z ##[command]git config gc.auto 0
2020-04-22T08:23:47.2607030Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-22T08:23:47.2610347Z ##[command]git config --get-all http.proxy
2020-04-22T08:23:47.2616722Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71420/merge:refs/remotes/pull/71420/merge
---
2020-04-22T08:27:16.0645810Z  ---> 78ad2f4d4aca
2020-04-22T08:27:16.0646067Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-22T08:27:16.0646435Z  ---> Using cache
2020-04-22T08:27:16.0646756Z  ---> 4d2dc61c4d00
2020-04-22T08:27:16.0648018Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-22T08:27:16.0649268Z  ---> 776b6266a8b7
2020-04-22T08:27:16.0649459Z Successfully built 776b6266a8b7
2020-04-22T08:27:16.0672855Z Successfully tagged rust-ci:latest
2020-04-22T08:27:16.1040423Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-22T08:27:16.1040423Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-22T08:27:16.1056527Z Looks like docker image is the same as before, not uploading
2020-04-22T08:27:24.3428625Z [CI_JOB_NAME=mingw-check]
2020-04-22T08:27:24.3733821Z [CI_JOB_NAME=mingw-check]
2020-04-22T08:27:24.3756335Z == clock drift check ==
2020-04-22T08:27:24.3765596Z   local time: Wed Apr 22 08:27:24 UTC 2020
2020-04-22T08:27:24.6653456Z   network time: Wed, 22 Apr 2020 08:27:24 GMT
2020-04-22T08:27:24.6679252Z Starting sccache server...
2020-04-22T08:27:24.7757552Z configure: processing command line
2020-04-22T08:27:24.7757838Z configure: 
2020-04-22T08:27:24.7758846Z configure: rust.parallel-compiler := True
---
2020-04-22T08:31:08.8954612Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-22T08:31:08.9988363Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-22T08:31:09.1832333Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-22T08:31:09.2745832Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-22T08:31:09.7703514Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-22T08:31:11.9685622Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-22T08:31:12.4108442Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-22T08:31:14.3601755Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-22T08:31:14.8008539Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-22T08:33:00.9171632Z configure: rust.dist-src        := False
2020-04-22T08:33:00.9172024Z configure: build.submodules     := False
2020-04-22T08:33:00.9172602Z configure: rust.debug-assertions := True
2020-04-22T08:33:00.9172993Z configure: llvm.ccache          := sccache
2020-04-22T08:33:00.9173669Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-22T08:33:00.9174427Z configure: writing `config.toml` in current directory
2020-04-22T08:33:00.9174771Z configure: 
2020-04-22T08:33:00.9175280Z configure: run `python /checkout/x.py --help`
2020-04-22T08:33:00.9175609Z configure: 
---
2020-04-22T08:34:33.1397489Z Hugepagesize:       2048 kB
2020-04-22T08:34:33.1397710Z DirectMap4k:      157632 kB
2020-04-22T08:34:33.1397910Z DirectMap2M:     2988032 kB
2020-04-22T08:34:33.1398111Z DirectMap1G:     6291456 kB
2020-04-22T08:34:33.1419476Z + python3 ../x.py test src/tools/expand-yaml-anchors
2020-04-22T08:34:34.4521764Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-22T08:34:34.4521764Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-22T08:34:34.4529563Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-22T08:34:34.6770103Z    Compiling unicode-xid v0.2.0
2020-04-22T08:34:34.7969606Z    Compiling syn v1.0.11
2020-04-22T08:34:35.5906294Z    Compiling linked-hash-map v0.5.2
2020-04-22T08:34:35.6177097Z    Compiling lazy_static v1.4.0
2020-04-22T08:34:35.6177097Z    Compiling lazy_static v1.4.0
2020-04-22T08:34:35.8142903Z    Compiling yaml-rust v0.4.3
2020-04-22T08:34:39.8495179Z    Compiling quote v1.0.2
2020-04-22T08:34:54.4246075Z    Compiling thiserror-impl v1.0.5
2020-04-22T08:34:59.0411647Z    Compiling thiserror v1.0.5
2020-04-22T08:34:59.1025841Z    Compiling yaml-merge-keys v0.4.0
2020-04-22T08:35:00.2174649Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-22T08:35:01.7528623Z Build completed successfully in 0:00:28
2020-04-22T08:35:01.7618320Z + python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-22T08:35:02.0212608Z     Finished dev [unoptimized] target(s) in 0.17s
2020-04-22T08:35:03.1090053Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-22T08:37:04.6534051Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-22T08:37:04.8695564Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-22T08:37:05.0467004Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-22T08:37:05.0627458Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-22T08:37:05.6190131Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-22T08:37:07.7197203Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-22T08:37:08.1747690Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-22T08:37:10.1446164Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-22T08:37:10.5513156Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-22T08:40:41.9784651Z    Compiling cargo_metadata v0.9.1
2020-04-22T08:40:46.1147406Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-04-22T08:40:56.4043139Z     Finished release [optimized] target(s) in 23.66s
2020-04-22T08:40:56.4132309Z tidy check
2020-04-22T08:40:58.7627505Z tidy error: /checkout/src/test/ui/associated-types/defaults-specialization.rs:3: line longer than 100 chars
2020-04-22T08:41:00.5853057Z tidy error: /checkout/src/test/ui/specialization/non-defaulted-item-fail.rs:1: line longer than 100 chars
2020-04-22T08:41:06.1113643Z some tidy checks failed
2020-04-22T08:41:06.1113927Z Found 491 error codes
2020-04-22T08:41:06.1114192Z Found 0 error codes with no tests
2020-04-22T08:41:06.1114439Z Done!
2020-04-22T08:41:06.1114439Z Done!
2020-04-22T08:41:06.1118766Z 
2020-04-22T08:41:06.1118923Z 
2020-04-22T08:41:06.1120333Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"
2020-04-22T08:41:06.1121158Z 
2020-04-22T08:41:06.1121268Z 
2020-04-22T08:41:06.1124490Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-04-22T08:41:06.1124891Z Build completed unsuccessfully in 0:00:34
2020-04-22T08:41:06.1124891Z Build completed unsuccessfully in 0:00:34
2020-04-22T08:41:06.1238890Z == clock drift check ==
2020-04-22T08:41:06.1255136Z   local time: Wed Apr 22 08:41:06 UTC 2020
2020-04-22T08:41:06.1896341Z   network time: Wed, 22 Apr 2020 08:41:06 GMT
2020-04-22T08:41:07.7277891Z 
2020-04-22T08:41:07.7277891Z 
2020-04-22T08:41:07.7362242Z ##[error]Bash exited with code '1'.
2020-04-22T08:41:07.7383023Z ##[section]Finishing: Run build
2020-04-22T08:41:07.7432787Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71420/merge to s
2020-04-22T08:41:07.7437569Z Task         : Get sources
2020-04-22T08:41:07.7438398Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-22T08:41:07.7438695Z Version      : 1.0.0
2020-04-22T08:41:07.7438907Z Author       : Microsoft
2020-04-22T08:41:07.7438907Z Author       : Microsoft
2020-04-22T08:41:07.7439252Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-22T08:41:07.7439624Z ==============================================================================
2020-04-22T08:41:08.0848366Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-22T08:41:08.0906093Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71420/merge to s
2020-04-22T08:41:08.1032494Z Cleaning up task key
2020-04-22T08:41:08.1034086Z Start cleaning up orphan processes.
2020-04-22T08:41:08.1285789Z Terminate orphan process: pid (4051) (python)
2020-04-22T08:41:08.1510221Z ##[section]Finishing: Finalize Job
