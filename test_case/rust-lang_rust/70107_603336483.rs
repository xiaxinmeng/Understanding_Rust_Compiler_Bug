plain
2020-03-24T15:10:40.1909903Z ========================== Starting Command Output ===========================
2020-03-24T15:10:40.1913685Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/116ef68b-1f83-4185-90fa-dfd73fcd43f7.sh
2020-03-24T15:10:40.1914139Z 
2020-03-24T15:10:40.1919152Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-24T15:10:40.1939914Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70107/merge to s
2020-03-24T15:10:40.1943522Z Task         : Get sources
2020-03-24T15:10:40.1943837Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-24T15:10:40.1944173Z Version      : 1.0.0
2020-03-24T15:10:40.1944382Z Author       : Microsoft
---
2020-03-24T15:10:41.4266057Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-24T15:10:41.4273574Z ##[command]git config gc.auto 0
2020-03-24T15:10:41.4277444Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-24T15:10:41.4280990Z ##[command]git config --get-all http.proxy
2020-03-24T15:10:41.4288562Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70107/merge:refs/remotes/pull/70107/merge
---
2020-03-24T15:17:53.0505305Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-24T15:17:53.4257246Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-24T15:17:55.5063160Z     Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-03-24T15:17:56.2247346Z     Checking rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-03-24T15:17:56.3365358Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-24T15:17:58.0682192Z     Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
2020-03-24T15:18:20.9107696Z     Checking rustc_infer v0.0.0 (/checkout/src/librustc_infer)
2020-03-24T15:18:23.8975036Z     Checking rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
2020-03-24T15:18:24.7409709Z     Checking rustc_symbol_mangling v0.0.0 (/checkout/src/librustc_symbol_mangling)
---
2020-03-24T15:19:50.9612831Z configure: build.locked-deps    := True
2020-03-24T15:19:50.9613329Z configure: llvm.ccache          := sccache
2020-03-24T15:19:50.9613997Z configure: build.cargo-native-static := True
2020-03-24T15:19:50.9614613Z configure: dist.missing-tools   := True
2020-03-24T15:19:50.9615359Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-03-24T15:19:50.9616178Z configure: writing `config.toml` in current directory
2020-03-24T15:19:50.9616532Z configure: 
2020-03-24T15:19:50.9617101Z configure: run `python /checkout/x.py --help`
2020-03-24T15:19:50.9617448Z configure: 
---
2020-03-24T15:23:26.4982746Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-24T15:23:27.0444714Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-24T15:23:29.0789990Z     Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-03-24T15:23:29.8186277Z     Checking rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-03-24T15:23:29.8909610Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-24T15:23:31.6662491Z     Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
2020-03-24T15:23:53.4341018Z     Checking rustc_infer v0.0.0 (/checkout/src/librustc_infer)
2020-03-24T15:23:56.5289435Z     Checking rustc_save_analysis v0.0.0 (/checkout/src/librustc_save_analysis)
2020-03-24T15:23:56.6269232Z     Checking rustc_symbol_mangling v0.0.0 (/checkout/src/librustc_symbol_mangling)
---
2020-03-24T15:27:33.4710500Z skip untracked path cpu-usage.csv during rustfmt invocations
2020-03-24T15:27:33.4711071Z skip untracked path src/doc/book/ during rustfmt invocations
2020-03-24T15:27:33.4715231Z skip untracked path src/doc/rust-by-example/ during rustfmt invocations
2020-03-24T15:27:33.4720645Z skip untracked path src/llvm-project/ during rustfmt invocations
2020-03-24T15:27:38.4360866Z Diff in /checkout/src/librustc_typeck/check/mod.rs at line 3289:
2020-03-24T15:27:38.4368476Z      pub fn to_const(&self, ast_c: &hir::AnonConst) -> &'tcx ty::Const<'tcx> {
2020-03-24T15:27:38.4373700Z          let c_def = self.tcx.hir().local_def_id(ast_c.hir_id);
2020-03-24T15:27:38.4378312Z          let constant = ty::Const::from_anon_const(self.tcx, c_def.expect_local());
2020-03-24T15:27:38.4383510Z -        self.register_const_wf_obligation(constant, self.tcx.def_span(c_def), ObligationCauseCode::MiscObligation);
2020-03-24T15:27:38.4389259Z +        self.register_const_wf_obligation(
2020-03-24T15:27:38.4393805Z +            constant,
2020-03-24T15:27:38.4431434Z +            self.tcx.def_span(c_def),
2020-03-24T15:27:38.4432221Z +            ObligationCauseCode::MiscObligation,
2020-03-24T15:27:38.4432806Z          constant
2020-03-24T15:27:38.4433210Z      }
2020-03-24T15:27:38.4433337Z  
2020-03-24T15:27:38.4433337Z  
2020-03-24T15:27:38.4442790Z Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustc_typeck/check/mod.rs"` failed.
2020-03-24T15:27:38.4443797Z If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
2020-03-24T15:27:38.4459298Z Build completed unsuccessfully in 0:00:38
2020-03-24T15:27:38.4520282Z == clock drift check ==
2020-03-24T15:27:38.4538351Z   local time: Tue Mar 24 15:27:38 UTC 2020
2020-03-24T15:27:38.7449278Z   network time: Tue, 24 Mar 2020 15:27:38 GMT
2020-03-24T15:27:38.7449278Z   network time: Tue, 24 Mar 2020 15:27:38 GMT
2020-03-24T15:27:38.7449853Z == end clock drift check ==
2020-03-24T15:27:40.3196078Z 
2020-03-24T15:27:40.3272711Z ##[error]Bash exited with code '1'.
2020-03-24T15:27:40.3291391Z ##[section]Finishing: Run build
2020-03-24T15:27:40.3343323Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70107/merge to s
2020-03-24T15:27:40.3350129Z Task         : Get sources
2020-03-24T15:27:40.3350504Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-24T15:27:40.3350826Z Version      : 1.0.0
2020-03-24T15:27:40.3351060Z Author       : Microsoft
2020-03-24T15:27:40.3351060Z Author       : Microsoft
2020-03-24T15:27:40.3351459Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-24T15:27:40.3351871Z ==============================================================================
2020-03-24T15:27:40.6648955Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-24T15:27:40.6705673Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70107/merge to s
2020-03-24T15:27:40.6804707Z Cleaning up task key
2020-03-24T15:27:40.6806003Z Start cleaning up orphan processes.
2020-03-24T15:27:40.7013422Z Terminate orphan process: pid (3941) (python)
2020-03-24T15:27:40.7648628Z ##[section]Finishing: Finalize Job
