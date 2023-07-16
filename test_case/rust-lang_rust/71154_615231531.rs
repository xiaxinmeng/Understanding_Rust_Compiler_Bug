plain
2020-04-17T11:53:24.5942942Z ========================== Starting Command Output ===========================
2020-04-17T11:53:24.5945602Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/7a92ffab-a8e6-4e95-901d-5881a3453f48.sh
2020-04-17T11:53:24.5945942Z 
2020-04-17T11:53:24.5949724Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-17T11:53:24.5967397Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71154/merge to s
2020-04-17T11:53:24.5970482Z Task         : Get sources
2020-04-17T11:53:24.5970717Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-17T11:53:24.5970939Z Version      : 1.0.0
2020-04-17T11:53:24.5971090Z Author       : Microsoft
---
2020-04-17T11:53:25.4324148Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-17T11:53:25.4330964Z ##[command]git config gc.auto 0
2020-04-17T11:53:25.4335073Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-17T11:53:25.4338607Z ##[command]git config --get-all http.proxy
2020-04-17T11:53:25.4344730Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71154/merge:refs/remotes/pull/71154/merge
---
2020-04-17T11:55:37.3803191Z  ---> 78ad2f4d4aca
2020-04-17T11:55:37.3803451Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-17T11:55:37.3808431Z  ---> Using cache
2020-04-17T11:55:37.3808718Z  ---> 4d2dc61c4d00
2020-04-17T11:55:37.3809599Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-17T11:55:37.3814487Z  ---> 776b6266a8b7
2020-04-17T11:55:37.3844021Z Successfully built 776b6266a8b7
2020-04-17T11:55:37.3881394Z Successfully tagged rust-ci:latest
2020-04-17T11:55:37.4111815Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-17T11:55:37.4111815Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-17T11:55:37.4126963Z Looks like docker image is the same as before, not uploading
2020-04-17T11:55:45.7122288Z [CI_JOB_NAME=mingw-check]
2020-04-17T11:55:45.7389645Z [CI_JOB_NAME=mingw-check]
2020-04-17T11:55:45.7406431Z == clock drift check ==
2020-04-17T11:55:45.7416091Z   local time: Fri Apr 17 11:55:45 UTC 2020
2020-04-17T11:55:45.8100141Z   network time: Fri, 17 Apr 2020 11:55:45 GMT
2020-04-17T11:55:45.8122058Z Starting sccache server...
2020-04-17T11:55:45.9045274Z configure: processing command line
2020-04-17T11:55:45.9045536Z configure: 
2020-04-17T11:55:45.9046365Z configure: rust.parallel-compiler := True
---
2020-04-17T11:59:00.9638063Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-17T11:59:01.1640291Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-17T11:59:01.3326363Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-17T11:59:01.3373872Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-17T11:59:01.8786368Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-17T11:59:03.9044467Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-17T11:59:04.3236578Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-17T11:59:06.1172971Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-17T11:59:06.4960196Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-17T12:00:41.7898418Z configure: rust.verify-llvm-ir  := True
2020-04-17T12:00:41.7898682Z configure: rust.channel         := nightly
2020-04-17T12:00:41.7899100Z configure: build.cargo-native-static := True
2020-04-17T12:00:41.7899376Z configure: build.submodules     := False
2020-04-17T12:00:41.7899905Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-17T12:00:41.7900358Z configure: writing `config.toml` in current directory
2020-04-17T12:00:41.7900571Z configure: 
2020-04-17T12:00:41.7900923Z configure: run `python /checkout/x.py --help`
2020-04-17T12:00:41.7901112Z configure: 
---
2020-04-17T12:02:01.7442478Z Hugepagesize:       2048 kB
2020-04-17T12:02:01.7442678Z DirectMap4k:      143296 kB
2020-04-17T12:02:01.7442876Z DirectMap2M:     5099520 kB
2020-04-17T12:02:01.7443088Z DirectMap1G:     4194304 kB
2020-04-17T12:02:01.7459521Z + python3 ../x.py test src/tools/expand-yaml-anchors
2020-04-17T12:02:02.9933769Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-17T12:02:02.9933769Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-17T12:02:02.9941278Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-17T12:02:03.2370163Z    Compiling unicode-xid v0.2.0
2020-04-17T12:02:03.3584351Z    Compiling syn v1.0.11
2020-04-17T12:02:04.0978542Z    Compiling linked-hash-map v0.5.2
2020-04-17T12:02:04.1466906Z    Compiling lazy_static v1.4.0
2020-04-17T12:02:04.1466906Z    Compiling lazy_static v1.4.0
2020-04-17T12:02:04.3001460Z    Compiling yaml-rust v0.4.3
2020-04-17T12:02:08.1121582Z    Compiling quote v1.0.2
2020-04-17T12:02:20.6705785Z    Compiling thiserror-impl v1.0.5
2020-04-17T12:02:24.7895607Z    Compiling thiserror v1.0.5
2020-04-17T12:02:24.8435881Z    Compiling yaml-merge-keys v0.4.0
2020-04-17T12:02:25.8902657Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-17T12:02:27.3275063Z Build completed successfully in 0:00:25
2020-04-17T12:02:27.3366057Z + python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-17T12:02:27.5643004Z     Finished dev [unoptimized] target(s) in 0.15s
2020-04-17T12:02:28.5342396Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-17T12:04:22.3364610Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-17T12:04:22.5411417Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-17T12:04:22.7062289Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-17T12:04:22.7254194Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-17T12:04:23.2559404Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-17T12:04:25.2162676Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-17T12:04:25.6476387Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-17T12:04:27.4992775Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-17T12:04:27.8905883Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-17T12:07:45.2035765Z    Compiling cargo_metadata v0.9.1
2020-04-17T12:07:49.2025186Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-04-17T12:07:58.5914558Z     Finished release [optimized] target(s) in 21.87s
2020-04-17T12:07:58.5991004Z tidy check
2020-04-17T12:07:58.8752369Z tidy error: /checkout/src/librustc_middle/query/mod.rs:473: trailing whitespace
2020-04-17T12:07:58.8752905Z tidy error: /checkout/src/librustc_middle/query/mod.rs:474: trailing whitespace
2020-04-17T12:07:59.0847865Z tidy error: /checkout/src/librustc_trait_selection/traits/select.rs:496: TODO is deprecated; use FIXME
2020-04-17T12:07:59.0920451Z tidy error: /checkout/src/librustc_trait_selection/traits/fulfill.rs:503: TODO is deprecated; use FIXME
2020-04-17T12:08:02.6665692Z tidy error: /checkout/src/test/ui/__check/issue-61936.rs:12: trailing whitespace
2020-04-17T12:08:02.6666319Z tidy error: /checkout/src/test/ui/__check/issue-61936.rs:25: trailing whitespace
2020-04-17T12:08:02.6666949Z tidy error: /checkout/src/test/ui/__check/issue-61936.rs:29: trailing whitespace
2020-04-17T12:08:02.6668192Z tidy error: /checkout/src/test/ui/__check/issue-61936.rs:34: trailing whitespace
2020-04-17T12:08:02.6669872Z tidy error: /checkout/src/test/ui/__check/issue-61936.rs:43: trailing whitespace
2020-04-17T12:08:06.9352991Z Found 490 error codes
2020-04-17T12:08:06.9353460Z Found 0 error codes with no tests
2020-04-17T12:08:06.9353661Z Done!
2020-04-17T12:08:06.9353806Z some tidy checks failed
2020-04-17T12:08:06.9353806Z some tidy checks failed
2020-04-17T12:08:06.9353926Z 
2020-04-17T12:08:06.9354011Z 
2020-04-17T12:08:06.9355132Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"
2020-04-17T12:08:06.9355720Z 
2020-04-17T12:08:06.9355820Z 
2020-04-17T12:08:06.9360607Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-04-17T12:08:06.9360917Z Build completed unsuccessfully in 0:00:31
2020-04-17T12:08:06.9360917Z Build completed unsuccessfully in 0:00:31
2020-04-17T12:08:06.9457925Z == clock drift check ==
2020-04-17T12:08:06.9478883Z   local time: Fri Apr 17 12:08:06 UTC 2020
2020-04-17T12:08:07.1226216Z   network time: Fri, 17 Apr 2020 12:08:07 GMT
2020-04-17T12:08:08.8383552Z 
2020-04-17T12:08:08.8383552Z 
2020-04-17T12:08:08.8446422Z ##[error]Bash exited with code '1'.
2020-04-17T12:08:08.8459037Z ##[section]Finishing: Run build
2020-04-17T12:08:08.8498918Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71154/merge to s
2020-04-17T12:08:08.8503196Z Task         : Get sources
2020-04-17T12:08:08.8503450Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-17T12:08:08.8503694Z Version      : 1.0.0
2020-04-17T12:08:08.8503861Z Author       : Microsoft
2020-04-17T12:08:08.8503861Z Author       : Microsoft
2020-04-17T12:08:08.8504125Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-17T12:08:08.8504433Z ==============================================================================
2020-04-17T12:08:09.1554428Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-17T12:08:09.1593461Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71154/merge to s
2020-04-17T12:08:09.1689239Z Cleaning up task key
2020-04-17T12:08:09.1690530Z Start cleaning up orphan processes.
2020-04-17T12:08:09.1870095Z Terminate orphan process: pid (3608) (python)
2020-04-17T12:08:09.2061488Z ##[section]Finishing: Finalize Job
