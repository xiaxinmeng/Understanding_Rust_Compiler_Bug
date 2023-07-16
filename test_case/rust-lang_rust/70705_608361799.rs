plain
2020-04-03T10:10:47.7660935Z ========================== Starting Command Output ===========================
2020-04-03T10:10:47.7664482Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/0f0d5bed-7f48-45b4-8493-768b2b037ae3.sh
2020-04-03T10:10:47.7664881Z 
2020-04-03T10:10:47.7669034Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-03T10:10:47.7687644Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70705/merge to s
2020-04-03T10:10:47.7690607Z Task         : Get sources
2020-04-03T10:10:47.7690895Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-03T10:10:47.7691173Z Version      : 1.0.0
2020-04-03T10:10:47.7691377Z Author       : Microsoft
---
2020-04-03T10:10:48.7589737Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-03T10:10:48.7598643Z ##[command]git config gc.auto 0
2020-04-03T10:10:48.7604910Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-03T10:10:48.7610787Z ##[command]git config --get-all http.proxy
2020-04-03T10:10:48.7620532Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70705/merge:refs/remotes/pull/70705/merge
---
2020-04-03T10:14:25.4756016Z  ---> 3fc1b512c57b
2020-04-03T10:14:25.4756248Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-03T10:14:25.4756630Z  ---> Using cache
2020-04-03T10:14:25.4756956Z  ---> 5ee4295733f4
2020-04-03T10:14:25.4758298Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-03T10:14:25.4759680Z  ---> 3d07a0fa42fe
2020-04-03T10:14:25.4759883Z Successfully built 3d07a0fa42fe
2020-04-03T10:14:25.4799399Z Successfully tagged rust-ci:latest
2020-04-03T10:14:25.5048529Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-03T10:14:25.5048529Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
2020-04-03T10:14:25.5062224Z Looks like docker image is the same as before, not uploading
2020-04-03T10:14:33.4688381Z [CI_JOB_NAME=mingw-check]
2020-04-03T10:14:33.4965664Z [CI_JOB_NAME=mingw-check]
2020-04-03T10:14:33.4999046Z == clock drift check ==
2020-04-03T10:14:33.5006973Z   local time: Fri Apr  3 10:14:33 UTC 2020
2020-04-03T10:14:33.6578193Z   network time: Fri, 03 Apr 2020 10:14:33 GMT
2020-04-03T10:14:33.6609053Z Starting sccache server...
2020-04-03T10:14:33.7470065Z configure: processing command line
2020-04-03T10:14:33.7473610Z configure: 
2020-04-03T10:14:33.7475056Z configure: rust.parallel-compiler := True
---
2020-04-03T10:17:47.5215257Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-03T10:17:47.6045680Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-03T10:17:47.7745033Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-03T10:17:47.8850798Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-03T10:17:48.3305278Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-03T10:17:50.3625798Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-03T10:17:50.7848063Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-03T10:17:52.6125781Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-03T10:17:52.9990098Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-03T10:19:29.1545941Z configure: build.locked-deps    := True
2020-04-03T10:19:29.1546235Z configure: llvm.ccache          := sccache
2020-04-03T10:19:29.1546717Z configure: build.cargo-native-static := True
2020-04-03T10:19:29.1547171Z configure: dist.missing-tools   := True
2020-04-03T10:19:29.1547743Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-04-03T10:19:29.1548294Z configure: writing `config.toml` in current directory
2020-04-03T10:19:29.1548525Z configure: 
2020-04-03T10:19:29.1549025Z configure: run `python /checkout/x.py --help`
2020-04-03T10:19:29.1549230Z configure: 
---
2020-04-03T10:20:43.2155337Z Hugepagesize:       2048 kB
2020-04-03T10:20:43.2155552Z DirectMap4k:      149440 kB
2020-04-03T10:20:43.2155783Z DirectMap2M:     4044800 kB
2020-04-03T10:20:43.2156004Z DirectMap1G:     5242880 kB
2020-04-03T10:20:43.2157520Z + python2.7 ../x.py test src/tools/expand-yaml-anchors
2020-04-03T10:20:44.4997679Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-03T10:20:44.4997679Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-04-03T10:20:44.5006601Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-04-03T10:20:44.7310311Z    Compiling unicode-xid v0.2.0
2020-04-03T10:20:44.8485671Z    Compiling syn v1.0.11
2020-04-03T10:20:45.6411098Z    Compiling linked-hash-map v0.5.2
2020-04-03T10:20:45.6772556Z    Compiling lazy_static v1.4.0
2020-04-03T10:20:45.6772556Z    Compiling lazy_static v1.4.0
2020-04-03T10:20:45.8559668Z    Compiling yaml-rust v0.4.3
2020-04-03T10:20:49.7996981Z    Compiling quote v1.0.2
2020-04-03T10:21:02.7549432Z    Compiling thiserror-impl v1.0.5
2020-04-03T10:21:07.0462305Z    Compiling thiserror v1.0.5
2020-04-03T10:21:07.1025386Z    Compiling yaml-merge-keys v0.4.0
2020-04-03T10:21:08.1701007Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-04-03T10:21:11.2539186Z Build completed successfully in 0:00:28
2020-04-03T10:21:11.2551489Z + python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-04-03T10:21:11.4795977Z     Finished dev [unoptimized] target(s) in 0.16s
2020-04-03T10:21:12.5080407Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-04-03T10:23:05.6138846Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-03T10:23:05.8017718Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-03T10:23:05.9834333Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-03T10:23:06.0117631Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-03T10:23:06.5444874Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-03T10:23:08.6223442Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-03T10:23:09.0688311Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-03T10:23:10.9392111Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-03T10:23:11.3336671Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-03T10:26:49.0592225Z skip untracked path src/llvm-project/ during rustfmt invocations
2020-04-03T10:26:51.2180648Z Diff in /checkout/src/librustc_trait_selection/traits/select.rs at line 2755:
2020-04-03T10:26:51.2181401Z      fn confirm_discriminant_kind_candidate(
2020-04-03T10:26:51.2181731Z          &mut self,
2020-04-03T10:26:51.2182706Z          obligation: &TraitObligation<'tcx>,
2020-04-03T10:26:51.2183358Z -    ) -> VtableDiscriminantKindData<'tcx, PredicateObligation<'tcx>>
2020-04-03T10:26:51.2183865Z -    {
2020-04-03T10:26:51.2184420Z +    ) -> VtableDiscriminantKindData<'tcx, PredicateObligation<'tcx>> {
2020-04-03T10:26:51.2184806Z          let tcx = self.tcx();
2020-04-03T10:26:51.2185340Z          // Okay to skip binder.
2020-04-03T10:26:51.2185340Z          // Okay to skip binder.
2020-04-03T10:26:51.2191864Z Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustc_trait_selection/traits/select.rs"` failed.
2020-04-03T10:26:51.2193055Z If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
2020-04-03T10:26:51.2200874Z Build completed unsuccessfully in 0:00:35
2020-04-03T10:26:51.2247106Z == clock drift check ==
2020-04-03T10:26:51.2266966Z   local time: Fri Apr  3 10:26:51 UTC 2020
2020-04-03T10:26:51.2266966Z   local time: Fri Apr  3 10:26:51 UTC 2020
2020-04-03T10:26:51.2880212Z   network time: Fri, 03 Apr 2020 10:26:51 GMT
2020-04-03T10:26:52.8597354Z 
2020-04-03T10:26:52.8597354Z 
2020-04-03T10:26:52.8668635Z ##[error]Bash exited with code '1'.
2020-04-03T10:26:52.8683664Z ##[section]Finishing: Run build
2020-04-03T10:26:52.8728631Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70705/merge to s
2020-04-03T10:26:52.8733159Z Task         : Get sources
2020-04-03T10:26:52.8733473Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-03T10:26:52.8733778Z Version      : 1.0.0
2020-04-03T10:26:52.8733982Z Author       : Microsoft
2020-04-03T10:26:52.8733982Z Author       : Microsoft
2020-04-03T10:26:52.8734302Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-03T10:26:52.8734690Z ==============================================================================
2020-04-03T10:26:53.1898372Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-03T10:26:53.1954128Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70705/merge to s
2020-04-03T10:26:53.2037604Z Cleaning up task key
2020-04-03T10:26:53.2038809Z Start cleaning up orphan processes.
2020-04-03T10:26:53.2216348Z Terminate orphan process: pid (3846) (python)
2020-04-03T10:26:53.2570932Z ##[section]Finishing: Finalize Job
