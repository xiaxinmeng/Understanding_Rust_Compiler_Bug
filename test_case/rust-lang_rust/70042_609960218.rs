plain
2020-04-06T16:38:17.3371138Z ========================== Starting Command Output ===========================
2020-04-06T16:38:17.3373782Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/6eda7669-b032-42e5-81b9-c8ae05583fb6.sh
2020-04-06T16:38:17.3374056Z 
2020-04-06T16:38:17.3378287Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-06T16:38:17.3397083Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70042/merge to s
2020-04-06T16:38:17.3400419Z Task         : Get sources
2020-04-06T16:38:17.3400730Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-06T16:38:17.3401029Z Version      : 1.0.0
2020-04-06T16:38:17.3401230Z Author       : Microsoft
---
2020-04-06T16:38:18.3343692Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-06T16:38:18.3351178Z ##[command]git config gc.auto 0
2020-04-06T16:38:18.3356729Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-06T16:38:18.3361409Z ##[command]git config --get-all http.proxy
2020-04-06T16:38:18.3368923Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70042/merge:refs/remotes/pull/70042/merge
---
2020-04-06T16:41:44.0929293Z  ---> 3fc1b512c57b
2020-04-06T16:41:44.0934523Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-06T16:41:44.0939466Z  ---> Using cache
2020-04-06T16:41:44.0939826Z  ---> 5ee4295733f4
2020-04-06T16:41:44.0941248Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-06T16:41:44.0943499Z  ---> 3d07a0fa42fe
2020-04-06T16:41:44.0998798Z Successfully built 3d07a0fa42fe
2020-04-06T16:41:44.1022802Z Successfully tagged rust-ci:latest
2020-04-06T16:41:44.1296233Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-06T16:41:44.1296233Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-06T16:41:44.1310914Z Looks like docker image is the same as before, not uploading
2020-04-06T16:41:52.3859771Z [CI_JOB_NAME=mingw-check]
2020-04-06T16:41:52.4106798Z [CI_JOB_NAME=mingw-check]
2020-04-06T16:41:52.4134543Z == clock drift check ==
2020-04-06T16:41:52.4141860Z   local time: Mon Apr  6 16:41:52 UTC 2020
2020-04-06T16:41:52.5694934Z   network time: Mon, 06 Apr 2020 16:41:52 GMT
2020-04-06T16:41:52.5718504Z Starting sccache server...
2020-04-06T16:41:52.6581302Z configure: processing command line
2020-04-06T16:41:52.6582175Z configure: 
2020-04-06T16:41:52.6583451Z configure: rust.parallel-compiler := True
---
2020-04-06T16:45:17.2622850Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-06T16:45:17.5196387Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-06T16:45:17.6461977Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-06T16:45:17.7136901Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-06T16:45:18.2187735Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-06T16:45:20.4019480Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-06T16:45:20.8433889Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-06T16:45:22.7797601Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-06T16:45:23.1899640Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-06T16:47:05.8535117Z configure: build.locked-deps    := True
2020-04-06T16:47:05.8535539Z configure: llvm.ccache          := sccache
2020-04-06T16:47:05.8536148Z configure: build.cargo-native-static := True
2020-04-06T16:47:05.8537003Z configure: dist.missing-tools   := True
2020-04-06T16:47:05.8537734Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-06T16:47:05.8538558Z configure: writing `config.toml` in current directory
2020-04-06T16:47:05.8538909Z configure: 
2020-04-06T16:47:05.8539458Z configure: run `python /checkout/x.py --help`
2020-04-06T16:47:05.8539813Z configure: 
---
2020-04-06T16:48:28.2377527Z Hugepagesize:       2048 kB
2020-04-06T16:48:28.2377865Z DirectMap4k:      128960 kB
2020-04-06T16:48:28.2378104Z DirectMap2M:     3016704 kB
2020-04-06T16:48:28.2378329Z DirectMap1G:     6291456 kB
2020-04-06T16:48:28.2393801Z + python2.7 ../x.py test src/tools/expand-yaml-anchors
2020-04-06T16:48:29.5476158Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-06T16:48:29.5476158Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-06T16:48:29.5483260Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-06T16:48:29.7731201Z    Compiling unicode-xid v0.2.0
2020-04-06T16:48:29.8915322Z    Compiling syn v1.0.11
2020-04-06T16:48:30.7245724Z    Compiling linked-hash-map v0.5.2
2020-04-06T16:48:30.7501756Z    Compiling lazy_static v1.4.0
2020-04-06T16:48:30.7501756Z    Compiling lazy_static v1.4.0
2020-04-06T16:48:30.9421541Z    Compiling yaml-rust v0.4.3
2020-04-06T16:48:35.0253914Z    Compiling quote v1.0.2
2020-04-06T16:48:48.6537336Z    Compiling thiserror-impl v1.0.5
2020-04-06T16:48:53.1012799Z    Compiling thiserror v1.0.5
2020-04-06T16:48:53.1616057Z    Compiling yaml-merge-keys v0.4.0
2020-04-06T16:48:54.2675528Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-06T16:48:58.1311537Z Build completed successfully in 0:00:29
2020-04-06T16:48:58.1322823Z + python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-06T16:48:58.3638137Z     Finished dev [unoptimized] target(s) in 0.17s
2020-04-06T16:48:59.4193434Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-06T16:50:59.5956396Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-06T16:50:59.6874391Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-06T16:50:59.8971046Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-06T16:50:59.9847403Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-06T16:51:00.4789633Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-06T16:51:02.6155494Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-06T16:51:03.0738730Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-06T16:51:05.0372205Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-06T16:51:05.4543896Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-06T16:54:56.6851992Z skip untracked path cpu-usage.csv during rustfmt invocations
2020-04-06T16:54:56.6856064Z skip untracked path src/doc/book/ during rustfmt invocations
2020-04-06T16:54:56.6857170Z skip untracked path src/doc/rust-by-example/ during rustfmt invocations
2020-04-06T16:54:56.6859030Z skip untracked path src/llvm-project/ during rustfmt invocations
2020-04-06T16:55:00.5687774Z Diff in /checkout/src/librustc_mir/transform/promote_consts.rs at line 210:
2020-04-06T16:55:00.5688257Z              Rvalue::Repeat(elem, n)
2020-04-06T16:55:00.5688640Z                  if self.ccx.tcx.features().const_in_array_repeat_expressions =>
2020-04-06T16:55:00.5688927Z              {
2020-04-06T16:55:00.5689970Z -                let is_copy = elem.ty(&self.ccx.body.local_decls, self.ccx.tcx).is_copy_modulo_regions(
2020-04-06T16:55:00.5690773Z -                    self.ccx.tcx,
2020-04-06T16:55:00.5691203Z -                    self.ccx.param_env,
2020-04-06T16:55:00.5691993Z -                );
2020-04-06T16:55:00.5692209Z +                let is_copy = elem
2020-04-06T16:55:00.5692209Z +                let is_copy = elem
2020-04-06T16:55:00.5692526Z +                    .ty(&self.ccx.body.local_decls, self.ccx.tcx)
2020-04-06T16:55:00.5692918Z +                    .is_copy_modulo_regions(self.ccx.tcx, self.ccx.param_env, self.span);
2020-04-06T16:55:00.5693426Z                  let n = n.try_eval_usize(self.ccx.tcx, self.ccx.param_env);
2020-04-06T16:55:00.5694023Z                  let length_requires_copy_or_promotion = match n {
2020-04-06T16:55:00.5694621Z                      // Unevaluable (e.g. too generic) -> assume > 1.
2020-04-06T16:55:00.5706141Z Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustc_mir/transform/promote_consts.rs"` failed.
2020-04-06T16:55:00.5707507Z If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
2020-04-06T16:55:00.5731139Z Build completed unsuccessfully in 0:00:39
2020-04-06T16:55:00.5771269Z == clock drift check ==
2020-04-06T16:55:00.5780790Z   local time: Mon Apr  6 16:55:00 UTC 2020
2020-04-06T16:55:00.5780790Z   local time: Mon Apr  6 16:55:00 UTC 2020
2020-04-06T16:55:00.6440797Z   network time: Mon, 06 Apr 2020 16:55:00 GMT
2020-04-06T16:55:02.1684292Z 
2020-04-06T16:55:02.1684292Z 
2020-04-06T16:55:02.1751848Z ##[error]Bash exited with code '1'.
2020-04-06T16:55:02.1765401Z ##[section]Finishing: Run build
2020-04-06T16:55:02.1812144Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70042/merge to s
2020-04-06T16:55:02.1817444Z Task         : Get sources
2020-04-06T16:55:02.1817792Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-06T16:55:02.1818112Z Version      : 1.0.0
2020-04-06T16:55:02.1818353Z Author       : Microsoft
2020-04-06T16:55:02.1818353Z Author       : Microsoft
2020-04-06T16:55:02.1818706Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-06T16:55:02.1819119Z ==============================================================================
2020-04-06T16:55:02.5098439Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-06T16:55:02.5144245Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70042/merge to s
2020-04-06T16:55:02.5233405Z Cleaning up task key
2020-04-06T16:55:02.5234732Z Start cleaning up orphan processes.
2020-04-06T16:55:02.5418548Z Terminate orphan process: pid (3931) (python)
2020-04-06T16:55:02.5618605Z ##[section]Finishing: Finalize Job
