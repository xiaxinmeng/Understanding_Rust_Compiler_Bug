plain
2020-04-13T19:23:17.4415966Z ========================== Starting Command Output ===========================
2020-04-13T19:23:17.4418353Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/1e3a2a48-f55a-4ec2-b00b-550438d59294.sh
2020-04-13T19:23:17.4418585Z 
2020-04-13T19:23:17.4422069Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-13T19:23:17.4440332Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-04-13T19:23:17.4443390Z Task         : Get sources
2020-04-13T19:23:17.4443666Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-13T19:23:17.4443932Z Version      : 1.0.0
2020-04-13T19:23:17.4444114Z Author       : Microsoft
---
2020-04-13T19:23:18.4397755Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-13T19:23:18.4405901Z ##[command]git config gc.auto 0
2020-04-13T19:23:18.4409413Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-13T19:23:18.4412585Z ##[command]git config --get-all http.proxy
2020-04-13T19:23:18.4418816Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69406/merge:refs/remotes/pull/69406/merge
---
2020-04-13T19:25:45.5331189Z  ---> 78ad2f4d4aca
2020-04-13T19:25:45.5334710Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-13T19:25:45.5338025Z  ---> Using cache
2020-04-13T19:25:45.5340731Z  ---> 4d2dc61c4d00
2020-04-13T19:25:45.5344831Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-13T19:25:45.5349820Z  ---> 776b6266a8b7
2020-04-13T19:25:45.5350013Z Successfully built 776b6266a8b7
2020-04-13T19:25:45.5368856Z Successfully tagged rust-ci:latest
2020-04-13T19:25:45.5667433Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-13T19:25:45.5667433Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-13T19:25:45.5680640Z Looks like docker image is the same as before, not uploading
2020-04-13T19:25:53.6499480Z [CI_JOB_NAME=mingw-check]
2020-04-13T19:25:53.6706014Z [CI_JOB_NAME=mingw-check]
2020-04-13T19:25:53.6729340Z == clock drift check ==
2020-04-13T19:25:53.6734775Z   local time: Mon Apr 13 19:25:53 UTC 2020
2020-04-13T19:25:53.9942769Z   network time: Mon, 13 Apr 2020 19:25:53 GMT
2020-04-13T19:25:53.9967279Z Starting sccache server...
2020-04-13T19:25:54.1084569Z configure: processing command line
2020-04-13T19:25:54.1084821Z configure: 
2020-04-13T19:25:54.1085750Z configure: rust.parallel-compiler := True
---
2020-04-13T19:28:17.5983375Z    Compiling rustc_driver v0.0.0 (/checkout/src/librustc_driver)
2020-04-13T19:28:17.6781423Z     Checking rustc_error_codes v0.0.0 (/checkout/src/librustc_error_codes)
2020-04-13T19:28:17.7959290Z    Compiling semver v0.9.0
2020-04-13T19:28:17.9242557Z     Checking crossbeam-utils v0.6.5
2020-04-13T19:28:18.2358952Z     Checking chalk-macros v0.1.1 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-13T19:28:18.4229090Z     Checking lock_api v0.3.1
2020-04-13T19:28:18.6478188Z     Checking unicode-normalization v0.1.11
2020-04-13T19:28:19.2747903Z     Checking arrayvec v0.4.7
2020-04-13T19:28:19.5802704Z     Checking itertools v0.8.0
---
2020-04-13T19:28:35.5449317Z     Checking parking_lot v0.10.0
2020-04-13T19:28:35.7548797Z     Checking rand_core v0.5.1
2020-04-13T19:28:35.8679770Z     Checking rls-span v0.5.1
2020-04-13T19:28:35.9477865Z     Checking serde_json v1.0.40
2020-04-13T19:28:36.3531663Z     Checking chalk-engine v0.9.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-13T19:28:38.8437598Z     Checking block-buffer v0.7.3
2020-04-13T19:28:38.9364262Z     Checking digest v0.8.1
2020-04-13T19:28:39.0270711Z     Checking backtrace v0.3.46
2020-04-13T19:28:39.2782083Z     Checking rand_chacha v0.2.1
---
2020-04-13T19:29:09.2887166Z     Checking rustc-rayon v0.3.0
2020-04-13T19:29:11.4975705Z     Checking rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
2020-04-13T19:29:12.9077244Z     Checking arena v0.0.0 (/checkout/src/libarena)
2020-04-13T19:29:20.8688518Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2020-04-13T19:29:20.8693509Z    Compiling chalk-derive v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-13T19:29:44.0335992Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2020-04-13T19:29:44.2357713Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-13T19:29:44.6458214Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-13T19:29:44.6458214Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-13T19:29:44.8434930Z     Checking chalk-ir v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-13T19:29:45.6644930Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-13T19:29:49.4927999Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-13T19:29:49.4927999Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-13T19:29:50.9827125Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-13T19:29:51.7244891Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-13T19:29:52.1593086Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-13T19:29:52.3067323Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-04-13T19:29:52.3067323Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-04-13T19:29:53.5133593Z     Checking chalk-rust-ir v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-13T19:29:54.6105208Z     Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-04-13T19:29:55.3454880Z     Checking chalk-solve v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-13T19:29:59.3382044Z     Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
2020-04-13T19:56:40.7502944Z error: could not compile `rustc_middle`.
2020-04-13T19:56:40.7624554Z 
2020-04-13T19:56:40.7628809Z Caused by:
2020-04-13T19:56:40.7628809Z Caused by:
2020-04-13T19:56:40.7678209Z   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_middle --edition=2018 src/librustc_middle/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata -C opt-level=3 -C debuginfo=0 -C metadata=c3a2881aff5e4d4a -C extra-filename=-c3a2881aff5e4d4a --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-4beaf876d25f5744.rmeta --extern backtrace=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbacktrace-b93755e97321c534.rmeta --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-7941118bbdbac8c6.rmeta --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-8a9bd9d8d175184d.rmeta --extern chalk_ir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libchalk_ir-b35c0e67b3a1bd51.rmeta --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-800c60dc1aa7f7d7.rmeta --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-02f0f6db374c810f.rmeta --extern measureme=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libmeasureme-22170c16ed51548a.rmeta --extern parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libparking_lot-1262d1209f836bea.rmeta --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-124a7a0460e1d85a.rmeta --extern rustc_rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon-ecb41c25aaec48e5.rmeta --extern rustc_rayon_core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon_core-305bf65d219b202b.rmeta --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-6eade0d298c9229d.rmeta --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-61fdd265992909f5.rmeta --extern rustc_attr=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_attr-edd453f8f9514470.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-c450cab2bc3b7475.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-88f8008f144e867f.rmeta --extern rustc_feature=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_feature-7b0a2d8d5d80b824.rmeta --extern rustc_hir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir-47bdacda590246b2.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-87a289f735fe3e83.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps/librustc_macros-02de1b3228ba9665.so --extern rustc_query_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_query_system-3eba29064d1b3a5b.rmeta --extern rustc_session=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-3f57b7f565eeb861.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-9f60ad89847e8b4b.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-064c6ae3c71df6c9.rmeta --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-8622965ebca5ff77.rmeta --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-f2f9c6d4ed07d9cb.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-c29aa775c67e8de6.rmeta --cfg=bootstrap -Zmacro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Wrust_2018_idioms -Wunused_lifetimes -Dwarnings -Zunstable-options '-Wrustc::internal' -Cprefer-dynamic --cfg=parallel_compiler -Zbinary-dep-depinfo -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-7615ed222efdef73/out` (signal: 9, SIGKILL: kill)
2020-04-13T19:56:40.7712255Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-13T19:56:40.7718269Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-04-13T19:56:40.7718621Z Build completed unsuccessfully in 0:30:46
2020-04-13T19:56:41.0542737Z == clock drift check ==
2020-04-13T19:56:41.0543311Z   local time: Mon Apr 13 19:56:40 UTC 2020
2020-04-13T19:56:41.0543311Z   local time: Mon Apr 13 19:56:40 UTC 2020
2020-04-13T19:56:41.2170357Z   network time: Mon, 13 Apr 2020 19:56:41 GMT
2020-04-13T19:56:42.7762365Z 
2020-04-13T19:56:42.7762365Z 
2020-04-13T19:56:43.0410740Z ##[error]Bash exited with code '1'.
2020-04-13T19:56:43.0832552Z ##[section]Finishing: Run build
2020-04-13T19:56:43.1176704Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-04-13T19:56:43.1356363Z Task         : Get sources
2020-04-13T19:56:43.1356701Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-13T19:56:43.1357325Z Version      : 1.0.0
2020-04-13T19:56:43.1357537Z Author       : Microsoft
2020-04-13T19:56:43.1357537Z Author       : Microsoft
2020-04-13T19:56:43.1359512Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-13T19:56:43.1362657Z ==============================================================================
2020-04-13T19:56:43.6884780Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-13T19:56:43.6949569Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-04-13T19:56:43.7273996Z Cleaning up task key
2020-04-13T19:56:43.7279675Z Start cleaning up orphan processes.
2020-04-13T19:56:43.7974564Z Terminate orphan process: pid (3472) (python)
2020-04-13T19:56:43.9218791Z ##[section]Finishing: Finalize Job
