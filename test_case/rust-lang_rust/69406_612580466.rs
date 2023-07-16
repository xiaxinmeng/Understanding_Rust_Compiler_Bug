plain
2020-04-12T06:57:39.7629952Z ========================== Starting Command Output ===========================
2020-04-12T06:57:39.7647085Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/7dd8bb49-f937-4e2a-bdc0-b682326319bd.sh
2020-04-12T06:57:40.0024758Z 
2020-04-12T06:57:40.0115893Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-12T06:57:40.0143893Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-04-12T06:57:40.0153547Z Task         : Get sources
2020-04-12T06:57:40.0154040Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-12T06:57:40.0154869Z Version      : 1.0.0
2020-04-12T06:57:40.0155104Z Author       : Microsoft
---
2020-04-12T06:57:42.3060556Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-12T06:57:42.3226001Z ##[command]git config gc.auto 0
2020-04-12T06:57:42.3268290Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-12T06:57:42.3304120Z ##[command]git config --get-all http.proxy
2020-04-12T06:57:42.3418774Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69406/merge:refs/remotes/pull/69406/merge
---
2020-04-12T07:01:43.6280147Z  ---> 78ad2f4d4aca
2020-04-12T07:01:43.6283590Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-12T07:01:43.6289020Z  ---> Using cache
2020-04-12T07:01:43.6294571Z  ---> 4d2dc61c4d00
2020-04-12T07:01:43.6311302Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-12T07:01:43.6317551Z  ---> 776b6266a8b7
2020-04-12T07:01:43.6351955Z Successfully built 776b6266a8b7
2020-04-12T07:01:43.6382494Z Successfully tagged rust-ci:latest
2020-04-12T07:01:43.7159033Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-12T07:01:43.7159033Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-12T07:01:43.7172327Z Looks like docker image is the same as before, not uploading
2020-04-12T07:01:45.1338945Z [CI_JOB_NAME=mingw-check]
2020-04-12T07:01:45.1633003Z [CI_JOB_NAME=mingw-check]
2020-04-12T07:01:45.1663392Z == clock drift check ==
2020-04-12T07:01:45.1675788Z   local time: Sun Apr 12 07:01:45 UTC 2020
2020-04-12T07:01:45.3662850Z   network time: Sun, 12 Apr 2020 07:01:45 GMT
2020-04-12T07:01:45.3693377Z Starting sccache server...
2020-04-12T07:01:45.4738631Z configure: processing command line
2020-04-12T07:01:45.4738858Z configure: 
2020-04-12T07:01:45.4739612Z configure: rust.parallel-compiler := True
---
2020-04-12T07:04:09.6450079Z    Compiling rustc_driver v0.0.0 (/checkout/src/librustc_driver)
2020-04-12T07:04:09.7710076Z     Checking rustc_error_codes v0.0.0 (/checkout/src/librustc_error_codes)
2020-04-12T07:04:09.8398812Z    Compiling semver v0.9.0
2020-04-12T07:04:09.9320436Z     Checking crossbeam-utils v0.6.5
2020-04-12T07:04:10.2745760Z     Checking chalk-macros v0.1.1 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-12T07:04:10.4471591Z     Checking lock_api v0.3.1
2020-04-12T07:04:10.7622043Z     Checking unicode-normalization v0.1.11
2020-04-12T07:04:11.4177217Z     Checking arrayvec v0.4.7
2020-04-12T07:04:11.6988038Z     Checking itertools v0.8.0
---
2020-04-12T07:04:30.5874145Z     Checking parking_lot v0.10.0
2020-04-12T07:04:30.8185187Z     Checking rand_core v0.5.1
2020-04-12T07:04:31.0012928Z     Checking rls-span v0.5.1
2020-04-12T07:04:31.0419086Z     Checking serde_json v1.0.40
2020-04-12T07:04:31.5752720Z     Checking chalk-engine v0.9.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-12T07:04:33.9766071Z     Checking block-buffer v0.7.3
2020-04-12T07:04:34.0857962Z     Checking digest v0.8.1
2020-04-12T07:04:34.1738104Z     Checking backtrace v0.3.46
2020-04-12T07:04:34.4291177Z     Checking rand_chacha v0.2.1
---
2020-04-12T07:04:41.1858033Z    Compiling synstructure v0.12.1
2020-04-12T07:05:09.2889368Z     Checking rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
2020-04-12T07:05:10.7592009Z     Checking arena v0.0.0 (/checkout/src/libarena)
2020-04-12T07:05:19.0732732Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2020-04-12T07:05:19.0734724Z    Compiling chalk-derive v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-12T07:05:42.7698465Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2020-04-12T07:05:43.4434368Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-12T07:05:43.8478152Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-12T07:05:43.8478152Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-12T07:05:43.9940077Z     Checking chalk-ir v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-12T07:05:44.6839881Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-12T07:05:48.7333817Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-12T07:05:48.7333817Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-12T07:05:50.1560277Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-12T07:05:50.9845306Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-12T07:05:51.4182527Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-12T07:05:51.4759823Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-04-12T07:05:51.4759823Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-04-12T07:05:52.6644736Z     Checking chalk-rust-ir v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-12T07:05:53.7525019Z     Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-04-12T07:05:54.5264740Z     Checking chalk-solve v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-12T07:05:58.5745462Z     Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
2020-04-12T07:34:03.9906012Z error: could not compile `rustc_middle`.
2020-04-12T07:34:03.9955379Z 
2020-04-12T07:34:03.9955631Z Caused by:
2020-04-12T07:34:03.9955631Z Caused by:
2020-04-12T07:34:04.0017062Z   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_middle --edition=2018 src/librustc_middle/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata -C opt-level=3 -C debuginfo=0 -C metadata=4f2e8b78c0d3975f -C extra-filename=-4f2e8b78c0d3975f --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-8fb50653cc5b9675.rmeta --extern backtrace=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbacktrace-b93755e97321c534.rmeta --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-7941118bbdbac8c6.rmeta --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-8a9bd9d8d175184d.rmeta --extern chalk_ir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libchalk_ir-b35c0e67b3a1bd51.rmeta --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-800c60dc1aa7f7d7.rmeta --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-02f0f6db374c810f.rmeta --extern measureme=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libmeasureme-22170c16ed51548a.rmeta --extern parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libparking_lot-1262d1209f836bea.rmeta --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-124a7a0460e1d85a.rmeta --extern rustc_rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon-ecb41c25aaec48e5.rmeta --extern rustc_rayon_core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon_core-305bf65d219b202b.rmeta --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-6eade0d298c9229d.rmeta --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-9dfeb28fa0788d42.rmeta --extern rustc_attr=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_attr-b4e187d30925d410.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-32b0d103c414b59d.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-846c9693b30e9921.rmeta --extern rustc_feature=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_feature-90677d3b4e486ab5.rmeta --extern rustc_hir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir-96436f7a94ec7575.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-87a289f735fe3e83.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps/librustc_macros-02de1b3228ba9665.so --extern rustc_query_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_query_system-94eb57692922fc80.rmeta --extern rustc_session=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-eb2a953ebfd0813d.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-2f4e53cd021a2f7c.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-11672d8a9c95dd2d.rmeta --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-8622965ebca5ff77.rmeta --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-f2f9c6d4ed07d9cb.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-c29aa775c67e8de6.rmeta --cfg=bootstrap -Zmacro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Wrust_2018_idioms -Wunused_lifetimes -Dwarnings -Zunstable-options '-Wrustc::internal' -Cprefer-dynamic --cfg=parallel_compiler -Zbinary-dep-depinfo -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-7615ed222efdef73/out` (signal: 9, SIGKILL: kill)
2020-04-12T07:34:04.0044720Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-12T07:34:04.0049994Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-04-12T07:34:04.0050689Z Build completed unsuccessfully in 0:32:18
2020-04-12T07:34:04.2443988Z == clock drift check ==
2020-04-12T07:34:04.2679738Z   local time: Sun Apr 12 07:34:04 UTC 2020
2020-04-12T07:34:04.2679738Z   local time: Sun Apr 12 07:34:04 UTC 2020
2020-04-12T07:34:04.4944065Z   network time: Sun, 12 Apr 2020 07:34:04 GMT
2020-04-12T07:34:05.8167059Z 
2020-04-12T07:34:05.8167059Z 
2020-04-12T07:34:06.0664688Z ##[error]Bash exited with code '1'.
2020-04-12T07:34:06.1151646Z ##[section]Finishing: Run build
2020-04-12T07:34:06.1802628Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-04-12T07:34:06.2196555Z Task         : Get sources
2020-04-12T07:34:06.2196910Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-12T07:34:06.2202066Z Version      : 1.0.0
2020-04-12T07:34:06.2202387Z Author       : Microsoft
2020-04-12T07:34:06.2202387Z Author       : Microsoft
2020-04-12T07:34:06.2202762Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-12T07:34:06.2207568Z ==============================================================================
2020-04-12T07:34:06.8045632Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-12T07:34:06.8111912Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-04-12T07:34:06.8685773Z Cleaning up task key
2020-04-12T07:34:06.8699675Z Start cleaning up orphan processes.
2020-04-12T07:34:06.9585744Z Terminate orphan process: pid (4324) (python)
2020-04-12T07:34:07.0699834Z ##[section]Finishing: Finalize Job
