plain
2020-03-20T17:05:21.1826693Z ========================== Starting Command Output ===========================
2020-03-20T17:05:21.1829151Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/88fd1f7b-b18a-492f-ad73-3448f16e3103.sh
2020-03-20T17:05:21.1829436Z 
2020-03-20T17:05:21.1833221Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-20T17:05:21.1882219Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69901/merge to s
2020-03-20T17:05:21.1885788Z Task         : Get sources
2020-03-20T17:05:21.1886108Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-20T17:05:21.1886413Z Version      : 1.0.0
2020-03-20T17:05:21.1886636Z Author       : Microsoft
---
2020-03-20T17:05:22.1784039Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-20T17:05:22.1790154Z ##[command]git config gc.auto 0
2020-03-20T17:05:22.1795719Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-20T17:05:22.1799098Z ##[command]git config --get-all http.proxy
2020-03-20T17:05:22.1805071Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69901/merge:refs/remotes/pull/69901/merge
---
2020-03-20T17:14:46.4633167Z configure: build.locked-deps    := True
2020-03-20T17:14:46.4633466Z configure: llvm.ccache          := sccache
2020-03-20T17:14:46.4633942Z configure: build.cargo-native-static := True
2020-03-20T17:14:46.4634447Z configure: dist.missing-tools   := True
2020-03-20T17:14:46.4635039Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-03-20T17:14:46.4635623Z configure: writing `config.toml` in current directory
2020-03-20T17:14:46.4635862Z configure: 
2020-03-20T17:14:46.4636263Z configure: run `python /checkout/x.py --help`
2020-03-20T17:14:46.4636511Z configure: 
---
2020-03-20T17:22:03.3302884Z skip untracked path cpu-usage.csv during rustfmt invocations
2020-03-20T17:22:03.3307035Z skip untracked path src/doc/book/ during rustfmt invocations
2020-03-20T17:22:03.3312770Z skip untracked path src/doc/rust-by-example/ during rustfmt invocations
2020-03-20T17:22:03.3319040Z skip untracked path src/llvm-project/ during rustfmt invocations
2020-03-20T17:22:05.5219643Z Diff in /checkout/src/librustc_passes/layout_test.rs at line 30:
2020-03-20T17:22:05.5220093Z          let item_def_id = self.tcx.hir().local_def_id(item.hir_id);
2020-03-20T17:22:05.5220530Z          match item.kind {
2020-03-20T17:22:05.5221443Z -            ItemKind::TyAlias(..) |
2020-03-20T17:22:05.5221929Z -            ItemKind::Enum(..) |
2020-03-20T17:22:05.5222363Z -            ItemKind::Struct(..) |
2020-03-20T17:22:05.5222363Z -            ItemKind::Struct(..) |
2020-03-20T17:22:05.5222810Z -            ItemKind::Union(..) => {
2020-03-20T17:22:05.5223094Z +            ItemKind::TyAlias(..)
2020-03-20T17:22:05.5228299Z +            | ItemKind::Enum(..)
2020-03-20T17:22:05.5232314Z +            | ItemKind::Struct(..)
2020-03-20T17:22:05.5236286Z +            | ItemKind::Union(..) => {
2020-03-20T17:22:05.5240255Z                  for attr in self.tcx.get_attrs(item_def_id).iter() {
2020-03-20T17:22:05.5249740Z                      if attr.check_name(sym::rustc_layout) {
2020-03-20T17:22:05.5250133Z                          self.dump_layout_of(item_def_id, item, attr);
2020-03-20T17:22:05.5269556Z Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustc_passes/layout_test.rs"` failed.
2020-03-20T17:22:05.5270637Z If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
2020-03-20T17:22:05.5271547Z Build completed unsuccessfully in 0:00:36
2020-03-20T17:22:05.5322862Z == clock drift check ==
2020-03-20T17:22:05.5352848Z   local time: Fri Mar 20 17:22:05 UTC 2020
2020-03-20T17:22:05.6942754Z   network time: Fri, 20 Mar 2020 17:22:05 GMT
2020-03-20T17:22:05.6942754Z   network time: Fri, 20 Mar 2020 17:22:05 GMT
2020-03-20T17:22:05.6945232Z == end clock drift check ==
2020-03-20T17:22:07.2813246Z 
2020-03-20T17:22:07.2880697Z ##[error]Bash exited with code '1'.
2020-03-20T17:22:07.2894620Z ##[section]Finishing: Run build
2020-03-20T17:22:07.2943787Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69901/merge to s
2020-03-20T17:22:07.2948769Z Task         : Get sources
2020-03-20T17:22:07.2949137Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-20T17:22:07.2949472Z Version      : 1.0.0
2020-03-20T17:22:07.2949704Z Author       : Microsoft
2020-03-20T17:22:07.2949704Z Author       : Microsoft
2020-03-20T17:22:07.2950081Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-20T17:22:07.2950494Z ==============================================================================
2020-03-20T17:22:07.6236549Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-20T17:22:07.6280733Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69901/merge to s
2020-03-20T17:22:07.6371689Z Cleaning up task key
2020-03-20T17:22:07.6373065Z Start cleaning up orphan processes.
2020-03-20T17:22:07.6552423Z Terminate orphan process: pid (3460) (python)
2020-03-20T17:22:07.6811839Z ##[section]Finishing: Finalize Job
