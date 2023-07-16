plain
2020-04-24T10:54:49.2940283Z ========================== Starting Command Output ===========================
2020-04-24T10:54:49.2943837Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/4b32c694-8f05-4d62-8ad5-1d5f2c376027.sh
2020-04-24T10:54:49.2944302Z 
2020-04-24T10:54:49.2948442Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-24T10:54:49.2970014Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71508/merge to s
2020-04-24T10:54:49.2973659Z Task         : Get sources
2020-04-24T10:54:49.2973948Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-24T10:54:49.2974226Z Version      : 1.0.0
2020-04-24T10:54:49.2974416Z Author       : Microsoft
---
2020-04-24T10:54:50.2843746Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-24T10:54:50.2850087Z ##[command]git config gc.auto 0
2020-04-24T10:54:50.2854234Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-24T10:54:50.2857485Z ##[command]git config --get-all http.proxy
2020-04-24T10:54:50.2863885Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71508/merge:refs/remotes/pull/71508/merge
---
2020-04-24T10:57:52.1874244Z  ---> f7353ccad5b1
2020-04-24T10:57:52.1874502Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-24T10:57:52.1876367Z  ---> Using cache
2020-04-24T10:57:52.1876771Z  ---> ed38efbaa060
2020-04-24T10:57:52.1910449Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-24T10:57:52.1911816Z  ---> c5008ef7ae8e
2020-04-24T10:57:52.1912017Z Successfully built c5008ef7ae8e
2020-04-24T10:57:52.1951531Z Successfully tagged rust-ci:latest
2020-04-24T10:57:52.2239378Z Built container sha256:c5008ef7ae8e94d7ef502e3cef26e61208e14ebdb36913f3a8bb86291bd6430b
2020-04-24T10:57:52.2239378Z Built container sha256:c5008ef7ae8e94d7ef502e3cef26e61208e14ebdb36913f3a8bb86291bd6430b
2020-04-24T10:57:52.2253893Z Looks like docker image is the same as before, not uploading
2020-04-24T10:57:58.7344007Z [CI_JOB_NAME=mingw-check]
2020-04-24T10:57:58.7588091Z [CI_JOB_NAME=mingw-check]
2020-04-24T10:57:58.7622588Z == clock drift check ==
2020-04-24T10:57:58.7628988Z   local time: Fri Apr 24 10:57:58 UTC 2020
2020-04-24T10:57:58.9246770Z   network time: Fri, 24 Apr 2020 10:57:58 GMT
2020-04-24T10:57:58.9270275Z Starting sccache server...
2020-04-24T10:57:59.0469189Z configure: processing command line
2020-04-24T10:57:59.0469967Z configure: 
2020-04-24T10:57:59.0471286Z configure: rust.parallel-compiler := True
---
2020-04-24T11:01:45.9674982Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-24T11:01:46.1412563Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-24T11:01:46.3420544Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-24T11:01:46.3849741Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-24T11:01:46.9981374Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-24T11:01:49.3945182Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-24T11:01:49.9188278Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-24T11:01:51.9442097Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-24T11:01:52.4004468Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-24T11:03:42.9419048Z configure: dist.missing-tools   := True
2020-04-24T11:03:42.9419514Z configure: build.submodules     := False
2020-04-24T11:03:42.9420152Z configure: build.cargo-native-static := True
2020-04-24T11:03:42.9420812Z configure: build.locked-deps    := True
2020-04-24T11:03:42.9421575Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-24T11:03:42.9422434Z configure: writing `config.toml` in current directory
2020-04-24T11:03:42.9422808Z configure: 
2020-04-24T11:03:42.9423383Z configure: run `python /checkout/x.py --help`
2020-04-24T11:03:42.9424017Z configure: 
---
2020-04-24T11:05:14.4273492Z Hugepagesize:       2048 kB
2020-04-24T11:05:14.4273695Z DirectMap4k:      147392 kB
2020-04-24T11:05:14.4273893Z DirectMap2M:     4046848 kB
2020-04-24T11:05:14.4274089Z DirectMap1G:     5242880 kB
2020-04-24T11:05:14.4301331Z + python3 ../x.py test src/tools/expand-yaml-anchors
2020-04-24T11:05:15.8505265Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-24T11:05:15.8505265Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-24T11:05:15.8538514Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-24T11:05:16.0941043Z    Compiling unicode-xid v0.2.0
2020-04-24T11:05:16.2426663Z    Compiling syn v1.0.11
2020-04-24T11:05:17.1257503Z    Compiling linked-hash-map v0.5.2
2020-04-24T11:05:17.1599666Z    Compiling lazy_static v1.4.0
2020-04-24T11:05:17.1599666Z    Compiling lazy_static v1.4.0
2020-04-24T11:05:17.3703909Z    Compiling yaml-rust v0.4.3
2020-04-24T11:05:21.7681997Z    Compiling quote v1.0.2
2020-04-24T11:05:36.3257176Z    Compiling thiserror-impl v1.0.5
2020-04-24T11:05:40.9988336Z    Compiling thiserror v1.0.5
2020-04-24T11:05:41.0628598Z    Compiling yaml-merge-keys v0.4.0
2020-04-24T11:05:42.2411476Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-24T11:05:44.0477302Z Build completed successfully in 0:00:29
2020-04-24T11:05:44.0577528Z + python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-24T11:05:44.3271671Z     Finished dev [unoptimized] target(s) in 0.17s
2020-04-24T11:05:45.4525252Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-24T11:07:50.8558136Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-24T11:07:50.9947420Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-24T11:07:51.2048196Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-24T11:07:51.2637017Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-24T11:07:51.8980178Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-24T11:07:54.2739745Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-24T11:07:54.7811110Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-24T11:07:57.5170205Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-24T11:07:57.5194356Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-24T11:12:02.0517378Z skip untracked path cpu-usage.csv during rustfmt invocations
2020-04-24T11:12:02.0517817Z skip untracked path src/doc/book/ during rustfmt invocations
2020-04-24T11:12:02.0518432Z skip untracked path src/doc/rust-by-example/ during rustfmt invocations
2020-04-24T11:12:02.0522834Z skip untracked path src/llvm-project/ during rustfmt invocations
2020-04-24T11:12:04.9702986Z Diff in /checkout/src/librustc_mir/const_eval/eval_queries.rs at line 200:
2020-04-24T11:12:04.9703338Z          // whether they become immediates.
2020-04-24T11:12:04.9703611Z          if is_static || cid.promoted.is_some() {
2020-04-24T11:12:04.9703864Z              let ptr = mplace.ptr.assert_ptr();
2020-04-24T11:12:04.9704759Z -            Ok(ConstValue::ByRef {
2020-04-24T11:12:04.9705268Z -                alloc: ecx.tcx.unwrap_memory(ptr.alloc_id),
2020-04-24T11:12:04.9705716Z -                offset: ptr.offset,
2020-04-24T11:12:04.9706065Z -            })
2020-04-24T11:12:04.9706434Z +            Ok(ConstValue::ByRef { alloc: ecx.tcx.unwrap_memory(ptr.alloc_id), offset: ptr.offset })
2020-04-24T11:12:04.9706763Z          } else {
2020-04-24T11:12:04.9706974Z              Ok(op_to_const(&ecx, mplace.into()))
2020-04-24T11:12:04.9707186Z          }
2020-04-24T11:12:04.9709121Z Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustc_mir/const_eval/eval_queries.rs"` failed.
2020-04-24T11:12:04.9710048Z If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
2020-04-24T11:12:04.9710793Z Build completed unsuccessfully in 0:00:40
2020-04-24T11:12:04.9742212Z == clock drift check ==
2020-04-24T11:12:04.9742212Z == clock drift check ==
2020-04-24T11:12:04.9756528Z   local time: Fri Apr 24 11:12:04 UTC 2020
2020-04-24T11:12:05.2642363Z   network time: Fri, 24 Apr 2020 11:12:05 GMT
2020-04-24T11:12:06.7629548Z 
2020-04-24T11:12:06.7629548Z 
2020-04-24T11:12:06.7700603Z ##[error]Bash exited with code '1'.
2020-04-24T11:12:06.7714542Z ##[section]Finishing: Run build
2020-04-24T11:12:06.7760514Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71508/merge to s
2020-04-24T11:12:06.7765365Z Task         : Get sources
2020-04-24T11:12:06.7765674Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-24T11:12:06.7765953Z Version      : 1.0.0
2020-04-24T11:12:06.7766171Z Author       : Microsoft
2020-04-24T11:12:06.7766171Z Author       : Microsoft
2020-04-24T11:12:06.7766491Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-24T11:12:06.7766850Z ==============================================================================
2020-04-24T11:12:07.1317040Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-24T11:12:07.1366730Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71508/merge to s
2020-04-24T11:12:07.1462876Z Cleaning up task key
2020-04-24T11:12:07.1464198Z Start cleaning up orphan processes.
2020-04-24T11:12:07.1661646Z Terminate orphan process: pid (3639) (python)
2020-04-24T11:12:07.1811632Z ##[section]Finishing: Finalize Job
