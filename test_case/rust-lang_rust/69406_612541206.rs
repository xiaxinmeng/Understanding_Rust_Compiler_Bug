plain
2020-04-11T22:57:52.0607537Z ========================== Starting Command Output ===========================
2020-04-11T22:57:52.0610546Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/382a34e0-1b22-4af2-bd45-719787363f45.sh
2020-04-11T22:57:52.2864061Z 
2020-04-11T22:57:52.2929757Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-11T22:57:52.2954244Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-04-11T22:57:52.2961794Z Task         : Get sources
2020-04-11T22:57:52.2962074Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-11T22:57:52.2962366Z Version      : 1.0.0
2020-04-11T22:57:52.2962549Z Author       : Microsoft
---
2020-04-11T22:57:54.4295974Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-11T22:57:54.4485622Z ##[command]git config gc.auto 0
2020-04-11T22:57:54.4526189Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-11T22:57:54.4553504Z ##[command]git config --get-all http.proxy
2020-04-11T22:57:54.4642207Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69406/merge:refs/remotes/pull/69406/merge
---
2020-04-11T23:01:51.5162760Z  ---> 78ad2f4d4aca
2020-04-11T23:01:51.5164538Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-11T23:01:51.5168472Z  ---> Using cache
2020-04-11T23:01:51.5168871Z  ---> 4d2dc61c4d00
2020-04-11T23:01:51.5170240Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-11T23:01:51.5173724Z  ---> 776b6266a8b7
2020-04-11T23:01:51.5206002Z Successfully built 776b6266a8b7
2020-04-11T23:01:51.5229220Z Successfully tagged rust-ci:latest
2020-04-11T23:01:51.5608869Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-11T23:01:51.5608869Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-11T23:01:51.5623531Z Looks like docker image is the same as before, not uploading
2020-04-11T23:01:58.8441865Z [CI_JOB_NAME=mingw-check]
2020-04-11T23:01:58.8675250Z [CI_JOB_NAME=mingw-check]
2020-04-11T23:01:58.8701629Z == clock drift check ==
2020-04-11T23:01:58.8711997Z   local time: Sat Apr 11 23:01:58 UTC 2020
2020-04-11T23:01:59.1788112Z   network time: Sat, 11 Apr 2020 23:01:59 GMT
2020-04-11T23:01:59.1806669Z Starting sccache server...
2020-04-11T23:01:59.2800859Z configure: processing command line
2020-04-11T23:01:59.2801176Z configure: 
2020-04-11T23:01:59.2802234Z configure: rust.parallel-compiler := True
---
2020-04-11T23:04:12.3852316Z     Checking once_cell v1.1.0
2020-04-11T23:04:12.5003016Z     Checking rustc_error_codes v0.0.0 (/checkout/src/librustc_error_codes)
2020-04-11T23:04:12.5181376Z    Compiling semver v0.9.0
2020-04-11T23:04:12.7586421Z     Checking crossbeam-utils v0.6.5
2020-04-11T23:04:13.0609081Z     Checking chalk-macros v0.1.1 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-11T23:04:13.2177340Z     Checking lock_api v0.3.1
2020-04-11T23:04:13.4612900Z     Checking unicode-normalization v0.1.11
2020-04-11T23:04:14.0242999Z     Checking arrayvec v0.4.7
2020-04-11T23:04:14.3383992Z     Checking itertools v0.8.0
---
2020-04-11T23:04:30.0946979Z     Checking parking_lot v0.10.0
2020-04-11T23:04:30.2924004Z     Checking rls-span v0.5.1
2020-04-11T23:04:30.3022560Z     Checking serde_json v1.0.40
2020-04-11T23:04:30.7870625Z     Checking rand_core v0.5.1
2020-04-11T23:04:31.0640179Z     Checking chalk-engine v0.9.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-11T23:04:33.3254011Z     Checking block-buffer v0.7.3
2020-04-11T23:04:33.4108630Z     Checking digest v0.8.1
2020-04-11T23:04:33.4959823Z     Checking backtrace v0.3.46
2020-04-11T23:04:33.7209059Z     Checking rls-data v0.19.0
---
2020-04-11T23:05:02.2740689Z     Checking rustc-rayon v0.3.0
2020-04-11T23:05:04.3299160Z     Checking rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
2020-04-11T23:05:05.6359530Z     Checking arena v0.0.0 (/checkout/src/libarena)
2020-04-11T23:05:12.7925847Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2020-04-11T23:05:12.7964461Z    Compiling chalk-derive v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-11T23:05:33.7871604Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2020-04-11T23:05:34.1851304Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-11T23:05:34.5413102Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-11T23:05:34.5413102Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-11T23:05:34.7155329Z     Checking chalk-ir v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-11T23:05:35.4188396Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-11T23:05:38.7680368Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-11T23:05:38.7680368Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-11T23:05:40.0383296Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-11T23:05:40.8035311Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-11T23:05:41.2045889Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-11T23:05:41.4940317Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-04-11T23:05:41.4940317Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-04-11T23:05:42.5521497Z     Checking chalk-rust-ir v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-11T23:05:43.5966405Z     Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-04-11T23:05:44.2573202Z     Checking chalk-solve v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c720442f4837e1f1cf616ae80e598f1808cadd9#2c720442)
2020-04-11T23:05:47.9588925Z     Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
2020-04-11T23:33:15.5933642Z error: could not compile `rustc_middle`.
2020-04-11T23:33:15.5964708Z 
2020-04-11T23:33:15.5965092Z Caused by:
2020-04-11T23:33:15.5965092Z Caused by:
2020-04-11T23:33:15.6023300Z   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_middle --edition=2018 src/librustc_middle/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata -C opt-level=3 -C debuginfo=0 -C metadata=4f2e8b78c0d3975f -C extra-filename=-4f2e8b78c0d3975f --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-8fb50653cc5b9675.rmeta --extern backtrace=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbacktrace-b93755e97321c534.rmeta --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-7941118bbdbac8c6.rmeta --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-8a9bd9d8d175184d.rmeta --extern chalk_ir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libchalk_ir-b35c0e67b3a1bd51.rmeta --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-800c60dc1aa7f7d7.rmeta --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-02f0f6db374c810f.rmeta --extern measureme=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libmeasureme-22170c16ed51548a.rmeta --extern parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libparking_lot-1262d1209f836bea.rmeta --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-124a7a0460e1d85a.rmeta --extern rustc_rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon-ecb41c25aaec48e5.rmeta --extern rustc_rayon_core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon_core-305bf65d219b202b.rmeta --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-6eade0d298c9229d.rmeta --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-9dfeb28fa0788d42.rmeta --extern rustc_attr=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_attr-b4e187d30925d410.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-32b0d103c414b59d.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-846c9693b30e9921.rmeta --extern rustc_feature=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_feature-90677d3b4e486ab5.rmeta --extern rustc_hir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir-96436f7a94ec7575.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-87a289f735fe3e83.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps/librustc_macros-02de1b3228ba9665.so --extern rustc_query_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_query_system-94eb57692922fc80.rmeta --extern rustc_session=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-eb2a953ebfd0813d.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-2f4e53cd021a2f7c.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-11672d8a9c95dd2d.rmeta --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-8622965ebca5ff77.rmeta --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-f2f9c6d4ed07d9cb.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-c29aa775c67e8de6.rmeta --cfg=bootstrap -Zmacro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Wrust_2018_idioms -Wunused_lifetimes -Dwarnings -Zunstable-options '-Wrustc::internal' -Cprefer-dynamic --cfg=parallel_compiler -Zbinary-dep-depinfo -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-7615ed222efdef73/out` (signal: 9, SIGKILL: kill)
2020-04-11T23:33:15.6070740Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-11T23:33:15.6093543Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-04-11T23:33:15.6093937Z Build completed unsuccessfully in 0:31:16
2020-04-11T23:33:15.8876403Z == clock drift check ==
2020-04-11T23:33:15.9006324Z   local time: Sat Apr 11 23:33:15 UTC 2020
2020-04-11T23:33:15.9006324Z   local time: Sat Apr 11 23:33:15 UTC 2020
2020-04-11T23:33:16.2919614Z   network time: Sat, 11 Apr 2020 23:33:16 GMT
2020-04-11T23:33:17.7173031Z 
2020-04-11T23:33:17.7173031Z 
2020-04-11T23:33:17.9513898Z ##[error]Bash exited with code '1'.
2020-04-11T23:33:18.0013196Z ##[section]Finishing: Run build
2020-04-11T23:33:18.0463360Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-04-11T23:33:18.0738763Z Task         : Get sources
2020-04-11T23:33:18.0739119Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-11T23:33:18.0741389Z Version      : 1.0.0
2020-04-11T23:33:18.0741658Z Author       : Microsoft
2020-04-11T23:33:18.0741658Z Author       : Microsoft
2020-04-11T23:33:18.0745615Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-11T23:33:18.0749302Z ==============================================================================
2020-04-11T23:33:18.6011607Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-11T23:33:18.6070554Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-04-11T23:33:18.6315584Z Cleaning up task key
2020-04-11T23:33:18.6316874Z Start cleaning up orphan processes.
2020-04-11T23:33:18.6813332Z Terminate orphan process: pid (4200) (python)
2020-04-11T23:33:18.7873699Z ##[section]Finishing: Finalize Job
