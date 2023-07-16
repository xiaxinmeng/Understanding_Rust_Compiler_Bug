plain
2020-01-21T15:35:35.5585191Z ========================== Starting Command Output ===========================
2020-01-21T15:35:35.5639426Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/27ae42d9-6f7f-4e30-b779-628b0e54c5b3.sh
2020-01-21T15:35:35.5639469Z 
2020-01-21T15:35:35.5643291Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-21T15:35:35.5647633Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68194/merge to s
2020-01-21T15:35:35.5648922Z Task         : Get sources
2020-01-21T15:35:35.5648948Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-21T15:35:35.5648973Z Version      : 1.0.0
2020-01-21T15:35:35.5648997Z Author       : Microsoft
---
2020-01-21T15:35:36.3801481Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-21T15:35:36.3880469Z ##[command]git config gc.auto 0
2020-01-21T15:35:36.3959908Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-21T15:35:36.4001892Z ##[command]git config --get-all http.proxy
2020-01-21T15:35:36.4149216Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68194/merge:refs/remotes/pull/68194/merge
---
2020-01-21T16:38:59.0343355Z     Finished release [optimized] target(s) in 1m 35s
2020-01-21T16:38:59.0592622Z Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> thumbv6m-none-eabi)
2020-01-21T16:38:59.0662903Z 
2020-01-21T16:38:59.0663069Z running 13 tests
2020-01-21T16:39:59.5739207Z ...iiiii.iiitest [run-make] run-make/thumb-none-qemu has been running for over 60 seconds
2020-01-21T16:40:37.1134569Z test result: ok. 5 passed; 0 failed; 8 ignored; 0 measured; 0 filtered out
2020-01-21T16:40:37.1134850Z 
2020-01-21T16:40:37.1140193Z  finished in 98.055
2020-01-21T16:40:37.1153243Z Building stage2 std artifacts (x86_64-unknown-linux-gnu -> thumbv7m-none-eabi)
---
2020-01-21T16:40:56.5632279Z exit code: 0
2020-01-21T16:40:56.5739747Z Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> thumbv7m-none-eabi)
2020-01-21T16:40:56.5770798Z 
2020-01-21T16:40:56.5771176Z running 13 tests
2020-01-21T16:41:57.0381363Z ...iiiii.iiitest [run-make] run-make/thumb-none-qemu has been running for over 60 seconds
2020-01-21T16:42:27.7220152Z test result: ok. 5 passed; 0 failed; 8 ignored; 0 measured; 0 filtered out
2020-01-21T16:42:27.7221971Z 
2020-01-21T16:42:27.7227132Z  finished in 91.148
2020-01-21T16:42:27.7239274Z Building stage2 std artifacts (x86_64-unknown-linux-gnu -> thumbv7em-none-eabi)
---
2020-01-21T16:42:46.6911649Z exit code: 0
2020-01-21T16:42:46.7019977Z Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> thumbv7em-none-eabi)
2020-01-21T16:42:46.7044113Z 
2020-01-21T16:42:46.7044554Z running 13 tests
2020-01-21T16:43:47.1605813Z ...iiiii.iiitest [run-make] run-make/thumb-none-qemu has been running for over 60 seconds
2020-01-21T16:44:18.3118591Z test result: ok. 5 passed; 0 failed; 8 ignored; 0 measured; 0 filtered out
2020-01-21T16:44:18.3118625Z 
2020-01-21T16:44:18.3122210Z  finished in 91.610
2020-01-21T16:44:18.3136489Z Building stage2 std artifacts (x86_64-unknown-linux-gnu -> thumbv7em-none-eabihf)
---
2020-01-21T16:44:37.9578991Z exit code: 0
2020-01-21T16:44:37.9706155Z Check compiletest suite=run-make mode=run-make (x86_64-unknown-linux-gnu -> thumbv7em-none-eabihf)
2020-01-21T16:44:37.9718840Z 
2020-01-21T16:44:37.9718956Z running 13 tests
2020-01-21T16:45:38.4601807Z ...iiiii.iiitest [run-make] run-make/thumb-none-qemu has been running for over 60 seconds
2020-01-21T16:46:10.7838131Z test result: ok. 5 passed; 0 failed; 8 ignored; 0 measured; 0 filtered out
2020-01-21T16:46:10.7853722Z 
2020-01-21T16:46:10.7853881Z  finished in 92.815
2020-01-21T16:46:10.7909921Z Build completed successfully in 1:04:59
---
2020-01-21T17:17:38.6338789Z -rw-r--r-- 1 vsts docker  13M Jan 21 17:17 rust-std-nightly-x86_64-unknown-redox.tar.xz
2020-01-21T17:17:38.6338821Z 
2020-01-21T17:17:38.6339195Z src/ci/scripts/upload-artifacts.sh: line 39: DEPLOY_BUCKET: unbound variable
2020-01-21T17:17:38.6339357Z 
2020-01-21T17:17:38.6398209Z ##[error]Bash exited with code '1'.
2020-01-21T17:17:38.6411271Z ##[section]Finishing: Upload artifacts
2020-01-21T17:17:38.6418736Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68194/merge to s
2020-01-21T17:17:38.6420708Z Task         : Get sources
2020-01-21T17:17:38.6420755Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-21T17:17:38.6420803Z Version      : 1.0.0
2020-01-21T17:17:38.6420865Z Author       : Microsoft
2020-01-21T17:17:38.6420865Z Author       : Microsoft
2020-01-21T17:17:38.6420913Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-21T17:17:38.6420964Z ==============================================================================
2020-01-21T17:17:39.0504918Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-21T17:17:39.0554242Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68194/merge to s
2020-01-21T17:17:39.0665278Z Cleaning up task key
2020-01-21T17:17:39.0665922Z Start cleaning up orphan processes.
2020-01-21T17:17:39.0769442Z Terminate orphan process: pid (3462) (python)
2020-01-21T17:17:39.0969372Z ##[section]Finishing: Finalize Job
