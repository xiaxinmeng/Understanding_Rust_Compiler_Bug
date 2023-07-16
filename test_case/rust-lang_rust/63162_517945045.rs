plain
2019-08-03T16:12:08.0887961Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-08-03T16:12:08.1084404Z ##[command]git config gc.auto 0
2019-08-03T16:12:08.1164653Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-08-03T16:12:08.1256525Z ##[command]git config --get-all http.proxy
2019-08-03T16:12:08.1408180Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/63162/merge:refs/remotes/pull/63162/merge
---
2019-08-03T16:12:42.5682680Z do so (now or later) by using -b with the checkout command again. Example:
2019-08-03T16:12:42.5682893Z 
2019-08-03T16:12:42.5683327Z   git checkout -b <new-branch-name>
2019-08-03T16:12:42.5683503Z 
2019-08-03T16:12:42.5683666Z HEAD is now at bb85357ee Merge 6187bfb837fecbdd7831cec9a683b16ff4378d8d into 8e917f48382c6afaf50568263b89d35fba5d98e4
2019-08-03T16:12:42.5824882Z ##[section]Finishing: Checkout
2019-08-03T16:12:42.5832033Z ##[section]Starting: Decide whether to run this job
2019-08-03T16:12:42.5834770Z Task         : Bash
2019-08-03T16:12:42.5834814Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-08-03T16:12:42.5834857Z Version      : 3.151.3
2019-08-03T16:12:42.5834914Z Author       : Microsoft Corporation
2019-08-03T16:12:42.5834914Z Author       : Microsoft Corporation
2019-08-03T16:12:42.5834961Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-08-03T16:12:42.5835010Z ==============================================================================
2019-08-03T16:12:42.7271569Z Generating script.
2019-08-03T16:12:42.7307338Z ========================== Starting Command Output ===========================
2019-08-03T16:12:42.7330564Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/50e4d0e0-c0fd-4c8c-a07e-20d4f20b0941.sh
2019-08-03T16:12:43.0903475Z Executing the job since submodules are updated
2019-08-03T16:12:43.1001554Z ##[section]Finishing: Decide whether to run this job
2019-08-03T16:12:43.1012329Z ==============================================================================
2019-08-03T16:12:43.1012446Z Task         : Bash
2019-08-03T16:12:43.1012498Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-08-03T16:12:43.1012548Z Version      : 3.151.3
---
2019-08-03T16:15:50.0879970Z  ---> 47e49f646fd5
2019-08-03T16:15:50.0924230Z Successfully built 47e49f646fd5
2019-08-03T16:15:50.1547111Z Successfully tagged rust-ci:latest
2019-08-03T16:15:50.1867579Z Built container sha256:47e49f646fd5053a97a31ed985f3b9acbbab947e043464427538b8fdd7736d8f
2019-08-03T16:15:50.1888000Z Uploading finished image to https://rust-lang-ci-sccache2.s3.amazonaws.com/docker/0a94e800dbdbef90fbb9cf381d041a9386910340a5cdd758b34797988dea18bff1c9fc0562ea46a76a5a5ba9a8bbf4046ce3ddf0fb0b70448e54334a8af215fe
2019-08-03T16:16:30.4926863Z upload failed: - to s3://rust-lang-ci-sccache2/docker/0a94e800dbdbef90fbb9cf381d041a9386910340a5cdd758b34797988dea18bff1c9fc0562ea46a76a5a5ba9a8bbf4046ce3ddf0fb0b70448e54334a8af215fe Unable to locate credentials
2019-08-03T16:16:31.4212000Z [CI_JOB_NAME=LinuxTools]
2019-08-03T16:16:31.4261000Z Starting sccache server...
2019-08-03T16:16:31.5098963Z configure: processing command line
2019-08-03T16:16:31.5099731Z configure: 
---
2019-08-03T18:02:36.4742016Z    Compiling rustbook v0.1.0 (/checkout/src/tools/rustbook)
2019-08-03T18:02:40.7644424Z     Finished release [optimized] target(s) in 8m 12s
2019-08-03T18:04:37.1148099Z Error: there are broken links
2019-08-03T18:04:37.1151031Z  Caused By: "https://github.com/rust-lang/compiler-team/blob/master/experts/MAP.md" returned 404 Not Found
2019-08-03T18:04:37.1160274Z  Caused By: "***/tree/master/src/test/run-pass/" returned 404 Not Found
2019-08-03T18:04:37.1161034Z  Caused By: There was an error while fetching "http://rust-lang.github.io/rfcs/1211-mir.html", http://rust-lang.github.io/rfcs/1211-mir.html: timed out
2019-08-03T18:04:37.1162270Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/rustc/mir/enum.Place.html" returned 404 Not Found
2019-08-03T18:04:37.1162792Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/rustc/mir/enum.Place.html" returned 404 Not Found
2019-08-03T18:04:37.1163346Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/rustc/mir/enum.Place.html" returned 404 Not Found
2019-08-03T18:04:37.1163911Z  Caused By: "https://doc.rust-lang.org/nightly/nightly-rustc/rustc/mir/enum.Place.html" returned 404 Not Found
---
2019-08-03T18:21:48.4539426Z      |
2019-08-03T18:21:48.4542929Z 1269 | #[bench]
2019-08-03T18:21:48.4543226Z      |   ^^^^^
2019-08-03T18:21:48.4550027Z      |
2019-08-03T18:21:48.4550527Z      = note: for more information, see ***/issues/50297
2019-08-03T18:21:48.4551001Z 
2019-08-03T18:21:48.4571422Z error[E0658]: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
2019-08-03T18:21:48.4571843Z     --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_data_structures-542.0.0/bit_set.rs:1284:3
2019-08-03T18:21:48.4572073Z      |
2019-08-03T18:21:48.4572073Z      |
2019-08-03T18:21:48.4572348Z 1284 | #[bench]
2019-08-03T18:21:48.4572704Z      |   ^^^^^
2019-08-03T18:21:48.4572929Z      |
2019-08-03T18:21:48.4574148Z      = note: for more information, see ***/issues/50297
2019-08-03T18:21:48.4574936Z 
2019-08-03T18:21:48.4575332Z error[E0658]: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
2019-08-03T18:21:48.4619111Z     --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_data_structures-542.0.0/bit_set.rs:1302:3
2019-08-03T18:21:48.4619520Z      |
2019-08-03T18:21:48.4619520Z      |
2019-08-03T18:21:48.4619783Z 1302 | #[bench]
2019-08-03T18:21:48.4620033Z      |   ^^^^^
2019-08-03T18:21:48.4620576Z      |
2019-08-03T18:21:48.4621446Z      = note: for more information, see ***/issues/50297
2019-08-03T18:21:48.4621855Z 
2019-08-03T18:21:48.4622182Z error[E0658]: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
2019-08-03T18:21:48.4622524Z     --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_data_structures-542.0.0/bit_set.rs:1320:3
2019-08-03T18:21:48.4622768Z      |
2019-08-03T18:21:48.4622768Z      |
2019-08-03T18:21:48.4623025Z 1320 | #[bench]
2019-08-03T18:21:48.4623282Z      |   ^^^^^
2019-08-03T18:21:48.4623934Z      |
2019-08-03T18:21:48.4624321Z      = note: for more information, see ***/issues/50297
2019-08-03T18:21:48.4624697Z 
2019-08-03T18:21:48.4625020Z error[E0658]: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
2019-08-03T18:21:48.4625360Z     --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_data_structures-542.0.0/bit_set.rs:1335:3
2019-08-03T18:21:48.4625607Z      |
2019-08-03T18:21:48.4625607Z      |
2019-08-03T18:21:48.4625862Z 1335 | #[bench]
2019-08-03T18:21:48.4626119Z      |   ^^^^^
2019-08-03T18:21:48.4626365Z      |
2019-08-03T18:21:48.4626831Z      = note: for more information, see ***/issues/50297
2019-08-03T18:21:48.4627173Z 
2019-08-03T18:21:50.3858552Z error: aborting due to 5 previous errors
2019-08-03T18:21:50.3858661Z 
2019-08-03T18:21:50.3864811Z For more information about this error, try `rustc --explain E0658`.
---
2019-08-03T18:22:15.5162174Z      |
2019-08-03T18:22:15.5167268Z 1269 | #[bench]
2019-08-03T18:22:15.5173421Z      |   ^^^^^
2019-08-03T18:22:15.5176771Z      |
2019-08-03T18:22:15.5180277Z      = note: for more information, see ***/issues/50297
2019-08-03T18:22:15.5189741Z 
2019-08-03T18:22:15.5218177Z error[E0658]: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
2019-08-03T18:22:15.5219739Z     --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_data_structures-542.0.0/bit_set.rs:1284:3
2019-08-03T18:22:15.5220004Z      |
2019-08-03T18:22:15.5220004Z      |
2019-08-03T18:22:15.5220242Z 1284 | #[bench]
2019-08-03T18:22:15.5220470Z      |   ^^^^^
2019-08-03T18:22:15.5220682Z      |
2019-08-03T18:22:15.5221012Z      = note: for more information, see ***/issues/50297
2019-08-03T18:22:15.5221474Z 
2019-08-03T18:22:15.5222742Z error[E0658]: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
2019-08-03T18:22:15.5223129Z     --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_data_structures-542.0.0/bit_set.rs:1302:3
2019-08-03T18:22:15.5223394Z      |
2019-08-03T18:22:15.5223394Z      |
2019-08-03T18:22:15.5224203Z 1302 | #[bench]
2019-08-03T18:22:15.5224506Z      |   ^^^^^
2019-08-03T18:22:15.5224750Z      |
2019-08-03T18:22:15.5225344Z      = note: for more information, see ***/issues/50297
2019-08-03T18:22:15.5225783Z 
2019-08-03T18:22:15.5226062Z error[E0658]: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
2019-08-03T18:22:15.5226386Z     --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_data_structures-542.0.0/bit_set.rs:1320:3
2019-08-03T18:22:15.5226808Z      |
2019-08-03T18:22:15.5226808Z      |
2019-08-03T18:22:15.5227034Z 1320 | #[bench]
2019-08-03T18:22:15.5227274Z      |   ^^^^^
2019-08-03T18:22:15.5227470Z      |
2019-08-03T18:22:15.5227832Z      = note: for more information, see ***/issues/50297
2019-08-03T18:22:15.5228170Z 
2019-08-03T18:22:15.5228449Z error[E0658]: use of unstable library feature 'test': `bench` is a part of custom test frameworks which are unstable
2019-08-03T18:22:15.5228855Z     --> /cargo/registry/src/github.com-1ecc6299db9ec823/rustc-ap-rustc_data_structures-542.0.0/bit_set.rs:1335:3
2019-08-03T18:22:15.5229104Z      |
2019-08-03T18:22:15.5229104Z      |
2019-08-03T18:22:15.5229329Z 1335 | #[bench]
2019-08-03T18:22:15.5229568Z      |   ^^^^^
2019-08-03T18:22:15.5229760Z      |
2019-08-03T18:22:15.5230077Z      = note: for more information, see ***/issues/50297
2019-08-03T18:22:15.5230387Z 
2019-08-03T18:22:17.3830433Z error: aborting due to 5 previous errors
2019-08-03T18:22:17.3832056Z 
2019-08-03T18:22:17.3837136Z For more information about this error, try `rustc --explain E0658`.
---
2019-08-03T18:22:30.7105792Z    Compiling vergen v3.0.4
2019-08-03T18:22:36.5447105Z    Compiling miri v0.1.0 (/checkout/src/tools/miri)
2019-08-03T18:24:31.0477099Z     Finished release [optimized] target(s) in 2m 10s
2019-08-03T18:24:31.2832289Z     Finished release [optimized] target(s) in 0.22s
2019-08-03T18:24:31.2855774Z      Running `build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/cargo-miri miri setup`
2019-08-03T18:24:31.2886173Z Installing xargo: `cargo install xargo -f`
---
2019-08-03T18:25:26.2465370Z    Compiling num-traits v0.1.43
2019-08-03T18:25:26.3141348Z    Compiling serde_json v0.8.6
2019-08-03T18:25:27.9633314Z    Compiling error-chain v0.7.2
2019-08-03T18:25:45.7700724Z     Finished release [optimized] target(s) in 1m 14s
2019-08-03T18:25:45.7795683Z   Installing /checkout/obj/build/bin/xargo
2019-08-03T18:25:45.7795800Z    Installed package `xargo v0.3.14` (executable `xargo`)
2019-08-03T18:25:45.7795885Z warning: be sure to add `/checkout/obj/build/bin` to your PATH to be able to run the installed binaries
2019-08-03T18:25:45.8276678Z error: couldn't execute `"cargo" "build" "--release" "--manifest-path" "/tmp/xargo.l8FBAlQa4q66/Cargo.toml" "--target" "x86_64-unknown-linux-gnu" "-p" "std"`
2019-08-03T18:25:45.8276825Z caused by: No such file or directory (os error 2)
2019-08-03T18:25:45.8277192Z stack backtrace:
2019-08-03T18:25:45.8277408Z    0: error_chain::make_backtrace::h933f4fdac8566a18 (0x5609ab73606c)
2019-08-03T18:25:45.8277481Z    1: <core::result::Result<T,E> as xargo::errors::ResultExt<T,E>>::chain_err::h4ba04b62a8d7b6aa (0x5609ab6e349c)
2019-08-03T18:25:45.8277575Z    2: <std::process::Command as xargo::extensions::CommandExt>::run_and_get_status::hfd58f9347f11ed7d (0x5609ab70060f)
2019-08-03T18:25:45.8277643Z    3: <std::process::Command as xargo::extensions::CommandExt>::run::h0fc4df8a5f272a2d (0x5609ab7002ae)
2019-08-03T18:25:45.8277705Z    4: xargo::sysroot::build::h21b2087c5566ecf0 (0x5609ab6f0e09)
2019-08-03T18:25:45.8277815Z    5: xargo::sysroot::update::h05a036fe48cf8048 (0x5609ab6f342e)
2019-08-03T18:25:45.8278387Z    6: xargo::main::h65b81eeeee0998cd (0x5609ab6df29b)
2019-08-03T18:25:45.8278447Z    7: std::rt::lang_start::{{closure}}::h37650177518a390e (0x5609ab6e8993)
2019-08-03T18:25:45.8278530Z    8: std::panicking::try::do_call::h13cd310076c0705a (0x5609ab761512)
2019-08-03T18:25:45.8278589Z    9: __rust_maybe_catch_panic (0x5609ab76b17a)
2019-08-03T18:25:45.8278647Z   10: std::rt::lang_start_internal::h7c267df922bfe0cb (0x5609ab762200)
2019-08-03T18:25:45.8290303Z   11: main (0x5609ab6e0362)
2019-08-03T18:25:45.8290374Z   12: __libc_start_main (0x7fde32db7830)
2019-08-03T18:25:45.8290429Z   13: _start (0x5609ab6cbb79)
2019-08-03T18:25:45.8290500Z   14: <unknown> (0x0)
2019-08-03T18:25:45.8290552Z fatal error: Failed to run xargo
2019-08-03T18:25:45.8290620Z 
2019-08-03T18:25:45.8290620Z 
2019-08-03T18:25:45.8291621Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "run" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/miri/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--bin" "cargo-miri" "--" "miri" "setup"
2019-08-03T18:25:45.8291775Z 
2019-08-03T18:25:45.8291827Z 
2019-08-03T18:25:45.8291933Z 
2019-08-03T18:25:45.8291984Z 2 command(s) did not execute successfully:
2019-08-03T18:25:45.8291984Z 2 command(s) did not execute successfully:
2019-08-03T18:25:45.8292018Z 
2019-08-03T18:25:45.8292483Z   - "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/rustbook" "linkcheck" "/checkout/src/doc/rustc-guide"
2019-08-03T18:25:45.8292532Z 
2019-08-03T18:25:45.8293093Z   - "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "run" "--target" "x86_64-unknown-linux-gnu" "-j" "2" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/miri/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--bin" "cargo-miri" "--" "miri" "setup"
2019-08-03T18:25:45.8297282Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --no-fail-fast src/doc/book src/doc/nomicon src/doc/reference src/doc/rust-by-example src/doc/embedded-book src/doc/edition-guide src/doc/rustc-guide src/tools/clippy src/tools/rls src/tools/rustfmt src/tools/miri
2019-08-03T18:25:45.8297898Z Build completed unsuccessfully in 2:07:11
2019-08-03T18:25:45.8364605Z {"edition-guide":"test-pass","rustbook":"test-fail","rustfmt":"build-fail","clippy-driver":"build-fail","nomicon":"test-pass","rls":"build-fail","embedded-book":"test-pass","rust-by-example":"test-pass","miri":"test-fail","reference":"test-pass","book":"test-pass"}
2019-08-03T18:25:45.8364708Z Verifying status of book...
---
2019-08-03T18:25:45.8448673Z Verifying status of clippy-driver...
2019-08-03T18:25:45.8459900Z Verifying status of miri...
2019-08-03T18:25:45.8472860Z This PR updated 'src/tools/miri', verifying if status is 'test-pass'...
2019-08-03T18:25:45.8484708Z 
2019-08-03T18:25:45.8485321Z ⚠️ We detected that this PR updated 'miri', but its tests failed.
2019-08-03T18:25:45.8485396Z 
2019-08-03T18:25:45.8485706Z If you do intend to update 'miri', please check the error messages above and
2019-08-03T18:25:45.8485768Z commit another update.
2019-08-03T18:25:45.8485801Z 
2019-08-03T18:25:45.8486102Z If you do NOT intend to update 'miri', please ensure you did not accidentally
2019-08-03T18:25:45.8486394Z change the submodule at 'src/tools/miri'. You may ask your reviewer for the
2019-08-03T18:25:45.8486450Z proper steps.
2019-08-03T18:25:46.7680138Z ##[error]Bash exited with code '3'.
2019-08-03T18:25:46.7728719Z ##[section]Starting: Checkout
2019-08-03T18:25:46.7730737Z ==============================================================================
2019-08-03T18:25:46.7730798Z Task         : Get sources
2019-08-03T18:25:46.7730867Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
