plain
2020-04-17T10:47:47.5575101Z ========================== Starting Command Output ===========================
2020-04-17T10:47:47.5579097Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/419715fd-35b4-45e8-9e1e-5a1d7eecc66c.sh
2020-04-17T10:47:47.5579547Z 
2020-04-17T10:47:47.5583376Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-17T10:47:47.5604615Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70073/merge to s
2020-04-17T10:47:47.5608092Z Task         : Get sources
2020-04-17T10:47:47.5608365Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-17T10:47:47.5608628Z Version      : 1.0.0
2020-04-17T10:47:47.5608807Z Author       : Microsoft
---
2020-04-17T10:47:48.5500221Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-17T10:47:48.5508975Z ##[command]git config gc.auto 0
2020-04-17T10:47:48.5515258Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-17T10:47:48.5521316Z ##[command]git config --get-all http.proxy
2020-04-17T10:47:48.5529850Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70073/merge:refs/remotes/pull/70073/merge
---
2020-04-17T10:50:08.5347439Z  ---> 78ad2f4d4aca
2020-04-17T10:50:08.5348173Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-17T10:50:08.5359947Z  ---> Using cache
2020-04-17T10:50:08.5360837Z  ---> 4d2dc61c4d00
2020-04-17T10:50:08.5362439Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-17T10:50:08.5368514Z  ---> 776b6266a8b7
2020-04-17T10:50:08.5387803Z Successfully built 776b6266a8b7
2020-04-17T10:50:08.5413259Z Successfully tagged rust-ci:latest
2020-04-17T10:50:08.5720549Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-17T10:50:08.5720549Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-17T10:50:08.5736895Z Looks like docker image is the same as before, not uploading
2020-04-17T10:50:13.9607703Z [CI_JOB_NAME=mingw-check]
2020-04-17T10:50:13.9885460Z [CI_JOB_NAME=mingw-check]
2020-04-17T10:50:13.9933991Z == clock drift check ==
2020-04-17T10:50:13.9946696Z   local time: Fri Apr 17 10:50:13 UTC 2020
2020-04-17T10:50:14.1040169Z   network time: Fri, 17 Apr 2020 10:50:14 GMT
2020-04-17T10:50:14.1067080Z Starting sccache server...
2020-04-17T10:50:14.2198528Z configure: processing command line
2020-04-17T10:50:14.2198783Z configure: 
2020-04-17T10:50:14.2199754Z configure: rust.parallel-compiler := True
---
2020-04-17T10:54:14.8578654Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-17T10:54:14.9056883Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-17T10:54:15.1230475Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-17T10:54:15.3231718Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-17T10:54:15.7913940Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-17T10:54:18.3920075Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-17T10:54:18.9148208Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-17T10:54:21.1989078Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-17T10:54:21.6764998Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-17T10:56:21.5076314Z configure: llvm.assertions      := True
2020-04-17T10:56:21.5076859Z configure: llvm.ccache          := sccache
2020-04-17T10:56:21.5077650Z configure: rust.channel         := nightly
2020-04-17T10:56:21.5078243Z configure: build.locked-deps    := True
2020-04-17T10:56:21.5078967Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-17T10:56:21.5079691Z configure: writing `config.toml` in current directory
2020-04-17T10:56:21.5080024Z configure: 
2020-04-17T10:56:21.5080524Z configure: run `python /checkout/x.py --help`
2020-04-17T10:56:21.5080850Z configure: 
---
2020-04-17T10:58:02.7927466Z Hugepagesize:       2048 kB
2020-04-17T10:58:02.7927662Z DirectMap4k:      104384 kB
2020-04-17T10:58:02.7927923Z DirectMap2M:     3041280 kB
2020-04-17T10:58:02.7928125Z DirectMap1G:     6291456 kB
2020-04-17T10:58:02.7941941Z + python3 ../x.py test src/tools/expand-yaml-anchors
2020-04-17T10:58:04.2783218Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-17T10:58:04.2783218Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-17T10:58:04.2794061Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-17T10:58:04.5273825Z    Compiling unicode-xid v0.2.0
2020-04-17T10:58:04.6729969Z    Compiling syn v1.0.11
2020-04-17T10:58:05.6038423Z    Compiling linked-hash-map v0.5.2
2020-04-17T10:58:05.6284710Z    Compiling lazy_static v1.4.0
2020-04-17T10:58:05.6284710Z    Compiling lazy_static v1.4.0
2020-04-17T10:58:05.8656703Z    Compiling yaml-rust v0.4.3
2020-04-17T10:58:10.6417617Z    Compiling quote v1.0.2
2020-04-17T10:58:26.4712107Z    Compiling thiserror-impl v1.0.5
2020-04-17T10:58:31.6943703Z    Compiling thiserror v1.0.5
2020-04-17T10:58:31.7567499Z    Compiling yaml-merge-keys v0.4.0
2020-04-17T10:58:33.0947757Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-17T10:58:34.8546509Z Build completed successfully in 0:00:32
2020-04-17T10:58:34.8639529Z + python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-17T10:58:35.1517438Z     Finished dev [unoptimized] target(s) in 0.19s
2020-04-17T10:58:36.3293728Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-17T11:00:53.4088898Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-17T11:00:53.4652349Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-17T11:00:53.6764283Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-17T11:00:53.8488616Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-17T11:00:54.3275807Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-17T11:00:56.8334215Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-17T11:00:57.3623526Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-17T11:00:59.6145428Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-17T11:01:00.0929766Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-17T11:05:17.6192074Z skip untracked path cpu-usage.csv during rustfmt invocations
2020-04-17T11:05:17.6198755Z skip untracked path src/doc/book/ during rustfmt invocations
2020-04-17T11:05:17.6199492Z skip untracked path src/doc/rust-by-example/ during rustfmt invocations
2020-04-17T11:05:17.6200008Z skip untracked path src/llvm-project/ during rustfmt invocations
2020-04-17T11:05:20.3236756Z Diff in /checkout/src/librustc_mir/transform/mod.rs at line 230:
2020-04-17T11:05:20.3237096Z              // What we need to do constant evaluation.
2020-04-17T11:05:20.3237373Z              &simplify::SimplifyCfg::new("initial"),
2020-04-17T11:05:20.3237867Z              &rustc_peek::SanityCheck,
2020-04-17T11:05:20.3238697Z -        ]]
2020-04-17T11:05:20.3238852Z +        ]],
2020-04-17T11:05:20.3239188Z      body.ensure_predecessors();
2020-04-17T11:05:20.3239188Z      body.ensure_predecessors();
2020-04-17T11:05:20.3239394Z      tcx.alloc_steal_mir(body)
2020-04-17T11:05:20.3239663Z Diff in /checkout/src/librustc_mir/transform/mod.rs at line 315:
2020-04-17T11:05:20.3239968Z          &simplify::SimplifyLocals,
2020-04-17T11:05:20.3240184Z      ];
2020-04-17T11:05:20.3240307Z  
2020-04-17T11:05:20.3240759Z -    let pre_codegen_cleanup: &[&dyn MirPass<'tcx>] = &[
2020-04-17T11:05:20.3241239Z -        &add_call_guards::CriticalCallEdges,
2020-04-17T11:05:20.3241674Z -        &dump_mir::Marker("PreCodegen"),
2020-04-17T11:05:20.3242025Z -    ];
2020-04-17T11:05:20.3242422Z +    let pre_codegen_cleanup: &[&dyn MirPass<'tcx>] =
2020-04-17T11:05:20.3242784Z +        &[&add_call_guards::CriticalCallEdges, &dump_mir::Marker("PreCodegen")];
2020-04-17T11:05:20.3243069Z  
2020-04-17T11:05:20.3243285Z      let mir_opt_level = tcx.sess.opts.debugging_opts.mir_opt_level;
2020-04-17T11:05:20.3243658Z  
2020-04-17T11:05:20.3259700Z Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustc_mir/transform/mod.rs"` failed.
2020-04-17T11:05:20.3265723Z If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
2020-04-17T11:05:20.3274367Z Build completed unsuccessfully in 0:00:41
2020-04-17T11:05:20.3379474Z == clock drift check ==
2020-04-17T11:05:20.3379474Z == clock drift check ==
2020-04-17T11:05:20.3413774Z   local time: Fri Apr 17 11:05:20 UTC 2020
2020-04-17T11:05:20.6469172Z   network time: Fri, 17 Apr 2020 11:05:20 GMT
2020-04-17T11:05:22.1843557Z 
2020-04-17T11:05:22.1843557Z 
2020-04-17T11:05:22.1922373Z ##[error]Bash exited with code '1'.
2020-04-17T11:05:22.1935837Z ##[section]Finishing: Run build
2020-04-17T11:05:22.1982369Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70073/merge to s
2020-04-17T11:05:22.1987332Z Task         : Get sources
2020-04-17T11:05:22.1987658Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-17T11:05:22.1987938Z Version      : 1.0.0
2020-04-17T11:05:22.1988139Z Author       : Microsoft
2020-04-17T11:05:22.1988139Z Author       : Microsoft
2020-04-17T11:05:22.1988472Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-17T11:05:22.1989165Z ==============================================================================
2020-04-17T11:05:22.5566454Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-17T11:05:22.5608471Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70073/merge to s
2020-04-17T11:05:22.5703134Z Cleaning up task key
2020-04-17T11:05:22.5704341Z Start cleaning up orphan processes.
2020-04-17T11:05:22.5927199Z Terminate orphan process: pid (3457) (python)
2020-04-17T11:05:22.6173878Z ##[section]Finishing: Finalize Job
