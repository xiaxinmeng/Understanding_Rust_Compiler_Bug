plain
2020-04-18T07:06:22.2686903Z ========================== Starting Command Output ===========================
2020-04-18T07:06:22.2692805Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/457f5dbc-91cb-4573-8506-4302e16189b3.sh
2020-04-18T07:06:22.2693286Z 
2020-04-18T07:06:22.2697957Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-18T07:06:22.2716525Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-04-18T07:06:22.2720124Z Task         : Get sources
2020-04-18T07:06:22.2720511Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-18T07:06:22.2720757Z Version      : 1.0.0
2020-04-18T07:06:22.2720944Z Author       : Microsoft
---
2020-04-18T07:06:23.2580420Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-18T07:06:23.2585979Z ##[command]git config gc.auto 0
2020-04-18T07:06:23.2589421Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-18T07:06:23.2594183Z ##[command]git config --get-all http.proxy
2020-04-18T07:06:23.2600741Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69406/merge:refs/remotes/pull/69406/merge
---
2020-04-18T07:08:47.0316741Z  ---> 78ad2f4d4aca
2020-04-18T07:08:47.0317010Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-18T07:08:47.0330035Z  ---> Using cache
2020-04-18T07:08:47.0330455Z  ---> 4d2dc61c4d00
2020-04-18T07:08:47.0331920Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-18T07:08:47.0333447Z  ---> 776b6266a8b7
2020-04-18T07:08:47.0359605Z Successfully built 776b6266a8b7
2020-04-18T07:08:47.0392928Z Successfully tagged rust-ci:latest
2020-04-18T07:08:47.0729085Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-18T07:08:47.0729085Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-18T07:08:47.0750286Z Looks like docker image is the same as before, not uploading
2020-04-18T07:08:52.4579170Z [CI_JOB_NAME=mingw-check]
2020-04-18T07:08:52.4882618Z [CI_JOB_NAME=mingw-check]
2020-04-18T07:08:52.4917459Z == clock drift check ==
2020-04-18T07:08:52.4941993Z   local time: Sat Apr 18 07:08:52 UTC 2020
2020-04-18T07:08:52.6524785Z   network time: Sat, 18 Apr 2020 07:08:52 GMT
2020-04-18T07:08:52.6553231Z Starting sccache server...
2020-04-18T07:08:52.7663323Z configure: processing command line
2020-04-18T07:08:52.7664259Z configure: 
2020-04-18T07:08:52.7666568Z configure: rust.parallel-compiler := True
---
2020-04-18T07:11:36.9554735Z    Compiling rustc_driver v0.0.0 (/checkout/src/librustc_driver)
2020-04-18T07:11:37.0920892Z     Checking rustc_error_codes v0.0.0 (/checkout/src/librustc_error_codes)
2020-04-18T07:11:37.1799509Z    Compiling semver v0.9.0
2020-04-18T07:11:37.3972074Z     Checking crossbeam-utils v0.6.5
2020-04-18T07:11:37.7731523Z     Checking chalk-macros v0.1.1 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-18T07:11:37.9940556Z     Checking lock_api v0.3.1
2020-04-18T07:11:38.2729166Z     Checking unicode-normalization v0.1.11
2020-04-18T07:11:38.9782896Z     Checking arrayvec v0.4.7
2020-04-18T07:11:39.3702946Z     Checking itertools v0.8.0
---
2020-04-18T07:11:59.3364874Z     Checking parking_lot v0.10.0
2020-04-18T07:11:59.5465081Z     Checking rand_core v0.5.1
2020-04-18T07:11:59.6740279Z     Checking rls-span v0.5.1
2020-04-18T07:11:59.7746035Z     Checking serde_json v1.0.40
2020-04-18T07:12:00.2974931Z     Checking chalk-engine v0.9.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-18T07:12:03.0343351Z     Checking block-buffer v0.7.3
2020-04-18T07:12:03.1390235Z     Checking digest v0.8.1
2020-04-18T07:12:03.2428581Z     Checking backtrace v0.3.46
2020-04-18T07:12:03.5181683Z     Checking rand_chacha v0.2.1
---
2020-04-18T07:12:11.1914962Z    Compiling synstructure v0.12.1
2020-04-18T07:12:42.3873055Z     Checking rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
2020-04-18T07:12:44.0308458Z     Checking arena v0.0.0 (/checkout/src/libarena)
2020-04-18T07:12:53.4657978Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2020-04-18T07:12:53.4706862Z    Compiling chalk-derive v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-18T07:13:20.6246457Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2020-04-18T07:13:21.2390888Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-18T07:13:21.6874328Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-18T07:13:21.6874328Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-18T07:13:21.9161437Z     Checking chalk-ir v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-18T07:13:22.7208106Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-18T07:13:27.3265562Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-18T07:13:27.3265562Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-18T07:13:29.0268615Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-18T07:13:29.6351507Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-18T07:13:30.1490032Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-18T07:13:30.5258805Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-04-18T07:13:30.5258805Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-04-18T07:13:31.9399473Z     Checking chalk-rust-ir v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-18T07:13:33.1405020Z     Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-04-18T07:13:33.9695080Z     Checking chalk-solve v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-18T07:13:38.6054942Z     Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
2020-04-18T07:42:50.3366245Z error: could not compile `rustc_middle`.
2020-04-18T07:42:50.3405005Z 
2020-04-18T07:42:50.3405235Z Caused by:
2020-04-18T07:42:50.3405235Z Caused by:
2020-04-18T07:42:50.3459517Z   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_middle --edition=2018 src/librustc_middle/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata -C opt-level=3 -C debuginfo=0 -C metadata=5cea4d705eb44a37 -C extra-filename=-5cea4d705eb44a37 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-46b664a0dee0f808.rmeta --extern backtrace=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbacktrace-098ca66689bbc763.rmeta --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-7941118bbdbac8c6.rmeta --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-8a9bd9d8d175184d.rmeta --extern chalk_ir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libchalk_ir-b35c0e67b3a1bd51.rmeta --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-2605b82e1ec92c3c.rmeta --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-02f0f6db374c810f.rmeta --extern measureme=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libmeasureme-8ffd4695ba02483c.rmeta --extern parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libparking_lot-6b0af6a3ef820d1d.rmeta --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-124a7a0460e1d85a.rmeta --extern rustc_rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon-27354e377c90cbb0.rmeta --extern rustc_rayon_core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon_core-d4ad53832d99e2f8.rmeta --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-6eade0d298c9229d.rmeta --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-f8f38091cc8a3122.rmeta --extern rustc_attr=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_attr-80e9c2f8f15fe8ea.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-d66730c03ff81c68.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-5959a611c3d6e901.rmeta --extern rustc_feature=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_feature-75bf1000e0ec9d8c.rmeta --extern rustc_hir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir-e9286f53934b91f9.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-87a289f735fe3e83.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps/librustc_macros-02de1b3228ba9665.so --extern rustc_query_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_query_system-813a41aa9de4d347.rmeta --extern rustc_session=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-60e66cfb2cb1dc1f.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-dd08768fc16d666a.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-d19a2783db085342.rmeta --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-8622965ebca5ff77.rmeta --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-f2f9c6d4ed07d9cb.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-c29aa775c67e8de6.rmeta --cfg=bootstrap -Zmacro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Wrust_2018_idioms -Wunused_lifetimes -Dwarnings -Zunstable-options '-Wrustc::internal' -Cprefer-dynamic --cfg=parallel_compiler -Zbinary-dep-depinfo -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-7615ed222efdef73/out` (signal: 9, SIGKILL: kill)
2020-04-18T07:42:50.3489191Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-18T07:42:50.3490366Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-04-18T07:42:50.3493571Z Build completed unsuccessfully in 0:33:57
2020-04-18T07:42:50.6112289Z == clock drift check ==
2020-04-18T07:42:50.6254056Z   local time: Sat Apr 18 07:42:50 UTC 2020
2020-04-18T07:42:50.6254056Z   local time: Sat Apr 18 07:42:50 UTC 2020
2020-04-18T07:42:50.9139068Z   network time: Sat, 18 Apr 2020 07:42:50 GMT
2020-04-18T07:42:52.5776481Z 
2020-04-18T07:42:52.5776481Z 
2020-04-18T07:42:52.8385760Z ##[error]Bash exited with code '1'.
2020-04-18T07:42:52.9078963Z ##[section]Finishing: Run build
2020-04-18T07:42:52.9487188Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-04-18T07:42:52.9783467Z Task         : Get sources
2020-04-18T07:42:52.9783810Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-18T07:42:52.9787417Z Version      : 1.0.0
2020-04-18T07:42:52.9791589Z Author       : Microsoft
2020-04-18T07:42:52.9791589Z Author       : Microsoft
2020-04-18T07:42:52.9795284Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-18T07:42:52.9800943Z ==============================================================================
2020-04-18T07:42:53.6280579Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-18T07:42:53.6345794Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-04-18T07:42:53.6889487Z Cleaning up task key
2020-04-18T07:42:53.6890949Z Start cleaning up orphan processes.
2020-04-18T07:42:53.7556633Z Terminate orphan process: pid (3318) (python)
2020-04-18T07:42:53.9667508Z ##[section]Finishing: Finalize Job
