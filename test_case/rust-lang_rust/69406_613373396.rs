plain
2020-04-14T09:47:09.2685890Z ========================== Starting Command Output ===========================
2020-04-14T09:47:09.2689991Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/08009507-edb9-4749-a19e-9eb6bb63fdf7.sh
2020-04-14T09:47:09.2690280Z 
2020-04-14T09:47:09.2696605Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-14T09:47:09.2716948Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-04-14T09:47:09.2723026Z Task         : Get sources
2020-04-14T09:47:09.2723329Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-14T09:47:09.2723609Z Version      : 1.0.0
2020-04-14T09:47:09.2723809Z Author       : Microsoft
---
2020-04-14T09:47:10.2671976Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-14T09:47:10.2677826Z ##[command]git config gc.auto 0
2020-04-14T09:47:10.2681354Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-14T09:47:10.2684548Z ##[command]git config --get-all http.proxy
2020-04-14T09:47:10.2691059Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69406/merge:refs/remotes/pull/69406/merge
---
2020-04-14T09:49:39.8112835Z  ---> 78ad2f4d4aca
2020-04-14T09:49:39.8113062Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-14T09:49:39.8113442Z  ---> Using cache
2020-04-14T09:49:39.8113802Z  ---> 4d2dc61c4d00
2020-04-14T09:49:39.8115057Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-14T09:49:39.8116322Z  ---> 776b6266a8b7
2020-04-14T09:49:39.8135115Z Successfully built 776b6266a8b7
2020-04-14T09:49:39.8180158Z Successfully tagged rust-ci:latest
2020-04-14T09:49:39.8441639Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-14T09:49:39.8441639Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-14T09:49:39.8453448Z Looks like docker image is the same as before, not uploading
2020-04-14T09:49:47.8745531Z [CI_JOB_NAME=mingw-check]
2020-04-14T09:49:47.8968119Z [CI_JOB_NAME=mingw-check]
2020-04-14T09:49:47.8990860Z == clock drift check ==
2020-04-14T09:49:47.9004741Z   local time: Tue Apr 14 09:49:47 UTC 2020
2020-04-14T09:49:47.9436217Z   network time: Tue, 14 Apr 2020 09:49:47 GMT
2020-04-14T09:49:47.9457056Z Starting sccache server...
2020-04-14T09:49:48.0411886Z configure: processing command line
2020-04-14T09:49:48.0412431Z configure: 
2020-04-14T09:49:48.0413462Z configure: rust.parallel-compiler := True
---
2020-04-14T09:51:57.7436756Z     Checking once_cell v1.1.0
2020-04-14T09:51:57.8425495Z     Checking rustc_error_codes v0.0.0 (/checkout/src/librustc_error_codes)
2020-04-14T09:51:57.8654939Z    Compiling semver v0.9.0
2020-04-14T09:51:58.0715496Z     Checking crossbeam-utils v0.6.5
2020-04-14T09:51:58.3964113Z     Checking chalk-macros v0.1.1 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-14T09:51:58.5785855Z     Checking lock_api v0.3.1
2020-04-14T09:51:58.8494855Z     Checking unicode-normalization v0.1.11
2020-04-14T09:51:59.3021169Z     Checking arrayvec v0.4.7
2020-04-14T09:51:59.6204070Z     Checking itertools v0.8.0
---
2020-04-14T09:52:15.0033036Z     Checking parking_lot v0.10.0
2020-04-14T09:52:15.1758502Z     Checking rls-span v0.5.1
2020-04-14T09:52:15.2028713Z     Checking serde_json v1.0.40
2020-04-14T09:52:15.6451441Z     Checking rand_core v0.5.1
2020-04-14T09:52:15.9308761Z     Checking chalk-engine v0.9.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-14T09:52:18.1161976Z     Checking digest v0.8.1
2020-04-14T09:52:18.1964947Z     Checking block-buffer v0.7.3
2020-04-14T09:52:18.2812658Z     Checking backtrace v0.3.46
2020-04-14T09:52:18.4967175Z     Checking rls-data v0.19.0
---
2020-04-14T09:52:46.3876449Z     Checking rustc-rayon v0.3.0
2020-04-14T09:52:48.4401118Z     Checking rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
2020-04-14T09:52:49.6327286Z     Checking arena v0.0.0 (/checkout/src/libarena)
2020-04-14T09:52:56.1306949Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2020-04-14T09:52:56.1308671Z    Compiling chalk-derive v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-14T09:53:12.1164676Z     Checking rustc_span v0.0.0 (/checkout/src/librustc_span)
2020-04-14T09:53:13.4607076Z     Checking chalk-ir v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-14T09:53:19.5333996Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-14T09:53:19.8727591Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-14T09:53:20.0283719Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-14T09:53:20.0283719Z     Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
2020-04-14T09:53:20.5271615Z     Checking chalk-rust-ir v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-14T09:53:21.4691525Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-14T09:53:21.4691525Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-14T09:53:22.7658339Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-14T09:53:23.5008613Z     Checking chalk-solve v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-14T09:53:24.5202600Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-14T09:53:25.5649721Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-14T09:53:25.9338801Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-04-14T09:53:26.9098810Z     Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-04-14T09:53:26.9098810Z     Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-04-14T09:53:27.5314563Z     Checking rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-04-14T09:53:29.0145703Z     Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
2020-04-14T10:20:06.9995909Z error: could not compile `rustc_middle`.
2020-04-14T10:20:07.0026746Z 
2020-04-14T10:20:07.0027384Z Caused by:
2020-04-14T10:20:07.0346744Z   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_middle --edition=2018 src/librustc_middle/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata -C opt-level=3 -C debuginfo=0 -C metadata=c3a2881aff5e4d4a -C extra-filename=-c3a2881aff5e4d4a --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-4beaf876d25f5744.rmeta --extern backtrace=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbacktrace-b93755e97321c534.rmeta --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-7941118bbdbac8c6.rmeta --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-8a9bd9d8d175184d.rmeta --extern chalk_ir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libchalk_ir-b35c0e67b3a1bd51.rmeta --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-800c60dc1aa7f7d7.rmeta --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-02f0f6db374c810f.rmeta --extern measureme=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libmeasureme-22170c16ed51548a.rmeta --extern parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libparking_lot-1262d1209f836bea.rmeta --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-124a7a0460e1d85a.rmeta --extern rustc_rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon-ecb41c25aaec48e5.rmeta --extern rustc_rayon_core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon_core-305bf65d219b202b.rmeta --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-6eade0d298c9229d.rmeta --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-61fdd265992909f5.rmeta --extern rustc_attr=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_attr-edd453f8f9514470.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-c450cab2bc3b7475.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-88f8008f144e867f.rmeta --extern rustc_feature=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_feature-7b0a2d8d5d80b824.rmeta --extern rustc_hir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir-47bdacda590246b2.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-87a289f735fe3e83.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps/librustc_macros-02de1b3228ba9665.so --extern rustc_query_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_query_system-3eba29064d1b3a5b.rmeta --extern rustc_session=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-3f57b7f565eeb861.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-9f60ad89847e8b4b.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-064c6ae3c71df6c9.rmeta --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-8622965ebca5ff77.rmeta --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-f2f9c6d4ed07d9cb.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-c29aa775c67e8de6.rmeta --cfg=bootstrap -Zmacro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Wrust_2018_idioms -Wunused_lifetimes -Dwarnings -Zunstable-options '-Wrustc::internal' -Cprefer-dynamic --cfg=parallel_compiler -Zbinary-dep-depinfo -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-7615ed222efdef73/out` (signal: 9, SIGKILL: kill)
2020-04-14T10:20:07.0854228Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-14T10:20:07.0899388Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-04-14T10:20:07.0899924Z Build completed unsuccessfully in 0:30:18
2020-04-14T10:20:07.3441661Z == clock drift check ==
2020-04-14T10:20:07.3441661Z == clock drift check ==
2020-04-14T10:20:07.3863755Z   local time: Tue Apr 14 10:20:07 UTC 2020
2020-04-14T10:20:07.7577939Z   network time: Tue, 14 Apr 2020 10:20:07 GMT
2020-04-14T10:20:09.1644427Z 
2020-04-14T10:20:09.1644427Z 
2020-04-14T10:20:09.3470825Z ##[error]Bash exited with code '1'.
2020-04-14T10:20:09.3873227Z ##[section]Finishing: Run build
2020-04-14T10:20:09.4290344Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-04-14T10:20:09.4543965Z Task         : Get sources
2020-04-14T10:20:09.4544343Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-14T10:20:09.4544630Z Version      : 1.0.0
2020-04-14T10:20:09.4544835Z Author       : Microsoft
2020-04-14T10:20:09.4544835Z Author       : Microsoft
2020-04-14T10:20:09.4550901Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-14T10:20:09.4551313Z ==============================================================================
2020-04-14T10:20:10.0061561Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-14T10:20:10.0115439Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-04-14T10:20:10.0331565Z Cleaning up task key
2020-04-14T10:20:10.0332776Z Start cleaning up orphan processes.
2020-04-14T10:20:10.0901025Z Terminate orphan process: pid (3607) (python)
2020-04-14T10:20:10.1915729Z ##[section]Finishing: Finalize Job
