plain
2020-04-04T18:10:10.4690901Z ========================== Starting Command Output ===========================
2020-04-04T18:10:10.4693548Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/aa96d745-e636-4877-a7fc-e7c7c377b5b0.sh
2020-04-04T18:10:10.4693863Z 
2020-04-04T18:10:10.4697933Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-04T18:10:10.4717114Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70782/merge to s
2020-04-04T18:10:10.4720359Z Task         : Get sources
2020-04-04T18:10:10.4720628Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-04T18:10:10.4720883Z Version      : 1.0.0
2020-04-04T18:10:10.4721055Z Author       : Microsoft
---
2020-04-04T18:10:11.4650093Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-04T18:10:11.4659168Z ##[command]git config gc.auto 0
2020-04-04T18:10:11.4668667Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-04T18:10:11.4676376Z ##[command]git config --get-all http.proxy
2020-04-04T18:10:11.4681911Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70782/merge:refs/remotes/pull/70782/merge
---
2020-04-04T18:12:44.4338976Z  ---> 3fc1b512c57b
2020-04-04T18:12:44.4339182Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-04T18:12:44.4339531Z  ---> Using cache
2020-04-04T18:12:44.4339823Z  ---> 5ee4295733f4
2020-04-04T18:12:44.4415430Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-04T18:12:44.4417194Z  ---> 3d07a0fa42fe
2020-04-04T18:12:44.4417456Z Successfully built 3d07a0fa42fe
2020-04-04T18:12:44.4463382Z Successfully tagged rust-ci:latest
2020-04-04T18:12:44.4963914Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-04T18:12:44.4963914Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-04T18:12:44.4978253Z Looks like docker image is the same as before, not uploading
2020-04-04T18:12:49.9257919Z [CI_JOB_NAME=mingw-check]
2020-04-04T18:12:49.9508910Z [CI_JOB_NAME=mingw-check]
2020-04-04T18:12:49.9529287Z == clock drift check ==
2020-04-04T18:12:49.9537240Z   local time: Sat Apr  4 18:12:49 UTC 2020
2020-04-04T18:12:50.2174602Z   network time: Sat, 04 Apr 2020 18:12:50 GMT
2020-04-04T18:12:50.2197337Z Starting sccache server...
2020-04-04T18:12:50.3066359Z configure: processing command line
2020-04-04T18:12:50.3067139Z configure: 
2020-04-04T18:12:50.3068044Z configure: rust.parallel-compiler := True
---
2020-04-04T18:16:26.1118501Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-04T18:16:26.1494155Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-04T18:16:26.3580840Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-04T18:16:26.5404071Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-04T18:16:26.9996301Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-04T18:16:29.3754522Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-04T18:16:29.8704673Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-04T18:16:31.9579164Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-04T18:16:32.4139178Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-04T18:18:19.1984738Z configure: build.locked-deps    := True
2020-04-04T18:18:19.1985215Z configure: llvm.ccache          := sccache
2020-04-04T18:18:19.1985829Z configure: build.cargo-native-static := True
2020-04-04T18:18:19.1986674Z configure: dist.missing-tools   := True
2020-04-04T18:18:19.1988321Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-04T18:18:19.1989396Z configure: writing `config.toml` in current directory
2020-04-04T18:18:19.1989916Z configure: 
2020-04-04T18:18:19.1991593Z configure: run `python /checkout/x.py --help`
2020-04-04T18:18:19.1991841Z configure: 
---
2020-04-04T18:19:40.3326370Z Hugepagesize:       2048 kB
2020-04-04T18:19:40.3326581Z DirectMap4k:       96192 kB
2020-04-04T18:19:40.3326774Z DirectMap2M:     4098048 kB
2020-04-04T18:19:40.3326967Z DirectMap1G:     5242880 kB
2020-04-04T18:19:40.3327580Z + python2.7 ../x.py test src/tools/expand-yaml-anchors
2020-04-04T18:19:41.6791856Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-04T18:19:41.6791856Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-04T18:19:41.6799818Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-04T18:19:41.9118797Z    Compiling unicode-xid v0.2.0
2020-04-04T18:19:42.0482416Z    Compiling syn v1.0.11
2020-04-04T18:19:42.9193298Z    Compiling linked-hash-map v0.5.2
2020-04-04T18:19:42.9415995Z    Compiling lazy_static v1.4.0
2020-04-04T18:19:42.9415995Z    Compiling lazy_static v1.4.0
2020-04-04T18:19:43.1585089Z    Compiling yaml-rust v0.4.3
2020-04-04T18:19:47.5189018Z    Compiling quote v1.0.2
2020-04-04T18:20:02.6113186Z    Compiling thiserror-impl v1.0.5
2020-04-04T18:20:07.1952889Z    Compiling thiserror v1.0.5
2020-04-04T18:20:07.2601534Z    Compiling yaml-merge-keys v0.4.0
2020-04-04T18:20:08.4441973Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-04T18:20:10.0647909Z Build completed successfully in 0:00:29
2020-04-04T18:20:10.0658414Z + python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-04T18:20:10.3106302Z     Finished dev [unoptimized] target(s) in 0.18s
2020-04-04T18:20:11.3741376Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-04T18:22:19.9875224Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-04T18:22:20.1370669Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-04T18:22:20.3426783Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-04T18:22:20.3990923Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-04T18:22:20.9896887Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-04T18:22:23.3498992Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-04T18:22:23.8458860Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-04T18:22:25.9987763Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-04T18:22:26.4578609Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-04T18:26:38.7942846Z Build completed successfully in 0:00:43
2020-04-04T18:26:38.7951267Z + /scripts/validate-toolstate.sh
2020-04-04T18:26:38.7995377Z Cloning into 'rust-toolstate'...
2020-04-04T18:26:39.4292797Z Traceback (most recent call last):
2020-04-04T18:26:39.4293138Z   File "../../src/tools/publish_toolstate.py", line 305, in <module>
2020-04-04T18:26:39.4293429Z     cur_datetime
2020-04-04T18:26:39.4293680Z   File "../../src/tools/publish_toolstate.py", line 205, in update_latest
2020-04-04T18:26:39.4294812Z     maintainers = ' '.join('@'+name for name in MAINTAINERS[tool])
2020-04-04T18:26:39.4295252Z KeyError: u'rustc-dev-guide'
2020-04-04T18:26:39.4336832Z   local time: Sat Apr  4 18:26:39 UTC 2020
2020-04-04T18:26:39.4336832Z   local time: Sat Apr  4 18:26:39 UTC 2020
2020-04-04T18:26:39.6221473Z   network time: Sat, 04 Apr 2020 18:26:39 GMT
2020-04-04T18:26:40.9596547Z 
2020-04-04T18:26:40.9596547Z 
2020-04-04T18:26:40.9664614Z ##[error]Bash exited with code '1'.
2020-04-04T18:26:40.9678252Z ##[section]Finishing: Run build
2020-04-04T18:26:40.9726411Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70782/merge to s
2020-04-04T18:26:40.9731249Z Task         : Get sources
2020-04-04T18:26:40.9731576Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-04T18:26:40.9731852Z Version      : 1.0.0
2020-04-04T18:26:40.9732046Z Author       : Microsoft
2020-04-04T18:26:40.9732046Z Author       : Microsoft
2020-04-04T18:26:40.9732377Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-04T18:26:40.9732732Z ==============================================================================
2020-04-04T18:26:41.3095266Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-04T18:26:41.3151304Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70782/merge to s
2020-04-04T18:26:41.3242349Z Cleaning up task key
2020-04-04T18:26:41.3243501Z Start cleaning up orphan processes.
2020-04-04T18:26:41.3467397Z Terminate orphan process: pid (3762) (python)
2020-04-04T18:26:41.3705627Z ##[section]Finishing: Finalize Job
