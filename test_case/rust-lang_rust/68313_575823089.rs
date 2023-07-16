plain
2020-01-17T22:35:50.7337436Z ========================== Starting Command Output ===========================
2020-01-17T22:35:50.7340788Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/7e4eada4-a71b-459d-8ba2-ab1c50de820a.sh
2020-01-17T22:35:50.7340969Z 
2020-01-17T22:35:50.7344827Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-17T22:35:50.7350865Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68313/merge to s
2020-01-17T22:35:50.7352425Z Task         : Get sources
2020-01-17T22:35:50.7352457Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-17T22:35:50.7352632Z Version      : 1.0.0
2020-01-17T22:35:50.7352669Z Author       : Microsoft
---
2020-01-17T22:35:55.0786796Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-17T22:35:55.1112176Z ##[command]git config gc.auto 0
2020-01-17T22:35:55.1191578Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-17T22:35:55.1246378Z ##[command]git config --get-all http.proxy
2020-01-17T22:35:55.1397263Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68313/merge:refs/remotes/pull/68313/merge
---
2020-01-17T22:41:08.8077419Z     Checking panic_abort v0.0.0 (/checkout/src/libpanic_abort)
2020-01-17T22:41:09.9019628Z     Checking backtrace v0.3.40
2020-01-17T22:41:10.4342389Z     Checking rustc-std-workspace-alloc v1.99.0 (/checkout/src/tools/rustc-std-workspace-alloc)
2020-01-17T22:41:10.4353294Z     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2020-01-17T22:41:11.9903985Z error: type `ip_mcast_type_v4` should have an upper camel case name
2020-01-17T22:41:11.9904832Z   --> src/libstd/sys_common/net.rs:49:14
2020-01-17T22:41:11.9905125Z    |
2020-01-17T22:41:11.9905505Z 49 |         type ip_mcast_type_v4 = c_int;
2020-01-17T22:41:11.9906099Z    |              ^^^^^^^^^^^^^^^^ help: convert the identifier to upper camel case: `IpMcastTypeV4`
2020-01-17T22:41:11.9906788Z    = note: `-D non-camel-case-types` implied by `-D warnings`
2020-01-17T22:41:11.9906836Z 
2020-01-17T22:41:13.9028497Z error: aborting due to previous error
2020-01-17T22:41:13.9029095Z 
---
2020-01-17T22:41:13.9031424Z   local time: Fri Jan 17 22:41:13 UTC 2020
2020-01-17T22:41:14.1421241Z   network time: Fri, 17 Jan 2020 22:41:14 GMT
2020-01-17T22:41:14.1423763Z == end clock drift check ==
2020-01-17T22:41:15.0968779Z 
2020-01-17T22:41:15.1103840Z ##[error]Bash exited with code '1'.
2020-01-17T22:41:15.1116769Z ##[section]Finishing: Run build
2020-01-17T22:41:15.1133924Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68313/merge to s
2020-01-17T22:41:15.1135944Z Task         : Get sources
2020-01-17T22:41:15.1136016Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-17T22:41:15.1136068Z Version      : 1.0.0
2020-01-17T22:41:15.1136117Z Author       : Microsoft
2020-01-17T22:41:15.1136117Z Author       : Microsoft
2020-01-17T22:41:15.1136186Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-17T22:41:15.1136242Z ==============================================================================
2020-01-17T22:41:15.5703021Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-17T22:41:15.5744636Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68313/merge to s
2020-01-17T22:41:15.5870049Z Cleaning up task key
2020-01-17T22:41:15.5870933Z Start cleaning up orphan processes.
2020-01-17T22:41:15.5994522Z Terminate orphan process: pid (3409) (python)
2020-01-17T22:41:15.6211377Z ##[section]Finishing: Finalize Job
