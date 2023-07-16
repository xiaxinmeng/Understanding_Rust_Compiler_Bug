plain
2020-04-24T06:19:06.9789617Z ========================== Starting Command Output ===========================
2020-04-24T06:19:06.9791768Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/c7e99ad0-66c0-4c63-8f64-b06c8c5d1df4.sh
2020-04-24T06:19:06.9791983Z 
2020-04-24T06:19:06.9795748Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-24T06:19:06.9812532Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-04-24T06:19:06.9815363Z Task         : Get sources
2020-04-24T06:19:06.9815619Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-24T06:19:06.9815887Z Version      : 1.0.0
2020-04-24T06:19:06.9816058Z Author       : Microsoft
---
2020-04-24T06:19:07.9737360Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-24T06:19:07.9743469Z ##[command]git config gc.auto 0
2020-04-24T06:19:07.9747556Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-24T06:19:07.9751330Z ##[command]git config --get-all http.proxy
2020-04-24T06:19:07.9758497Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69406/merge:refs/remotes/pull/69406/merge
---
2020-04-24T06:21:22.6253776Z  ---> f7353ccad5b1
2020-04-24T06:21:22.6254090Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-24T06:21:22.6256862Z  ---> Using cache
2020-04-24T06:21:22.6257285Z  ---> ed38efbaa060
2020-04-24T06:21:22.6258433Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-24T06:21:22.6260780Z  ---> c5008ef7ae8e
2020-04-24T06:21:22.6286936Z Successfully built c5008ef7ae8e
2020-04-24T06:21:22.6327280Z Successfully tagged rust-ci:latest
2020-04-24T06:21:22.6587615Z Built container sha256:c5008ef7ae8e94d7ef502e3cef26e61208e14ebdb36913f3a8bb86291bd6430b
2020-04-24T06:21:22.6587615Z Built container sha256:c5008ef7ae8e94d7ef502e3cef26e61208e14ebdb36913f3a8bb86291bd6430b
2020-04-24T06:21:22.6598682Z Looks like docker image is the same as before, not uploading
2020-04-24T06:21:29.5141116Z [CI_JOB_NAME=mingw-check]
2020-04-24T06:21:29.5436697Z [CI_JOB_NAME=mingw-check]
2020-04-24T06:21:29.5464938Z == clock drift check ==
2020-04-24T06:21:29.5474398Z   local time: Fri Apr 24 06:21:29 UTC 2020
2020-04-24T06:21:29.8368780Z   network time: Fri, 24 Apr 2020 06:21:29 GMT
2020-04-24T06:21:29.8386956Z Starting sccache server...
2020-04-24T06:21:29.9460680Z configure: processing command line
2020-04-24T06:21:29.9461424Z configure: 
2020-04-24T06:21:29.9462789Z configure: rust.parallel-compiler := True
---
2020-04-24T06:21:47.5575926Z 
2020-04-24T06:21:47.5576857Z ######################################################################## 100.0%
2020-04-24T06:21:48.1671620Z extracting /checkout/obj/build/cache/2020-01-31/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz
2020-04-24T06:21:48.2810034Z     Updating crates.io index
2020-04-24T06:21:56.7757839Z     Updating git repository `https://github.com/jackh726/chalk.git`
---
2020-04-24T06:23:47.7868331Z    Compiling rustc_driver v0.0.0 (/checkout/src/librustc_driver)
2020-04-24T06:23:47.8807115Z     Checking once_cell v1.3.1
2020-04-24T06:23:47.9777042Z     Checking rustc_error_codes v0.0.0 (/checkout/src/librustc_error_codes)
2020-04-24T06:23:48.0238494Z     Checking crossbeam-utils v0.6.6
2020-04-24T06:23:48.2634635Z     Checking chalk-macros v0.1.1 (https://github.com/jackh726/chalk.git?rev=51b3e22c65a32c63f6bcaa3ceaeeba3d13b3996b#51b3e22c)
2020-04-24T06:23:48.3788620Z    Compiling semver v0.9.0
2020-04-24T06:23:48.3863439Z    Compiling crossbeam-utils v0.7.2
2020-04-24T06:23:48.6475999Z    Compiling memoffset v0.5.4
2020-04-24T06:23:48.9046171Z    Compiling crossbeam-epoch v0.8.2
---
2020-04-24T06:23:51.9316018Z    Compiling backtrace-sys v0.1.36
2020-04-24T06:23:52.0365043Z    Compiling rustc_llvm v0.0.0 (/checkout/src/librustc_llvm)
2020-04-24T06:23:53.0119311Z     Checking petgraph v0.4.13
2020-04-24T06:23:54.5043444Z     Checking crossbeam-queue v0.1.2
2020-04-24T06:23:54.6252959Z     Checking chalk-engine v0.9.0 (https://github.com/jackh726/chalk.git?rev=51b3e22c65a32c63f6bcaa3ceaeeba3d13b3996b#51b3e22c)
2020-04-24T06:24:02.4457273Z     Checking num_cpus v1.13.0
2020-04-24T06:24:02.5707920Z     Checking memmap v0.7.0
2020-04-24T06:24:02.6825800Z     Checking parking_lot_core v0.7.2
2020-04-24T06:24:02.8872775Z     Checking jobserver v0.1.21
---
2020-04-24T06:24:37.8621165Z     Checking rustc-rayon v0.3.0
2020-04-24T06:24:39.8344528Z     Checking rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
2020-04-24T06:24:41.1684615Z     Checking arena v0.0.0 (/checkout/src/libarena)
2020-04-24T06:24:47.5861127Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2020-04-24T06:24:47.5861965Z    Compiling chalk-derive v0.1.0 (https://github.com/jackh726/chalk.git?rev=51b3e22c65a32c63f6bcaa3ceaeeba3d13b3996b#51b3e22c)
2020-04-24T06:24:55.1213749Z     Checking chalk-ir v0.1.0 (https://github.com/jackh726/chalk.git?rev=51b3e22c65a32c63f6bcaa3ceaeeba3d13b3996b#51b3e22c)
2020-04-24T06:25:03.5951294Z     Checking rustc_span v0.0.0 (/checkout/src/librustc_span)
2020-04-24T06:25:05.2770170Z     Checking chalk-rust-ir v0.1.0 (https://github.com/jackh726/chalk.git?rev=51b3e22c65a32c63f6bcaa3ceaeeba3d13b3996b#51b3e22c)
2020-04-24T06:25:10.5997101Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-24T06:25:11.1067777Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-24T06:25:11.1067777Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-24T06:25:11.3319270Z     Checking chalk-solve v0.1.0 (https://github.com/jackh726/chalk.git?rev=51b3e22c65a32c63f6bcaa3ceaeeba3d13b3996b#51b3e22c)
2020-04-24T06:25:12.0662331Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-24T06:25:15.0442601Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-24T06:25:15.0442601Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-24T06:25:16.5759439Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-24T06:25:16.8374905Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-24T06:25:17.6545368Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-24T06:25:17.8464234Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-04-24T06:25:20.0117229Z     Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-04-24T06:25:21.0852645Z     Checking rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-04-24T06:25:21.0852645Z     Checking rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-04-24T06:25:22.9285714Z     Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
2020-04-24T06:53:35.3055857Z error: could not compile `rustc_middle`.
2020-04-24T06:53:35.3096726Z 
2020-04-24T06:53:35.3096921Z Caused by:
2020-04-24T06:53:35.3161458Z   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_middle --edition=2018 src/librustc_middle/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata -C opt-level=3 -C debuginfo=0 -C metadata=0560495ca092fcf7 -C extra-filename=-0560495ca092fcf7 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-25f7ad37d7b44bfd.rmeta --extern backtrace=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbacktrace-cae46d33973acddd.rmeta --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-7941118bbdbac8c6.rmeta --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-578f4e978c4fe098.rmeta --extern chalk_ir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libchalk_ir-e693d9fab60d1664.rmeta --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-02f0f6db374c810f.rmeta --extern measureme=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libmeasureme-d8f34bcd747ef233.rmeta --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-020009c4018cd54c.rmeta --extern rustc_rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon-36fc0d8754f78b68.rmeta --extern rustc_rayon_core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon_core-0a1964920475305f.rmeta --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-2cf06908d85596c9.rmeta --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-cd2699af5c1fd0fc.rmeta --extern rustc_attr=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_attr-69bb78424dfc5d26.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-245103f2a7fa0261.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-a1b26e3100b846d7.rmeta --extern rustc_feature=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_feature-988e44cb75b2bc1a.rmeta --extern rustc_hir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir-9516629ac9c75078.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-c45c5577f3053db3.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps/librustc_macros-85fe15c29322ad7b.so --extern rustc_query_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_query_system-fa43a766fd323608.rmeta --extern rustc_session=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-bb1625276b5a72f9.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-8fed64c3a54a0885.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-e6509720009584f1.rmeta --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-8622965ebca5ff77.rmeta --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-a3a521ddba18136c.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-850965e3ade87054.rmeta --cfg=bootstrap -Zmacro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Wrust_2018_idioms -Wunused_lifetimes -Dwarnings -Zunstable-options '-Wrustc::internal' -Cprefer-dynamic --cfg=parallel_compiler -Zbinary-dep-depinfo -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-978fa7533b4cfd77/out` (signal: 9, SIGKILL: kill)
2020-04-24T06:53:35.3206621Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-24T06:53:35.3213163Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-04-24T06:53:35.3213463Z Build completed unsuccessfully in 0:32:05
2020-04-24T06:53:35.7158372Z == clock drift check ==
2020-04-24T06:53:35.7158372Z == clock drift check ==
2020-04-24T06:53:35.7361703Z   local time: Fri Apr 24 06:53:35 UTC 2020
2020-04-24T06:53:36.1290498Z   network time: Fri, 24 Apr 2020 06:53:36 GMT
2020-04-24T06:53:37.4963328Z 
2020-04-24T06:53:37.4963328Z 
2020-04-24T06:53:37.7230205Z ##[error]Bash exited with code '1'.
2020-04-24T06:53:37.7786419Z ##[section]Finishing: Run build
2020-04-24T06:53:37.8286270Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-04-24T06:53:37.8712839Z Task         : Get sources
2020-04-24T06:53:37.8713136Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-24T06:53:37.8718293Z Version      : 1.0.0
2020-04-24T06:53:37.8718520Z Author       : Microsoft
2020-04-24T06:53:37.8718520Z Author       : Microsoft
2020-04-24T06:53:37.8721171Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-24T06:53:37.8726899Z ==============================================================================
2020-04-24T06:53:38.5990300Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-24T06:53:38.6040033Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-04-24T06:53:38.6251502Z Cleaning up task key
2020-04-24T06:53:38.6252565Z Start cleaning up orphan processes.
2020-04-24T06:53:38.6807252Z Terminate orphan process: pid (4245) (python)
2020-04-24T06:53:38.7893820Z ##[section]Finishing: Finalize Job
