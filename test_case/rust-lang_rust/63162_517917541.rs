plain
2019-08-03T09:15:24.2830272Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-03T09:15:24.3054586Z ##[command]git config gc.auto 0
2019-08-03T09:15:24.3131718Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-03T09:15:24.3195560Z ##[command]git config --get-all http.proxy
2019-08-03T09:15:24.3334068Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63162/merge:refs/remotes/pull/63162/merge
---
2019-08-03T09:15:58.7940998Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-03T09:15:58.7941035Z 
2019-08-03T09:15:58.7941312Z   git checkout -b <new-branch-name>
2019-08-03T09:15:58.7941539Z 
2019-08-03T09:15:58.7941595Z HEAD is now at 77b95dae6 Merge 7f1e46665f0f46520e9510c32ef09881be208044 into d7270712cb446aad0817040bbca73a8d024f67b0
2019-08-03T09:15:58.8101025Z ##[section]Finishing: Checkout
2019-08-03T09:15:58.8108631Z ##[section]Starting: Decide whether to run this job
2019-08-03T09:15:58.8111793Z Task         : Bash
2019-08-03T09:15:58.8111845Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-08-03T09:15:58.8111897Z Version      : 3.151.3
2019-08-03T09:15:58.8111970Z Author       : Microsoft Corporation
2019-08-03T09:15:58.8111970Z Author       : Microsoft Corporation
2019-08-03T09:15:58.8112028Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-08-03T09:15:58.8112089Z ==============================================================================
2019-08-03T09:15:58.9593904Z Generating script.
2019-08-03T09:15:58.9630266Z ========================== Starting Command Output ===========================
2019-08-03T09:15:58.9656568Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/36a5b62a-d6da-45d5-8f91-59fa48b6c39f.sh
2019-08-03T09:15:59.3342761Z Executing the job since submodules are updated
2019-08-03T09:15:59.3442362Z ##[section]Finishing: Decide whether to run this job
2019-08-03T09:15:59.3452574Z ==============================================================================
2019-08-03T09:15:59.3452673Z Task         : Bash
2019-08-03T09:15:59.3452713Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-08-03T09:15:59.3452755Z Version      : 3.151.3
---
2019-08-03T09:19:17.0872049Z  ---> 63446e6c52d6
2019-08-03T09:19:17.0912304Z Successfully built 63446e6c52d6
2019-08-03T09:19:17.1797219Z Successfully tagged rust-ci:latest
2019-08-03T09:19:17.2306680Z Built container sha256:63446e6c52d6ad2b2266c2aa74e6b43ae8afa4a81890d43746101d74fec05abf
2019-08-03T09:19:17.2322735Z Uploading finished image to https://rust-lang-ci-sccache2.s3.amazonaws.com/docker/0a94e800dbdbef90fbb9cf381d041a9386910340a5cdd758b34797988dea18bff1c9fc0562ea46a76a5a5ba9a8bbf4046ce3ddf0fb0b70448e54334a8af215fe
2019-08-03T09:19:57.4489646Z upload failed: - to s3://rust-lang-ci-sccache2/docker/0a94e800dbdbef90fbb9cf381d041a9386910340a5cdd758b34797988dea18bff1c9fc0562ea46a76a5a5ba9a8bbf4046ce3ddf0fb0b70448e54334a8af215fe Unable to locate credentials
2019-08-03T09:19:58.4574938Z [CI_JOB_NAME=LinuxTools]
2019-08-03T09:19:58.4627363Z Starting sccache server...
2019-08-03T09:19:58.5585898Z configure: processing command line
2019-08-03T09:19:58.5586035Z configure: 
---
2019-08-03T11:04:48.7846084Z    Compiling rustbook v0.1.0 (/checkout/src/tools/rustbook)
2019-08-03T11:04:53.0184557Z     Finished release [optimized] target(s) in 8m 13s
2019-08-03T11:06:30.5161441Z Error: there are broken links
2019-08-03T11:06:30.5163377Z  Caused By: "https://github.com/rust-lang/compiler-team/blob/master/experts/MAP.md" returned 404 Not Found
2019-08-03T11:06:30.5164585Z  Caused By: "***/tree/master/src/test/run-pass/" returned 404 Not Found
2019-08-03T11:06:30.5166231Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/rustc/mir/enum.Place.html" returned 404 Not Found
2019-08-03T11:06:30.5166840Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/rustc/mir/enum.Place.html" returned 404 Not Found
2019-08-03T11:06:30.5167498Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/rustc/mir/enum.Place.html" returned 404 Not Found
2019-08-03T11:06:30.5168091Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/rustc/mir/enum.Place.html" returned 404 Not Found
---
2019-08-03T11:23:20.6375160Z      |
2019-08-03T11:23:20.6375518Z 1269 | #[bench]
2019-08-03T11:23:20.6375841Z      |   ^^^^^
2019-08-03T11:23:20.6376113Z      |
2019-08-03T11:23:20.6376626Z      = note: for more information, see ***/issues/50297
2019-08-03T11:23:20.6377063Z 
2019-08-03T11:23:20.6377468Z error[E0658]: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
2019-08-03T11:23:20.6377876Z     --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_data_structures-542.0.0/bit_set.rs:1284:3
2019-08-03T11:23:20.6379039Z      |
2019-08-03T11:23:20.6379039Z      |
2019-08-03T11:23:20.6379404Z 1284 | #[bench]
2019-08-03T11:23:20.6379719Z      |   ^^^^^
2019-08-03T11:23:20.6379986Z      |
2019-08-03T11:23:20.6380435Z      = note: for more information, see ***/issues/50297
2019-08-03T11:23:20.6380861Z 
2019-08-03T11:23:20.6381261Z error[E0658]: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
2019-08-03T11:23:20.6381668Z     --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_data_structures-542.0.0/bit_set.rs:1302:3
2019-08-03T11:23:20.6382017Z      |
2019-08-03T11:23:20.6382017Z      |
2019-08-03T11:23:20.6382522Z 1302 | #[bench]
2019-08-03T11:23:20.6382886Z      |   ^^^^^
2019-08-03T11:23:20.6383156Z      |
2019-08-03T11:23:20.6383594Z      = note: for more information, see ***/issues/50297
2019-08-03T11:23:20.6384029Z 
2019-08-03T11:23:20.6384412Z error[E0658]: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
2019-08-03T11:23:20.6384815Z     --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_data_structures-542.0.0/bit_set.rs:1320:3
2019-08-03T11:23:20.6385129Z      |
2019-08-03T11:23:20.6385129Z      |
2019-08-03T11:23:20.6385448Z 1320 | #[bench]
2019-08-03T11:23:20.6385760Z      |   ^^^^^
2019-08-03T11:23:20.6386044Z      |
2019-08-03T11:23:20.6386448Z      = note: for more information, see ***/issues/50297
2019-08-03T11:23:20.6386878Z 
2019-08-03T11:23:20.6387257Z error[E0658]: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
2019-08-03T11:23:20.6387659Z     --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_data_structures-542.0.0/bit_set.rs:1335:3
2019-08-03T11:23:20.6387970Z      |
2019-08-03T11:23:20.6387970Z      |
2019-08-03T11:23:20.6388611Z 1335 | #[bench]
2019-08-03T11:23:20.6388994Z      |   ^^^^^
2019-08-03T11:23:20.6389292Z      |
2019-08-03T11:23:20.6389710Z      = note: for more information, see ***/issues/50297
2019-08-03T11:23:20.6390147Z 
2019-08-03T11:23:22.4257047Z error: aborting due to 5 previous errors
2019-08-03T11:23:22.4257566Z 
2019-08-03T11:23:22.4258059Z For more information about this error, try `rustc --explain E0658`.
---
2019-08-03T11:23:47.7515033Z      |
2019-08-03T11:23:47.7515541Z 1269 | #[bench]
2019-08-03T11:23:47.7516007Z      |   ^^^^^
2019-08-03T11:23:47.7520837Z      |
2019-08-03T11:23:47.7524904Z      = note: for more information, see ***/issues/50297
2019-08-03T11:23:47.7534114Z 
2019-08-03T11:23:47.7538492Z error[E0658]: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
2019-08-03T11:23:47.7541612Z     --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_data_structures-542.0.0/bit_set.rs:1284:3
2019-08-03T11:23:47.7545067Z      |
2019-08-03T11:23:47.7545067Z      |
2019-08-03T11:23:47.7551662Z 1284 | #[bench]
2019-08-03T11:23:47.7558545Z      |   ^^^^^
2019-08-03T11:23:47.7608073Z      |
2019-08-03T11:23:47.7609003Z      = note: for more information, see ***/issues/50297
2019-08-03T11:23:47.7609820Z 
2019-08-03T11:23:47.7610320Z error[E0658]: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
2019-08-03T11:23:47.7610877Z     --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_data_structures-542.0.0/bit_set.rs:1302:3
2019-08-03T11:23:47.7611325Z      |
2019-08-03T11:23:47.7611325Z      |
2019-08-03T11:23:47.7612092Z 1302 | #[bench]
2019-08-03T11:23:47.7612686Z      |   ^^^^^
2019-08-03T11:23:47.7613111Z      |
2019-08-03T11:23:47.7613671Z      = note: for more information, see ***/issues/50297
2019-08-03T11:23:47.7614602Z 
2019-08-03T11:23:47.7615146Z error[E0658]: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
2019-08-03T11:23:47.7615703Z     --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_data_structures-542.0.0/bit_set.rs:1320:3
2019-08-03T11:23:47.7616115Z      |
2019-08-03T11:23:47.7616115Z      |
2019-08-03T11:23:47.7616593Z 1320 | #[bench]
2019-08-03T11:23:47.7617034Z      |   ^^^^^
2019-08-03T11:23:47.7617438Z      |
2019-08-03T11:23:47.7618002Z      = note: for more information, see ***/issues/50297
2019-08-03T11:23:47.7618724Z 
2019-08-03T11:23:47.7619816Z error[E0658]: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
2019-08-03T11:23:47.7620478Z     --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_data_structures-542.0.0/bit_set.rs:1335:3
2019-08-03T11:23:47.7620995Z      |
2019-08-03T11:23:47.7620995Z      |
2019-08-03T11:23:47.7621447Z 1335 | #[bench]
2019-08-03T11:23:47.7622234Z      |   ^^^^^
2019-08-03T11:23:47.7622721Z      |
2019-08-03T11:23:47.7623385Z      = note: for more information, see ***/issues/50297
2019-08-03T11:23:47.7624116Z 
2019-08-03T11:23:49.5370552Z error: aborting due to 5 previous errors
2019-08-03T11:23:49.5370661Z 
2019-08-03T11:23:49.5371125Z For more information about this error, try `rustc --explain E0658`.
---
2019-08-03T11:24:02.9195252Z    Compiling vergen v3.0.4
2019-08-03T11:24:08.6344243Z    Compiling miri v0.1.0 (/checkout/src/tools/miri)
2019-08-03T11:26:02.9703918Z     Finished release [optimized] target(s) in 2m 10s
2019-08-03T11:26:03.2295376Z     Finished release [optimized] target(s) in 0.24s
2019-08-03T11:26:03.2423602Z      Running `build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo-miri miri setup`
2019-08-03T11:26:03.2460342Z Installing xargo: `cargo install xargo -f`
---
2019-08-03T11:26:56.9855066Z    Compiling num-traits v0.1.43
2019-08-03T11:26:57.0505257Z    Compiling serde_json v0.8.6
2019-08-03T11:26:59.2415514Z    Compiling error-chain v0.7.2
2019-08-03T11:27:16.4209973Z     Finished release [optimized] target(s) in 1m 13s
2019-08-03T11:27:16.4312619Z   Installing /cargo/bin/xargo
2019-08-03T11:27:16.4313020Z    Installed package `xargo v0.3.14` (executable `xargo`)
2019-08-03T11:27:16.4315525Z warning: be sure to add `/cargo/bin` to your PATH to be able to run the installed binaries
2019-08-03T11:27:16.4369449Z thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/libcore/result.rs:1084:5
2019-08-03T11:27:16.4395804Z    0: std::panicking::default_hook::{{closure}}
2019-08-03T11:27:16.4396057Z    1: std::panicking::default_hook
2019-08-03T11:27:16.4396240Z    2: std::panicking::rust_panic_with_hook
2019-08-03T11:27:16.4396383Z    3: std::panicking::continue_panic_fmt
2019-08-03T11:27:16.4396383Z    3: std::panicking::continue_panic_fmt
2019-08-03T11:27:16.4396522Z    4: rust_begin_unwind
2019-08-03T11:27:16.4396680Z    5: core::panicking::panic_fmt
2019-08-03T11:27:16.4397053Z    6: core::result::unwrap_failed
2019-08-03T11:27:16.4397189Z    7: cargo_miri::in_cargo_miri
2019-08-03T11:27:16.4397342Z    8: cargo_miri::main
2019-08-03T11:27:16.4397488Z    9: std::rt::lang_start::{{closure}}
2019-08-03T11:27:16.4397624Z   10: std::panicking::try::do_call
2019-08-03T11:27:16.4397779Z   11: __rust_maybe_catch_panic
2019-08-03T11:27:16.4399849Z   13: main
2019-08-03T11:27:16.4400100Z   14: __libc_start_main
2019-08-03T11:27:16.4400314Z   15: _start
2019-08-03T11:27:16.4400314Z   15: _start
2019-08-03T11:27:16.4400480Z note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
2019-08-03T11:27:16.4400784Z 
2019-08-03T11:27:16.4400784Z 
2019-08-03T11:27:16.4402064Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "run" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/miri/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--bin" "cargo-miri" "--" "miri" "setup"
2019-08-03T11:27:16.4402680Z 
2019-08-03T11:27:16.4402874Z 
2019-08-03T11:27:16.4412554Z 
2019-08-03T11:27:16.4412847Z 2 command(s) did not execute successfully:
2019-08-03T11:27:16.4412847Z 2 command(s) did not execute successfully:
2019-08-03T11:27:16.4412985Z 
2019-08-03T11:27:16.4413655Z   - "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rustbook" "linkcheck" "/checkout/src/doc/rustc-guide"
2019-08-03T11:27:16.4413857Z 
2019-08-03T11:27:16.4414549Z   - "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "run" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/miri/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--bin" "cargo-miri" "--" "miri" "setup"
2019-08-03T11:27:16.4420937Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --no-fail-fast src/doc/book src/doc/nomicon src/doc/reference src/doc/rust-by-example src/doc/embedded-book src/doc/edition-guide src/doc/rustc-guide src/tools/clippy src/tools/rls src/tools/rustfmt src/tools/miri
2019-08-03T11:27:16.4421259Z Build completed unsuccessfully in 2:05:13
2019-08-03T11:27:16.4491224Z {"edition-guide":"test-pass","rustbook":"test-fail","embedded-book":"test-pass","book":"test-pass","clippy-driver":"build-fail","rls":"build-fail","rust-by-example":"test-pass","miri":"test-fail","reference":"test-pass","rustfmt":"build-fail","nomicon":"test-pass"}
2019-08-03T11:27:16.4492630Z Verifying status of book...
---
2019-08-03T11:27:16.4592045Z Verifying status of clippy-driver...
2019-08-03T11:27:16.4603824Z Verifying status of miri...
2019-08-03T11:27:16.4614054Z This PR updated 'src/tools/miri', verifying if status is 'test-pass'...
2019-08-03T11:27:16.4625509Z 
2019-08-03T11:27:16.4639019Z ⚠️ We detected that this PR updated 'miri', but its tests failed.
2019-08-03T11:27:16.4639080Z 
2019-08-03T11:27:16.4639390Z If you do intend to update 'miri', please check the error messages above and
2019-08-03T11:27:16.4639444Z commit another update.
2019-08-03T11:27:16.4639489Z 
2019-08-03T11:27:16.4639754Z If you do NOT intend to update 'miri', please ensure you did not accidentally
2019-08-03T11:27:16.4640015Z change the submodule at 'src/tools/miri'. You may ask your reviewer for the
2019-08-03T11:27:16.4640080Z proper steps.
2019-08-03T11:27:17.2370595Z ##[error]Bash exited with code '3'.
2019-08-03T11:27:17.2420459Z ##[section]Starting: Checkout
2019-08-03T11:27:17.2422608Z ==============================================================================
2019-08-03T11:27:17.2422847Z Task         : Get sources
2019-08-03T11:27:17.2422895Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
