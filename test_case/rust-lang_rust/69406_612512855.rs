plain
2020-04-11T19:45:32.2834691Z ========================== Starting Command Output ===========================
2020-04-11T19:45:32.2839029Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/773bc3cd-2ac1-4aea-9f12-ef91fdf50597.sh
2020-04-11T19:45:32.2839550Z 
2020-04-11T19:45:32.2845513Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-11T19:45:32.2865912Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-04-11T19:45:32.2868968Z Task         : Get sources
2020-04-11T19:45:32.2869216Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-11T19:45:32.2869585Z Version      : 1.0.0
2020-04-11T19:45:32.2869724Z Author       : Microsoft
---
2020-04-11T19:45:33.2957368Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-11T19:45:33.2962717Z ##[command]git config gc.auto 0
2020-04-11T19:45:33.2965796Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-11T19:45:33.2968455Z ##[command]git config --get-all http.proxy
2020-04-11T19:45:33.2973218Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69406/merge:refs/remotes/pull/69406/merge
---
2020-04-11T19:48:07.2811892Z  ---> 78ad2f4d4aca
2020-04-11T19:48:07.2812135Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-11T19:48:07.2817266Z  ---> Using cache
2020-04-11T19:48:07.2818033Z  ---> 4d2dc61c4d00
2020-04-11T19:48:07.2819642Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-11T19:48:07.2823381Z  ---> 776b6266a8b7
2020-04-11T19:48:07.2876581Z Successfully built 776b6266a8b7
2020-04-11T19:48:07.2902497Z Successfully tagged rust-ci:latest
2020-04-11T19:48:07.3213653Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-11T19:48:07.3213653Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-11T19:48:07.3235849Z Looks like docker image is the same as before, not uploading
2020-04-11T19:48:07.9124145Z [CI_JOB_NAME=mingw-check]
2020-04-11T19:48:07.9387123Z [CI_JOB_NAME=mingw-check]
2020-04-11T19:48:07.9421179Z == clock drift check ==
2020-04-11T19:48:07.9429071Z   local time: Sat Apr 11 19:48:07 UTC 2020
2020-04-11T19:48:08.1346474Z   network time: Sat, 11 Apr 2020 19:48:08 GMT
2020-04-11T19:48:08.1372307Z Starting sccache server...
2020-04-11T19:48:08.2462565Z configure: processing command line
2020-04-11T19:48:08.2463192Z configure: 
2020-04-11T19:48:08.2464248Z configure: rust.parallel-compiler := True
---
2020-04-11T19:50:24.7133132Z    Compiling rustc_driver v0.0.0 (/checkout/src/librustc_driver)
2020-04-11T19:50:24.7378434Z     Checking rustc_error_codes v0.0.0 (/checkout/src/librustc_error_codes)
2020-04-11T19:50:24.9201228Z    Compiling semver v0.9.0
2020-04-11T19:50:25.0234960Z     Checking crossbeam-utils v0.6.5
2020-04-11T19:50:25.3289193Z     Checking chalk-macros v0.1.1 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-11T19:50:25.5143080Z     Checking lock_api v0.3.1
2020-04-11T19:50:25.7908775Z     Checking unicode-normalization v0.1.11
2020-04-11T19:50:26.4131761Z     Checking arrayvec v0.4.7
2020-04-11T19:50:26.7158942Z     Checking itertools v0.8.0
---
2020-04-11T19:50:41.6512110Z     Checking rustc_apfloat v0.0.0 (/checkout/src/librustc_apfloat)
2020-04-11T19:50:42.5447621Z     Checking flate2 v1.0.12
2020-04-11T19:50:42.8205512Z     Checking parking_lot v0.10.0
2020-04-11T19:50:42.9754000Z     Checking rand_core v0.5.1
2020-04-11T19:50:43.0261352Z     Checking chalk-engine v0.9.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-11T19:50:44.4776044Z     Checking rls-span v0.5.1
2020-04-11T19:50:44.8938037Z     Checking serde_json v1.0.40
2020-04-11T19:50:46.0414888Z     Checking digest v0.8.1
2020-04-11T19:50:46.1306787Z     Checking block-buffer v0.7.3
---
2020-04-11T19:51:15.4067639Z     Checking rustc-rayon v0.3.0
2020-04-11T19:51:17.4707580Z     Checking rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
2020-04-11T19:51:18.8867116Z     Checking arena v0.0.0 (/checkout/src/libarena)
2020-04-11T19:51:26.1644849Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2020-04-11T19:51:26.1645965Z    Compiling chalk-derive v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-11T19:51:47.3674195Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2020-04-11T19:51:47.6714412Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-11T19:51:48.0346335Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-11T19:51:48.0346335Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-11T19:51:48.2188800Z     Checking chalk-ir v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-11T19:51:48.9769750Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-11T19:51:52.4696549Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-11T19:51:52.4696549Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-11T19:51:53.8126616Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-11T19:51:54.5554341Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-11T19:51:54.9549647Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-11T19:51:55.2343108Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-04-11T19:51:55.2343108Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-04-11T19:51:56.2748833Z     Checking chalk-rust-ir v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-11T19:51:57.3159956Z     Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-04-11T19:51:57.9814129Z     Checking chalk-solve v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-11T19:52:01.7078316Z     Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
2020-04-11T20:19:40.7210528Z error: could not compile `rustc_middle`.
2020-04-11T20:19:40.7250576Z 
2020-04-11T20:19:40.7250782Z Caused by:
2020-04-11T20:19:40.7250782Z Caused by:
2020-04-11T20:19:40.7293143Z   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_middle --edition=2018 src/librustc_middle/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata -C opt-level=3 -C debuginfo=0 -C metadata=4f2e8b78c0d3975f -C extra-filename=-4f2e8b78c0d3975f --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-8fb50653cc5b9675.rmeta --extern backtrace=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbacktrace-b93755e97321c534.rmeta --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-7941118bbdbac8c6.rmeta --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-8a9bd9d8d175184d.rmeta --extern chalk_ir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libchalk_ir-b35c0e67b3a1bd51.rmeta --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-800c60dc1aa7f7d7.rmeta --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-02f0f6db374c810f.rmeta --extern measureme=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libmeasureme-22170c16ed51548a.rmeta --extern parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libparking_lot-1262d1209f836bea.rmeta --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-124a7a0460e1d85a.rmeta --extern rustc_rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon-ecb41c25aaec48e5.rmeta --extern rustc_rayon_core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon_core-305bf65d219b202b.rmeta --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-6eade0d298c9229d.rmeta --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-9dfeb28fa0788d42.rmeta --extern rustc_attr=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_attr-b4e187d30925d410.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-32b0d103c414b59d.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-846c9693b30e9921.rmeta --extern rustc_feature=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_feature-90677d3b4e486ab5.rmeta --extern rustc_hir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir-96436f7a94ec7575.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-87a289f735fe3e83.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps/librustc_macros-02de1b3228ba9665.so --extern rustc_query_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_query_system-94eb57692922fc80.rmeta --extern rustc_session=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-eb2a953ebfd0813d.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-2f4e53cd021a2f7c.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-11672d8a9c95dd2d.rmeta --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-8622965ebca5ff77.rmeta --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-f2f9c6d4ed07d9cb.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-c29aa775c67e8de6.rmeta --cfg=bootstrap -Zmacro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Wrust_2018_idioms -Wunused_lifetimes -Dwarnings -Zunstable-options '-Wrustc::internal' -Cprefer-dynamic --cfg=parallel_compiler -Zbinary-dep-depinfo -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-7615ed222efdef73/out` (signal: 9, SIGKILL: kill)
2020-04-11T20:19:40.7331695Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-11T20:19:40.7337918Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-04-11T20:19:40.7338280Z Build completed unsuccessfully in 0:31:32
2020-04-11T20:19:41.1977419Z == clock drift check ==
2020-04-11T20:19:41.1978065Z   local time: Sat Apr 11 20:19:40 UTC 2020
2020-04-11T20:19:41.1978065Z   local time: Sat Apr 11 20:19:40 UTC 2020
2020-04-11T20:19:41.6926223Z   network time: Sat, 11 Apr 2020 20:19:41 GMT
2020-04-11T20:19:43.2206241Z 
2020-04-11T20:19:43.2206241Z 
2020-04-11T20:19:43.4292767Z ##[error]Bash exited with code '1'.
2020-04-11T20:19:43.4959956Z ##[section]Finishing: Run build
2020-04-11T20:19:43.5391257Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-04-11T20:19:43.5645321Z Task         : Get sources
2020-04-11T20:19:43.5652647Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-11T20:19:43.5652991Z Version      : 1.0.0
2020-04-11T20:19:43.5653233Z Author       : Microsoft
2020-04-11T20:19:43.5653233Z Author       : Microsoft
2020-04-11T20:19:43.5659536Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-11T20:19:43.5659972Z ==============================================================================
2020-04-11T20:19:44.0990787Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-11T20:19:44.1039182Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-04-11T20:19:44.1550178Z Cleaning up task key
2020-04-11T20:19:44.1551416Z Start cleaning up orphan processes.
2020-04-11T20:19:44.2285898Z Terminate orphan process: pid (3887) (python)
2020-04-11T20:19:44.3520543Z ##[section]Finishing: Finalize Job
