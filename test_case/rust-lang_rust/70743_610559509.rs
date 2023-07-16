plain
2020-04-07T16:53:14.9509686Z ========================== Starting Command Output ===========================
2020-04-07T16:53:14.9531050Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/385aafe0-6f15-4acc-a923-dc8d3e046e65.sh
2020-04-07T16:53:15.4693154Z 
2020-04-07T16:53:15.4741666Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-07T16:53:15.4762682Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70743/merge to s
2020-04-07T16:53:15.4767549Z Task         : Get sources
2020-04-07T16:53:15.4767833Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-07T16:53:15.4768113Z Version      : 1.0.0
2020-04-07T16:53:15.4768307Z Author       : Microsoft
---
2020-04-07T16:53:21.2527347Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-07T16:53:21.2539266Z ##[command]git config gc.auto 0
2020-04-07T16:53:21.2543428Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-07T16:53:21.2546819Z ##[command]git config --get-all http.proxy
2020-04-07T16:53:21.2555109Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70743/merge:refs/remotes/pull/70743/merge
---
2020-04-07T16:59:12.3564037Z  ---> 3fc1b512c57b
2020-04-07T16:59:12.3564286Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-07T16:59:12.3564715Z  ---> Using cache
2020-04-07T16:59:12.3565071Z  ---> 5ee4295733f4
2020-04-07T16:59:12.3566512Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-07T16:59:12.3567959Z  ---> 3d07a0fa42fe
2020-04-07T16:59:12.3568196Z Successfully built 3d07a0fa42fe
2020-04-07T16:59:12.3568637Z Successfully tagged rust-ci:latest
2020-04-07T16:59:12.3569019Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-07T16:59:12.3569019Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-07T16:59:12.3569445Z Looks like docker image is the same as before, not uploading
2020-04-07T16:59:16.0099016Z [CI_JOB_NAME=mingw-check]
2020-04-07T16:59:16.0351113Z [CI_JOB_NAME=mingw-check]
2020-04-07T16:59:16.0376885Z == clock drift check ==
2020-04-07T16:59:16.0385099Z   local time: Tue Apr  7 16:59:16 UTC 2020
2020-04-07T16:59:16.1036475Z   network time: Tue, 07 Apr 2020 16:59:16 GMT
2020-04-07T16:59:16.1059764Z Starting sccache server...
2020-04-07T16:59:16.1890961Z configure: processing command line
2020-04-07T16:59:16.1891349Z configure: 
2020-04-07T16:59:16.1892303Z configure: rust.parallel-compiler := True
---
2020-04-07T17:02:56.7468953Z     Checking rustc_span v0.0.0 (/checkout/src/librustc_span)
2020-04-07T17:03:01.5373732Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2020-04-07T17:03:02.8268641Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-07T17:03:03.0745606Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-07T17:03:03.2554887Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-07T17:03:03.8740846Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-07T17:03:04.1138287Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-07T17:03:05.6607867Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-07T17:03:06.1869118Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
---
2020-04-07T17:05:05.6556677Z configure: build.locked-deps    := True
2020-04-07T17:05:05.6557279Z configure: llvm.ccache          := sccache
2020-04-07T17:05:05.6557845Z configure: build.cargo-native-static := True
2020-04-07T17:05:05.6558342Z configure: dist.missing-tools   := True
2020-04-07T17:05:05.6559246Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-07T17:05:05.6559837Z configure: writing `config.toml` in current directory
2020-04-07T17:05:05.6560101Z configure: 
2020-04-07T17:05:05.6560540Z configure: run `python /checkout/x.py --help`
2020-04-07T17:05:05.6560774Z configure: 
---
2020-04-07T17:06:30.1101866Z Hugepagesize:       2048 kB
2020-04-07T17:06:30.1102254Z DirectMap4k:      153536 kB
2020-04-07T17:06:30.1102479Z DirectMap2M:     4040704 kB
2020-04-07T17:06:30.1102688Z DirectMap1G:     5242880 kB
2020-04-07T17:06:30.1120923Z + python2.7 ../x.py test src/tools/expand-yaml-anchors
2020-04-07T17:06:31.3297353Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-07T17:06:31.3297353Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-07T17:06:31.3307236Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-07T17:06:31.5539907Z    Compiling unicode-xid v0.2.0
2020-04-07T17:06:31.6674336Z    Compiling syn v1.0.11
2020-04-07T17:06:32.5316960Z    Compiling linked-hash-map v0.5.2
2020-04-07T17:06:32.5368199Z    Compiling lazy_static v1.4.0
2020-04-07T17:06:32.5368199Z    Compiling lazy_static v1.4.0
2020-04-07T17:06:32.7723498Z    Compiling yaml-rust v0.4.3
2020-04-07T17:06:37.3700193Z    Compiling quote v1.0.2
2020-04-07T17:06:52.6291406Z    Compiling thiserror-impl v1.0.5
2020-04-07T17:06:57.4661991Z    Compiling thiserror v1.0.5
2020-04-07T17:06:57.5315736Z    Compiling yaml-merge-keys v0.4.0
2020-04-07T17:06:58.5829679Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-07T17:07:00.9803450Z + python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-07T17:07:00.9803965Z Build completed successfully in 0:00:30
2020-04-07T17:07:01.2307038Z     Finished dev [unoptimized] target(s) in 0.18s
2020-04-07T17:07:02.3820770Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-07T17:09:19.8706312Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-07T17:09:20.1900399Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-07T17:09:20.2955566Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-07T17:09:20.4142735Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-07T17:09:20.9758798Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-07T17:09:23.4437804Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-07T17:09:23.9824392Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-07T17:09:26.3087392Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-07T17:09:26.7985683Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-07T17:13:25.4714116Z skip untracked path src/doc/rust-by-example/ during rustfmt invocations
2020-04-07T17:13:25.4715195Z skip untracked path src/llvm-project/ during rustfmt invocations
2020-04-07T17:13:31.4994278Z Diff in /checkout/src/librustc_mir_build/hair/pattern/const_to_pat.rs at line 316:
2020-04-07T17:13:31.4997569Z                  }
2020-04-07T17:13:31.5000476Z                  _ => unimplemented!("{}", cv.ty),
2020-04-07T17:13:31.5008866Z              },
2020-04-07T17:13:31.5012807Z -            ty::Bool | ty::Char | ty::Int(_) | ty::Uint(_) | ty::FnDef(..) => PatKind::Constant { value: cv },
2020-04-07T17:13:31.5017371Z +            ty::Bool | ty::Char | ty::Int(_) | ty::Uint(_) | ty::FnDef(..) => {
2020-04-07T17:13:31.5022430Z +                PatKind::Constant { value: cv }
2020-04-07T17:13:31.5028181Z              // FIXME: these can have very suprising behaviour where optimization levels or other
2020-04-07T17:13:31.5031115Z              // compilation choices change the runtime behaviour of the match.
2020-04-07T17:13:31.5031115Z              // compilation choices change the runtime behaviour of the match.
2020-04-07T17:13:31.5036431Z              // See ***/issues/70861 for examples.
2020-04-07T17:13:31.5048853Z Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustc_mir_build/hair/pattern/const_to_pat.rs"` failed.
2020-04-07T17:13:31.5051258Z If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
2020-04-07T17:13:31.5061174Z Build completed unsuccessfully in 0:00:40
2020-04-07T17:13:31.5103625Z == clock drift check ==
2020-04-07T17:13:31.5120068Z   local time: Tue Apr  7 17:13:31 UTC 2020
2020-04-07T17:13:31.5120068Z   local time: Tue Apr  7 17:13:31 UTC 2020
2020-04-07T17:13:31.7964091Z   network time: Tue, 07 Apr 2020 17:13:31 GMT
2020-04-07T17:13:33.6731580Z 
2020-04-07T17:13:33.6731580Z 
2020-04-07T17:13:33.6794783Z ##[error]Bash exited with code '1'.
2020-04-07T17:13:33.6806403Z ##[section]Finishing: Run build
2020-04-07T17:13:33.6844667Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70743/merge to s
2020-04-07T17:13:33.6848977Z Task         : Get sources
2020-04-07T17:13:33.6849253Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-07T17:13:33.6849512Z Version      : 1.0.0
2020-04-07T17:13:33.6849679Z Author       : Microsoft
2020-04-07T17:13:33.6849679Z Author       : Microsoft
2020-04-07T17:13:33.6849965Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-07T17:13:33.6850286Z ==============================================================================
2020-04-07T17:13:33.9821524Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-07T17:13:33.9871074Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70743/merge to s
2020-04-07T17:13:33.9947431Z Cleaning up task key
2020-04-07T17:13:33.9948403Z Start cleaning up orphan processes.
2020-04-07T17:13:34.0103251Z Terminate orphan process: pid (3828) (python)
2020-04-07T17:13:34.0587029Z ##[section]Finishing: Finalize Job
