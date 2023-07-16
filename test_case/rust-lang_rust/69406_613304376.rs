plain
2020-04-14T07:24:22.6044365Z ========================== Starting Command Output ===========================
2020-04-14T07:24:22.6049612Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/80bc4eab-4e5c-4619-b959-6cb9b213609e.sh
2020-04-14T07:24:22.6050120Z 
2020-04-14T07:24:22.6055259Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-14T07:24:22.6073307Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-04-14T07:24:22.6076548Z Task         : Get sources
2020-04-14T07:24:22.6076827Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-14T07:24:22.6077073Z Version      : 1.0.0
2020-04-14T07:24:22.6077243Z Author       : Microsoft
---
2020-04-14T07:24:23.5941715Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-14T07:24:23.5948741Z ##[command]git config gc.auto 0
2020-04-14T07:24:23.5952464Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-14T07:24:23.5955695Z ##[command]git config --get-all http.proxy
2020-04-14T07:24:23.5961850Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69406/merge:refs/remotes/pull/69406/merge
---
2020-04-14T07:27:41.9673530Z  ---> 78ad2f4d4aca
2020-04-14T07:27:41.9673778Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-14T07:27:41.9674215Z  ---> Using cache
2020-04-14T07:27:41.9674786Z  ---> 4d2dc61c4d00
2020-04-14T07:27:41.9676222Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-14T07:27:41.9678054Z  ---> 776b6266a8b7
2020-04-14T07:27:41.9741607Z Successfully built 776b6266a8b7
2020-04-14T07:27:41.9804607Z Successfully tagged rust-ci:latest
2020-04-14T07:27:43.1230341Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-14T07:27:43.1230341Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-14T07:27:43.1256938Z Looks like docker image is the same as before, not uploading
2020-04-14T07:27:50.7500210Z [CI_JOB_NAME=mingw-check]
2020-04-14T07:27:50.7788361Z [CI_JOB_NAME=mingw-check]
2020-04-14T07:27:50.7819313Z == clock drift check ==
2020-04-14T07:27:50.7832690Z   local time: Tue Apr 14 07:27:50 UTC 2020
2020-04-14T07:27:50.9622321Z   network time: Tue, 14 Apr 2020 07:27:50 GMT
2020-04-14T07:27:50.9651570Z Starting sccache server...
2020-04-14T07:27:51.0749079Z configure: processing command line
2020-04-14T07:27:51.0749430Z configure: 
2020-04-14T07:27:51.0750365Z configure: rust.parallel-compiler := True
---
2020-04-14T07:30:23.4707976Z    Compiling rustc_driver v0.0.0 (/checkout/src/librustc_driver)
2020-04-14T07:30:23.5859711Z     Checking rustc_error_codes v0.0.0 (/checkout/src/librustc_error_codes)
2020-04-14T07:30:23.7173995Z    Compiling semver v0.9.0
2020-04-14T07:30:23.9369200Z     Checking crossbeam-utils v0.6.5
2020-04-14T07:30:24.3019435Z     Checking chalk-macros v0.1.1 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-14T07:30:24.5065501Z     Checking lock_api v0.3.1
2020-04-14T07:30:24.7899941Z     Checking unicode-normalization v0.1.11
2020-04-14T07:30:25.5708927Z     Checking arrayvec v0.4.7
2020-04-14T07:30:25.8577091Z     Checking itertools v0.8.0
---
2020-04-14T07:30:42.8461367Z     Checking generic-array v0.12.3
2020-04-14T07:30:43.6082929Z     Checking flate2 v1.0.12
2020-04-14T07:30:44.2252041Z     Checking parking_lot v0.10.0
2020-04-14T07:30:44.4609878Z     Checking rand_core v0.5.1
2020-04-14T07:30:44.6031991Z     Checking chalk-engine v0.9.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-14T07:30:46.1629804Z     Checking rls-span v0.5.1
2020-04-14T07:30:46.6163604Z     Checking serde_json v1.0.40
2020-04-14T07:30:48.0701093Z     Checking digest v0.8.1
2020-04-14T07:30:48.1797649Z     Checking block-buffer v0.7.3
---
2020-04-14T07:31:22.9748838Z     Checking rustc-rayon v0.3.0
2020-04-14T07:31:25.4377151Z     Checking rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
2020-04-14T07:31:27.0813781Z     Checking arena v0.0.0 (/checkout/src/libarena)
2020-04-14T07:31:34.9760645Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2020-04-14T07:31:34.9766232Z    Compiling chalk-derive v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-14T07:31:54.0614516Z     Checking rustc_span v0.0.0 (/checkout/src/librustc_span)
2020-04-14T07:31:55.7662781Z     Checking chalk-ir v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-14T07:32:03.1332978Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-14T07:32:03.6322881Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-14T07:32:03.8387792Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-14T07:32:03.8387792Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-14T07:32:04.0153061Z     Checking chalk-rust-ir v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-14T07:32:05.1933810Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-14T07:32:05.1933810Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-14T07:32:06.7670295Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-14T07:32:07.6274993Z     Checking chalk-solve v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-14T07:32:09.1139987Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-14T07:32:09.8911407Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-14T07:32:10.3438145Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-04-14T07:32:11.5670251Z     Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-04-14T07:32:11.5670251Z     Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-04-14T07:32:12.3211026Z     Checking rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-04-14T07:32:13.9936743Z     Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
2020-04-14T07:57:58.7225141Z error: could not compile `rustc_middle`.
2020-04-14T07:57:58.7254777Z 
2020-04-14T07:57:58.7255120Z Caused by:
2020-04-14T07:57:58.7569513Z   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_middle --edition=2018 src/librustc_middle/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata -C opt-level=3 -C debuginfo=0 -C metadata=c3a2881aff5e4d4a -C extra-filename=-c3a2881aff5e4d4a --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-4beaf876d25f5744.rmeta --extern backtrace=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbacktrace-b93755e97321c534.rmeta --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-7941118bbdbac8c6.rmeta --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-8a9bd9d8d175184d.rmeta --extern chalk_ir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libchalk_ir-b35c0e67b3a1bd51.rmeta --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-800c60dc1aa7f7d7.rmeta --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-02f0f6db374c810f.rmeta --extern measureme=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libmeasureme-22170c16ed51548a.rmeta --extern parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libparking_lot-1262d1209f836bea.rmeta --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-124a7a0460e1d85a.rmeta --extern rustc_rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon-ecb41c25aaec48e5.rmeta --extern rustc_rayon_core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon_core-305bf65d219b202b.rmeta --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-6eade0d298c9229d.rmeta --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-61fdd265992909f5.rmeta --extern rustc_attr=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_attr-edd453f8f9514470.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-c450cab2bc3b7475.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-88f8008f144e867f.rmeta --extern rustc_feature=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_feature-7b0a2d8d5d80b824.rmeta --extern rustc_hir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir-47bdacda590246b2.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-87a289f735fe3e83.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps/librustc_macros-02de1b3228ba9665.so --extern rustc_query_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_query_system-3eba29064d1b3a5b.rmeta --extern rustc_session=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-3f57b7f565eeb861.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-9f60ad89847e8b4b.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-064c6ae3c71df6c9.rmeta --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-8622965ebca5ff77.rmeta --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-f2f9c6d4ed07d9cb.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-c29aa775c67e8de6.rmeta --cfg=bootstrap -Zmacro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Wrust_2018_idioms -Wunused_lifetimes -Dwarnings -Zunstable-options '-Wrustc::internal' -Cprefer-dynamic --cfg=parallel_compiler -Zbinary-dep-depinfo -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-7615ed222efdef73/out` (signal: 9, SIGKILL: kill)
2020-04-14T07:57:58.7600177Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-14T07:57:58.7604576Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-04-14T07:57:58.7605008Z Build completed unsuccessfully in 0:30:07
2020-04-14T07:57:59.0294205Z == clock drift check ==
2020-04-14T07:57:59.0294205Z == clock drift check ==
2020-04-14T07:57:59.0601795Z   local time: Tue Apr 14 07:57:59 UTC 2020
2020-04-14T07:57:59.4761948Z   network time: Tue, 14 Apr 2020 07:57:59 GMT
2020-04-14T07:58:00.9093884Z 
2020-04-14T07:58:00.9093884Z 
2020-04-14T07:58:01.1253872Z ##[error]Bash exited with code '1'.
2020-04-14T07:58:01.1866667Z ##[section]Finishing: Run build
2020-04-14T07:58:01.2250941Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-04-14T07:58:01.2468137Z Task         : Get sources
2020-04-14T07:58:01.2468528Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-14T07:58:01.2473928Z Version      : 1.0.0
2020-04-14T07:58:01.2474208Z Author       : Microsoft
2020-04-14T07:58:01.2474208Z Author       : Microsoft
2020-04-14T07:58:01.2474588Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-14T07:58:01.2477710Z ==============================================================================
2020-04-14T07:58:01.7928743Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-14T07:58:01.7972774Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-04-14T07:58:01.8242038Z Cleaning up task key
2020-04-14T07:58:01.8243254Z Start cleaning up orphan processes.
2020-04-14T07:58:01.8784664Z Terminate orphan process: pid (4260) (python)
2020-04-14T07:58:01.9823775Z ##[section]Finishing: Finalize Job
