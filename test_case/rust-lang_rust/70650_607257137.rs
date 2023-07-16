plain
2020-04-01T11:58:44.5964634Z ========================== Starting Command Output ===========================
2020-04-01T11:58:44.5967998Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/cb56fdb1-35ab-4fc8-8071-7c301fb977b2.sh
2020-04-01T11:58:44.5968422Z 
2020-04-01T11:58:44.5972728Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-01T11:58:44.5992664Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70650/merge to s
2020-04-01T11:58:44.5996338Z Task         : Get sources
2020-04-01T11:58:44.5996642Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-01T11:58:44.5996952Z Version      : 1.0.0
2020-04-01T11:58:44.5997152Z Author       : Microsoft
---
2020-04-01T11:58:45.5955823Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-01T11:58:45.5961277Z ##[command]git config gc.auto 0
2020-04-01T11:58:45.5965690Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-01T11:58:45.5970909Z ##[command]git config --get-all http.proxy
2020-04-01T11:58:45.5979680Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70650/merge:refs/remotes/pull/70650/merge
---
2020-04-01T12:01:15.3840977Z  ---> 3fc1b512c57b
2020-04-01T12:01:15.3841356Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-01T12:01:15.3849677Z  ---> Using cache
2020-04-01T12:01:15.3850190Z  ---> 5ee4295733f4
2020-04-01T12:01:15.3851605Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-01T12:01:15.3853046Z  ---> 3d07a0fa42fe
2020-04-01T12:01:15.3883320Z Successfully built 3d07a0fa42fe
2020-04-01T12:01:15.3904755Z Successfully tagged rust-ci:latest
2020-04-01T12:01:15.4272451Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-01T12:01:15.4272451Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-01T12:01:15.4287050Z Looks like docker image is the same as before, not uploading
2020-04-01T12:01:22.1887716Z [CI_JOB_NAME=mingw-check]
2020-04-01T12:01:22.2083115Z [CI_JOB_NAME=mingw-check]
2020-04-01T12:01:22.2113369Z == clock drift check ==
2020-04-01T12:01:22.2118479Z   local time: Wed Apr  1 12:01:22 UTC 2020
2020-04-01T12:01:22.2721189Z   network time: Wed, 01 Apr 2020 12:01:22 GMT
2020-04-01T12:01:22.2744998Z Starting sccache server...
2020-04-01T12:01:22.3575194Z configure: processing command line
2020-04-01T12:01:22.3575884Z configure: 
2020-04-01T12:01:22.3576932Z configure: rust.parallel-compiler := True
---
2020-04-01T12:04:41.6760620Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-01T12:04:41.7058070Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-01T12:04:41.8851834Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-01T12:04:42.0625435Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-01T12:04:42.4589558Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-01T12:04:44.5860077Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-01T12:04:45.0406421Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-01T12:04:46.9754744Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-01T12:04:47.3784159Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-01T12:06:24.4537467Z configure: build.locked-deps    := True
2020-04-01T12:06:24.4537959Z configure: llvm.ccache          := sccache
2020-04-01T12:06:24.4538642Z configure: build.cargo-native-static := True
2020-04-01T12:06:24.4539319Z configure: dist.missing-tools   := True
2020-04-01T12:06:24.4540112Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-01T12:06:24.4541046Z configure: writing `config.toml` in current directory
2020-04-01T12:06:24.4541452Z configure: 
2020-04-01T12:06:24.4542054Z configure: run `python /checkout/x.py --help`
2020-04-01T12:06:24.4542468Z configure: 
---
2020-04-01T12:07:39.0601660Z Hugepagesize:       2048 kB
2020-04-01T12:07:39.0601903Z DirectMap4k:      135104 kB
2020-04-01T12:07:39.0602127Z DirectMap2M:     5107712 kB
2020-04-01T12:07:39.0602351Z DirectMap1G:     4194304 kB
2020-04-01T12:07:39.0632643Z + python2.7 ../x.py test src/tools/expand-yaml-anchors
2020-04-01T12:07:40.2698479Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-01T12:07:40.2698479Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-01T12:07:40.2705059Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-01T12:07:40.4874404Z    Compiling unicode-xid v0.2.0
2020-04-01T12:07:40.5985115Z    Compiling syn v1.0.11
2020-04-01T12:07:41.3231819Z    Compiling linked-hash-map v0.5.2
2020-04-01T12:07:41.3564460Z    Compiling lazy_static v1.4.0
2020-04-01T12:07:41.3564460Z    Compiling lazy_static v1.4.0
2020-04-01T12:07:41.5206919Z    Compiling yaml-rust v0.4.3
2020-04-01T12:07:45.4224962Z    Compiling quote v1.0.2
2020-04-01T12:07:58.4025252Z    Compiling thiserror-impl v1.0.5
2020-04-01T12:08:02.7510150Z    Compiling thiserror v1.0.5
2020-04-01T12:08:02.8104391Z    Compiling yaml-merge-keys v0.4.0
2020-04-01T12:08:03.8743719Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-01T12:08:05.9001514Z Build completed successfully in 0:00:26
2020-04-01T12:08:05.9012317Z + python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-01T12:08:06.1282737Z     Finished dev [unoptimized] target(s) in 0.16s
2020-04-01T12:08:07.1330865Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-01T12:09:57.1322778Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-01T12:09:57.3707981Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-01T12:09:57.5164464Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-01T12:09:57.5533956Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-01T12:09:58.0430987Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-01T12:10:00.0630011Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-01T12:10:00.5091820Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-01T12:10:02.3636027Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-01T12:10:02.7962276Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-01T12:13:32.8171164Z Build completed successfully in 0:00:37
2020-04-01T12:13:32.8182196Z + /scripts/validate-toolstate.sh
2020-04-01T12:13:32.8231043Z Cloning into 'rust-toolstate'...
2020-04-01T12:13:33.4665495Z Traceback (most recent call last):
2020-04-01T12:13:33.4666481Z   File "../../src/tools/publish_toolstate.py", line 300, in <module>
2020-04-01T12:13:33.4667098Z     cur_datetime
2020-04-01T12:13:33.4667654Z   File "../../src/tools/publish_toolstate.py", line 200, in update_latest
2020-04-01T12:13:33.4669025Z     maintainers = ' '.join('@'+name for name in MAINTAINERS[tool])
2020-04-01T12:13:33.4669845Z KeyError: u'clippy-driver'
2020-04-01T12:13:33.4710455Z   local time: Wed Apr  1 12:13:33 UTC 2020
2020-04-01T12:13:33.4710455Z   local time: Wed Apr  1 12:13:33 UTC 2020
2020-04-01T12:13:33.5143697Z   network time: Wed, 01 Apr 2020 12:13:33 GMT
2020-04-01T12:13:35.0364787Z 
2020-04-01T12:13:35.0364787Z 
2020-04-01T12:13:35.0433846Z ##[error]Bash exited with code '1'.
2020-04-01T12:13:35.0444959Z ##[section]Finishing: Run build
2020-04-01T12:13:35.0496710Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70650/merge to s
2020-04-01T12:13:35.0501780Z Task         : Get sources
2020-04-01T12:13:35.0502165Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-01T12:13:35.0502490Z Version      : 1.0.0
2020-04-01T12:13:35.0502722Z Author       : Microsoft
2020-04-01T12:13:35.0502722Z Author       : Microsoft
2020-04-01T12:13:35.0503097Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-01T12:13:35.0503518Z ==============================================================================
2020-04-01T12:13:35.3495792Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-01T12:13:35.3543230Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70650/merge to s
2020-04-01T12:13:35.3625787Z Cleaning up task key
2020-04-01T12:13:35.3627129Z Start cleaning up orphan processes.
2020-04-01T12:13:35.3786284Z Terminate orphan process: pid (3783) (python)
2020-04-01T12:13:35.3987429Z ##[section]Finishing: Finalize Job
