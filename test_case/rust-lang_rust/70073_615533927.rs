plain
2020-04-18T01:08:11.2897159Z ========================== Starting Command Output ===========================
2020-04-18T01:08:11.2902351Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/7a312f20-afbf-45cd-9f4a-43af5ced95fd.sh
2020-04-18T01:08:11.2902824Z 
2020-04-18T01:08:11.2907613Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-18T01:08:11.2934805Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70073/merge to s
2020-04-18T01:08:11.2938946Z Task         : Get sources
2020-04-18T01:08:11.2939259Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-18T01:08:11.2939563Z Version      : 1.0.0
2020-04-18T01:08:11.2939775Z Author       : Microsoft
---
2020-04-18T01:08:12.4660220Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-18T01:08:12.4671888Z ##[command]git config gc.auto 0
2020-04-18T01:08:12.4678205Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-18T01:08:12.4685641Z ##[command]git config --get-all http.proxy
2020-04-18T01:08:12.4696640Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70073/merge:refs/remotes/pull/70073/merge
---
2020-04-18T01:10:33.2604562Z  ---> 78ad2f4d4aca
2020-04-18T01:10:33.2604841Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-18T01:10:33.2614409Z  ---> Using cache
2020-04-18T01:10:33.2614856Z  ---> 4d2dc61c4d00
2020-04-18T01:10:33.2616294Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-18T01:10:33.2637406Z  ---> 776b6266a8b7
2020-04-18T01:10:33.2699377Z Successfully built 776b6266a8b7
2020-04-18T01:10:33.2726884Z Successfully tagged rust-ci:latest
2020-04-18T01:10:33.3031572Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-18T01:10:33.3031572Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-18T01:10:33.3045521Z Looks like docker image is the same as before, not uploading
2020-04-18T01:10:39.8803547Z [CI_JOB_NAME=mingw-check]
2020-04-18T01:10:39.9104825Z [CI_JOB_NAME=mingw-check]
2020-04-18T01:10:39.9134720Z == clock drift check ==
2020-04-18T01:10:39.9151618Z   local time: Sat Apr 18 01:10:39 UTC 2020
2020-04-18T01:10:40.1085116Z   network time: Sat, 18 Apr 2020 01:10:40 GMT
2020-04-18T01:10:40.1110621Z Starting sccache server...
2020-04-18T01:10:40.2318363Z configure: processing command line
2020-04-18T01:10:40.2318634Z configure: 
2020-04-18T01:10:40.2319556Z configure: rust.parallel-compiler := True
---
2020-04-18T01:14:43.7698733Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-18T01:14:43.9053200Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-18T01:14:44.1075778Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-18T01:14:44.2214729Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-18T01:14:44.8079744Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-18T01:14:47.3891456Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-04-18T01:14:47.9051046Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-18T01:14:50.1439272Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-18T01:14:50.6365574Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-04-18T01:15:28.9212879Z     Checking rustc_mir v0.0.0 (/checkout/src/librustc_mir)
2020-04-18T01:15:30.0415658Z error[E0405]: cannot find trait `MirPas` in this scope
2020-04-18T01:15:30.0416336Z    --> src/librustc_mir/transform/mod.rs:313:34
2020-04-18T01:15:30.0416997Z     |
2020-04-18T01:15:30.0417639Z 129 | / pub trait MirPass<'tcx> {
2020-04-18T01:15:30.0418601Z 130 | |     fn name(&self) -> Cow<'_, str> {
2020-04-18T01:15:30.0419420Z 131 | |         default_name::<Self>()
2020-04-18T01:15:30.0420728Z 133 | |
2020-04-18T01:15:30.0420728Z 133 | |
2020-04-18T01:15:30.0421620Z 134 | |     fn run_pass(&self, tcx: TyCtxt<'tcx>, source: MirSource<'tcx>, body: &mut BodyAndCache<'tcx>);
2020-04-18T01:15:30.0423313Z     | |_- similarly named trait `MirPass` defined here
2020-04-18T01:15:30.0423848Z ...
2020-04-18T01:15:30.0423848Z ...
2020-04-18T01:15:30.0424483Z 313 |       let no_optimizations: &[&dyn MirPas<'tcx>] = &[
2020-04-18T01:15:30.0425913Z 
2020-04-18T01:15:34.2718094Z     Checking rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
2020-04-18T01:15:34.4478059Z     Checking rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
2020-04-18T01:15:36.1915113Z error: aborting due to previous error
2020-04-18T01:15:36.1915113Z error: aborting due to previous error
2020-04-18T01:15:36.1916085Z 
2020-04-18T01:15:36.1917235Z For more information about this error, try `rustc --explain E0405`.
2020-04-18T01:15:36.2114935Z error: could not compile `rustc_mir`.
2020-04-18T01:15:36.2115809Z 
2020-04-18T01:15:36.2116581Z To learn more, run the command again with --verbose.
2020-04-18T01:15:36.2170525Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-18T01:15:36.2197131Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-04-18T01:15:36.2197523Z Build completed unsuccessfully in 0:04:55
2020-04-18T01:15:36.2310392Z == clock drift check ==
2020-04-18T01:15:36.2326926Z   local time: Sat Apr 18 01:15:36 UTC 2020
2020-04-18T01:15:36.2326926Z   local time: Sat Apr 18 01:15:36 UTC 2020
2020-04-18T01:15:36.4269857Z   network time: Sat, 18 Apr 2020 01:15:36 GMT
2020-04-18T01:15:37.1744312Z 
2020-04-18T01:15:37.1744312Z 
2020-04-18T01:15:37.1830971Z ##[error]Bash exited with code '1'.
2020-04-18T01:15:37.1846260Z ##[section]Finishing: Run build
2020-04-18T01:15:37.1893337Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70073/merge to s
2020-04-18T01:15:37.1898864Z Task         : Get sources
2020-04-18T01:15:37.1899214Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-18T01:15:37.1899556Z Version      : 1.0.0
2020-04-18T01:15:37.1899786Z Author       : Microsoft
2020-04-18T01:15:37.1899786Z Author       : Microsoft
2020-04-18T01:15:37.1900144Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-18T01:15:37.1900572Z ==============================================================================
2020-04-18T01:15:37.5814902Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-18T01:15:37.5870166Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70073/merge to s
2020-04-18T01:15:37.5974165Z Cleaning up task key
2020-04-18T01:15:37.5975765Z Start cleaning up orphan processes.
2020-04-18T01:15:37.6190399Z Terminate orphan process: pid (3730) (python)
2020-04-18T01:15:37.6396063Z ##[section]Finishing: Finalize Job
