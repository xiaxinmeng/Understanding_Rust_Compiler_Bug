plain
2019-08-03T13:38:20.2216836Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-03T13:38:20.2424879Z ##[command]git config gc.auto 0
2019-08-03T13:38:20.2498837Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-03T13:38:20.2549358Z ##[command]git config --get-all http.proxy
2019-08-03T13:38:20.2682313Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63162/merge:refs/remotes/pull/63162/merge
---
2019-08-03T13:38:53.0318589Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-03T13:38:53.0318781Z 
2019-08-03T13:38:53.0319139Z   git checkout -b <new-branch-name>
2019-08-03T13:38:53.0319290Z 
2019-08-03T13:38:53.0319434Z HEAD is now at 5ef76725d Merge 97a1952a183c512edd4f6fd996c712ad945daa63 into 8e917f48382c6afaf50568263b89d35fba5d98e4
2019-08-03T13:38:53.0478428Z ##[section]Finishing: Checkout
2019-08-03T13:38:53.0485129Z ##[section]Starting: Decide whether to run this job
2019-08-03T13:38:53.0489509Z Task         : Bash
2019-08-03T13:38:53.0489555Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-08-03T13:38:53.0489794Z Version      : 3.151.3
2019-08-03T13:38:53.0489857Z Author       : Microsoft Corporation
2019-08-03T13:38:53.0489857Z Author       : Microsoft Corporation
2019-08-03T13:38:53.0489906Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-08-03T13:38:53.0489956Z ==============================================================================
2019-08-03T13:38:53.1905711Z Generating script.
2019-08-03T13:38:53.1937761Z ========================== Starting Command Output ===========================
2019-08-03T13:38:53.1958404Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/81200c12-187f-4701-bc65-9a53c6cd4fd1.sh
2019-08-03T13:38:53.2294426Z Executing the job since submodules are updated
2019-08-03T13:38:53.2391024Z ##[section]Finishing: Decide whether to run this job
2019-08-03T13:38:53.2400486Z ==============================================================================
2019-08-03T13:38:53.2400559Z Task         : Bash
2019-08-03T13:38:53.2400604Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-08-03T13:38:53.2400858Z Version      : 3.151.3
---
2019-08-03T13:42:04.0913762Z Removing intermediate container e0a2b4831766
2019-08-03T13:42:04.0915120Z  ---> 52af5539b8da
2019-08-03T13:42:04.0953400Z Successfully built 52af5539b8da
2019-08-03T13:42:04.1605446Z Successfully tagged rust-ci:latest
2019-08-03T13:42:04.2199950Z Built container sha256:52af5539b8da37849a199bde4c653aaf4df42b5365665c5ca61ce52d8fc3b11a
2019-08-03T13:42:04.2214078Z Uploading finished image to https://rust-lang-ci-sccache2.s3.amazonaws.com/docker/0a94e800dbdbef90fbb9cf381d041a9386910340a5cdd758b34797988dea18bff1c9fc0562ea46a76a5a5ba9a8bbf4046ce3ddf0fb0b70448e54334a8af215fe
2019-08-03T13:42:44.6823144Z upload failed: - to s3://rust-lang-ci-sccache2/docker/0a94e800dbdbef90fbb9cf381d041a9386910340a5cdd758b34797988dea18bff1c9fc0562ea46a76a5a5ba9a8bbf4046ce3ddf0fb0b70448e54334a8af215fe Unable to locate credentials
2019-08-03T13:42:45.6686489Z [CI_JOB_NAME=LinuxTools]
2019-08-03T13:42:45.6752831Z Starting sccache server...
2019-08-03T13:42:45.7681731Z configure: processing command line
2019-08-03T13:42:45.7682297Z configure: 
---
2019-08-03T15:28:20.5240248Z    Compiling rustbook v0.1.0 (/checkout/src/tools/rustbook)
2019-08-03T15:28:24.7179663Z     Finished release [optimized] target(s) in 8m 18s
2019-08-03T15:30:03.2031543Z Error: there are broken links
2019-08-03T15:30:03.2105202Z  Caused By: "https://github.com/rust-lang/compiler-team/blob/master/experts/MAP.md" returned 404 Not Found
2019-08-03T15:30:03.2106100Z  Caused By: "***/tree/master/src/test/run-pass/" returned 404 Not Found
2019-08-03T15:30:03.2106729Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/rustc/mir/enum.Place.html" returned 404 Not Found
2019-08-03T15:30:03.2107332Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/rustc/mir/enum.Place.html" returned 404 Not Found
2019-08-03T15:30:03.2107649Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/rustc/mir/enum.Place.html" returned 404 Not Found
2019-08-03T15:30:03.2107933Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/rustc/mir/enum.Place.html" returned 404 Not Found
---
2019-08-03T15:47:07.7101796Z      |
2019-08-03T15:47:07.7102340Z 1269 | #[bench]
2019-08-03T15:47:07.7102757Z      |   ^^^^^
2019-08-03T15:47:07.7103121Z      |
2019-08-03T15:47:07.7103716Z      = note: for more information, see ***/issues/50297
2019-08-03T15:47:07.7104693Z 
2019-08-03T15:47:07.7109699Z error[E0658]: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
2019-08-03T15:47:07.7111370Z     --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_data_structures-542.0.0/bit_set.rs:1284:3
2019-08-03T15:47:07.7112122Z      |
2019-08-03T15:47:07.7112122Z      |
2019-08-03T15:47:07.7112688Z 1284 | #[bench]
2019-08-03T15:47:07.7113218Z      |   ^^^^^
2019-08-03T15:47:07.7113601Z      |
2019-08-03T15:47:07.7114164Z      = note: for more information, see ***/issues/50297
2019-08-03T15:47:07.7115302Z 
2019-08-03T15:47:07.7115727Z error[E0658]: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
2019-08-03T15:47:07.7116179Z     --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_data_structures-542.0.0/bit_set.rs:1302:3
2019-08-03T15:47:07.7116516Z      |
2019-08-03T15:47:07.7116516Z      |
2019-08-03T15:47:07.7117054Z 1302 | #[bench]
2019-08-03T15:47:07.7117465Z      |   ^^^^^
2019-08-03T15:47:07.7117823Z      |
2019-08-03T15:47:07.7118276Z      = note: for more information, see ***/issues/50297
2019-08-03T15:47:07.7119314Z 
2019-08-03T15:47:07.7119722Z error[E0658]: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
2019-08-03T15:47:07.7120290Z     --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_data_structures-542.0.0/bit_set.rs:1320:3
2019-08-03T15:47:07.7120685Z      |
2019-08-03T15:47:07.7120685Z      |
2019-08-03T15:47:07.7121061Z 1320 | #[bench]
2019-08-03T15:47:07.7122415Z      |   ^^^^^
2019-08-03T15:47:07.7122838Z      |
2019-08-03T15:47:07.7123369Z      = note: for more information, see ***/issues/50297
2019-08-03T15:47:07.7124053Z 
2019-08-03T15:47:07.7124526Z error[E0658]: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
2019-08-03T15:47:07.7125607Z     --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_data_structures-542.0.0/bit_set.rs:1335:3
2019-08-03T15:47:07.7126007Z      |
2019-08-03T15:47:07.7126007Z      |
2019-08-03T15:47:07.7127137Z 1335 | #[bench]
2019-08-03T15:47:07.7127400Z      |   ^^^^^
2019-08-03T15:47:07.7127599Z      |
2019-08-03T15:47:07.7128092Z      = note: for more information, see ***/issues/50297
2019-08-03T15:47:07.7128601Z 
2019-08-03T15:47:09.6540005Z error: aborting due to 5 previous errors
2019-08-03T15:47:09.6540106Z 
2019-08-03T15:47:09.6540477Z For more information about this error, try `rustc --explain E0658`.
---
2019-08-03T15:47:33.4422559Z      |
2019-08-03T15:47:33.4423151Z 1269 | #[bench]
2019-08-03T15:47:33.4423688Z      |   ^^^^^
2019-08-03T15:47:33.4424236Z      |
2019-08-03T15:47:33.4424955Z      = note: for more information, see ***/issues/50297
2019-08-03T15:47:33.4426342Z 
2019-08-03T15:47:33.4435947Z error[E0658]: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
2019-08-03T15:47:33.4437308Z     --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_data_structures-542.0.0/bit_set.rs:1284:3
2019-08-03T15:47:33.4437890Z      |
2019-08-03T15:47:33.4437890Z      |
2019-08-03T15:47:33.4438292Z 1284 | #[bench]
2019-08-03T15:47:33.4438697Z      |   ^^^^^
2019-08-03T15:47:33.4439030Z      |
2019-08-03T15:47:33.4439527Z      = note: for more information, see ***/issues/50297
2019-08-03T15:47:33.4440148Z 
2019-08-03T15:47:33.4452566Z error[E0658]: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
2019-08-03T15:47:33.4453213Z     --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_data_structures-542.0.0/bit_set.rs:1302:3
2019-08-03T15:47:33.4453885Z      |
2019-08-03T15:47:33.4453885Z      |
2019-08-03T15:47:33.4454370Z 1302 | #[bench]
2019-08-03T15:47:33.4454782Z      |   ^^^^^
2019-08-03T15:47:33.4455644Z      |
2019-08-03T15:47:33.4456194Z      = note: for more information, see ***/issues/50297
2019-08-03T15:47:33.4456886Z 
2019-08-03T15:47:33.4465901Z error[E0658]: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
2019-08-03T15:47:33.4466546Z     --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_data_structures-542.0.0/bit_set.rs:1320:3
2019-08-03T15:47:33.4466933Z      |
2019-08-03T15:47:33.4466933Z      |
2019-08-03T15:47:33.4467418Z 1320 | #[bench]
2019-08-03T15:47:33.4467823Z      |   ^^^^^
2019-08-03T15:47:33.4468182Z      |
2019-08-03T15:47:33.4468721Z      = note: for more information, see ***/issues/50297
2019-08-03T15:47:33.4469386Z 
2019-08-03T15:47:33.4477437Z error[E0658]: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
2019-08-03T15:47:33.4478065Z     --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_data_structures-542.0.0/bit_set.rs:1335:3
2019-08-03T15:47:33.4478462Z      |
2019-08-03T15:47:33.4478462Z      |
2019-08-03T15:47:33.4479108Z 1335 | #[bench]
2019-08-03T15:47:33.4479552Z      |   ^^^^^
2019-08-03T15:47:33.4479924Z      |
2019-08-03T15:47:33.4480455Z      = note: for more information, see ***/issues/50297
2019-08-03T15:47:33.4481201Z 
2019-08-03T15:47:35.3668578Z error: aborting due to 5 previous errors
2019-08-03T15:47:35.3668846Z 
2019-08-03T15:47:35.3669162Z For more information about this error, try `rustc --explain E0658`.
---
2019-08-03T15:47:49.1958869Z    Compiling vergen v3.0.4
2019-08-03T15:47:54.8486917Z    Compiling miri v0.1.0 (/checkout/src/tools/miri)
2019-08-03T15:49:49.2867342Z     Finished release [optimized] target(s) in 2m 10s
2019-08-03T15:49:49.5230015Z     Finished release [optimized] target(s) in 0.22s
2019-08-03T15:49:49.5358500Z      Running `build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo-miri miri setup`
2019-08-03T15:49:49.5393412Z Installing xargo: `cargo install xargo -f`
---
2019-08-03T15:50:42.9340377Z    Compiling num-traits v0.1.43
2019-08-03T15:50:42.9972338Z    Compiling serde_json v0.8.6
2019-08-03T15:50:46.2525308Z    Compiling error-chain v0.7.2
2019-08-03T15:51:03.0359351Z     Finished release [optimized] target(s) in 1m 13s
2019-08-03T15:51:03.0458116Z   Installing /cargo/bin/xargo
2019-08-03T15:51:03.0458354Z    Installed package `xargo v0.3.14` (executable `xargo`)
2019-08-03T15:51:03.0466615Z warning: be sure to add `/cargo/bin` to your PATH to be able to run the installed binaries
2019-08-03T15:51:03.0955459Z error: couldn't execute `"cargo" "build" "--release" "--manifest-path" "/tmp/xargo.cMq7J4LzNbva/Cargo.toml" "--target" "x86_64-unknown-linux-gnu" "-p" "std"`
2019-08-03T15:51:03.0955617Z caused by: No such file or directory (os error 2)
2019-08-03T15:51:03.0955724Z stack backtrace:
2019-08-03T15:51:03.0955771Z    0: error_chain::make_backtrace::h933f4fdac8566a18 (0x555dd78c306c)
2019-08-03T15:51:03.0955831Z    1: <core::result::Result<T,E> as xargo::errors::ResultExt<T,E>>::chain_err::h4ba04b62a8d7b6aa (0x555dd787049c)
2019-08-03T15:51:03.0955922Z    2: <std::process::Command as xargo::extensions::CommandExt>::run_and_get_status::hfd58f9347f11ed7d (0x555dd788d60f)
2019-08-03T15:51:03.0955981Z    3: <std::process::Command as xargo::extensions::CommandExt>::run::h0fc4df8a5f272a2d (0x555dd788d2ae)
2019-08-03T15:51:03.0956034Z    4: xargo::sysroot::build::h21b2087c5566ecf0 (0x555dd787de09)
2019-08-03T15:51:03.0956104Z    5: xargo::sysroot::update::h05a036fe48cf8048 (0x555dd788042e)
2019-08-03T15:51:03.0956154Z    6: xargo::main::h65b81eeeee0998cd (0x555dd786c29b)
2019-08-03T15:51:03.0956211Z    7: std::rt::lang_start::{{closure}}::h37650177518a390e (0x555dd7875993)
2019-08-03T15:51:03.0956279Z    8: std::panicking::try::do_call::h13cd310076c0705a (0x555dd78ee512)
2019-08-03T15:51:03.0956326Z    9: __rust_maybe_catch_panic (0x555dd78f817a)
2019-08-03T15:51:03.0956376Z   10: std::rt::lang_start_internal::h7c267df922bfe0cb (0x555dd78ef200)
2019-08-03T15:51:03.0956439Z   11: main (0x555dd786d362)
2019-08-03T15:51:03.0956484Z   12: __libc_start_main (0x7f094d752830)
2019-08-03T15:51:03.0956528Z   13: _start (0x555dd7858b79)
2019-08-03T15:51:03.0956596Z   14: <unknown> (0x0)
2019-08-03T15:51:03.0956639Z fatal error: Failed to run xargo
2019-08-03T15:51:03.0962007Z 
2019-08-03T15:51:03.0962007Z 
2019-08-03T15:51:03.0962784Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "run" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/miri/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--bin" "cargo-miri" "--" "miri" "setup"
2019-08-03T15:51:03.0963293Z 
2019-08-03T15:51:03.0963506Z 
2019-08-03T15:51:03.0974264Z 
2019-08-03T15:51:03.0974418Z 2 command(s) did not execute successfully:
2019-08-03T15:51:03.0974418Z 2 command(s) did not execute successfully:
2019-08-03T15:51:03.0974478Z 
2019-08-03T15:51:03.0974965Z   - "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rustbook" "linkcheck" "/checkout/src/doc/rustc-guide"
2019-08-03T15:51:03.0975168Z 
2019-08-03T15:51:03.0976087Z   - "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "run" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/miri/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--bin" "cargo-miri" "--" "miri" "setup"
2019-08-03T15:51:03.0988284Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --no-fail-fast src/doc/book src/doc/nomicon src/doc/reference src/doc/rust-by-example src/doc/embedded-book src/doc/edition-guide src/doc/rustc-guide src/tools/clippy src/tools/rls src/tools/rustfmt src/tools/miri
2019-08-03T15:51:03.0988581Z Build completed unsuccessfully in 2:06:05
2019-08-03T15:51:03.1060755Z {"book":"test-pass","rust-by-example":"test-pass","clippy-driver":"build-fail","nomicon":"test-pass","reference":"test-pass","edition-guide":"test-pass","rls":"build-fail","miri":"test-fail","rustfmt":"build-fail","rustbook":"test-fail","embedded-book":"test-pass"}
2019-08-03T15:51:03.1061128Z Verifying status of book...
---
2019-08-03T15:51:03.1156969Z Verifying status of clippy-driver...
2019-08-03T15:51:03.1167660Z Verifying status of miri...
2019-08-03T15:51:03.1180633Z This PR updated 'src/tools/miri', verifying if status is 'test-pass'...
2019-08-03T15:51:03.1191226Z 
2019-08-03T15:51:03.1191881Z ⚠️ We detected that this PR updated 'miri', but its tests failed.
2019-08-03T15:51:03.1191925Z 
2019-08-03T15:51:03.1192189Z If you do intend to update 'miri', please check the error messages above and
2019-08-03T15:51:03.1192450Z commit another update.
2019-08-03T15:51:03.1192516Z 
2019-08-03T15:51:03.1192844Z If you do NOT intend to update 'miri', please ensure you did not accidentally
2019-08-03T15:51:03.1193633Z change the submodule at 'src/tools/miri'. You may ask your reviewer for the
2019-08-03T15:51:03.1194005Z proper steps.
2019-08-03T15:51:04.1122400Z ##[error]Bash exited with code '3'.
2019-08-03T15:51:04.1181798Z ##[section]Starting: Checkout
2019-08-03T15:51:04.1183733Z ==============================================================================
2019-08-03T15:51:04.1183801Z Task         : Get sources
2019-08-03T15:51:04.1183863Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
