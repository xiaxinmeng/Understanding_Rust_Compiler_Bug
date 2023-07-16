plain
2020-04-15T21:34:11.8008790Z ========================== Starting Command Output ===========================
2020-04-15T21:34:11.8011671Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/908b50b4-aafe-48c3-a968-7aad2cefcd56.sh
2020-04-15T21:34:11.8011936Z 
2020-04-15T21:34:11.8017149Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-15T21:34:11.8036573Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-04-15T21:34:11.8039742Z Task         : Get sources
2020-04-15T21:34:11.8040022Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-15T21:34:11.8040315Z Version      : 1.0.0
2020-04-15T21:34:11.8040502Z Author       : Microsoft
---
2020-04-15T21:34:13.0524871Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-15T21:34:13.0532307Z ##[command]git config gc.auto 0
2020-04-15T21:34:13.0537272Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-15T21:34:13.0543616Z ##[command]git config --get-all http.proxy
2020-04-15T21:34:13.0554160Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69406/merge:refs/remotes/pull/69406/merge
---
2020-04-15T21:36:42.1569434Z  ---> 78ad2f4d4aca
2020-04-15T21:36:42.1569661Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-15T21:36:42.1578218Z  ---> Using cache
2020-04-15T21:36:42.1578572Z  ---> 4d2dc61c4d00
2020-04-15T21:36:42.1582042Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-15T21:36:42.1583353Z  ---> 776b6266a8b7
2020-04-15T21:36:42.1613326Z Successfully built 776b6266a8b7
2020-04-15T21:36:42.1653832Z Successfully tagged rust-ci:latest
2020-04-15T21:36:42.1966523Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-15T21:36:42.1966523Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-15T21:36:42.1982111Z Looks like docker image is the same as before, not uploading
2020-04-15T21:36:42.9677652Z [CI_JOB_NAME=mingw-check]
2020-04-15T21:36:42.9977354Z [CI_JOB_NAME=mingw-check]
2020-04-15T21:36:43.0009339Z == clock drift check ==
2020-04-15T21:36:43.0024813Z   local time: Wed Apr 15 21:36:43 UTC 2020
2020-04-15T21:36:43.3154320Z   network time: Wed, 15 Apr 2020 21:36:43 GMT
2020-04-15T21:36:43.3178310Z Starting sccache server...
2020-04-15T21:36:43.4553195Z configure: processing command line
2020-04-15T21:36:43.4554155Z configure: 
2020-04-15T21:36:43.4556128Z configure: rust.parallel-compiler := True
---
2020-04-15T21:39:09.6183224Z     Checking once_cell v1.1.0
2020-04-15T21:39:09.8471353Z     Checking rustc_error_codes v0.0.0 (/checkout/src/librustc_error_codes)
2020-04-15T21:39:09.9241591Z    Compiling semver v0.9.0
2020-04-15T21:39:10.1318672Z     Checking crossbeam-utils v0.6.5
2020-04-15T21:39:10.4435345Z     Checking chalk-macros v0.1.1 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-15T21:39:10.6245770Z     Checking lock_api v0.3.1
2020-04-15T21:39:10.9199829Z     Checking unicode-normalization v0.1.11
2020-04-15T21:39:11.4795565Z     Checking arrayvec v0.4.7
2020-04-15T21:39:11.8308501Z     Checking itertools v0.8.0
---
2020-04-15T21:39:28.8277791Z     Checking parking_lot v0.10.0
2020-04-15T21:39:29.0459309Z     Checking rls-span v0.5.1
2020-04-15T21:39:29.1174407Z     Checking serde_json v1.0.40
2020-04-15T21:39:29.4895897Z     Checking rand_core v0.5.1
2020-04-15T21:39:29.8646648Z     Checking chalk-engine v0.9.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-15T21:39:32.3481184Z     Checking digest v0.8.1
2020-04-15T21:39:32.4419950Z     Checking block-buffer v0.7.3
2020-04-15T21:39:32.5337366Z     Checking backtrace v0.3.46
2020-04-15T21:39:32.7775127Z     Checking rls-data v0.19.0
---
2020-04-15T21:40:02.3403697Z     Checking rustc-rayon v0.3.0
2020-04-15T21:40:04.5044027Z     Checking rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
2020-04-15T21:40:05.8652015Z     Checking arena v0.0.0 (/checkout/src/libarena)
2020-04-15T21:40:13.5839253Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2020-04-15T21:40:13.5840741Z    Compiling chalk-derive v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-15T21:40:35.5005813Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2020-04-15T21:40:35.7890066Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-15T21:40:36.1825040Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-15T21:40:36.1825040Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-15T21:40:36.3737204Z     Checking chalk-ir v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-15T21:40:37.2317861Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-15T21:40:40.8837443Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-15T21:40:40.8837443Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-15T21:40:42.2952865Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-15T21:40:42.9871249Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-15T21:40:43.4035069Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-15T21:40:43.5469973Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-04-15T21:40:43.5469973Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-04-15T21:40:44.6559086Z     Checking chalk-rust-ir v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-15T21:40:45.7213518Z     Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-04-15T21:40:46.4357327Z     Checking chalk-solve v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-15T21:40:50.2750976Z     Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
2020-04-15T22:11:01.9650568Z error: could not compile `rustc_middle`.
2020-04-15T22:11:01.9721974Z 
2020-04-15T22:11:01.9722173Z Caused by:
2020-04-15T22:11:01.9722173Z Caused by:
2020-04-15T22:11:01.9770542Z   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_middle --edition=2018 src/librustc_middle/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata -C opt-level=3 -C debuginfo=0 -C metadata=c3a2881aff5e4d4a -C extra-filename=-c3a2881aff5e4d4a --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-4beaf876d25f5744.rmeta --extern backtrace=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbacktrace-b93755e97321c534.rmeta --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-7941118bbdbac8c6.rmeta --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-8a9bd9d8d175184d.rmeta --extern chalk_ir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libchalk_ir-b35c0e67b3a1bd51.rmeta --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-800c60dc1aa7f7d7.rmeta --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-02f0f6db374c810f.rmeta --extern measureme=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libmeasureme-22170c16ed51548a.rmeta --extern parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libparking_lot-1262d1209f836bea.rmeta --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-124a7a0460e1d85a.rmeta --extern rustc_rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon-ecb41c25aaec48e5.rmeta --extern rustc_rayon_core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon_core-305bf65d219b202b.rmeta --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-6eade0d298c9229d.rmeta --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-61fdd265992909f5.rmeta --extern rustc_attr=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_attr-edd453f8f9514470.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-c450cab2bc3b7475.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-88f8008f144e867f.rmeta --extern rustc_feature=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_feature-7b0a2d8d5d80b824.rmeta --extern rustc_hir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir-47bdacda590246b2.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-87a289f735fe3e83.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps/librustc_macros-02de1b3228ba9665.so --extern rustc_query_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_query_system-3eba29064d1b3a5b.rmeta --extern rustc_session=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-3f57b7f565eeb861.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-9f60ad89847e8b4b.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-064c6ae3c71df6c9.rmeta --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-8622965ebca5ff77.rmeta --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-f2f9c6d4ed07d9cb.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-c29aa775c67e8de6.rmeta --cfg=bootstrap -Zmacro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Wrust_2018_idioms -Wunused_lifetimes -Dwarnings -Zunstable-options '-Wrustc::internal' -Cprefer-dynamic --cfg=parallel_compiler -Zbinary-dep-depinfo -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-7615ed222efdef73/out` (signal: 9, SIGKILL: kill)
2020-04-15T22:11:01.9988055Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-15T22:11:02.0030412Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-04-15T22:11:02.0033368Z Build completed unsuccessfully in 0:34:18
2020-04-15T22:11:02.2331052Z == clock drift check ==
2020-04-15T22:11:02.2507362Z   local time: Wed Apr 15 22:11:02 UTC 2020
2020-04-15T22:11:02.2507362Z   local time: Wed Apr 15 22:11:02 UTC 2020
2020-04-15T22:11:02.4244993Z   network time: Wed, 15 Apr 2020 22:11:02 GMT
2020-04-15T22:11:04.4333276Z 
2020-04-15T22:11:04.4333276Z 
2020-04-15T22:11:04.8258003Z ##[error]Bash exited with code '1'.
2020-04-15T22:11:04.8804436Z ##[section]Finishing: Run build
2020-04-15T22:11:04.9184167Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-04-15T22:11:04.9503054Z Task         : Get sources
2020-04-15T22:11:04.9503514Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-15T22:11:04.9503877Z Version      : 1.0.0
2020-04-15T22:11:04.9504133Z Author       : Microsoft
2020-04-15T22:11:04.9504133Z Author       : Microsoft
2020-04-15T22:11:04.9504550Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-15T22:11:04.9505013Z ==============================================================================
2020-04-15T22:11:05.4903891Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-15T22:11:05.4956614Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-04-15T22:11:05.5297323Z Cleaning up task key
2020-04-15T22:11:05.5298732Z Start cleaning up orphan processes.
2020-04-15T22:11:05.7107947Z Terminate orphan process: pid (4726) (python)
2020-04-15T22:11:05.7187589Z ##[section]Finishing: Finalize Job
