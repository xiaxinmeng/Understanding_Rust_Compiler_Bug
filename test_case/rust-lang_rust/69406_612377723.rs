plain
2020-04-11T08:32:08.9741020Z ========================== Starting Command Output ===========================
2020-04-11T08:32:08.9744369Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/1cafc683-cd62-48d1-ab70-4313247cdde7.sh
2020-04-11T08:32:08.9744946Z 
2020-04-11T08:32:08.9751066Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-11T08:32:08.9771190Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-04-11T08:32:08.9776500Z Task         : Get sources
2020-04-11T08:32:08.9776798Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-11T08:32:08.9777089Z Version      : 1.0.0
2020-04-11T08:32:08.9777283Z Author       : Microsoft
---
2020-04-11T08:32:10.5722654Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-11T08:32:10.5735792Z ##[command]git config gc.auto 0
2020-04-11T08:32:10.5745312Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-11T08:32:10.5754824Z ##[command]git config --get-all http.proxy
2020-04-11T08:32:10.5769416Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69406/merge:refs/remotes/pull/69406/merge
---
2020-04-11T08:35:59.7544035Z  ---> 78ad2f4d4aca
2020-04-11T08:35:59.7544284Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-11T08:35:59.7548872Z  ---> Using cache
2020-04-11T08:35:59.7549233Z  ---> 4d2dc61c4d00
2020-04-11T08:35:59.7550672Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-11T08:35:59.7556059Z  ---> 776b6266a8b7
2020-04-11T08:35:59.7594403Z Successfully built 776b6266a8b7
2020-04-11T08:35:59.7624276Z Successfully tagged rust-ci:latest
2020-04-11T08:35:59.7945601Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-11T08:35:59.7945601Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-11T08:35:59.7961427Z Looks like docker image is the same as before, not uploading
2020-04-11T08:36:06.1471957Z [CI_JOB_NAME=mingw-check]
2020-04-11T08:36:06.1714633Z [CI_JOB_NAME=mingw-check]
2020-04-11T08:36:06.1759620Z == clock drift check ==
2020-04-11T08:36:06.1770583Z   local time: Sat Apr 11 08:36:06 UTC 2020
2020-04-11T08:36:06.2418643Z   network time: Sat, 11 Apr 2020 08:36:06 GMT
2020-04-11T08:36:06.2460887Z Starting sccache server...
2020-04-11T08:36:06.3797363Z configure: processing command line
2020-04-11T08:36:06.3798228Z configure: 
2020-04-11T08:36:06.3799323Z configure: rust.parallel-compiler := True
---
2020-04-11T08:39:05.3548880Z     Checking once_cell v1.1.0
2020-04-11T08:39:05.5488330Z     Checking rustc_error_codes v0.0.0 (/checkout/src/librustc_error_codes)
2020-04-11T08:39:05.5963473Z    Compiling semver v0.9.0
2020-04-11T08:39:05.7681539Z     Checking crossbeam-utils v0.6.5
2020-04-11T08:39:06.1538421Z     Checking chalk-macros v0.1.1 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-11T08:39:06.3737434Z     Checking lock_api v0.3.1
2020-04-11T08:39:06.7296904Z     Checking unicode-normalization v0.1.11
2020-04-11T08:39:07.6345438Z     Checking arrayvec v0.4.7
2020-04-11T08:39:08.1162649Z     Checking itertools v0.8.0
---
2020-04-11T08:39:30.4566221Z     Checking flate2 v1.0.12
2020-04-11T08:39:30.6710919Z     Checking parking_lot v0.10.0
2020-04-11T08:39:31.0303526Z     Checking rls-span v0.5.1
2020-04-11T08:39:31.1279677Z     Checking serde_json v1.0.40
2020-04-11T08:39:31.5902670Z     Checking chalk-engine v0.9.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-11T08:39:34.4296596Z     Checking rand_core v0.5.1
2020-04-11T08:39:34.8014849Z     Checking block-buffer v0.7.3
2020-04-11T08:39:34.9143498Z     Checking digest v0.8.1
2020-04-11T08:39:35.0567667Z     Checking backtrace v0.3.46
---
2020-04-11T08:39:43.8407336Z    Compiling synstructure v0.12.1
2020-04-11T08:40:17.9736244Z     Checking rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
2020-04-11T08:40:19.8624888Z     Checking arena v0.0.0 (/checkout/src/libarena)
2020-04-11T08:40:30.6528311Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2020-04-11T08:40:30.6571579Z    Compiling chalk-derive v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-11T08:41:00.0353173Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2020-04-11T08:41:00.9132895Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-11T08:41:01.3802806Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-11T08:41:01.3802806Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-11T08:41:01.5520845Z     Checking chalk-ir v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-11T08:41:02.5125786Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-11T08:41:07.5203943Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-11T08:41:07.5203943Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-11T08:41:09.5767397Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-11T08:41:09.8095587Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-11T08:41:10.5445901Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-11T08:41:10.9632797Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-04-11T08:41:10.9632797Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-04-11T08:41:12.5660594Z     Checking chalk-rust-ir v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-11T08:41:13.8958522Z     Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-04-11T08:41:14.8389685Z     Checking chalk-solve v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-11T08:41:19.7405247Z     Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
2020-04-11T09:11:48.9343588Z error: could not compile `rustc_middle`.
2020-04-11T09:11:48.9372574Z 
2020-04-11T09:11:48.9372844Z Caused by:
2020-04-11T09:11:48.9372844Z Caused by:
2020-04-11T09:11:48.9442447Z   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_middle --edition=2018 src/librustc_middle/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata -C opt-level=3 -C debuginfo=0 -C metadata=4f2e8b78c0d3975f -C extra-filename=-4f2e8b78c0d3975f --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-8fb50653cc5b9675.rmeta --extern backtrace=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbacktrace-b93755e97321c534.rmeta --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-7941118bbdbac8c6.rmeta --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-8a9bd9d8d175184d.rmeta --extern chalk_ir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libchalk_ir-b35c0e67b3a1bd51.rmeta --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-800c60dc1aa7f7d7.rmeta --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-02f0f6db374c810f.rmeta --extern measureme=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libmeasureme-22170c16ed51548a.rmeta --extern parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libparking_lot-1262d1209f836bea.rmeta --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-124a7a0460e1d85a.rmeta --extern rustc_rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon-ecb41c25aaec48e5.rmeta --extern rustc_rayon_core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon_core-305bf65d219b202b.rmeta --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-6eade0d298c9229d.rmeta --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-9dfeb28fa0788d42.rmeta --extern rustc_attr=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_attr-b4e187d30925d410.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-32b0d103c414b59d.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-846c9693b30e9921.rmeta --extern rustc_feature=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_feature-90677d3b4e486ab5.rmeta --extern rustc_hir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir-96436f7a94ec7575.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-87a289f735fe3e83.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps/librustc_macros-02de1b3228ba9665.so --extern rustc_query_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_query_system-94eb57692922fc80.rmeta --extern rustc_session=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-eb2a953ebfd0813d.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-2f4e53cd021a2f7c.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-11672d8a9c95dd2d.rmeta --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-8622965ebca5ff77.rmeta --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-f2f9c6d4ed07d9cb.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-c29aa775c67e8de6.rmeta --cfg=bootstrap -Zmacro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Wrust_2018_idioms -Wunused_lifetimes -Dwarnings -Zunstable-options '-Wrustc::internal' -Cprefer-dynamic --cfg=parallel_compiler -Zbinary-dep-depinfo -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-7615ed222efdef73/out` (signal: 9, SIGKILL: kill)
2020-04-11T09:11:49.0491430Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-11T09:11:49.0515174Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-04-11T09:11:49.0515558Z Build completed unsuccessfully in 0:35:42
2020-04-11T09:11:49.2936657Z == clock drift check ==
2020-04-11T09:11:49.3220879Z   local time: Sat Apr 11 09:11:49 UTC 2020
2020-04-11T09:11:49.3220879Z   local time: Sat Apr 11 09:11:49 UTC 2020
2020-04-11T09:11:49.4920607Z   network time: Sat, 11 Apr 2020 09:11:49 GMT
2020-04-11T09:11:51.1616453Z 
2020-04-11T09:11:51.1616453Z 
2020-04-11T09:11:51.3998592Z ##[error]Bash exited with code '1'.
2020-04-11T09:11:51.4433291Z ##[section]Finishing: Run build
2020-04-11T09:11:51.5132337Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-04-11T09:11:51.5515808Z Task         : Get sources
2020-04-11T09:11:51.5519856Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-11T09:11:51.5520216Z Version      : 1.0.0
2020-04-11T09:11:51.5525595Z Author       : Microsoft
2020-04-11T09:11:51.5525595Z Author       : Microsoft
2020-04-11T09:11:51.5526133Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-11T09:11:51.5526544Z ==============================================================================
2020-04-11T09:11:52.1162238Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-11T09:11:52.1231428Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-04-11T09:11:52.1742814Z Cleaning up task key
2020-04-11T09:11:52.1744917Z Start cleaning up orphan processes.
2020-04-11T09:11:52.3139041Z Terminate orphan process: pid (4840) (python)
2020-04-11T09:11:52.4948114Z ##[section]Finishing: Finalize Job
