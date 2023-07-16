plain
2020-04-20T01:05:07.7890114Z ========================== Starting Command Output ===========================
2020-04-20T01:05:07.7906628Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/478ddafa-a471-41c7-8e74-3da09694fee1.sh
2020-04-20T01:05:07.8112606Z 
2020-04-20T01:05:07.8164104Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-20T01:05:07.8179120Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71006/merge to s
2020-04-20T01:05:07.8181993Z Task         : Get sources
2020-04-20T01:05:07.8182222Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-20T01:05:07.8182459Z Version      : 1.0.0
2020-04-20T01:05:07.8182614Z Author       : Microsoft
---
2020-04-20T01:05:08.6102294Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-20T01:05:08.6106942Z ##[command]git config gc.auto 0
2020-04-20T01:05:08.6110020Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-20T01:05:08.6112836Z ##[command]git config --get-all http.proxy
2020-04-20T01:05:08.6118492Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71006/merge:refs/remotes/pull/71006/merge
---
2020-04-20T01:08:38.8966367Z  ---> 78ad2f4d4aca
2020-04-20T01:08:38.8966560Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-20T01:08:38.8971296Z  ---> Using cache
2020-04-20T01:08:38.8971714Z  ---> 4d2dc61c4d00
2020-04-20T01:08:38.8973169Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-20T01:08:38.8974697Z  ---> 776b6266a8b7
2020-04-20T01:08:38.9021633Z Successfully built 776b6266a8b7
2020-04-20T01:08:38.9079987Z Successfully tagged rust-ci:latest
2020-04-20T01:08:38.9312768Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-20T01:08:38.9312768Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-20T01:08:38.9326970Z Looks like docker image is the same as before, not uploading
2020-04-20T01:08:40.7147279Z [CI_JOB_NAME=mingw-check]
2020-04-20T01:08:40.7454855Z [CI_JOB_NAME=mingw-check]
2020-04-20T01:08:40.7483931Z == clock drift check ==
2020-04-20T01:08:40.7491976Z   local time: Mon Apr 20 01:08:40 UTC 2020
2020-04-20T01:08:41.0187870Z   network time: Mon, 20 Apr 2020 01:08:41 GMT
2020-04-20T01:08:41.0216394Z Starting sccache server...
2020-04-20T01:08:41.1239906Z configure: processing command line
2020-04-20T01:08:41.1240633Z configure: 
2020-04-20T01:08:41.1241738Z configure: rust.parallel-compiler := True
---
2020-04-20T01:12:18.2570113Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-20T01:12:18.2886140Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-20T01:12:18.4614253Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-20T01:12:18.6819274Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-20T01:12:19.0325666Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-20T01:12:21.2022507Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-20T01:12:21.6607323Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-20T01:12:23.5329947Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-20T01:12:23.9341910Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-20T01:14:03.4053720Z configure: llvm.assertions      := True
2020-04-20T01:14:03.4054168Z configure: rust.channel         := nightly
2020-04-20T01:14:03.4054811Z configure: rust.debug-assertions := True
2020-04-20T01:14:03.4056778Z configure: build.cargo-native-static := True
2020-04-20T01:14:03.4057801Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-20T01:14:03.4058890Z configure: writing `config.toml` in current directory
2020-04-20T01:14:03.4059429Z configure: 
2020-04-20T01:14:03.4060044Z configure: run `python /checkout/x.py --help`
2020-04-20T01:14:03.4060506Z configure: 
---
2020-04-20T01:15:31.2014260Z Hugepagesize:       2048 kB
2020-04-20T01:15:31.2014450Z DirectMap4k:      149440 kB
2020-04-20T01:15:31.2014628Z DirectMap2M:     4044800 kB
2020-04-20T01:15:31.2014807Z DirectMap1G:     5242880 kB
2020-04-20T01:15:31.2085511Z + python3 ../x.py test src/tools/expand-yaml-anchors
2020-04-20T01:15:32.4272388Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-20T01:15:32.4272388Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-20T01:15:32.4281448Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-20T01:15:32.6346788Z    Compiling unicode-xid v0.2.0
2020-04-20T01:15:32.7570424Z    Compiling syn v1.0.11
2020-04-20T01:15:33.5352574Z    Compiling linked-hash-map v0.5.2
2020-04-20T01:15:33.5531758Z    Compiling lazy_static v1.4.0
2020-04-20T01:15:33.5531758Z    Compiling lazy_static v1.4.0
2020-04-20T01:15:33.7409895Z    Compiling yaml-rust v0.4.3
2020-04-20T01:15:37.7272553Z    Compiling quote v1.0.2
2020-04-20T01:15:51.1795661Z    Compiling thiserror-impl v1.0.5
2020-04-20T01:15:55.5974983Z    Compiling thiserror v1.0.5
2020-04-20T01:15:55.6484152Z    Compiling yaml-merge-keys v0.4.0
2020-04-20T01:15:56.7445697Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-20T01:15:58.4223554Z Build completed successfully in 0:00:27
2020-04-20T01:15:58.4303737Z + python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-20T01:15:58.6681564Z     Finished dev [unoptimized] target(s) in 0.16s
2020-04-20T01:15:59.6332732Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-20T01:17:50.1447983Z     Checking rustc_span v0.0.0 (/checkout/src/librustc_span)
2020-04-20T01:17:54.3773869Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2020-04-20T01:17:55.5163940Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-20T01:17:55.5481545Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-20T01:17:55.7246945Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-20T01:17:56.4000564Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-20T01:17:56.4611692Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-20T01:17:57.8839104Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-20T01:17:58.3153138Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
---
2020-04-20T01:21:46.2825764Z skip untracked path cpu-usage.csv during rustfmt invocations
2020-04-20T01:21:46.2830615Z skip untracked path src/doc/book/ during rustfmt invocations
2020-04-20T01:21:46.2835025Z skip untracked path src/doc/rust-by-example/ during rustfmt invocations
2020-04-20T01:21:46.2835980Z skip untracked path src/llvm-project/ during rustfmt invocations
2020-04-20T01:21:51.1168553Z Diff in /checkout/src/librustc_mir/dataflow/framework/direction.rs at line 3:
2020-04-20T01:21:51.1173251Z  use rustc_middle::ty::{self, TyCtxt};
2020-04-20T01:21:51.1177263Z  use std::ops::RangeInclusive;
2020-04-20T01:21:51.1181443Z  
2020-04-20T01:21:51.1185473Z +use super::visitor::{ResultsVisitable, ResultsVisitor};
2020-04-20T01:21:51.1195228Z  use super::{Analysis, Effect, EffectIndex, GenKillAnalysis, GenKillSet};
2020-04-20T01:21:51.1200234Z -use super::visitor::{ResultsVisitor, ResultsVisitable};
2020-04-20T01:21:51.1228355Z  pub trait Direction {
2020-04-20T01:21:51.1228873Z      fn is_forward() -> bool;
2020-04-20T01:21:51.1228873Z      fn is_forward() -> bool;
2020-04-20T01:21:51.1229138Z Diff in /checkout/src/librustc_mir/dataflow/framework/direction.rs at line 11:
2020-04-20T01:21:51.1229374Z  
2020-04-20T01:21:51.1229741Z -    fn is_backward() -> bool { !Self::is_forward() }
2020-04-20T01:21:51.1230109Z +    fn is_backward() -> bool {
2020-04-20T01:21:51.1230310Z +        !Self::is_forward()
2020-04-20T01:21:51.1230566Z  
2020-04-20T01:21:51.1230566Z  
2020-04-20T01:21:51.1230748Z      /// Applies all effects between the given `EffectIndex`s.
2020-04-20T01:21:51.1230954Z      ///
2020-04-20T01:21:51.1231193Z Diff in /checkout/src/librustc_mir/dataflow/framework/direction.rs at line 64:
2020-04-20T01:21:51.1231434Z  pub struct Backward;
2020-04-20T01:21:51.1231713Z  impl Direction for Backward {
2020-04-20T01:21:51.1231713Z  impl Direction for Backward {
2020-04-20T01:21:51.1232057Z -    fn is_forward() -> bool { false }
2020-04-20T01:21:51.1232400Z +    fn is_forward() -> bool {
2020-04-20T01:21:51.1232682Z +    }
2020-04-20T01:21:51.1232800Z  
2020-04-20T01:21:51.1232800Z  
2020-04-20T01:21:51.1232944Z      fn apply_effects_in_block<A>(
2020-04-20T01:21:51.1233125Z          analysis: &A,
2020-04-20T01:21:51.1233379Z Diff in /checkout/src/librustc_mir/dataflow/framework/direction.rs at line 190:
2020-04-20T01:21:51.1233645Z          results: &R,
2020-04-20T01:21:51.1234047Z          vis: &mut impl ResultsVisitor<'mir, 'tcx, FlowState = F>,
2020-04-20T01:21:51.1234257Z      ) where
2020-04-20T01:21:51.1234610Z -        R: ResultsVisitable<'tcx, FlowState = F>
2020-04-20T01:21:51.1235001Z +        R: ResultsVisitable<'tcx, FlowState = F>,
2020-04-20T01:21:51.1235186Z      {
2020-04-20T01:21:51.1235381Z          results.reset_to_block_fixpoint(state, block);
2020-04-20T01:21:51.1235550Z  
2020-04-20T01:21:51.1235771Z Diff in /checkout/src/librustc_mir/dataflow/framework/direction.rs at line 271:
2020-04-20T01:21:51.1236026Z  pub struct Forward;
2020-04-20T01:21:51.1236286Z  impl Direction for Forward {
2020-04-20T01:21:51.1236286Z  impl Direction for Forward {
2020-04-20T01:21:51.1236632Z -    fn is_forward() -> bool { true }
2020-04-20T01:21:51.1236957Z +    fn is_forward() -> bool {
2020-04-20T01:21:51.1237234Z +    }
2020-04-20T01:21:51.1237354Z  
2020-04-20T01:21:51.1237354Z  
2020-04-20T01:21:51.1237497Z      fn apply_effects_in_block<A>(
2020-04-20T01:21:51.1237677Z          analysis: &A,
2020-04-20T01:21:51.1237946Z Diff in /checkout/src/librustc_mir/dataflow/framework/direction.rs at line 393:
2020-04-20T01:21:51.1238196Z          results: &R,
2020-04-20T01:21:51.1238594Z          vis: &mut impl ResultsVisitor<'mir, 'tcx, FlowState = F>,
2020-04-20T01:21:51.1238824Z      ) where
2020-04-20T01:21:51.1239162Z -        R: ResultsVisitable<'tcx, FlowState = F>
2020-04-20T01:21:51.1239552Z +        R: ResultsVisitable<'tcx, FlowState = F>,
2020-04-20T01:21:51.1239744Z      {
2020-04-20T01:21:51.1239927Z          results.reset_to_block_fixpoint(state, block);
2020-04-20T01:21:51.1240095Z  
2020-04-20T01:21:51.1240851Z Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustc_mir/dataflow/framework/direction.rs"` failed.
2020-04-20T01:21:51.1241642Z If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
2020-04-20T01:21:51.1284672Z Build completed unsuccessfully in 0:00:39
2020-04-20T01:21:51.1336830Z == clock drift check ==
2020-04-20T01:21:51.1352011Z   local time: Mon Apr 20 01:21:51 UTC 2020
2020-04-20T01:21:51.1352011Z   local time: Mon Apr 20 01:21:51 UTC 2020
2020-04-20T01:21:51.4102966Z   network time: Mon, 20 Apr 2020 01:21:51 GMT
2020-04-20T01:21:52.8260180Z 
2020-04-20T01:21:52.8260180Z 
2020-04-20T01:21:52.8344880Z ##[error]Bash exited with code '1'.
2020-04-20T01:21:52.8373446Z ##[section]Finishing: Run build
2020-04-20T01:21:52.8418714Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71006/merge to s
2020-04-20T01:21:52.8423473Z Task         : Get sources
2020-04-20T01:21:52.8423773Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-20T01:21:52.8424030Z Version      : 1.0.0
2020-04-20T01:21:52.8424212Z Author       : Microsoft
2020-04-20T01:21:52.8424212Z Author       : Microsoft
2020-04-20T01:21:52.8424518Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-20T01:21:52.8424846Z ==============================================================================
2020-04-20T01:21:53.1530807Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-20T01:21:53.1570642Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71006/merge to s
2020-04-20T01:21:53.1651039Z Cleaning up task key
2020-04-20T01:21:53.1652159Z Start cleaning up orphan processes.
2020-04-20T01:21:53.1820105Z Terminate orphan process: pid (3499) (python)
2020-04-20T01:21:53.2422497Z ##[section]Finishing: Finalize Job
