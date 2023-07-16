plain
2020-04-24T05:29:41.1803168Z ========================== Starting Command Output ===========================
2020-04-24T05:29:41.1805371Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/ec2e9418-fba9-4fc5-b63c-7aaaf5efcffa.sh
2020-04-24T05:29:41.1805613Z 
2020-04-24T05:29:41.1809248Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-24T05:29:41.1827979Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71497/merge to s
2020-04-24T05:29:41.1830733Z Task         : Get sources
2020-04-24T05:29:41.1831008Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-24T05:29:41.1831212Z Version      : 1.0.0
2020-04-24T05:29:41.1831352Z Author       : Microsoft
---
2020-04-24T05:29:42.1926479Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-24T05:29:42.1930464Z ##[command]git config gc.auto 0
2020-04-24T05:29:42.1933117Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-24T05:29:42.1935527Z ##[command]git config --get-all http.proxy
2020-04-24T05:29:42.1940205Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71497/merge:refs/remotes/pull/71497/merge
---
2020-04-24T05:31:58.3868765Z  ---> f7353ccad5b1
2020-04-24T05:31:58.3869113Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-24T05:31:58.3869608Z  ---> Using cache
2020-04-24T05:31:58.3870012Z  ---> ed38efbaa060
2020-04-24T05:31:58.3872659Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-24T05:31:58.3874559Z  ---> c5008ef7ae8e
2020-04-24T05:31:58.3931415Z Successfully built c5008ef7ae8e
2020-04-24T05:31:58.3959823Z Successfully tagged rust-ci:latest
2020-04-24T05:31:58.4218112Z Built container sha256:c5008ef7ae8e94d7ef502e3cef26e61208e14ebdb36913f3a8bb86291bd6430b
2020-04-24T05:31:58.4218112Z Built container sha256:c5008ef7ae8e94d7ef502e3cef26e61208e14ebdb36913f3a8bb86291bd6430b
2020-04-24T05:31:58.4233917Z Looks like docker image is the same as before, not uploading
2020-04-24T05:32:06.7871115Z [CI_JOB_NAME=mingw-check]
2020-04-24T05:32:06.8068557Z [CI_JOB_NAME=mingw-check]
2020-04-24T05:32:06.8092521Z == clock drift check ==
2020-04-24T05:32:06.8100496Z   local time: Fri Apr 24 05:32:06 UTC 2020
2020-04-24T05:32:07.0972528Z   network time: Fri, 24 Apr 2020 05:32:07 GMT
2020-04-24T05:32:07.0995646Z Starting sccache server...
2020-04-24T05:32:07.1997397Z configure: processing command line
2020-04-24T05:32:07.1997606Z configure: 
2020-04-24T05:32:07.1998774Z configure: rust.parallel-compiler := True
---
2020-04-24T05:35:36.9568354Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-24T05:35:37.0904055Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-24T05:35:37.2678580Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-24T05:35:37.3172666Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-24T05:35:37.8294932Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-24T05:35:40.0865032Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-24T05:35:40.5392225Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-24T05:35:42.4119155Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-24T05:35:42.8077836Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-24T05:37:18.9590346Z configure: rust.dist-src        := False
2020-04-24T05:37:18.9590836Z configure: build.locked-deps    := True
2020-04-24T05:37:18.9591333Z configure: rust.debug-assertions := True
2020-04-24T05:37:18.9591853Z configure: build.cargo-native-static := True
2020-04-24T05:37:18.9592441Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-24T05:37:18.9593081Z configure: writing `config.toml` in current directory
2020-04-24T05:37:18.9593511Z configure: 
2020-04-24T05:37:18.9593925Z configure: run `python /checkout/x.py --help`
2020-04-24T05:37:18.9594632Z configure: 
---
2020-04-24T05:38:39.1584402Z Hugepagesize:       2048 kB
2020-04-24T05:38:39.1584604Z DirectMap4k:      131008 kB
2020-04-24T05:38:39.1584789Z DirectMap2M:     4063232 kB
2020-04-24T05:38:39.1584974Z DirectMap1G:     5242880 kB
2020-04-24T05:38:39.1636348Z + python3 ../x.py test src/tools/expand-yaml-anchors
2020-04-24T05:38:40.3621330Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-24T05:38:40.3621330Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-24T05:38:40.3675616Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-24T05:38:40.5436109Z    Compiling unicode-xid v0.2.0
2020-04-24T05:38:40.6576568Z    Compiling syn v1.0.11
2020-04-24T05:38:41.4026288Z    Compiling linked-hash-map v0.5.2
2020-04-24T05:38:41.4885045Z    Compiling lazy_static v1.4.0
2020-04-24T05:38:41.4885045Z    Compiling lazy_static v1.4.0
2020-04-24T05:38:41.6134414Z    Compiling yaml-rust v0.4.3
2020-04-24T05:38:45.6080030Z    Compiling quote v1.0.2
2020-04-24T05:38:58.9697911Z    Compiling thiserror-impl v1.0.5
2020-04-24T05:39:03.2229062Z    Compiling thiserror v1.0.5
2020-04-24T05:39:03.2723298Z    Compiling yaml-merge-keys v0.4.0
2020-04-24T05:39:04.3603345Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-24T05:39:08.4210495Z Build completed successfully in 0:00:29
2020-04-24T05:39:08.4293730Z + python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-24T05:39:08.6796480Z     Finished dev [unoptimized] target(s) in 0.16s
2020-04-24T05:39:09.6656846Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-24T05:41:04.2099642Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-24T05:41:04.2223733Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-24T05:41:04.3982702Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-24T05:41:04.5916347Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-24T05:41:04.9636244Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-24T05:41:07.1039610Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-24T05:41:07.5419769Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-24T05:41:09.4401023Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-24T05:41:09.8529721Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-24T05:44:26.6011129Z    Compiling cargo_metadata v0.9.1
2020-04-24T05:44:29.8348480Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-04-24T05:44:39.8405568Z     Finished release [optimized] target(s) in 22.21s
2020-04-24T05:44:39.8494483Z tidy check
2020-04-24T05:44:40.0260706Z tidy error: /checkout/src/librustc_codegen_llvm/base.rs:86: TODO is deprecated; use FIXME
2020-04-24T05:44:40.4774553Z tidy error: /checkout/src/librustc_codegen_ssa/base.rs:604: trailing whitespace
2020-04-24T05:44:40.4862632Z tidy error: /checkout/src/librustc_codegen_ssa/traits/backend.rs:92: TODO is deprecated; use FIXME
2020-04-24T05:44:48.0237975Z Found 492 error codes
2020-04-24T05:44:48.0238180Z Found 0 error codes with no tests
2020-04-24T05:44:48.0238313Z Done!
2020-04-24T05:44:48.0238429Z some tidy checks failed
2020-04-24T05:44:48.0238429Z some tidy checks failed
2020-04-24T05:44:48.0244451Z 
2020-04-24T05:44:48.0278543Z 
2020-04-24T05:44:48.0279991Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"
2020-04-24T05:44:48.0280529Z 
2020-04-24T05:44:48.0280598Z 
2020-04-24T05:44:48.0280777Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-04-24T05:44:48.0281035Z Build completed unsuccessfully in 0:00:31
2020-04-24T05:44:48.0281035Z Build completed unsuccessfully in 0:00:31
2020-04-24T05:44:48.0339057Z == clock drift check ==
2020-04-24T05:44:48.0354813Z   local time: Fri Apr 24 05:44:48 UTC 2020
2020-04-24T05:44:48.1963361Z   network time: Fri, 24 Apr 2020 05:44:48 GMT
2020-04-24T05:44:50.0683813Z 
2020-04-24T05:44:50.0683813Z 
2020-04-24T05:44:50.0742279Z ##[error]Bash exited with code '1'.
2020-04-24T05:44:50.0764548Z ##[section]Finishing: Run build
2020-04-24T05:44:50.0809703Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71497/merge to s
2020-04-24T05:44:50.0814243Z Task         : Get sources
2020-04-24T05:44:50.0814542Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-24T05:44:50.0814925Z Version      : 1.0.0
2020-04-24T05:44:50.0815257Z Author       : Microsoft
2020-04-24T05:44:50.0815257Z Author       : Microsoft
2020-04-24T05:44:50.0815532Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-24T05:44:50.0815823Z ==============================================================================
2020-04-24T05:44:50.3745618Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-24T05:44:50.3783834Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71497/merge to s
2020-04-24T05:44:50.3857362Z Cleaning up task key
2020-04-24T05:44:50.3858355Z Start cleaning up orphan processes.
2020-04-24T05:44:50.4014739Z Terminate orphan process: pid (3642) (python)
2020-04-24T05:44:50.4147833Z ##[section]Finishing: Finalize Job
