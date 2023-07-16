plain
2020-04-08T14:33:50.9583789Z ========================== Starting Command Output ===========================
2020-04-08T14:33:50.9586443Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/98b3ff92-06c0-477b-9a72-eabddd7908b2.sh
2020-04-08T14:33:50.9586702Z 
2020-04-08T14:33:50.9591558Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-08T14:33:50.9611381Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70928/merge to s
2020-04-08T14:33:50.9614926Z Task         : Get sources
2020-04-08T14:33:50.9615263Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-08T14:33:50.9615543Z Version      : 1.0.0
2020-04-08T14:33:50.9615735Z Author       : Microsoft
---
2020-04-08T14:33:52.1867629Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-08T14:33:52.1875853Z ##[command]git config gc.auto 0
2020-04-08T14:33:52.1881615Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-08T14:33:52.1888670Z ##[command]git config --get-all http.proxy
2020-04-08T14:33:52.1898935Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70928/merge:refs/remotes/pull/70928/merge
---
2020-04-08T14:37:15.2654908Z  ---> 3fc1b512c57b
2020-04-08T14:37:15.2655442Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-08T14:37:15.2655854Z  ---> Using cache
2020-04-08T14:37:15.2656167Z  ---> 5ee4295733f4
2020-04-08T14:37:15.2657881Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-08T14:37:15.2659187Z  ---> 3d07a0fa42fe
2020-04-08T14:37:15.2717384Z Successfully built 3d07a0fa42fe
2020-04-08T14:37:15.2769170Z Successfully tagged rust-ci:latest
2020-04-08T14:37:15.3110310Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-08T14:37:15.3110310Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-08T14:37:15.3124223Z Looks like docker image is the same as before, not uploading
2020-04-08T14:37:23.0446389Z [CI_JOB_NAME=mingw-check]
2020-04-08T14:37:23.0718559Z [CI_JOB_NAME=mingw-check]
2020-04-08T14:37:23.0749162Z == clock drift check ==
2020-04-08T14:37:23.0759219Z   local time: Wed Apr  8 14:37:23 UTC 2020
2020-04-08T14:37:23.3647607Z   network time: Wed, 08 Apr 2020 14:37:23 GMT
2020-04-08T14:37:23.3678145Z Starting sccache server...
2020-04-08T14:37:23.4522506Z configure: processing command line
2020-04-08T14:37:23.4522727Z configure: 
2020-04-08T14:37:23.4523524Z configure: rust.parallel-compiler := True
---
2020-04-08T14:41:04.6660176Z     Checking rustc_span v0.0.0 (/checkout/src/librustc_span)
2020-04-08T14:41:09.3847513Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2020-04-08T14:41:10.6426696Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-08T14:41:10.6627845Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-08T14:41:10.8595263Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-08T14:41:11.7227429Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-08T14:41:11.7352482Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-08T14:41:13.3378744Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-08T14:41:13.8449108Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
---
2020-04-08T14:43:11.2341418Z configure: build.locked-deps    := True
2020-04-08T14:43:11.2341718Z configure: llvm.ccache          := sccache
2020-04-08T14:43:11.2342902Z configure: build.cargo-native-static := True
2020-04-08T14:43:11.2343357Z configure: dist.missing-tools   := True
2020-04-08T14:43:11.2343946Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-08T14:43:11.2344772Z configure: writing `config.toml` in current directory
2020-04-08T14:43:11.2345029Z configure: 
2020-04-08T14:43:11.2345667Z configure: run `python /checkout/x.py --help`
2020-04-08T14:43:11.2345895Z configure: 
---
2020-04-08T14:44:37.9873562Z Hugepagesize:       2048 kB
2020-04-08T14:44:37.9873787Z DirectMap4k:      145344 kB
2020-04-08T14:44:37.9873992Z DirectMap2M:     4048896 kB
2020-04-08T14:44:37.9874198Z DirectMap1G:     5242880 kB
2020-04-08T14:44:37.9886191Z + python2.7 ../x.py test src/tools/expand-yaml-anchors
2020-04-08T14:44:39.3265931Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-08T14:44:39.3265931Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-08T14:44:39.3279121Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-08T14:44:39.5535345Z    Compiling unicode-xid v0.2.0
2020-04-08T14:44:39.6821404Z    Compiling syn v1.0.11
2020-04-08T14:44:40.5800880Z    Compiling linked-hash-map v0.5.2
2020-04-08T14:44:40.5811673Z    Compiling lazy_static v1.4.0
2020-04-08T14:44:40.5811673Z    Compiling lazy_static v1.4.0
2020-04-08T14:44:40.8163942Z    Compiling yaml-rust v0.4.3
2020-04-08T14:44:45.3482552Z    Compiling quote v1.0.2
2020-04-08T14:45:00.8308795Z    Compiling thiserror-impl v1.0.5
2020-04-08T14:45:05.8444763Z    Compiling thiserror v1.0.5
2020-04-08T14:45:05.9050432Z    Compiling yaml-merge-keys v0.4.0
2020-04-08T14:45:07.1866825Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-08T14:45:09.4065534Z Build completed successfully in 0:00:31
2020-04-08T14:45:09.4071260Z + python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-08T14:45:09.6670500Z     Finished dev [unoptimized] target(s) in 0.19s
2020-04-08T14:45:10.7883457Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-08T14:47:18.0216409Z     Checking rustc_span v0.0.0 (/checkout/src/librustc_span)
2020-04-08T14:47:22.6575838Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2020-04-08T14:47:23.9255137Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-08T14:47:24.0553277Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-08T14:47:24.2674162Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-08T14:47:25.0251629Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-08T14:47:25.0954961Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-08T14:47:26.6791218Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-08T14:47:27.1957408Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
---
2020-04-08T14:51:44.5693449Z skip untracked path cpu-usage.csv during rustfmt invocations
2020-04-08T14:51:44.5699312Z skip untracked path src/doc/book/ during rustfmt invocations
2020-04-08T14:51:44.5699984Z skip untracked path src/doc/rust-by-example/ during rustfmt invocations
2020-04-08T14:51:44.5704729Z skip untracked path src/llvm-project/ during rustfmt invocations
2020-04-08T14:51:48.5386079Z Diff in /checkout/src/librustc_mir/borrow_check/mod.rs at line 14:
2020-04-08T14:51:48.5386493Z      Operand, Place, PlaceElem, PlaceRef, ReadOnlyBodyAndCache,
2020-04-08T14:51:48.5386714Z  };
2020-04-08T14:51:48.5387029Z  use rustc_middle::mir::{AggregateKind, BasicBlock, BorrowCheckResult, BorrowKind};
2020-04-08T14:51:48.5388060Z -use rustc_middle::mir::{Field, ProjectionElem, Promoted, Op, Statement, StatementKind};
2020-04-08T14:51:48.5388490Z +use rustc_middle::mir::{Field, Op, ProjectionElem, Promoted, Statement, StatementKind};
2020-04-08T14:51:48.5388879Z  use rustc_middle::mir::{Terminator, TerminatorKind};
2020-04-08T14:51:48.5389169Z  use rustc_middle::ty::query::Providers;
2020-04-08T14:51:48.5389451Z  use rustc_middle::ty::{self, RegionVid, TyCtxt};
2020-04-08T14:51:48.5389786Z Diff in /checkout/src/librustc_mir/borrow_check/mod.rs at line 1335:
2020-04-08T14:51:48.5390129Z                          let stmt = &bbd.statements[loc.statement_index];
2020-04-08T14:51:48.5390520Z                          debug!("temporary assigned in: stmt={:?}", stmt);
2020-04-08T14:51:48.5390804Z  
2020-04-08T14:51:48.5391567Z -                        if let StatementKind::Assign(box (_, Op::Ref(_, _, source))) = stmt.kind
2020-04-08T14:51:48.5392082Z -                        {
2020-04-08T14:51:48.5392855Z +                        if let StatementKind::Assign(box (_, Op::Ref(_, _, source))) = stmt.kind {
2020-04-08T14:51:48.5393302Z                              propagate_closure_used_mut_place(self, source);
2020-04-08T14:51:48.5396743Z                              bug!(
2020-04-08T14:51:48.5396743Z                              bug!(
2020-04-08T14:51:48.5399456Z Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustc_mir/borrow_check/mod.rs"` failed.
2020-04-08T14:51:48.5400689Z If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
2020-04-08T14:51:48.5412972Z Build completed unsuccessfully in 0:00:41
2020-04-08T14:51:48.5459833Z == clock drift check ==
2020-04-08T14:51:48.5472700Z   local time: Wed Apr  8 14:51:48 UTC 2020
2020-04-08T14:51:48.5472700Z   local time: Wed Apr  8 14:51:48 UTC 2020
2020-04-08T14:51:48.8354256Z   network time: Wed, 08 Apr 2020 14:51:48 GMT
2020-04-08T14:51:50.3779991Z 
2020-04-08T14:51:50.3779991Z 
2020-04-08T14:51:50.3868514Z ##[error]Bash exited with code '1'.
2020-04-08T14:51:50.3886987Z ##[section]Finishing: Run build
2020-04-08T14:51:50.3971570Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70928/merge to s
2020-04-08T14:51:50.3977001Z Task         : Get sources
2020-04-08T14:51:50.3977381Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-08T14:51:50.3977697Z Version      : 1.0.0
2020-04-08T14:51:50.3977905Z Author       : Microsoft
2020-04-08T14:51:50.3977905Z Author       : Microsoft
2020-04-08T14:51:50.3978405Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-08T14:51:50.3978809Z ==============================================================================
2020-04-08T14:51:50.7366056Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-08T14:51:50.7405463Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70928/merge to s
2020-04-08T14:51:50.7493784Z Cleaning up task key
2020-04-08T14:51:50.7495660Z Start cleaning up orphan processes.
2020-04-08T14:51:50.7673269Z Terminate orphan process: pid (3511) (python)
2020-04-08T14:51:50.7850129Z ##[section]Finishing: Finalize Job
