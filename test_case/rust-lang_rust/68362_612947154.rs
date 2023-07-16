plain
2020-04-13T14:26:30.6873848Z ========================== Starting Command Output ===========================
2020-04-13T14:26:30.6875898Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/1f0ff8ec-abe9-42de-a95e-e8317bc9703a.sh
2020-04-13T14:26:30.6876106Z 
2020-04-13T14:26:30.6879426Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-13T14:26:30.6898090Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68362/merge to s
2020-04-13T14:26:30.6901057Z Task         : Get sources
2020-04-13T14:26:30.6901284Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-13T14:26:30.6901545Z Version      : 1.0.0
2020-04-13T14:26:30.6901695Z Author       : Microsoft
---
2020-04-13T14:26:31.9298726Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-13T14:26:31.9305020Z ##[command]git config gc.auto 0
2020-04-13T14:26:31.9313221Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-13T14:26:31.9317740Z ##[command]git config --get-all http.proxy
2020-04-13T14:26:31.9329610Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68362/merge:refs/remotes/pull/68362/merge
---
2020-04-13T14:29:04.2084253Z  ---> 78ad2f4d4aca
2020-04-13T14:29:04.2084456Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-13T14:29:04.2084784Z  ---> Using cache
2020-04-13T14:29:04.2085076Z  ---> 4d2dc61c4d00
2020-04-13T14:29:04.2086215Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-13T14:29:04.2087359Z  ---> 776b6266a8b7
2020-04-13T14:29:04.2087538Z Successfully built 776b6266a8b7
2020-04-13T14:29:04.2136969Z Successfully tagged rust-ci:latest
2020-04-13T14:29:04.2407014Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-13T14:29:04.2407014Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-13T14:29:04.2422621Z Looks like docker image is the same as before, not uploading
2020-04-13T14:29:11.7270331Z [CI_JOB_NAME=mingw-check]
2020-04-13T14:29:11.7507913Z [CI_JOB_NAME=mingw-check]
2020-04-13T14:29:11.7530938Z == clock drift check ==
2020-04-13T14:29:11.7541604Z   local time: Mon Apr 13 14:29:11 UTC 2020
2020-04-13T14:29:11.8601215Z   network time: Mon, 13 Apr 2020 14:29:11 GMT
2020-04-13T14:29:11.8625855Z Starting sccache server...
2020-04-13T14:29:11.9664636Z configure: processing command line
2020-04-13T14:29:11.9665437Z configure: 
2020-04-13T14:29:11.9666976Z configure: rust.parallel-compiler := True
---
2020-04-13T14:32:58.7840051Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-13T14:32:58.9818103Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-13T14:32:59.1958114Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-13T14:32:59.2198025Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-13T14:32:59.8175171Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-13T14:33:02.1373747Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-13T14:33:02.6145974Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-13T14:33:04.7229392Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-13T14:33:05.1753085Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-13T14:34:52.1151353Z configure: build.cargo-native-static := True
2020-04-13T14:34:52.1151967Z configure: rust.channel         := nightly
2020-04-13T14:34:52.1152563Z configure: rust.debug-assertions := True
2020-04-13T14:34:52.1153009Z configure: llvm.ccache          := sccache
2020-04-13T14:34:52.1153717Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-13T14:34:52.1154672Z configure: writing `config.toml` in current directory
2020-04-13T14:34:52.1155144Z configure: 
2020-04-13T14:34:52.1155896Z configure: run `python /checkout/x.py --help`
2020-04-13T14:34:52.1156397Z configure: 
---
2020-04-13T14:36:20.0938168Z Hugepagesize:       2048 kB
2020-04-13T14:36:20.0938505Z DirectMap4k:      122816 kB
2020-04-13T14:36:20.0938766Z DirectMap2M:     5120000 kB
2020-04-13T14:36:20.0938961Z DirectMap1G:     4194304 kB
2020-04-13T14:36:20.0951680Z + python3 ../x.py test src/tools/expand-yaml-anchors
2020-04-13T14:36:21.3987600Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-13T14:36:21.3987600Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-13T14:36:21.3993754Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-13T14:36:21.6286363Z    Compiling unicode-xid v0.2.0
2020-04-13T14:36:21.7529774Z    Compiling syn v1.0.11
2020-04-13T14:36:22.5904118Z    Compiling linked-hash-map v0.5.2
2020-04-13T14:36:22.6048961Z    Compiling lazy_static v1.4.0
2020-04-13T14:36:22.6048961Z    Compiling lazy_static v1.4.0
2020-04-13T14:36:22.8123433Z    Compiling yaml-rust v0.4.3
2020-04-13T14:36:27.0872234Z    Compiling quote v1.0.2
2020-04-13T14:36:42.0828957Z    Compiling thiserror-impl v1.0.5
2020-04-13T14:36:46.2060627Z    Compiling thiserror v1.0.5
2020-04-13T14:36:46.2683509Z    Compiling yaml-merge-keys v0.4.0
2020-04-13T14:36:47.4274990Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-13T14:36:51.1388400Z Build completed successfully in 0:00:30
2020-04-13T14:36:51.1482911Z + python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-13T14:36:51.4013614Z     Finished dev [unoptimized] target(s) in 0.17s
2020-04-13T14:36:52.4625683Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-13T14:39:00.3265820Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-13T14:39:00.3342474Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-13T14:39:00.5301833Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-13T14:39:00.7460824Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-13T14:39:01.1524489Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-13T14:39:03.4491997Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-13T14:39:03.9786820Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-13T14:39:06.0788452Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-13T14:39:06.5336218Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-13T14:43:08.4793811Z skip untracked path cpu-usage.csv during rustfmt invocations
2020-04-13T14:43:08.4796461Z skip untracked path src/doc/book/ during rustfmt invocations
2020-04-13T14:43:08.4797229Z skip untracked path src/doc/rust-by-example/ during rustfmt invocations
2020-04-13T14:43:08.4801918Z skip untracked path src/llvm-project/ during rustfmt invocations
2020-04-13T14:43:14.7560411Z Diff in /checkout/src/librustc_infer/infer/region_constraints/mod.rs at line 13:
2020-04-13T14:43:14.7561987Z  use rustc_index::vec::IndexVec;
2020-04-13T14:43:14.7570431Z  use rustc_middle::ty::ReStatic;
2020-04-13T14:43:14.7570696Z  use rustc_middle::ty::{self, Ty, TyCtxt};
2020-04-13T14:43:14.7579229Z -use rustc_middle::ty::{OutlivesPredicate, ReLateBound, ReVar, RegionKind, RegionOutlivesPredicate};
2020-04-13T14:43:14.7625995Z +use rustc_middle::ty::{
2020-04-13T14:43:14.7627602Z +    OutlivesPredicate, ReLateBound, ReVar, RegionKind, RegionOutlivesPredicate,
2020-04-13T14:43:14.7627855Z +};
2020-04-13T14:43:14.7628069Z  use rustc_middle::ty::{Region, RegionVid};
2020-04-13T14:43:14.7628443Z  
2020-04-13T14:43:14.7628443Z  
2020-04-13T14:43:14.7629474Z Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustc_infer/infer/region_constraints/mod.rs"` failed.
2020-04-13T14:43:14.7630403Z If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
2020-04-13T14:43:14.7731755Z Build completed unsuccessfully in 0:00:43
2020-04-13T14:43:14.7773164Z == clock drift check ==
2020-04-13T14:43:14.7785251Z   local time: Mon Apr 13 14:43:14 UTC 2020
2020-04-13T14:43:14.7785251Z   local time: Mon Apr 13 14:43:14 UTC 2020
2020-04-13T14:43:14.9500303Z   network time: Mon, 13 Apr 2020 14:43:14 GMT
2020-04-13T14:43:16.4352242Z 
2020-04-13T14:43:16.4352242Z 
2020-04-13T14:43:16.4426942Z ##[error]Bash exited with code '1'.
2020-04-13T14:43:16.4440549Z ##[section]Finishing: Run build
2020-04-13T14:43:16.4487221Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68362/merge to s
2020-04-13T14:43:16.4491985Z Task         : Get sources
2020-04-13T14:43:16.4492267Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-13T14:43:16.4492540Z Version      : 1.0.0
2020-04-13T14:43:16.4492720Z Author       : Microsoft
2020-04-13T14:43:16.4492720Z Author       : Microsoft
2020-04-13T14:43:16.4493005Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-13T14:43:16.4493520Z ==============================================================================
2020-04-13T14:43:16.7660019Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-13T14:43:16.7702375Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68362/merge to s
2020-04-13T14:43:16.7793392Z Cleaning up task key
2020-04-13T14:43:16.7794691Z Start cleaning up orphan processes.
2020-04-13T14:43:16.7983341Z Terminate orphan process: pid (4852) (python)
2020-04-13T14:43:16.8210821Z ##[section]Finishing: Finalize Job
