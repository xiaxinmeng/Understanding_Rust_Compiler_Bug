plain
2020-04-12T05:14:06.0142685Z ========================== Starting Command Output ===========================
2020-04-12T05:14:06.0145544Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/3f376bcc-ad16-4b77-acf2-45a1d7f34199.sh
2020-04-12T05:14:06.0145834Z 
2020-04-12T05:14:06.0150409Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-12T05:14:06.0171454Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-04-12T05:14:06.0178801Z Task         : Get sources
2020-04-12T05:14:06.0179175Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-12T05:14:06.0179496Z Version      : 1.0.0
2020-04-12T05:14:06.0179711Z Author       : Microsoft
---
2020-04-12T05:14:07.0242258Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-12T05:14:07.0252025Z ##[command]git config gc.auto 0
2020-04-12T05:14:07.0266377Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-12T05:14:07.0272926Z ##[command]git config --get-all http.proxy
2020-04-12T05:14:07.0283604Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69406/merge:refs/remotes/pull/69406/merge
---
2020-04-12T05:16:24.5474146Z  ---> 78ad2f4d4aca
2020-04-12T05:16:24.5474431Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-12T05:16:24.5474861Z  ---> Using cache
2020-04-12T05:16:24.5475235Z  ---> 4d2dc61c4d00
2020-04-12T05:16:24.5476782Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-12T05:16:24.5478359Z  ---> 776b6266a8b7
2020-04-12T05:16:24.5478594Z Successfully built 776b6266a8b7
2020-04-12T05:16:24.5479063Z Successfully tagged rust-ci:latest
2020-04-12T05:16:24.5479495Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-12T05:16:24.5479495Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-12T05:16:24.5479936Z Looks like docker image is the same as before, not uploading
2020-04-12T05:16:33.0184982Z [CI_JOB_NAME=mingw-check]
2020-04-12T05:16:33.0496765Z [CI_JOB_NAME=mingw-check]
2020-04-12T05:16:33.0529507Z == clock drift check ==
2020-04-12T05:16:33.0554073Z   local time: Sun Apr 12 05:16:33 UTC 2020
2020-04-12T05:16:33.1454205Z   network time: Sun, 12 Apr 2020 05:16:33 GMT
2020-04-12T05:16:33.1483378Z Starting sccache server...
2020-04-12T05:16:33.2679648Z configure: processing command line
2020-04-12T05:16:33.2680414Z configure: 
2020-04-12T05:16:33.2681556Z configure: rust.parallel-compiler := True
---
2020-04-12T05:19:16.1149506Z     Checking once_cell v1.1.0
2020-04-12T05:19:16.2607887Z     Checking rustc_error_codes v0.0.0 (/checkout/src/librustc_error_codes)
2020-04-12T05:19:16.2705681Z    Compiling semver v0.9.0
2020-04-12T05:19:16.4522637Z     Checking crossbeam-utils v0.6.5
2020-04-12T05:19:16.8549295Z     Checking chalk-macros v0.1.1 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-12T05:19:17.0382923Z     Checking lock_api v0.3.1
2020-04-12T05:19:17.3548799Z     Checking unicode-normalization v0.1.11
2020-04-12T05:19:18.1236479Z     Checking arrayvec v0.4.7
2020-04-12T05:19:18.5020775Z     Checking itertools v0.8.0
---
2020-04-12T05:19:38.2781495Z     Checking parking_lot v0.10.0
2020-04-12T05:19:38.3339735Z     Checking rand_core v0.5.1
2020-04-12T05:19:38.5650432Z     Checking rls-span v0.5.1
2020-04-12T05:19:38.6201441Z     Checking serde_json v1.0.40
2020-04-12T05:19:39.1677433Z     Checking chalk-engine v0.9.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-12T05:19:42.1068439Z     Checking block-buffer v0.7.3
2020-04-12T05:19:42.1075052Z     Checking digest v0.8.1
2020-04-12T05:19:42.1075586Z     Checking backtrace v0.3.46
2020-04-12T05:19:42.3523531Z     Checking rand_chacha v0.2.1
---
2020-04-12T05:19:49.9749836Z    Compiling synstructure v0.12.1
2020-04-12T05:20:20.6356770Z     Checking rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
2020-04-12T05:20:22.2784917Z     Checking arena v0.0.0 (/checkout/src/libarena)
2020-04-12T05:20:32.0376633Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2020-04-12T05:20:32.0390091Z    Compiling chalk-derive v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-12T05:20:58.0330800Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2020-04-12T05:20:59.1000859Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-12T05:20:59.3830028Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-12T05:20:59.3830028Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-12T05:20:59.5531573Z     Checking chalk-ir v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-12T05:21:00.3247711Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-12T05:21:00.3247711Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-12T05:21:04.7253496Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-12T05:21:07.2992826Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-12T05:21:07.2992826Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-12T05:21:07.3233369Z     Checking chalk-rust-ir v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-12T05:21:08.3059712Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-12T05:21:08.5565616Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-04-12T05:21:08.5565616Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-04-12T05:21:10.5740184Z     Checking chalk-solve v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-12T05:21:14.0009606Z     Checking rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-04-12T05:21:15.8747606Z     Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
2020-04-12T05:51:27.6473761Z error: could not compile `rustc_middle`.
2020-04-12T05:51:27.6893981Z 
2020-04-12T05:51:27.6893981Z 
2020-04-12T05:51:27.6894295Z Caused by:
2020-04-12T05:51:27.6954586Z   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_middle --edition=2018 src/librustc_middle/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata -C opt-level=3 -C debuginfo=0 -C metadata=4f2e8b78c0d3975f -C extra-filename=-4f2e8b78c0d3975f --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-8fb50653cc5b9675.rmeta --extern backtrace=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbacktrace-b93755e97321c534.rmeta --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-7941118bbdbac8c6.rmeta --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-8a9bd9d8d175184d.rmeta --extern chalk_ir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libchalk_ir-b35c0e67b3a1bd51.rmeta --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-800c60dc1aa7f7d7.rmeta --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-02f0f6db374c810f.rmeta --extern measureme=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libmeasureme-22170c16ed51548a.rmeta --extern parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libparking_lot-1262d1209f836bea.rmeta --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-124a7a0460e1d85a.rmeta --extern rustc_rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon-ecb41c25aaec48e5.rmeta --extern rustc_rayon_core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon_core-305bf65d219b202b.rmeta --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-6eade0d298c9229d.rmeta --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-9dfeb28fa0788d42.rmeta --extern rustc_attr=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_attr-b4e187d30925d410.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-32b0d103c414b59d.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-846c9693b30e9921.rmeta --extern rustc_feature=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_feature-90677d3b4e486ab5.rmeta --extern rustc_hir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir-96436f7a94ec7575.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-87a289f735fe3e83.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps/librustc_macros-02de1b3228ba9665.so --extern rustc_query_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_query_system-94eb57692922fc80.rmeta --extern rustc_session=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-eb2a953ebfd0813d.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-2f4e53cd021a2f7c.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-11672d8a9c95dd2d.rmeta --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-8622965ebca5ff77.rmeta --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-f2f9c6d4ed07d9cb.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-c29aa775c67e8de6.rmeta --cfg=bootstrap -Zmacro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Wrust_2018_idioms -Wunused_lifetimes -Dwarnings -Zunstable-options '-Wrustc::internal' -Cprefer-dynamic --cfg=parallel_compiler -Zbinary-dep-depinfo -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-7615ed222efdef73/out` (signal: 9, SIGKILL: kill)
2020-04-12T05:51:27.7124595Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-12T05:51:27.7164408Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-04-12T05:51:27.7165243Z Build completed unsuccessfully in 0:34:54
2020-04-12T05:51:28.0147799Z == clock drift check ==
2020-04-12T05:51:28.0415090Z   local time: Sun Apr 12 05:51:28 UTC 2020
2020-04-12T05:51:28.0415090Z   local time: Sun Apr 12 05:51:28 UTC 2020
2020-04-12T05:51:28.4228012Z   network time: Sun, 12 Apr 2020 05:51:28 GMT
2020-04-12T05:51:29.9868875Z 
2020-04-12T05:51:29.9868875Z 
2020-04-12T05:51:30.2471210Z ##[error]Bash exited with code '1'.
2020-04-12T05:51:30.3145901Z ##[section]Finishing: Run build
2020-04-12T05:51:30.3572245Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-04-12T05:51:30.3838913Z Task         : Get sources
2020-04-12T05:51:30.3839251Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-12T05:51:30.3839552Z Version      : 1.0.0
2020-04-12T05:51:30.3843181Z Author       : Microsoft
2020-04-12T05:51:30.3843181Z Author       : Microsoft
2020-04-12T05:51:30.3844005Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-12T05:51:30.3844936Z ==============================================================================
2020-04-12T05:51:30.9301252Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-12T05:51:30.9355346Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-04-12T05:51:30.9635796Z Cleaning up task key
2020-04-12T05:51:30.9646088Z Start cleaning up orphan processes.
2020-04-12T05:51:31.0248402Z Terminate orphan process: pid (4090) (python)
2020-04-12T05:51:31.1815529Z ##[section]Finishing: Finalize Job
