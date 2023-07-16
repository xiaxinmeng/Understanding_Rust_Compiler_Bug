plain
2020-04-24T01:32:54.8224075Z ========================== Starting Command Output ===========================
2020-04-24T01:32:54.8226580Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/f600a56a-230e-4989-9c69-fdd96808d8ed.sh
2020-04-24T01:32:54.8226819Z 
2020-04-24T01:32:54.8230622Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-24T01:32:54.8248471Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-04-24T01:32:54.8251278Z Task         : Get sources
2020-04-24T01:32:54.8251531Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-24T01:32:54.8251798Z Version      : 1.0.0
2020-04-24T01:32:54.8251967Z Author       : Microsoft
---
2020-04-24T01:32:55.8177040Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-24T01:32:55.8182731Z ##[command]git config gc.auto 0
2020-04-24T01:32:55.8186147Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-24T01:32:55.8189327Z ##[command]git config --get-all http.proxy
2020-04-24T01:32:55.8195287Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69406/merge:refs/remotes/pull/69406/merge
---
2020-04-24T01:36:49.8167643Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-24T01:36:49.8852278Z  ---> Running in 8e34839b66ab
2020-04-24T01:36:50.7534402Z Removing intermediate container 8e34839b66ab
2020-04-24T01:36:50.7536108Z  ---> 86c4aa69d68e
2020-04-24T01:36:50.7538448Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-24T01:36:51.6962776Z Removing intermediate container b3cdda3adacd
2020-04-24T01:36:51.6963687Z  ---> 0b5a804aaa79
2020-04-24T01:36:51.7005235Z Successfully built 0b5a804aaa79
2020-04-24T01:36:51.7185794Z Successfully tagged rust-ci:latest
2020-04-24T01:36:51.7185794Z Successfully tagged rust-ci:latest
2020-04-24T01:36:51.7474192Z Built container sha256:0b5a804aaa79f0afeb8289d272d64a65aaaff682a10c2bd33a089b297a3e8851
2020-04-24T01:36:51.7492066Z Uploading finished image to https://rust-lang-ci-sccache2.s3.amazonaws.com/docker/838e28e8b1831563a9751ece08254430dfb8f50fa19290a8548616acc385ac53e8dd4547696ad1d81b393189f24a20c8a290afeb44f50533ab560c1c356d5eb9
2020-04-24T01:37:57.3220886Z upload failed: - to s3://rust-lang-ci-sccache2/docker/838e28e8b1831563a9751ece08254430dfb8f50fa19290a8548616acc385ac53e8dd4547696ad1d81b393189f24a20c8a290afeb44f50533ab560c1c356d5eb9 An error occurred (InvalidAccessKeyId) when calling the CreateMultipartUpload operation: The AWS Access Key Id you provided does not exist in our records.
2020-04-24T01:37:57.7894205Z [CI_JOB_NAME=mingw-check]
2020-04-24T01:37:57.7926437Z == clock drift check ==
2020-04-24T01:37:57.7926437Z == clock drift check ==
2020-04-24T01:37:57.7934505Z   local time: Fri Apr 24 01:37:57 UTC 2020
2020-04-24T01:37:57.9496106Z   network time: Fri, 24 Apr 2020 01:37:57 GMT
2020-04-24T01:37:57.9525803Z Starting sccache server...
2020-04-24T01:37:58.0654225Z configure: processing command line
2020-04-24T01:37:58.0655906Z configure: 
2020-04-24T01:37:58.0658625Z configure: rust.parallel-compiler := True
---
2020-04-24T01:40:39.7682467Z    Compiling rustc_driver v0.0.0 (/checkout/src/librustc_driver)
2020-04-24T01:40:39.9086055Z     Checking once_cell v1.3.1
2020-04-24T01:40:40.4526787Z     Checking rustc_error_codes v0.0.0 (/checkout/src/librustc_error_codes)
2020-04-24T01:40:40.4532327Z     Checking crossbeam-utils v0.6.6
2020-04-24T01:40:40.4533280Z     Checking chalk-macros v0.1.1 (https://github.com/rust-lang/chalk.git?rev=2c94b0e86dbc272b6a57bf2e0f5be23072180949#2c94b0e8)
2020-04-24T01:40:40.4534669Z    Compiling semver v0.9.0
2020-04-24T01:40:40.4535507Z    Compiling crossbeam-utils v0.7.2
2020-04-24T01:40:40.7187314Z    Compiling memoffset v0.5.4
2020-04-24T01:40:41.0079295Z    Compiling crossbeam-epoch v0.8.2
---
2020-04-24T01:40:44.0337129Z     Checking unicode-security v0.0.2
2020-04-24T01:40:44.2490546Z     Checking petgraph v0.4.13
2020-04-24T01:40:44.4151804Z    Compiling backtrace-sys v0.1.36
2020-04-24T01:40:45.6161796Z    Compiling rustc_llvm v0.0.0 (/checkout/src/librustc_llvm)
2020-04-24T01:40:46.7839500Z     Checking chalk-engine v0.9.0 (https://github.com/rust-lang/chalk.git?rev=2c94b0e86dbc272b6a57bf2e0f5be23072180949#2c94b0e8)
2020-04-24T01:40:48.1233426Z    Compiling rustc_version v0.2.3
2020-04-24T01:40:56.3507938Z     Checking num_cpus v1.13.0
2020-04-24T01:40:56.4848868Z     Checking memmap v0.7.0
2020-04-24T01:40:56.6072527Z     Checking parking_lot_core v0.7.2
---
2020-04-24T01:41:35.0583273Z     Checking rustc-rayon v0.3.0
2020-04-24T01:41:37.2767835Z     Checking rustc_data_structures v0.0.0 (/checkout/src/librustc_data_structures)
2020-04-24T01:41:38.6906775Z     Checking arena v0.0.0 (/checkout/src/libarena)
2020-04-24T01:41:46.1091019Z    Compiling rustc_macros v0.1.0 (/checkout/src/librustc_macros)
2020-04-24T01:41:46.1091978Z    Compiling chalk-derive v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c94b0e86dbc272b6a57bf2e0f5be23072180949#2c94b0e8)
2020-04-24T01:41:54.3926921Z     Checking chalk-ir v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c94b0e86dbc272b6a57bf2e0f5be23072180949#2c94b0e8)
2020-04-24T01:42:07.7660090Z     Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
2020-04-24T01:42:08.2199901Z     Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
2020-04-24T01:42:08.6144221Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-24T01:42:08.6144221Z     Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
2020-04-24T01:42:08.8013301Z     Checking chalk-rust-ir v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c94b0e86dbc272b6a57bf2e0f5be23072180949#2c94b0e8)
2020-04-24T01:42:09.5535513Z     Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
2020-04-24T01:42:10.2999470Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-24T01:42:10.2999470Z     Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
2020-04-24T01:42:11.7046604Z     Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
2020-04-24T01:42:12.5365922Z     Checking chalk-solve v0.1.0 (https://github.com/rust-lang/chalk.git?rev=2c94b0e86dbc272b6a57bf2e0f5be23072180949#2c94b0e8)
2020-04-24T01:42:13.7664647Z     Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
2020-04-24T01:42:15.7234507Z     Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
2020-04-24T01:42:15.8522573Z     Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
2020-04-24T01:42:16.9703671Z     Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-04-24T01:42:16.9703671Z     Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
2020-04-24T01:42:17.7086777Z     Checking rustc_expand v0.0.0 (/checkout/src/librustc_expand)
2020-04-24T01:42:19.2645979Z     Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
2020-04-24T02:10:23.1879402Z error: could not compile `rustc_middle`.
2020-04-24T02:10:23.1917695Z 
2020-04-24T02:10:23.1917898Z Caused by:
2020-04-24T02:10:23.1968753Z   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc_middle --edition=2018 src/librustc_middle/lib.rs --error-format=json --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata -C opt-level=3 -C debuginfo=0 -C metadata=7cae9df305b47264 -C extra-filename=-7cae9df305b47264 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-25f7ad37d7b44bfd.rmeta --extern backtrace=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbacktrace-cae46d33973acddd.rmeta --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-7941118bbdbac8c6.rmeta --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-578f4e978c4fe098.rmeta --extern chalk_ir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libchalk_ir-74c798309f5c31a4.rmeta --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-02f0f6db374c810f.rmeta --extern measureme=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libmeasureme-d8f34bcd747ef233.rmeta --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-020009c4018cd54c.rmeta --extern rustc_rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon-36fc0d8754f78b68.rmeta --extern rustc_rayon_core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon_core-0a1964920475305f.rmeta --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-2cf06908d85596c9.rmeta --extern rustc_ast=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_ast-cd2699af5c1fd0fc.rmeta --extern rustc_attr=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_attr-69bb78424dfc5d26.rmeta --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-245103f2a7fa0261.rmeta --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-a1b26e3100b846d7.rmeta --extern rustc_feature=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_feature-988e44cb75b2bc1a.rmeta --extern rustc_hir=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_hir-9516629ac9c75078.rmeta --extern rustc_index=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_index-c45c5577f3053db3.rmeta --extern rustc_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps/librustc_macros-85fe15c29322ad7b.so --extern rustc_query_system=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_query_system-fa43a766fd323608.rmeta --extern rustc_session=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_session-bb1625276b5a72f9.rmeta --extern rustc_span=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_span-8fed64c3a54a0885.rmeta --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-e6509720009584f1.rmeta --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-8622965ebca5ff77.rmeta --extern rustc_serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-a3a521ddba18136c.rmeta --extern smallvec=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsmallvec-850965e3ade87054.rmeta --cfg=bootstrap -Zmacro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Wrust_2018_idioms -Wunused_lifetimes -Dwarnings -Zunstable-options '-Wrustc::internal' -Cprefer-dynamic --cfg=parallel_compiler -Zbinary-dep-depinfo -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-978fa7533b4cfd77/out` (signal: 9, SIGKILL: kill)
2020-04-24T02:10:23.2013688Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-24T02:10:23.2018887Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-04-24T02:10:23.2019334Z Build completed unsuccessfully in 0:32:24
2020-04-24T02:10:23.5359126Z == clock drift check ==
2020-04-24T02:10:23.5359126Z == clock drift check ==
2020-04-24T02:10:23.5366533Z   local time: Fri Apr 24 02:10:23 UTC 2020
2020-04-24T02:10:23.7362422Z   network time: Fri, 24 Apr 2020 02:10:23 GMT
2020-04-24T02:10:25.2357008Z 
2020-04-24T02:10:25.2357008Z 
2020-04-24T02:10:25.4820977Z ##[error]Bash exited with code '1'.
2020-04-24T02:10:25.5221212Z ##[section]Finishing: Run build
2020-04-24T02:10:25.5619663Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-04-24T02:10:25.5840942Z Task         : Get sources
2020-04-24T02:10:25.5841353Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-24T02:10:25.5841637Z Version      : 1.0.0
2020-04-24T02:10:25.5841842Z Author       : Microsoft
2020-04-24T02:10:25.5841842Z Author       : Microsoft
2020-04-24T02:10:25.5845318Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-24T02:10:25.5845681Z ==============================================================================
2020-04-24T02:10:26.1262036Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-24T02:10:26.1315027Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69406/merge to s
2020-04-24T02:10:26.1658758Z Cleaning up task key
2020-04-24T02:10:26.1660473Z Start cleaning up orphan processes.
2020-04-24T02:10:26.2193435Z Terminate orphan process: pid (3716) (python)
2020-04-24T02:10:26.3244356Z ##[section]Finishing: Finalize Job
