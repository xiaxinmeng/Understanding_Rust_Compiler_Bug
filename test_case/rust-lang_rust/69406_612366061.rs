plain
2020-04-11T07:54:24.8963029Z ========================== Starting Command Output ===========================
2020-04-11T07:54:24.8965010Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/d8b1d200-f817-4bd9-ade6-dc678ebf4acf.sh
2020-04-11T07:54:24.8965261Z 
2020-04-11T07:54:24.8967969Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-11T07:54:24.8986114Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-04-11T07:54:24.8990447Z Task         : Get sources
2020-04-11T07:54:24.8990696Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-11T07:54:24.8990955Z Version      : 1.0.0
2020-04-11T07:54:24.8991115Z Author       : Microsoft
---
2020-04-11T07:54:28.0569194Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-11T07:54:28.0578614Z ##[command]git config gc.auto 0
2020-04-11T07:54:28.0584351Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-11T07:54:28.0590168Z ##[command]git config --get-all http.proxy
2020-04-11T07:54:28.0602218Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69406/merge:refs/remotes/pull/69406/merge
---
2020-04-11T07:58:06.0236309Z  ---> 78ad2f4d4aca
2020-04-11T07:58:06.0236613Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-11T07:58:06.0239402Z  ---> Using cache
2020-04-11T07:58:06.0239666Z  ---> 4d2dc61c4d00
2020-04-11T07:58:06.0240628Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-11T07:58:06.0243572Z  ---> 776b6266a8b7
2020-04-11T07:58:06.0272553Z Successfully built 776b6266a8b7
2020-04-11T07:58:06.0306806Z Successfully tagged rust-ci:latest
2020-04-11T07:58:06.0659167Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-11T07:58:06.0659167Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-11T07:58:06.6195002Z Looks like docker image is the same as before, not uploading
2020-04-11T07:58:12.1434207Z [CI_JOB_NAME=mingw-check]
2020-04-11T07:58:12.1620907Z [CI_JOB_NAME=mingw-check]
2020-04-11T07:58:12.1647215Z == clock drift check ==
2020-04-11T07:58:12.1657264Z   local time: Sat Apr 11 07:58:12 UTC 2020
2020-04-11T07:58:12.4880187Z   network time: Sat, 11 Apr 2020 07:58:12 GMT
2020-04-11T07:58:12.4899977Z Starting sccache server...
2020-04-11T07:58:12.5936561Z configure: processing command line
2020-04-11T07:58:12.5936869Z configure: 
2020-04-11T07:58:12.5956657Z configure: rust.parallel-compiler := True
---
2020-04-11T08:00:16.7396319Z    Compiling rustc_driver v0.0.0 (/checkout/src/librustc_driver)
2020-04-11T08:00:16.7886309Z     Checking rustc_error_codes v0.0.0 (/checkout/src/librustc_error_codes)
2020-04-11T08:00:16.9084739Z    Compiling semver v0.9.0
2020-04-11T08:00:16.9448485Z     Checking crossbeam-utils v0.6.5
2020-04-11T08:00:17.2328256Z     Checking chalk-macros v0.1.1 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-11T08:00:17.4032913Z     Checking lock_api v0.3.1
2020-04-11T08:00:17.6439454Z     Checking unicode-normalization v0.1.11
2020-04-11T08:00:18.2970879Z     Checking arrayvec v0.4.7
2020-04-11T08:00:18.5581672Z     Checking itertools v0.8.0
---
2020-04-11T08:00:33.2591894Z     Checking parking_lot v0.10.0
2020-04-11T08:00:33.4642359Z     Checking rls-span v0.5.1
2020-04-11T08:00:33.6774075Z     Checking serde_json v1.0.40
2020-04-11T08:00:33.9520778Z     Checking rand_core v0.5.1
2020-04-11T08:00:34.1818675Z     Checking chalk-engine v0.9.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-11T08:00:36.2834280Z     Checking digest v0.8.1
2020-04-11T08:00:36.3685240Z     Checking block-buffer v0.7.3
2020-04-11T08:00:36.4535656Z     Checking backtrace v0.3.46
2020-04-11T08:00:36.6729926Z     Checking rls-data v0.19.0
---
2020-04-11T08:01:03.6800323Z     Checking rustc-rayon v0.3.0
2020-04-11T08:01:04.7128771Z     Checking rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
2020-04-11T08:01:05.8437629Z     Checking arena v0.0.0 (/checkout/src/libarena)
2020-04-11T08:01:12.8164160Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2020-04-11T08:01:12.8168743Z    Compiling chalk-derive v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-11T08:01:32.4108879Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2020-04-11T08:01:33.2005414Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-11T08:01:33.2006107Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-11T08:01:33.2006107Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-11T08:01:33.2966397Z     Checking chalk-ir v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-11T08:01:33.9484140Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-11T08:01:37.2147511Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-11T08:01:37.2147511Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-11T08:01:38.5032711Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-11T08:01:39.2200917Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-11T08:01:39.5940950Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-11T08:01:39.7998046Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-04-11T08:01:39.7998046Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-04-11T08:01:40.8151246Z     Checking chalk-rust-ir v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-11T08:01:41.8027824Z     Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-04-11T08:01:42.4237138Z     Checking chalk-solve v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-11T08:01:45.7949585Z     Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
2020-04-11T08:27:27.8712676Z error: could not compile `rustc_middle`.
2020-04-11T08:27:27.8732974Z 
2020-04-11T08:27:27.8733178Z Caused by:
2020-04-11T08:27:27.8733178Z Caused by:
2020-04-11T08:27:27.8781981Z   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_middle --edition=2018 src/librustc_middle/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata -C opt-level=3 -C debuginfo=0 -C metadata=4f2e8b78c0d3975f -C extra-filename=-4f2e8b78c0d3975f --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-8fb50653cc5b9675.rmeta --extern backtrace=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbacktrace-b93755e97321c534.rmeta --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-7941118bbdbac8c6.rmeta --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-8a9bd9d8d175184d.rmeta --extern chalk_ir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libchalk_ir-b35c0e67b3a1bd51.rmeta --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-800c60dc1aa7f7d7.rmeta --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-02f0f6db374c810f.rmeta --extern measureme=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libmeasureme-22170c16ed51548a.rmeta --extern parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libparking_lot-1262d1209f836bea.rmeta --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-124a7a0460e1d85a.rmeta --extern rustc_rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon-ecb41c25aaec48e5.rmeta --extern rustc_rayon_core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon_core-305bf65d219b202b.rmeta --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-6eade0d298c9229d.rmeta --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-9dfeb28fa0788d42.rmeta --extern rustc_attr=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_attr-b4e187d30925d410.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-32b0d103c414b59d.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-846c9693b30e9921.rmeta --extern rustc_feature=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_feature-90677d3b4e486ab5.rmeta --extern rustc_hir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir-96436f7a94ec7575.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-87a289f735fe3e83.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps/librustc_macros-02de1b3228ba9665.so --extern rustc_query_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_query_system-94eb57692922fc80.rmeta --extern rustc_session=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-eb2a953ebfd0813d.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-2f4e53cd021a2f7c.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-11672d8a9c95dd2d.rmeta --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-8622965ebca5ff77.rmeta --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-f2f9c6d4ed07d9cb.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-c29aa775c67e8de6.rmeta --cfg=bootstrap -Zmacro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Wrust_2018_idioms -Wunused_lifetimes -Dwarnings -Zunstable-options '-Wrustc::internal' -Cprefer-dynamic --cfg=parallel_compiler -Zbinary-dep-depinfo -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-7615ed222efdef73/out` (signal: 9, SIGKILL: kill)
2020-04-11T08:27:27.8804891Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-11T08:27:27.8826908Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-04-11T08:27:27.8831460Z Build completed unsuccessfully in 0:29:15
2020-04-11T08:27:28.1650437Z == clock drift check ==
2020-04-11T08:27:28.1820927Z   local time: Sat Apr 11 08:27:28 UTC 2020
2020-04-11T08:27:28.1820927Z   local time: Sat Apr 11 08:27:28 UTC 2020
2020-04-11T08:27:28.6072754Z   network time: Sat, 11 Apr 2020 08:27:28 GMT
2020-04-11T08:27:30.0701780Z 
2020-04-11T08:27:30.0701780Z 
2020-04-11T08:27:30.3034295Z ##[error]Bash exited with code '1'.
2020-04-11T08:27:30.3373960Z ##[section]Finishing: Run build
2020-04-11T08:27:30.3704653Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-04-11T08:27:30.3933135Z Task         : Get sources
2020-04-11T08:27:30.3933497Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-11T08:27:30.3936218Z Version      : 1.0.0
2020-04-11T08:27:30.3936465Z Author       : Microsoft
2020-04-11T08:27:30.3936465Z Author       : Microsoft
2020-04-11T08:27:30.3936830Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-11T08:27:30.3937301Z ==============================================================================
2020-04-11T08:27:30.8789638Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-11T08:27:30.9382364Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-04-11T08:27:30.9807168Z Cleaning up task key
2020-04-11T08:27:30.9808376Z Start cleaning up orphan processes.
2020-04-11T08:27:31.0454686Z Terminate orphan process: pid (3843) (python)
2020-04-11T08:27:31.3243893Z ##[section]Finishing: Finalize Job
