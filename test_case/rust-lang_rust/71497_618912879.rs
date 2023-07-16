plain
2020-04-24T07:45:59.4272836Z ========================== Starting Command Output ===========================
2020-04-24T07:45:59.4277101Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/6bcc49c5-3e25-4b00-a9ef-1964e946e014.sh
2020-04-24T07:45:59.4277547Z 
2020-04-24T07:45:59.4281940Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-24T07:45:59.4299795Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71497/merge to s
2020-04-24T07:45:59.4302913Z Task         : Get sources
2020-04-24T07:45:59.4303287Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-24T07:45:59.4303556Z Version      : 1.0.0
2020-04-24T07:45:59.4303733Z Author       : Microsoft
---
2020-04-24T07:46:00.6413139Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-24T07:46:00.6422511Z ##[command]git config gc.auto 0
2020-04-24T07:46:00.6429038Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-24T07:46:00.6433843Z ##[command]git config --get-all http.proxy
2020-04-24T07:46:00.6444488Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71497/merge:refs/remotes/pull/71497/merge
---
2020-04-24T07:48:27.8971704Z  ---> f7353ccad5b1
2020-04-24T07:48:27.8972151Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-24T07:48:27.8972580Z  ---> Using cache
2020-04-24T07:48:27.8972914Z  ---> ed38efbaa060
2020-04-24T07:48:27.8975671Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-24T07:48:27.8977398Z  ---> c5008ef7ae8e
2020-04-24T07:48:27.9004831Z Successfully built c5008ef7ae8e
2020-04-24T07:48:27.9080382Z Successfully tagged rust-ci:latest
2020-04-24T07:48:28.0435332Z Built container sha256:c5008ef7ae8e94d7ef502e3cef26e61208e14ebdb36913f3a8bb86291bd6430b
2020-04-24T07:48:28.0435332Z Built container sha256:c5008ef7ae8e94d7ef502e3cef26e61208e14ebdb36913f3a8bb86291bd6430b
2020-04-24T07:48:28.0452464Z Looks like docker image is the same as before, not uploading
2020-04-24T07:48:36.0107160Z [CI_JOB_NAME=mingw-check]
2020-04-24T07:48:36.0346544Z [CI_JOB_NAME=mingw-check]
2020-04-24T07:48:36.0380691Z == clock drift check ==
2020-04-24T07:48:36.0391517Z   local time: Fri Apr 24 07:48:36 UTC 2020
2020-04-24T07:48:36.1761292Z   network time: Fri, 24 Apr 2020 07:48:36 GMT
2020-04-24T07:48:36.1792091Z Starting sccache server...
2020-04-24T07:48:36.2924506Z configure: processing command line
2020-04-24T07:48:36.2926131Z configure: 
2020-04-24T07:48:36.2927143Z configure: rust.parallel-compiler := True
---
2020-04-24T07:52:42.6588749Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-24T07:52:42.7463552Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-24T07:52:42.9547817Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-24T07:52:43.1025122Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-24T07:52:43.6138730Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-24T07:52:46.0276067Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-24T07:52:46.5476334Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-24T07:52:48.7579630Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-24T07:52:49.2283912Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-24T07:54:46.3455514Z configure: rust.verify-llvm-ir  := True
2020-04-24T07:54:46.3455951Z configure: rust.channel         := nightly
2020-04-24T07:54:46.3456456Z configure: llvm.ccache          := sccache
2020-04-24T07:54:46.3456856Z configure: build.submodules     := False
2020-04-24T07:54:46.3457588Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-24T07:54:46.3458387Z configure: writing `config.toml` in current directory
2020-04-24T07:54:46.3458720Z configure: 
2020-04-24T07:54:46.3459294Z configure: run `python /checkout/x.py --help`
2020-04-24T07:54:46.3459663Z configure: 
---
2020-04-24T07:56:25.6026205Z Hugepagesize:       2048 kB
2020-04-24T07:56:25.6026404Z DirectMap4k:      114624 kB
2020-04-24T07:56:25.6026618Z DirectMap2M:     4079616 kB
2020-04-24T07:56:25.6026816Z DirectMap1G:     5242880 kB
2020-04-24T07:56:25.6056071Z + python3 ../x.py test src/tools/expand-yaml-anchors
2020-04-24T07:56:27.1191479Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-24T07:56:27.1191479Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-24T07:56:27.1201876Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-24T07:56:27.3774190Z    Compiling unicode-xid v0.2.0
2020-04-24T07:56:27.5214202Z    Compiling syn v1.0.11
2020-04-24T07:56:28.4476017Z    Compiling linked-hash-map v0.5.2
2020-04-24T07:56:28.4575805Z    Compiling lazy_static v1.4.0
2020-04-24T07:56:28.4575805Z    Compiling lazy_static v1.4.0
2020-04-24T07:56:28.6953959Z    Compiling yaml-rust v0.4.3
2020-04-24T07:56:33.2941370Z    Compiling quote v1.0.2
2020-04-24T07:56:48.8792090Z    Compiling thiserror-impl v1.0.5
2020-04-24T07:56:53.9646933Z    Compiling thiserror v1.0.5
2020-04-24T07:56:54.0344741Z    Compiling yaml-merge-keys v0.4.0
2020-04-24T07:56:55.2937701Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-24T07:56:57.3909644Z Build completed successfully in 0:00:31
2020-04-24T07:56:57.4053560Z + python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-24T07:56:57.6876973Z     Finished dev [unoptimized] target(s) in 0.18s
2020-04-24T07:56:58.8495419Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-24T07:59:15.3961010Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-24T07:59:15.5840828Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-24T07:59:15.7966786Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-24T07:59:15.8591561Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-24T07:59:16.4703198Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-24T07:59:18.9240536Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-24T07:59:19.4894481Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-24T07:59:21.7173442Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-24T07:59:22.1956942Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-24T08:03:17.6465392Z    Compiling cargo_metadata v0.9.1
2020-04-24T08:03:21.5552726Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-04-24T08:03:33.3430605Z     Finished release [optimized] target(s) in 26.75s
2020-04-24T08:03:33.3516124Z tidy check
2020-04-24T08:03:33.5496235Z tidy error: /checkout/src/librustc_codegen_llvm/base.rs:99: trailing whitespace
2020-04-24T08:03:33.5496758Z tidy error: /checkout/src/librustc_codegen_llvm/base.rs:121: trailing whitespace
2020-04-24T08:03:33.5497132Z tidy error: /checkout/src/librustc_codegen_llvm/base.rs:122: trailing whitespace
2020-04-24T08:03:33.5497518Z tidy error: /checkout/src/librustc_codegen_llvm/base.rs:125: trailing whitespace
2020-04-24T08:03:34.1283365Z tidy error: /checkout/src/librustc_codegen_ssa/base.rs:604: trailing whitespace
2020-04-24T08:03:34.1396778Z tidy error: /checkout/src/librustc_codegen_ssa/traits/backend.rs:92: TODO is deprecated; use FIXME
2020-04-24T08:03:42.0069995Z Found 492 error codes
2020-04-24T08:03:42.0070782Z Found 0 error codes with no tests
2020-04-24T08:03:42.0071085Z Done!
2020-04-24T08:03:42.0071380Z some tidy checks failed
2020-04-24T08:03:42.0071380Z some tidy checks failed
2020-04-24T08:03:42.0083549Z 
2020-04-24T08:03:42.0083866Z 
2020-04-24T08:03:42.0085735Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo"
2020-04-24T08:03:42.0086747Z 
2020-04-24T08:03:42.0086973Z 
2020-04-24T08:03:42.0094987Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-04-24T08:03:42.0095481Z Build completed unsuccessfully in 0:00:36
2020-04-24T08:03:42.0095481Z Build completed unsuccessfully in 0:00:36
2020-04-24T08:03:42.0205999Z == clock drift check ==
2020-04-24T08:03:42.0222563Z   local time: Fri Apr 24 08:03:42 UTC 2020
2020-04-24T08:03:42.2868811Z   network time: Fri, 24 Apr 2020 08:03:42 GMT
2020-04-24T08:03:43.8110199Z 
2020-04-24T08:03:43.8110199Z 
2020-04-24T08:03:43.8188326Z ##[error]Bash exited with code '1'.
2020-04-24T08:03:43.8203112Z ##[section]Finishing: Run build
2020-04-24T08:03:43.8250954Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71497/merge to s
2020-04-24T08:03:43.8255863Z Task         : Get sources
2020-04-24T08:03:43.8256174Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-24T08:03:43.8256463Z Version      : 1.0.0
2020-04-24T08:03:43.8256684Z Author       : Microsoft
2020-04-24T08:03:43.8256684Z Author       : Microsoft
2020-04-24T08:03:43.8257895Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-24T08:03:43.8258273Z ==============================================================================
2020-04-24T08:03:44.1892843Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-24T08:03:44.1942492Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71497/merge to s
2020-04-24T08:03:44.2038278Z Cleaning up task key
2020-04-24T08:03:44.2039541Z Start cleaning up orphan processes.
2020-04-24T08:03:44.2261635Z Terminate orphan process: pid (4671) (python)
2020-04-24T08:03:44.2551574Z ##[section]Finishing: Finalize Job
