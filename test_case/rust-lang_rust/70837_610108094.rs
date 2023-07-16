plain
2020-04-06T22:59:36.5396765Z ========================== Starting Command Output ===========================
2020-04-06T22:59:36.5400189Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/c1184397-a608-4c59-934d-b6f1b16d30a1.sh
2020-04-06T22:59:36.5400634Z 
2020-04-06T22:59:36.5405400Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-06T22:59:36.5426492Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70837/merge to s
2020-04-06T22:59:36.5430052Z Task         : Get sources
2020-04-06T22:59:36.5430379Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-06T22:59:36.5430693Z Version      : 1.0.0
2020-04-06T22:59:36.5430905Z Author       : Microsoft
---
2020-04-06T22:59:37.5311186Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-06T22:59:37.5316534Z ##[command]git config gc.auto 0
2020-04-06T22:59:37.5320271Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-06T22:59:37.5323935Z ##[command]git config --get-all http.proxy
2020-04-06T22:59:37.5330015Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70837/merge:refs/remotes/pull/70837/merge
---
2020-04-06T23:01:53.7882308Z  ---> 3fc1b512c57b
2020-04-06T23:01:53.7882790Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-06T23:01:53.7883493Z  ---> Using cache
2020-04-06T23:01:53.7884078Z  ---> 5ee4295733f4
2020-04-06T23:01:53.7885898Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-06T23:01:53.7888127Z  ---> 3d07a0fa42fe
2020-04-06T23:01:53.7956665Z Successfully built 3d07a0fa42fe
2020-04-06T23:01:53.7998635Z Successfully tagged rust-ci:latest
2020-04-06T23:01:53.8278068Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-06T23:01:53.8278068Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-06T23:01:53.8293944Z Looks like docker image is the same as before, not uploading
2020-04-06T23:02:02.0352862Z [CI_JOB_NAME=mingw-check]
2020-04-06T23:02:02.0604106Z [CI_JOB_NAME=mingw-check]
2020-04-06T23:02:02.0630836Z == clock drift check ==
2020-04-06T23:02:02.0642761Z   local time: Mon Apr  6 23:02:02 UTC 2020
2020-04-06T23:02:02.3541536Z   network time: Mon, 06 Apr 2020 23:02:02 GMT
2020-04-06T23:02:02.3572274Z Starting sccache server...
2020-04-06T23:02:02.4522685Z configure: processing command line
2020-04-06T23:02:02.4523866Z configure: 
2020-04-06T23:02:02.4525307Z configure: rust.parallel-compiler := True
---
2020-04-06T23:05:43.4908047Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-06T23:05:43.6509136Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-06T23:05:43.8502348Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-06T23:05:43.9146897Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-06T23:05:44.4569727Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-06T23:05:46.7568262Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-06T23:05:47.2505775Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-06T23:05:49.3046871Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-06T23:05:49.7542602Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-06T23:07:35.8029876Z configure: build.locked-deps    := True
2020-04-06T23:07:35.8030188Z configure: llvm.ccache          := sccache
2020-04-06T23:07:35.8030696Z configure: build.cargo-native-static := True
2020-04-06T23:07:35.8031184Z configure: dist.missing-tools   := True
2020-04-06T23:07:35.8031789Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-06T23:07:35.8032378Z configure: writing `config.toml` in current directory
2020-04-06T23:07:35.8032621Z configure: 
2020-04-06T23:07:35.8033062Z configure: run `python /checkout/x.py --help`
2020-04-06T23:07:35.8033296Z configure: 
---
2020-04-06T23:09:08.8584908Z Hugepagesize:       2048 kB
2020-04-06T23:09:08.8585151Z DirectMap4k:      131008 kB
2020-04-06T23:09:08.8585376Z DirectMap2M:     4063232 kB
2020-04-06T23:09:08.8585603Z DirectMap1G:     5242880 kB
2020-04-06T23:09:08.8684926Z + python2.7 ../x.py test src/tools/expand-yaml-anchors
2020-04-06T23:09:10.2529102Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-06T23:09:10.2529102Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-06T23:09:10.2536910Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-06T23:09:10.4931235Z    Compiling unicode-xid v0.2.0
2020-04-06T23:09:10.6200770Z    Compiling syn v1.0.11
2020-04-06T23:09:11.4486141Z    Compiling linked-hash-map v0.5.2
2020-04-06T23:09:11.4917404Z    Compiling lazy_static v1.4.0
2020-04-06T23:09:11.4917404Z    Compiling lazy_static v1.4.0
2020-04-06T23:09:11.6680965Z    Compiling yaml-rust v0.4.3
2020-04-06T23:09:15.9263287Z    Compiling quote v1.0.2
2020-04-06T23:09:30.3278612Z    Compiling thiserror-impl v1.0.5
2020-04-06T23:09:35.0780398Z    Compiling thiserror v1.0.5
2020-04-06T23:09:35.1421612Z    Compiling yaml-merge-keys v0.4.0
2020-04-06T23:09:36.3297976Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-06T23:09:40.9709752Z Build completed successfully in 0:00:32
2020-04-06T23:09:40.9721197Z + python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-06T23:09:41.2183687Z     Finished dev [unoptimized] target(s) in 0.18s
2020-04-06T23:09:42.3539922Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-06T23:11:47.4100861Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-06T23:11:47.5505329Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-06T23:11:47.7567188Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-06T23:11:47.8225019Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-06T23:11:48.3701288Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-06T23:11:50.6185391Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-06T23:11:51.1179207Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-06T23:11:53.1828159Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-06T23:11:53.6266722Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-06T23:15:53.9985031Z skip untracked path cpu-usage.csv during rustfmt invocations
2020-04-06T23:15:53.9986532Z skip untracked path src/doc/book/ during rustfmt invocations
2020-04-06T23:15:53.9988870Z skip untracked path src/doc/rust-by-example/ during rustfmt invocations
2020-04-06T23:15:53.9989487Z skip untracked path src/llvm-project/ during rustfmt invocations
2020-04-06T23:15:54.0619254Z Diff in /checkout/src/librustc_metadata/locator.rs at line 556:
2020-04-06T23:15:54.0620526Z                  (&file[(dylib_prefix.len())..(file.len() - dypair.1.len())], CrateFlavor::Dylib)
2020-04-06T23:15:54.0620915Z              } else {
2020-04-06T23:15:54.0621260Z                  if file.starts_with(&staticlib_prefix) && file.ends_with(&staticpair.1) {
2020-04-06T23:15:54.0621852Z -                    staticlibs.push(CrateMismatch {
2020-04-06T23:15:54.0622620Z -                        path: spf.path.clone(),
2020-04-06T23:15:54.0623160Z -                        got: "static".to_string(),
2020-04-06T23:15:54.0623825Z +                    staticlibs
2020-04-06T23:15:54.0623825Z +                    staticlibs
2020-04-06T23:15:54.0624234Z +                        .push(CrateMismatch { path: spf.path.clone(), got: "static".to_string() });
2020-04-06T23:15:54.0624600Z                  }
2020-04-06T23:15:54.0624823Z                  return FileDoesntMatch;
2020-04-06T23:15:54.0625145Z              };
2020-04-06T23:15:54.0626085Z Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustc_metadata/locator.rs"` failed.
2020-04-06T23:15:54.0627092Z If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
2020-04-06T23:15:54.0627942Z Build completed unsuccessfully in 0:00:34
2020-04-06T23:15:54.0672586Z == clock drift check ==
2020-04-06T23:15:54.0691692Z   local time: Mon Apr  6 23:15:54 UTC 2020
2020-04-06T23:15:54.0691692Z   local time: Mon Apr  6 23:15:54 UTC 2020
2020-04-06T23:15:54.3570984Z   network time: Mon, 06 Apr 2020 23:15:54 GMT
2020-04-06T23:15:55.8983374Z 
2020-04-06T23:15:55.8983374Z 
2020-04-06T23:15:55.9052961Z ##[error]Bash exited with code '1'.
2020-04-06T23:15:55.9067103Z ##[section]Finishing: Run build
2020-04-06T23:15:55.9120980Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70837/merge to s
2020-04-06T23:15:55.9126435Z Task         : Get sources
2020-04-06T23:15:55.9126806Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-06T23:15:55.9127128Z Version      : 1.0.0
2020-04-06T23:15:55.9127721Z Author       : Microsoft
2020-04-06T23:15:55.9127721Z Author       : Microsoft
2020-04-06T23:15:55.9128118Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-06T23:15:55.9128908Z ==============================================================================
2020-04-06T23:15:56.2422056Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-06T23:15:56.2468177Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70837/merge to s
2020-04-06T23:15:56.2570945Z Cleaning up task key
2020-04-06T23:15:56.2572225Z Start cleaning up orphan processes.
2020-04-06T23:15:56.2770220Z Terminate orphan process: pid (3793) (python)
2020-04-06T23:15:56.3050791Z ##[section]Finishing: Finalize Job
