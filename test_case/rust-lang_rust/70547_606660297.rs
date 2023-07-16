plain
2020-03-31T12:28:09.4207024Z ========================== Starting Command Output ===========================
2020-03-31T12:28:09.4209954Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/fefdbd16-ff8c-4dd5-8fd9-1f201e838b01.sh
2020-03-31T12:28:09.4210184Z 
2020-03-31T12:28:09.4214167Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-31T12:28:09.4232483Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70547/merge to s
2020-03-31T12:28:09.4236018Z Task         : Get sources
2020-03-31T12:28:09.4236238Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-31T12:28:09.4236449Z Version      : 1.0.0
2020-03-31T12:28:09.4236592Z Author       : Microsoft
---
2020-03-31T12:28:10.4128921Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-31T12:28:10.4133865Z ##[command]git config gc.auto 0
2020-03-31T12:28:10.4137298Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-31T12:28:10.4140301Z ##[command]git config --get-all http.proxy
2020-03-31T12:28:10.4145741Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70547/merge:refs/remotes/pull/70547/merge
---
2020-03-31T12:30:30.9268893Z  ---> 3fc1b512c57b
2020-03-31T12:30:30.9269202Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-03-31T12:30:30.9269683Z  ---> Using cache
2020-03-31T12:30:30.9270020Z  ---> 5ee4295733f4
2020-03-31T12:30:30.9271852Z Step 7/7 : ENV SCRIPT python2.7 ../x.py test src/tools/expand-yaml-anchors &&            python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python2.7 ../x.py build --stage 0 src/tools/build-manifest &&            python2.7 ../x.py test --stage 0 src/tools/compiletest &&            python2.7 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-03-31T12:30:30.9275080Z  ---> 3d07a0fa42fe
2020-03-31T12:30:30.9302167Z Successfully built 3d07a0fa42fe
2020-03-31T12:30:30.9336648Z Successfully tagged rust-ci:latest
2020-03-31T12:30:30.9575085Z Built container sha256:3d07a0fa42feb5754fc13bb2f7010ebe13e4b8b8cdbebed0c75d8da320c8c8ad
---
2020-03-31T12:33:29.8898982Z     Checking rustc_span v0.0.0 (/checkout/src/librustc_span)
2020-03-31T12:33:33.4945248Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2020-03-31T12:33:34.4587823Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-03-31T12:33:34.4941432Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-31T12:33:34.6627487Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-31T12:33:35.3145468Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-31T12:33:35.3525178Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-03-31T12:33:36.5284109Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-31T12:33:36.9327151Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
---
2020-03-31T12:35:03.9724156Z configure: build.locked-deps    := True
2020-03-31T12:35:03.9724375Z configure: llvm.ccache          := sccache
2020-03-31T12:35:03.9724752Z configure: build.cargo-native-static := True
2020-03-31T12:35:03.9725070Z configure: dist.missing-tools   := True
2020-03-31T12:35:03.9725467Z configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
2020-03-31T12:35:03.9725864Z configure: writing `config.toml` in current directory
2020-03-31T12:35:03.9726028Z configure: 
2020-03-31T12:35:03.9726318Z configure: run `python /checkout/x.py --help`
2020-03-31T12:35:03.9726474Z configure: 
---
2020-03-31T12:36:11.3736928Z Hugepagesize:       2048 kB
2020-03-31T12:36:11.3737152Z DirectMap4k:      153536 kB
2020-03-31T12:36:11.3737358Z DirectMap2M:     4040704 kB
2020-03-31T12:36:11.3737564Z DirectMap1G:     5242880 kB
2020-03-31T12:36:11.3738389Z + python2.7 ../x.py test src/tools/expand-yaml-anchors
2020-03-31T12:36:12.5636216Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-03-31T12:36:12.5636216Z Ensuring the YAML anchors in the GitHub Actions config were expanded
2020-03-31T12:36:12.5637861Z Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
2020-03-31T12:36:12.7702628Z    Compiling unicode-xid v0.2.0
2020-03-31T12:36:12.8841010Z    Compiling syn v1.0.11
2020-03-31T12:36:13.6189149Z    Compiling linked-hash-map v0.5.2
2020-03-31T12:36:13.6564222Z    Compiling lazy_static v1.4.0
2020-03-31T12:36:13.6564222Z    Compiling lazy_static v1.4.0
2020-03-31T12:36:13.8203257Z    Compiling yaml-rust v0.4.3
2020-03-31T12:36:17.5776663Z    Compiling quote v1.0.2
2020-03-31T12:36:30.7605688Z    Compiling thiserror-impl v1.0.5
2020-03-31T12:36:34.8657647Z    Compiling thiserror v1.0.5
2020-03-31T12:36:34.9223658Z    Compiling yaml-merge-keys v0.4.0
2020-03-31T12:36:35.9439801Z    Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
2020-03-31T12:36:37.3515649Z Build completed successfully in 0:00:25
2020-03-31T12:36:37.3528071Z + python2.7 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
2020-03-31T12:36:37.5675225Z     Finished dev [unoptimized] target(s) in 0.15s
2020-03-31T12:36:38.5851497Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
2020-03-31T12:38:20.4058625Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-03-31T12:38:20.4925143Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-03-31T12:38:20.6414115Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-03-31T12:38:20.7164544Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-03-31T12:38:21.0952145Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-03-31T12:38:22.7127912Z     Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
2020-03-31T12:38:23.0802037Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-03-31T12:38:24.6660749Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-03-31T12:38:24.9956519Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
2020-03-31T12:41:34.9892949Z skip untracked path src/doc/rust-by-example/ during rustfmt invocations
2020-03-31T12:41:34.9897067Z skip untracked path src/llvm-project/ during rustfmt invocations
2020-03-31T12:41:36.0691854Z Diff in /checkout/src/librustc_codegen_llvm/attributes.rs at line 237:
2020-03-31T12:41:36.0694282Z  
2020-03-31T12:41:36.0694573Z  /// Composite function which sets LLVM attributes for function depending on its AST (`#[attribute]`)
2020-03-31T12:41:36.0696461Z -pub fn from_fn_attrs(
2020-03-31T12:41:36.0696461Z -pub fn from_fn_attrs(
2020-03-31T12:41:36.0696844Z -    cx: &CodegenCx<'ll, 'tcx>,
2020-03-31T12:41:36.0697371Z -    llfn: &'ll Value,
2020-03-31T12:41:36.0698923Z -) {
2020-03-31T12:41:36.0698923Z -) {
2020-03-31T12:41:36.0699967Z +pub fn from_fn_attrs(cx: &CodegenCx<'ll, 'tcx>, llfn: &'ll Value, instance: ty::Instance<'tcx>) {
2020-03-31T12:41:36.0701331Z      let codegen_fn_attrs = cx.tcx.codegen_fn_attrs(instance.def_id());
2020-03-31T12:41:36.0701785Z      match codegen_fn_attrs.optimize {
2020-03-31T12:41:36.0701785Z      match codegen_fn_attrs.optimize {
2020-03-31T12:41:36.0702861Z Running `"/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustfmt" "--config-path" "/checkout" "--edition" "2018" "--unstable-features" "--skip-children" "--check" "/checkout/src/librustc_codegen_llvm/attributes.rs"` failed.
2020-03-31T12:41:36.0703840Z If you're running `tidy`, try again with `--bless` flag. Or, you just want to format code, run `./x.py fmt` instead.
2020-03-31T12:41:36.0712604Z Build completed unsuccessfully in 0:00:33
2020-03-31T12:41:36.0760403Z == clock drift check ==
2020-03-31T12:41:36.0773580Z   local time: Tue Mar 31 12:41:36 UTC 2020
2020-03-31T12:41:36.2328167Z   network time: Tue, 31 Mar 2020 12:41:36 GMT
2020-03-31T12:41:36.2328167Z   network time: Tue, 31 Mar 2020 12:41:36 GMT
2020-03-31T12:41:36.2332498Z == end clock drift check ==
2020-03-31T12:41:37.8212571Z 
2020-03-31T12:41:37.8285172Z ##[error]Bash exited with code '1'.
2020-03-31T12:41:37.8308911Z ##[section]Finishing: Run build
2020-03-31T12:41:37.8383517Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70547/merge to s
2020-03-31T12:41:37.8391523Z Task         : Get sources
2020-03-31T12:41:37.8392151Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-31T12:41:37.8392755Z Version      : 1.0.0
2020-03-31T12:41:37.8393186Z Author       : Microsoft
2020-03-31T12:41:37.8393186Z Author       : Microsoft
2020-03-31T12:41:37.8393823Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-31T12:41:37.8394572Z ==============================================================================
2020-03-31T12:41:38.1190067Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-31T12:41:38.1241344Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70547/merge to s
2020-03-31T12:41:38.1324411Z Cleaning up task key
2020-03-31T12:41:38.1325577Z Start cleaning up orphan processes.
2020-03-31T12:41:38.1549891Z Terminate orphan process: pid (3527) (python)
2020-03-31T12:41:38.1720446Z ##[section]Finishing: Finalize Job
