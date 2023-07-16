plain
2020-03-23T14:52:17.2433341Z ========================== Starting Command Output ===========================
2020-03-23T14:52:17.2438548Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/62529456-3480-471f-86e8-f4847b27e0e0.sh
2020-03-23T14:52:17.2439082Z 
2020-03-23T14:52:17.2444663Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-23T14:52:17.2464836Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70319/merge to s
2020-03-23T14:52:17.2468100Z Task         : Get sources
2020-03-23T14:52:17.2468412Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-23T14:52:17.2468712Z Version      : 1.0.0
2020-03-23T14:52:17.2468932Z Author       : Microsoft
---
2020-03-23T14:52:18.2338220Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-23T14:52:18.2344929Z ##[command]git config gc.auto 0
2020-03-23T14:52:18.2349969Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-23T14:52:18.2355448Z ##[command]git config --get-all http.proxy
2020-03-23T14:52:18.2362995Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70319/merge:refs/remotes/pull/70319/merge
---
2020-03-23T15:00:31.1042196Z configure: build.locked-deps    := True
2020-03-23T15:00:31.1042502Z configure: llvm.ccache          := sccache
2020-03-23T15:00:31.1043007Z configure: build.cargo-native-static := True
2020-03-23T15:00:31.1043472Z configure: dist.missing-tools   := True
2020-03-23T15:00:31.1044144Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-03-23T15:00:31.1044713Z configure: writing `config.toml` in current directory
2020-03-23T15:00:31.1044960Z configure: 
2020-03-23T15:00:31.1045392Z configure: run `python /checkout/x.py --help`
2020-03-23T15:00:31.1045624Z configure: 
---
2020-03-23T15:07:55.4420291Z skip untracked path cpu-usage.csv during rustfmt invocations
2020-03-23T15:07:55.4425441Z skip untracked path src/doc/book/ during rustfmt invocations
2020-03-23T15:07:55.4429910Z skip untracked path src/doc/rust-by-example/ during rustfmt invocations
2020-03-23T15:07:55.4434625Z skip untracked path src/llvm-project/ during rustfmt invocations
2020-03-23T15:07:58.5455029Z Diff in /checkout/src/librustc/ty/normalize_erasing_regions.rs at line 94:
2020-03-23T15:07:58.5455590Z  
2020-03-23T15:07:58.5457169Z      fn fold_ty(&mut self, ty: Ty<'tcx>) -> Ty<'tcx> {
2020-03-23T15:07:58.5457169Z      fn fold_ty(&mut self, ty: Ty<'tcx>) -> Ty<'tcx> {
2020-03-23T15:07:58.5457930Z -        self.tcx.normalize_generic_arg_after_erasing_regions(self.param_env.and(GenericArg::from(ty))).expect_ty()
2020-03-23T15:07:58.5458334Z +        self.tcx
2020-03-23T15:07:58.5458734Z +            .normalize_generic_arg_after_erasing_regions(self.param_env.and(GenericArg::from(ty)))
2020-03-23T15:07:58.5459139Z +            .expect_ty()
2020-03-23T15:07:58.5459438Z  
2020-03-23T15:07:58.5459438Z  
2020-03-23T15:07:58.5459974Z      fn fold_const(&mut self, c: &'tcx ty::Const<'tcx>) -> &'tcx ty::Const<'tcx> {
2020-03-23T15:07:58.5460411Z Diff in /checkout/src/librustc/ty/normalize_erasing_regions.rs at line 101:
2020-03-23T15:07:58.5461143Z -        self.tcx.normalize_generic_arg_after_erasing_regions(self.param_env.and(GenericArg::from(c))).expect_const()
2020-03-23T15:07:58.5461555Z +        self.tcx
2020-03-23T15:07:58.5461915Z +            .normalize_generic_arg_after_erasing_regions(self.param_env.and(GenericArg::from(c)))
2020-03-23T15:07:58.5462295Z +            .expect_const()
2020-03-23T15:07:58.5462618Z  }
2020-03-23T15:07:58.5462737Z  
2020-03-23T15:07:58.5462737Z  
2020-03-23T15:07:58.5463655Z Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustc/ty/normalize_erasing_regions.rs"` failed.
2020-03-23T15:07:58.5464639Z If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
2020-03-23T15:07:58.5479595Z Build completed unsuccessfully in 0:00:39
2020-03-23T15:07:58.5527249Z == clock drift check ==
2020-03-23T15:07:58.5548908Z   local time: Mon Mar 23 15:07:58 UTC 2020
2020-03-23T15:07:58.8433414Z   network time: Mon, 23 Mar 2020 15:07:58 GMT
2020-03-23T15:07:58.8433414Z   network time: Mon, 23 Mar 2020 15:07:58 GMT
2020-03-23T15:07:58.8437368Z == end clock drift check ==
2020-03-23T15:08:00.3979893Z 
2020-03-23T15:08:00.4047088Z ##[error]Bash exited with code '1'.
2020-03-23T15:08:00.4060656Z ##[section]Finishing: Run build
2020-03-23T15:08:00.4110830Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70319/merge to s
2020-03-23T15:08:00.4116096Z Task         : Get sources
2020-03-23T15:08:00.4116462Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-23T15:08:00.4116781Z Version      : 1.0.0
2020-03-23T15:08:00.4117008Z Author       : Microsoft
2020-03-23T15:08:00.4117008Z Author       : Microsoft
2020-03-23T15:08:00.4117385Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-23T15:08:00.4117798Z ==============================================================================
2020-03-23T15:08:00.7509213Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-23T15:08:00.7575040Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70319/merge to s
2020-03-23T15:08:00.7669169Z Cleaning up task key
2020-03-23T15:08:00.7670578Z Start cleaning up orphan processes.
2020-03-23T15:08:00.7869752Z Terminate orphan process: pid (3423) (python)
2020-03-23T15:08:00.8190972Z ##[section]Finishing: Finalize Job
