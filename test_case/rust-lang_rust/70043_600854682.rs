plain
2020-03-18T19:53:52.1089330Z ========================== Starting Command Output ===========================
2020-03-18T19:53:52.1137663Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/b2c61b24-4f7a-4084-a577-9a82b16129a1.sh
2020-03-18T19:53:52.1137959Z 
2020-03-18T19:53:52.1143440Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-18T19:53:52.1163636Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70043/merge to s
2020-03-18T19:53:52.1167249Z Task         : Get sources
2020-03-18T19:53:52.1167543Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-18T19:53:52.1167824Z Version      : 1.0.0
2020-03-18T19:53:52.1168044Z Author       : Microsoft
---
2020-03-18T19:53:53.1150616Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-18T19:53:53.1159099Z ##[command]git config gc.auto 0
2020-03-18T19:53:53.1166286Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-18T19:53:53.1172593Z ##[command]git config --get-all http.proxy
2020-03-18T19:53:53.1182416Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70043/merge:refs/remotes/pull/70043/merge
---
2020-03-18T20:02:26.7152106Z configure: build.locked-deps    := True
2020-03-18T20:02:26.7152455Z configure: llvm.ccache          := sccache
2020-03-18T20:02:26.7152990Z configure: build.cargo-native-static := True
2020-03-18T20:02:26.7153510Z configure: dist.missing-tools   := True
2020-03-18T20:02:26.7154206Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-03-18T20:02:26.7154826Z configure: writing `config.toml` in current directory
2020-03-18T20:02:26.7155109Z configure: 
2020-03-18T20:02:26.7155564Z configure: run `python /checkout/x.py --help`
2020-03-18T20:02:26.7155825Z configure: 
---
2020-03-18T20:09:54.8580155Z skip untracked path cpu-usage.csv during rustfmt invocations
2020-03-18T20:09:54.8589736Z skip untracked path src/doc/book/ during rustfmt invocations
2020-03-18T20:09:54.8590592Z skip untracked path src/doc/rust-by-example/ during rustfmt invocations
2020-03-18T20:09:54.8591301Z skip untracked path src/llvm-project/ during rustfmt invocations
2020-03-18T20:09:56.7947117Z Diff in /checkout/src/librustc_resolve/build_reduced_graph.rs at line 853:
2020-03-18T20:09:56.7947707Z          let expansion = ExpnId::root(); // FIXME(jseyfried) intercrate hygiene
2020-03-18T20:09:56.7948091Z          // Record primary definitions.
2020-03-18T20:09:56.7949459Z -            Res::Def(
2020-03-18T20:09:56.7949459Z -            Res::Def(
2020-03-18T20:09:56.7950073Z -                kind @ (DefKind::Mod | DefKind::Enum | DefKind::Trait),
2020-03-18T20:09:56.7951037Z -            ) => {
2020-03-18T20:09:56.7951037Z -            ) => {
2020-03-18T20:09:56.7951423Z +            Res::Def(kind @ (DefKind::Mod | DefKind::Enum | DefKind::Trait), def_id) => {
2020-03-18T20:09:56.7951864Z                  let module = self.r.new_module(
2020-03-18T20:09:56.7952689Z                      ModuleKind::Def(kind, def_id, ident.name),
2020-03-18T20:09:56.7952689Z                      ModuleKind::Def(kind, def_id, ident.name),
2020-03-18T20:09:56.7957573Z Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustc_resolve/build_reduced_graph.rs"` failed.
2020-03-18T20:09:56.7958746Z If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
2020-03-18T20:09:56.7967457Z Build completed unsuccessfully in 0:00:05
2020-03-18T20:09:56.8023364Z == clock drift check ==
2020-03-18T20:09:56.8038762Z   local time: Wed Mar 18 20:09:56 UTC 2020
2020-03-18T20:09:57.0907751Z   network time: Wed, 18 Mar 2020 20:09:57 GMT
2020-03-18T20:09:57.0907751Z   network time: Wed, 18 Mar 2020 20:09:57 GMT
2020-03-18T20:09:57.0908444Z == end clock drift check ==
2020-03-18T20:09:57.8006729Z 
2020-03-18T20:09:57.8094231Z ##[error]Bash exited with code '1'.
2020-03-18T20:09:57.8108766Z ##[section]Finishing: Run build
2020-03-18T20:09:57.8155992Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70043/merge to s
2020-03-18T20:09:57.8160882Z Task         : Get sources
2020-03-18T20:09:57.8161207Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-18T20:09:57.8161486Z Version      : 1.0.0
2020-03-18T20:09:57.8161685Z Author       : Microsoft
2020-03-18T20:09:57.8161685Z Author       : Microsoft
2020-03-18T20:09:57.8162014Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-18T20:09:57.8162372Z ==============================================================================
2020-03-18T20:09:58.1870267Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-18T20:09:58.1913400Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70043/merge to s
2020-03-18T20:09:58.1994401Z Cleaning up task key
2020-03-18T20:09:58.1995517Z Start cleaning up orphan processes.
2020-03-18T20:09:58.2196664Z Terminate orphan process: pid (3500) (python)
2020-03-18T20:09:58.2342432Z ##[section]Finishing: Finalize Job
