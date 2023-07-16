plain
2020-04-22T05:49:16.0889408Z ========================== Starting Command Output ===========================
2020-04-22T05:49:16.0891552Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/3e6f7e91-b202-4490-b372-6f4f3533619a.sh
2020-04-22T05:49:16.0891769Z 
2020-04-22T05:49:16.0894532Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-22T05:49:16.0912101Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71413/merge to s
2020-04-22T05:49:16.0914619Z Task         : Get sources
2020-04-22T05:49:16.0914843Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-22T05:49:16.0915080Z Version      : 1.0.0
2020-04-22T05:49:16.0915231Z Author       : Microsoft
---
2020-04-22T05:49:17.0956295Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-22T05:49:17.0963192Z ##[command]git config gc.auto 0
2020-04-22T05:49:17.0967674Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-22T05:49:17.0972416Z ##[command]git config --get-all http.proxy
2020-04-22T05:49:17.0981104Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71413/merge:refs/remotes/pull/71413/merge
---
2020-04-22T05:51:39.3650691Z  ---> 78ad2f4d4aca
2020-04-22T05:51:39.3650983Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-22T05:51:39.3652182Z  ---> Using cache
2020-04-22T05:51:39.3652617Z  ---> 4d2dc61c4d00
2020-04-22T05:51:39.3653710Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-22T05:51:39.3657132Z  ---> 776b6266a8b7
2020-04-22T05:51:39.3685214Z Successfully built 776b6266a8b7
2020-04-22T05:51:39.3714050Z Successfully tagged rust-ci:latest
2020-04-22T05:51:39.3953080Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-22T05:51:39.3953080Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-22T05:51:39.3966937Z Looks like docker image is the same as before, not uploading
2020-04-22T05:51:47.5380738Z [CI_JOB_NAME=mingw-check]
2020-04-22T05:51:47.5594219Z [CI_JOB_NAME=mingw-check]
2020-04-22T05:51:47.5616828Z == clock drift check ==
2020-04-22T05:51:47.5640615Z   local time: Wed Apr 22 05:51:47 UTC 2020
2020-04-22T05:51:47.6426109Z   network time: Wed, 22 Apr 2020 05:51:47 GMT
2020-04-22T05:51:47.6449598Z Starting sccache server...
2020-04-22T05:51:47.7495871Z configure: processing command line
2020-04-22T05:51:47.7496684Z configure: 
2020-04-22T05:51:47.7497795Z configure: rust.parallel-compiler := True
---
2020-04-22T05:55:02.6832403Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-22T05:55:02.6838501Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-22T05:55:02.7998775Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-22T05:55:02.9572630Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-22T05:55:03.3441278Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-22T05:55:05.3515095Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-22T05:55:05.7808731Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-22T05:55:07.6624499Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-22T05:55:08.0676212Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-22T05:56:44.4603208Z configure: llvm.ccache          := sccache
2020-04-22T05:56:44.4603448Z configure: llvm.assertions      := True
2020-04-22T05:56:44.4603854Z configure: dist.missing-tools   := True
2020-04-22T05:56:44.4604247Z configure: rust.debug-assertions := True
2020-04-22T05:56:44.4604746Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-22T05:56:44.4605217Z configure: writing `config.toml` in current directory
2020-04-22T05:56:44.4605413Z configure: 
2020-04-22T05:56:44.4605778Z configure: run `python /checkout/x.py --help`
2020-04-22T05:56:44.4605971Z configure: 
---
2020-04-22T05:58:05.7091968Z Hugepagesize:       2048 kB
2020-04-22T05:58:05.7092149Z DirectMap4k:      135104 kB
2020-04-22T05:58:05.7092333Z DirectMap2M:     4059136 kB
2020-04-22T05:58:05.7092515Z DirectMap1G:     5242880 kB
2020-04-22T05:58:05.7100163Z + python3 ../x.py test src/tools/expand-yaml-anchors
2020-04-22T05:58:06.9670622Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-22T05:58:06.9670622Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-22T05:58:06.9676608Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-22T05:58:07.1808847Z    Compiling unicode-xid v0.2.0
2020-04-22T05:58:07.3057672Z    Compiling syn v1.0.11
2020-04-22T05:58:08.0901600Z    Compiling linked-hash-map v0.5.2
2020-04-22T05:58:08.1294726Z    Compiling lazy_static v1.4.0
2020-04-22T05:58:08.1294726Z    Compiling lazy_static v1.4.0
2020-04-22T05:58:08.3002257Z    Compiling yaml-rust v0.4.3
2020-04-22T05:58:12.1026764Z    Compiling quote v1.0.2
2020-04-22T05:58:24.7160223Z    Compiling thiserror-impl v1.0.5
2020-04-22T05:58:28.8213357Z    Compiling thiserror v1.0.5
2020-04-22T05:58:28.8831069Z    Compiling yaml-merge-keys v0.4.0
2020-04-22T05:58:29.9065614Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-22T05:58:31.3354349Z Build completed successfully in 0:00:25
2020-04-22T05:58:31.3442320Z + python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-22T05:58:31.5745765Z     Finished dev [unoptimized] target(s) in 0.15s
2020-04-22T05:58:32.5667291Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-22T06:00:24.2456382Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-22T06:00:24.4309053Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-22T06:00:24.6163970Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-22T06:00:24.6236375Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-22T06:00:25.1621408Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-22T06:00:27.1950363Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-22T06:00:27.6493292Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-22T06:00:29.5283561Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-22T06:00:29.9228353Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-22T06:03:42.7896440Z    Compiling cargo_metadata v0.9.1
2020-04-22T06:03:46.0097902Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-04-22T06:03:55.5412102Z     Finished release [optimized] target(s) in 21.47s
2020-04-22T06:03:55.5471321Z tidy check
2020-04-22T06:03:56.4089302Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0432.md:9: line longer than 80 chars
2020-04-22T06:03:56.4089904Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0432.md:13: line longer than 80 chars
2020-04-22T06:03:56.4090399Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0432.md:28: line longer than 80 chars
2020-04-22T06:03:56.4096708Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0432.md:38: line longer than 80 chars
2020-04-22T06:04:03.7565376Z Found 491 error codes
2020-04-22T06:04:03.7566098Z Found 0 error codes with no tests
2020-04-22T06:04:03.7566539Z Done!
2020-04-22T06:04:03.7566865Z some tidy checks failed
2020-04-22T06:04:03.7566865Z some tidy checks failed
2020-04-22T06:04:03.7570484Z 
2020-04-22T06:04:03.7570843Z 
2020-04-22T06:04:03.7572405Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"
2020-04-22T06:04:03.7640219Z 
2020-04-22T06:04:03.7640326Z 
2020-04-22T06:04:03.7640546Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-04-22T06:04:03.7641008Z Build completed unsuccessfully in 0:00:30
2020-04-22T06:04:03.7641008Z Build completed unsuccessfully in 0:00:30
2020-04-22T06:04:03.7685050Z == clock drift check ==
2020-04-22T06:04:03.7700788Z   local time: Wed Apr 22 06:04:03 UTC 2020
2020-04-22T06:04:03.9772164Z   network time: Wed, 22 Apr 2020 06:04:03 GMT
2020-04-22T06:04:05.5972377Z 
2020-04-22T06:04:05.5972377Z 
2020-04-22T06:04:05.6036849Z ##[error]Bash exited with code '1'.
2020-04-22T06:04:05.6048754Z ##[section]Finishing: Run build
2020-04-22T06:04:05.6094369Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71413/merge to s
2020-04-22T06:04:05.6099014Z Task         : Get sources
2020-04-22T06:04:05.6099320Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-22T06:04:05.6099601Z Version      : 1.0.0
2020-04-22T06:04:05.6099821Z Author       : Microsoft
2020-04-22T06:04:05.6099821Z Author       : Microsoft
2020-04-22T06:04:05.6100139Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-22T06:04:05.6100495Z ==============================================================================
2020-04-22T06:04:05.9332301Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-22T06:04:05.9381182Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71413/merge to s
2020-04-22T06:04:05.9468487Z Cleaning up task key
2020-04-22T06:04:05.9469681Z Start cleaning up orphan processes.
2020-04-22T06:04:05.9630340Z Terminate orphan process: pid (3817) (python)
2020-04-22T06:04:05.9849938Z ##[section]Finishing: Finalize Job
