plain
2020-03-03T06:38:40.5711652Z ========================== Starting Command Output ===========================
2020-03-03T06:38:40.5717729Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/8dae38c4-7cf3-4d1b-9163-1ea037e8c357.sh
2020-03-03T06:38:40.5718643Z 
2020-03-03T06:38:40.5722437Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-03T06:38:40.5741653Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69478/merge to s
2020-03-03T06:38:40.5746133Z Task         : Get sources
2020-03-03T06:38:40.5746426Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-03T06:38:40.5746725Z Version      : 1.0.0
2020-03-03T06:38:40.5746920Z Author       : Microsoft
---
2020-03-03T06:38:42.1794205Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-03T06:38:42.1809034Z ##[command]git config gc.auto 0
2020-03-03T06:38:42.1815365Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-03T06:38:42.1821916Z ##[command]git config --get-all http.proxy
2020-03-03T06:38:42.1832661Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69478/merge:refs/remotes/pull/69478/merge
---
2020-03-03T06:45:36.7253788Z     Finished release [optimized] target(s) in 1m 36s
2020-03-03T06:45:36.7357564Z tidy check
2020-03-03T06:45:38.1677511Z * 591 error codes
2020-03-03T06:45:38.1678305Z * highest error code: E0748
2020-03-03T06:45:38.2165593Z tidy error: /checkout/src/librustc_feature/active.rs:346: no tracking issue for feature abi_avr_interrupt
2020-03-03T06:45:38.2166776Z tidy error: /checkout/src/librustc_feature/active.rs:349: feature decl_macro is not sorted by "since" (version number)
2020-03-03T06:45:39.5497591Z some tidy checks failed
2020-03-03T06:45:39.5498505Z Found 489 error codes
2020-03-03T06:45:39.5498937Z Found 0 error codes with no tests
2020-03-03T06:45:39.5499821Z Done!
2020-03-03T06:45:39.5499821Z Done!
2020-03-03T06:45:39.5531271Z 
2020-03-03T06:45:39.5531641Z 
2020-03-03T06:45:39.5533544Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-03-03T06:45:39.5534702Z 
2020-03-03T06:45:39.5534938Z 
2020-03-03T06:45:39.5535329Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-03-03T06:45:39.5535805Z Build completed unsuccessfully in 0:01:48
2020-03-03T06:45:39.5535805Z Build completed unsuccessfully in 0:01:48
2020-03-03T06:45:39.5563559Z == clock drift check ==
2020-03-03T06:45:39.5574188Z   local time: Tue Mar  3 06:45:39 UTC 2020
2020-03-03T06:45:39.8390527Z   network time: Tue, 03 Mar 2020 06:45:39 GMT
2020-03-03T06:45:39.8396739Z == end clock drift check ==
2020-03-03T06:45:40.6591071Z 
2020-03-03T06:45:40.6692630Z ##[error]Bash exited with code '1'.
2020-03-03T06:45:40.6705596Z ##[section]Finishing: Run build
2020-03-03T06:45:40.6748218Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69478/merge to s
2020-03-03T06:45:40.6753178Z Task         : Get sources
2020-03-03T06:45:40.6753505Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-03T06:45:40.6753783Z Version      : 1.0.0
2020-03-03T06:45:40.6753978Z Author       : Microsoft
2020-03-03T06:45:40.6753978Z Author       : Microsoft
2020-03-03T06:45:40.6754305Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-03T06:45:40.6754661Z ==============================================================================
2020-03-03T06:45:41.0070205Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-03T06:45:41.0114634Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69478/merge to s
2020-03-03T06:45:41.0201292Z Cleaning up task key
2020-03-03T06:45:41.0202559Z Start cleaning up orphan processes.
2020-03-03T06:45:41.0509364Z Terminate orphan process: pid (4212) (python)
2020-03-03T06:45:41.0544790Z ##[section]Finishing: Finalize Job
