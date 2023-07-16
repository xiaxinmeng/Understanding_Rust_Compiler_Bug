plain
2020-04-12T16:24:00.6994739Z ========================== Starting Command Output ===========================
2020-04-12T16:24:00.6998654Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/39f0da22-251d-4818-a96f-fe6db9513260.sh
2020-04-12T16:24:00.6998987Z 
2020-04-12T16:24:00.7004182Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-12T16:24:00.7024099Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-04-12T16:24:00.7027381Z Task         : Get sources
2020-04-12T16:24:00.7027858Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-12T16:24:00.7028175Z Version      : 1.0.0
2020-04-12T16:24:00.7028380Z Author       : Microsoft
---
2020-04-12T16:24:01.6906733Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-12T16:24:01.6913068Z ##[command]git config gc.auto 0
2020-04-12T16:24:01.6920101Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-12T16:24:01.6924093Z ##[command]git config --get-all http.proxy
2020-04-12T16:24:01.6932195Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69406/merge:refs/remotes/pull/69406/merge
---
2020-04-12T16:27:29.2744037Z  ---> 78ad2f4d4aca
2020-04-12T16:27:29.2744357Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-12T16:27:29.2748309Z  ---> Using cache
2020-04-12T16:27:29.2748754Z  ---> 4d2dc61c4d00
2020-04-12T16:27:29.2750420Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-12T16:27:29.2754070Z  ---> 776b6266a8b7
2020-04-12T16:27:29.2786869Z Successfully built 776b6266a8b7
2020-04-12T16:27:29.2822814Z Successfully tagged rust-ci:latest
2020-04-12T16:27:29.3498828Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-12T16:27:29.3498828Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-12T16:27:29.3511843Z Looks like docker image is the same as before, not uploading
2020-04-12T16:27:37.5685462Z [CI_JOB_NAME=mingw-check]
2020-04-12T16:27:37.5965741Z [CI_JOB_NAME=mingw-check]
2020-04-12T16:27:37.6050690Z == clock drift check ==
2020-04-12T16:27:37.6052122Z   local time: Sun Apr 12 16:27:37 UTC 2020
2020-04-12T16:27:37.6926884Z   network time: Sun, 12 Apr 2020 16:27:37 GMT
2020-04-12T16:27:37.6952119Z Starting sccache server...
2020-04-12T16:27:37.8096634Z configure: processing command line
2020-04-12T16:27:37.8097075Z configure: 
2020-04-12T16:27:37.8098132Z configure: rust.parallel-compiler := True
---
2020-04-12T16:30:09.5204031Z    Compiling rustc_driver v0.0.0 (/checkout/src/librustc_driver)
2020-04-12T16:30:09.5684295Z     Checking rustc_error_codes v0.0.0 (/checkout/src/librustc_error_codes)
2020-04-12T16:30:09.7431970Z    Compiling semver v0.9.0
2020-04-12T16:30:09.8430478Z     Checking crossbeam-utils v0.6.5
2020-04-12T16:30:10.1819687Z     Checking chalk-macros v0.1.1 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-12T16:30:10.3868139Z     Checking lock_api v0.3.1
2020-04-12T16:30:10.6639592Z     Checking unicode-normalization v0.1.11
2020-04-12T16:30:11.4286007Z     Checking arrayvec v0.4.7
2020-04-12T16:30:11.7489419Z     Checking itertools v0.8.0
---
2020-04-12T16:30:29.6985027Z     Checking parking_lot v0.10.0
2020-04-12T16:30:29.7613626Z     Checking rand_core v0.5.1
2020-04-12T16:30:29.9814777Z     Checking rls-span v0.5.1
2020-04-12T16:30:30.0251380Z     Checking serde_json v1.0.40
2020-04-12T16:30:30.5418055Z     Checking chalk-engine v0.9.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-12T16:30:33.0695547Z     Checking digest v0.8.1
2020-04-12T16:30:33.1627721Z     Checking block-buffer v0.7.3
2020-04-12T16:30:33.2614452Z     Checking backtrace v0.3.46
2020-04-12T16:30:33.5220441Z     Checking rand_chacha v0.2.1
---
2020-04-12T16:31:06.1327201Z     Checking rustc-rayon v0.3.0
2020-04-12T16:31:08.4598998Z     Checking rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
2020-04-12T16:31:09.9740259Z     Checking arena v0.0.0 (/checkout/src/libarena)
2020-04-12T16:31:17.8124423Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2020-04-12T16:31:17.8125575Z    Compiling chalk-derive v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-12T16:31:41.6377693Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2020-04-12T16:31:42.1303257Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-12T16:31:42.5654269Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-12T16:31:42.5654269Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-12T16:31:42.7773297Z     Checking chalk-ir v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-12T16:31:43.5457019Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-12T16:31:47.4825966Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-12T16:31:47.4825966Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-12T16:31:48.9750275Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-12T16:31:49.8113039Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-12T16:31:50.2623357Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-12T16:31:50.3745257Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-04-12T16:31:50.3745257Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-04-12T16:31:51.5661011Z     Checking chalk-rust-ir v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-12T16:31:52.6890192Z     Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-04-12T16:31:53.4148020Z     Checking chalk-solve v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-12T16:31:57.5714983Z     Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
2020-04-12T17:00:07.3794415Z error: could not compile `rustc_middle`.
2020-04-12T17:00:07.3817744Z 
2020-04-12T17:00:07.3817966Z Caused by:
2020-04-12T17:00:07.3817966Z Caused by:
2020-04-12T17:00:07.3869926Z   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_middle --edition=2018 src/librustc_middle/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata -C opt-level=3 -C debuginfo=0 -C metadata=4f2e8b78c0d3975f -C extra-filename=-4f2e8b78c0d3975f --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-8fb50653cc5b9675.rmeta --extern backtrace=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbacktrace-b93755e97321c534.rmeta --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-7941118bbdbac8c6.rmeta --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-8a9bd9d8d175184d.rmeta --extern chalk_ir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libchalk_ir-b35c0e67b3a1bd51.rmeta --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-800c60dc1aa7f7d7.rmeta --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-02f0f6db374c810f.rmeta --extern measureme=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libmeasureme-22170c16ed51548a.rmeta --extern parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libparking_lot-1262d1209f836bea.rmeta --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-124a7a0460e1d85a.rmeta --extern rustc_rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon-ecb41c25aaec48e5.rmeta --extern rustc_rayon_core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon_core-305bf65d219b202b.rmeta --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-6eade0d298c9229d.rmeta --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-9dfeb28fa0788d42.rmeta --extern rustc_attr=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_attr-b4e187d30925d410.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-32b0d103c414b59d.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-846c9693b30e9921.rmeta --extern rustc_feature=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_feature-90677d3b4e486ab5.rmeta --extern rustc_hir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir-96436f7a94ec7575.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-87a289f735fe3e83.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps/librustc_macros-02de1b3228ba9665.so --extern rustc_query_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_query_system-94eb57692922fc80.rmeta --extern rustc_session=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-eb2a953ebfd0813d.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-2f4e53cd021a2f7c.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-11672d8a9c95dd2d.rmeta --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-8622965ebca5ff77.rmeta --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-f2f9c6d4ed07d9cb.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-c29aa775c67e8de6.rmeta --cfg=bootstrap -Zmacro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Wrust_2018_idioms -Wunused_lifetimes -Dwarnings -Zunstable-options '-Wrustc::internal' -Cprefer-dynamic --cfg=parallel_compiler -Zbinary-dep-depinfo -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-7615ed222efdef73/out` (signal: 9, SIGKILL: kill)
2020-04-12T17:00:07.3892408Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-12T17:00:07.3897249Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-04-12T17:00:07.3897585Z Build completed unsuccessfully in 0:32:29
2020-04-12T17:00:07.4928221Z == clock drift check ==
2020-04-12T17:00:07.5071943Z   local time: Sun Apr 12 17:00:07 UTC 2020
2020-04-12T17:00:07.5071943Z   local time: Sun Apr 12 17:00:07 UTC 2020
2020-04-12T17:00:07.9376868Z   network time: Sun, 12 Apr 2020 17:00:07 GMT
2020-04-12T17:00:09.3345135Z 
2020-04-12T17:00:09.3345135Z 
2020-04-12T17:00:09.5384939Z ##[error]Bash exited with code '1'.
2020-04-12T17:00:09.5710701Z ##[section]Finishing: Run build
2020-04-12T17:00:09.6057635Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-04-12T17:00:09.6282409Z Task         : Get sources
2020-04-12T17:00:09.6285265Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-12T17:00:09.6285601Z Version      : 1.0.0
2020-04-12T17:00:09.6285830Z Author       : Microsoft
2020-04-12T17:00:09.6285830Z Author       : Microsoft
2020-04-12T17:00:09.6286249Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-12T17:00:09.6286660Z ==============================================================================
2020-04-12T17:00:10.1757090Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-12T17:00:10.1817372Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-04-12T17:00:10.2127996Z Cleaning up task key
2020-04-12T17:00:10.2133995Z Start cleaning up orphan processes.
2020-04-12T17:00:10.2620114Z Terminate orphan process: pid (3723) (python)
2020-04-12T17:00:10.3902522Z ##[section]Finishing: Finalize Job
