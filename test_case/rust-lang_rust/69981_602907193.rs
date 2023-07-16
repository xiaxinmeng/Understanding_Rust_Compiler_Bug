plain
2020-03-23T21:40:57.8004674Z ========================== Starting Command Output ===========================
2020-03-23T21:40:57.8009002Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/e1f2882b-29a4-4756-8bb8-0420bf0f9d93.sh
2020-03-23T21:40:57.8009510Z 
2020-03-23T21:40:57.8016385Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-23T21:40:57.8035123Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69981/merge to s
2020-03-23T21:40:57.8038250Z Task         : Get sources
2020-03-23T21:40:57.8038533Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-23T21:40:57.8038827Z Version      : 1.0.0
2020-03-23T21:40:57.8039015Z Author       : Microsoft
---
2020-03-23T21:40:58.8140198Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-23T21:40:58.8146788Z ##[command]git config gc.auto 0
2020-03-23T21:40:58.8151548Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-23T21:40:58.8156391Z ##[command]git config --get-all http.proxy
2020-03-23T21:40:58.8164173Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69981/merge:refs/remotes/pull/69981/merge
---
2020-03-23T21:49:17.3122457Z configure: build.locked-deps    := True
2020-03-23T21:49:17.3122756Z configure: llvm.ccache          := sccache
2020-03-23T21:49:17.3123345Z configure: build.cargo-native-static := True
2020-03-23T21:49:17.3123813Z configure: dist.missing-tools   := True
2020-03-23T21:49:17.3124369Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-03-23T21:49:17.3124883Z configure: writing `config.toml` in current directory
2020-03-23T21:49:17.3125122Z configure: 
2020-03-23T21:49:17.3125490Z configure: run `python /checkout/x.py --help`
2020-03-23T21:49:17.3125699Z configure: 
---
2020-03-23T21:56:09.8102803Z skip untracked path src/doc/rust-by-example/ during rustfmt invocations
2020-03-23T21:56:09.8105979Z skip untracked path src/llvm-project/ during rustfmt invocations
2020-03-23T21:56:12.7595602Z Diff in /checkout/src/librustc/ty/sty.rs at line 2390:
2020-03-23T21:56:12.7599864Z  
2020-03-23T21:56:12.7604144Z          let body_id = match tcx.hir().get(hir_id) {
2020-03-23T21:56:12.7608083Z              hir::Node::AnonConst(ac) => ac.body,
2020-03-23T21:56:12.7612469Z -            _ => span_bug!(tcx.def_span(def_id.to_def_id()), "from_anon_const can only process anonymous constants"),
2020-03-23T21:56:12.7617573Z +            _ => span_bug!(
2020-03-23T21:56:12.7622729Z +                tcx.def_span(def_id.to_def_id()),
2020-03-23T21:56:12.7623014Z +                "from_anon_const can only process anonymous constants"
2020-03-23T21:56:12.7653032Z          };
2020-03-23T21:56:12.7653146Z  
2020-03-23T21:56:12.7653338Z          let expr = &tcx.hir().body(body_id).value;
2020-03-23T21:56:12.7653338Z          let expr = &tcx.hir().body(body_id).value;
2020-03-23T21:56:12.7664037Z Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustc/ty/sty.rs"` failed.
2020-03-23T21:56:12.7665401Z If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
2020-03-23T21:56:12.7679497Z Build completed unsuccessfully in 0:00:35
2020-03-23T21:56:12.7727987Z == clock drift check ==
2020-03-23T21:56:12.7741400Z   local time: Mon Mar 23 21:56:12 UTC 2020
2020-03-23T21:56:13.3137507Z   network time: Mon, 23 Mar 2020 21:56:13 GMT
2020-03-23T21:56:13.3137507Z   network time: Mon, 23 Mar 2020 21:56:13 GMT
2020-03-23T21:56:13.3142349Z == end clock drift check ==
2020-03-23T21:56:14.7972934Z 
2020-03-23T21:56:14.8037810Z ##[error]Bash exited with code '1'.
2020-03-23T21:56:14.8051161Z ##[section]Finishing: Run build
2020-03-23T21:56:14.8095397Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69981/merge to s
2020-03-23T21:56:14.8099967Z Task         : Get sources
2020-03-23T21:56:14.8100681Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-23T21:56:14.8100993Z Version      : 1.0.0
2020-03-23T21:56:14.8101224Z Author       : Microsoft
2020-03-23T21:56:14.8101224Z Author       : Microsoft
2020-03-23T21:56:14.8101554Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-23T21:56:14.8101935Z ==============================================================================
2020-03-23T21:56:15.1136056Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-23T21:56:15.1182511Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69981/merge to s
2020-03-23T21:56:15.1264545Z Cleaning up task key
2020-03-23T21:56:15.1265871Z Start cleaning up orphan processes.
2020-03-23T21:56:15.1464669Z Terminate orphan process: pid (3864) (python)
2020-03-23T21:56:15.1718387Z ##[section]Finishing: Finalize Job
