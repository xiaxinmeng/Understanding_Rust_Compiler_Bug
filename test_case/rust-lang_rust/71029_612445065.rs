plain
2020-04-11T15:16:44.0650667Z ========================== Starting Command Output ===========================
2020-04-11T15:16:44.0653643Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/caabb19f-27da-471e-9f86-6e8baaedec14.sh
2020-04-11T15:16:44.0654033Z 
2020-04-11T15:16:44.0659133Z ##[section]Finishing: Disable git automatic line ending conversion
2020-04-11T15:16:44.0678980Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71029/merge to s
2020-04-11T15:16:44.0682547Z Task         : Get sources
2020-04-11T15:16:44.0682839Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-11T15:16:44.0683173Z Version      : 1.0.0
2020-04-11T15:16:44.0683363Z Author       : Microsoft
---
2020-04-11T15:16:45.2872093Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-04-11T15:16:45.2878195Z ##[command]git config gc.auto 0
2020-04-11T15:16:45.2883310Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-04-11T15:16:45.2888752Z ##[command]git config --get-all http.proxy
2020-04-11T15:16:45.2897841Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71029/merge:refs/remotes/pull/71029/merge
---
2020-04-11T15:18:58.8268505Z  ---> 78ad2f4d4aca
2020-04-11T15:18:58.8268819Z Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
2020-04-11T15:18:58.8269213Z  ---> Using cache
2020-04-11T15:18:58.8269594Z  ---> 4d2dc61c4d00
2020-04-11T15:18:58.8271166Z Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
2020-04-11T15:18:58.8273193Z  ---> 776b6266a8b7
2020-04-11T15:18:58.8273374Z Successfully built 776b6266a8b7
2020-04-11T15:18:58.8273981Z Successfully tagged rust-ci:latest
2020-04-11T15:18:58.8274493Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-11T15:18:58.8274493Z Built container sha256:776b6266a8b7d63e2d3c2b5a784dbf521184a904fb10bf818c6b5c7e1ab74d4a
2020-04-11T15:18:58.8274833Z Looks like docker image is the same as before, not uploading
2020-04-11T15:19:05.4397536Z [CI_JOB_NAME=mingw-check]
2020-04-11T15:19:05.4622985Z [CI_JOB_NAME=mingw-check]
2020-04-11T15:19:05.4651310Z == clock drift check ==
2020-04-11T15:19:05.4660176Z   local time: Sat Apr 11 15:19:05 UTC 2020
2020-04-11T15:19:05.5232413Z   network time: Sat, 11 Apr 2020 15:19:05 GMT
2020-04-11T15:19:05.5255151Z Starting sccache server...
2020-04-11T15:19:05.6327762Z configure: processing command line
2020-04-11T15:19:05.6328521Z configure: 
2020-04-11T15:19:05.6329879Z configure: rust.parallel-compiler := True
---
2020-04-11T15:20:59.0484108Z    Compiling hashbrown v0.6.2
2020-04-11T15:20:59.1678179Z error: failed to run custom build command for `compiler_builtins v0.1.25`
2020-04-11T15:20:59.1678945Z 
2020-04-11T15:20:59.1679100Z Caused by:
2020-04-11T15:20:59.1679834Z   process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/release/build/compiler_builtins-2069b1987fa53be5/build-script-build` (exit code: 101)
2020-04-11T15:20:59.1680472Z --- stdout
2020-04-11T15:20:59.1680834Z cargo:rerun-if-changed=build.rs
2020-04-11T15:20:59.1681327Z cargo:compiler-rt=/cargo/registry/src/github.com-1ecc6299db9ec823/compiler_builtins-0.1.25/compiler-rt
2020-04-11T15:20:59.1681765Z cargo:rustc-cfg=feature="unstable"
2020-04-11T15:20:59.1682462Z --- stderr
2020-04-11T15:20:59.1682462Z --- stderr
2020-04-11T15:20:59.1683417Z thread 'main' panicked at 'RUST_COMPILER_RT_ROOT is not set', /cargo/registry/src/github.com-1ecc6299db9ec823/compiler_builtins-0.1.25/build.rs:423:21
2020-04-11T15:20:59.1684074Z 
2020-04-11T15:20:59.1684491Z warning: build failed, waiting for other jobs to finish...
2020-04-11T15:20:59.3407779Z error: build failed
2020-04-11T15:20:59.3430913Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-11T15:20:59.3430913Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--message-format" "json-render-diagnostics"
2020-04-11T15:20:59.3431592Z expected success, got: exit code: 101
2020-04-11T15:20:59.3445971Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
2020-04-11T15:20:59.3446278Z Build completed unsuccessfully in 0:01:53
2020-04-11T15:20:59.3551235Z == clock drift check ==
2020-04-11T15:20:59.3566308Z   local time: Sat Apr 11 15:20:59 UTC 2020
2020-04-11T15:20:59.4933624Z   network time: Sat, 11 Apr 2020 15:20:59 GMT
2020-04-11T15:21:00.3887601Z 
2020-04-11T15:21:00.3887601Z 
2020-04-11T15:21:00.3982767Z ##[error]Bash exited with code '1'.
2020-04-11T15:21:00.3995708Z ##[section]Finishing: Run build
2020-04-11T15:21:00.4039165Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/71029/merge to s
2020-04-11T15:21:00.4043639Z Task         : Get sources
2020-04-11T15:21:00.4043935Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-04-11T15:21:00.4044531Z Version      : 1.0.0
2020-04-11T15:21:00.4044884Z Author       : Microsoft
2020-04-11T15:21:00.4044884Z Author       : Microsoft
2020-04-11T15:21:00.4045179Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-04-11T15:21:00.4045686Z ==============================================================================
2020-04-11T15:21:00.7010495Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-04-11T15:21:00.7055042Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/71029/merge to s
2020-04-11T15:21:00.7145504Z Cleaning up task key
2020-04-11T15:21:00.7147006Z Start cleaning up orphan processes.
2020-04-11T15:21:00.7352965Z Terminate orphan process: pid (3964) (python)
2020-04-11T15:21:00.7492841Z ##[section]Finishing: Finalize Job
