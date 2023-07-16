plain
2020-03-23T22:17:03.0522807Z ========================== Starting Command Output ===========================
2020-03-23T22:17:03.0540866Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/ec74ffb1-6882-4029-b3b3-09f7ddbb4498.sh
2020-03-23T22:17:03.0731661Z 
2020-03-23T22:17:03.0787200Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-23T22:17:03.0811327Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70335/merge to s
2020-03-23T22:17:03.0814897Z Task         : Get sources
2020-03-23T22:17:03.0815211Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-23T22:17:03.0815871Z Version      : 1.0.0
2020-03-23T22:17:03.0816379Z Author       : Microsoft
---
2020-03-23T22:17:03.8443458Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-23T22:17:03.8448852Z ##[command]git config gc.auto 0
2020-03-23T22:17:03.8452465Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-23T22:17:03.8455828Z ##[command]git config --get-all http.proxy
2020-03-23T22:17:03.8462501Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70335/merge:refs/remotes/pull/70335/merge
---
2020-03-23T22:25:13.6199513Z configure: build.locked-deps    := True
2020-03-23T22:25:13.6199819Z configure: llvm.ccache          := sccache
2020-03-23T22:25:13.6200323Z configure: build.cargo-native-static := True
2020-03-23T22:25:13.6200802Z configure: dist.missing-tools   := True
2020-03-23T22:25:13.6201380Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-03-23T22:25:13.6201967Z configure: writing `config.toml` in current directory
2020-03-23T22:25:13.6202202Z configure: 
2020-03-23T22:25:13.6202596Z configure: run `python /checkout/x.py --help`
2020-03-23T22:25:13.6202842Z configure: 
---
2020-03-23T22:32:37.8978988Z skip untracked path src/doc/rust-by-example/ during rustfmt invocations
2020-03-23T22:32:37.8979788Z skip untracked path src/llvm-project/ during rustfmt invocations
2020-03-23T22:32:41.1148539Z Diff in /checkout/src/librustc/ty/sty.rs at line 2274:
2020-03-23T22:32:41.1148821Z  
2020-03-23T22:32:41.1149067Z          let body_id = match tcx.hir().get(hir_id) {
2020-03-23T22:32:41.1149404Z              hir::Node::AnonConst(ac) => ac.body,
2020-03-23T22:32:41.1150809Z -            _ => span_bug!(tcx.def_span(def_id.to_def_id()), "from_anon_const can only process anonymous constants"),
2020-03-23T22:32:41.1151208Z +            _ => span_bug!(
2020-03-23T22:32:41.1151468Z +                tcx.def_span(def_id.to_def_id()),
2020-03-23T22:32:41.1151816Z +                "from_anon_const can only process anonymous constants"
2020-03-23T22:32:41.1152254Z          };
2020-03-23T22:32:41.1152390Z  
2020-03-23T22:32:41.1152618Z          let expr = &tcx.hir().body(body_id).value;
2020-03-23T22:32:41.1152618Z          let expr = &tcx.hir().body(body_id).value;
2020-03-23T22:32:41.1172869Z Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustc/ty/sty.rs"` failed.
2020-03-23T22:32:41.1173934Z If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
2020-03-23T22:32:41.1189769Z Build completed unsuccessfully in 0:00:39
2020-03-23T22:32:41.1237289Z == clock drift check ==
2020-03-23T22:32:41.1249322Z   local time: Mon Mar 23 22:32:41 UTC 2020
2020-03-23T22:32:41.3871624Z   network time: Mon, 23 Mar 2020 22:32:41 GMT
2020-03-23T22:32:41.3871624Z   network time: Mon, 23 Mar 2020 22:32:41 GMT
2020-03-23T22:32:41.3872796Z == end clock drift check ==
2020-03-23T22:32:42.8101405Z 
2020-03-23T22:32:42.8168995Z ##[error]Bash exited with code '1'.
2020-03-23T22:32:42.8188701Z ##[section]Finishing: Run build
2020-03-23T22:32:42.8237641Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70335/merge to s
2020-03-23T22:32:42.8242577Z Task         : Get sources
2020-03-23T22:32:42.8242922Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-23T22:32:42.8243234Z Version      : 1.0.0
2020-03-23T22:32:42.8243475Z Author       : Microsoft
2020-03-23T22:32:42.8243475Z Author       : Microsoft
2020-03-23T22:32:42.8243824Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-23T22:32:42.8244230Z ==============================================================================
2020-03-23T22:32:43.1537730Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-23T22:32:43.1588948Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70335/merge to s
2020-03-23T22:32:43.1679076Z Cleaning up task key
2020-03-23T22:32:43.1680549Z Start cleaning up orphan processes.
2020-03-23T22:32:43.1864812Z Terminate orphan process: pid (4078) (python)
2020-03-23T22:32:43.2084269Z ##[section]Finishing: Finalize Job
