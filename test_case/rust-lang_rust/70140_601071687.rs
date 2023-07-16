plain
2020-03-19T09:10:55.6594288Z ========================== Starting Command Output ===========================
2020-03-19T09:10:55.6599106Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/624fb5ef-bd23-4005-879d-83bf86977f01.sh
2020-03-19T09:10:55.6599806Z 
2020-03-19T09:10:55.6603741Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-19T09:10:55.6621082Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70140/merge to s
2020-03-19T09:10:55.6623919Z Task         : Get sources
2020-03-19T09:10:55.6624172Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-19T09:10:55.6624415Z Version      : 1.0.0
2020-03-19T09:10:55.6624580Z Author       : Microsoft
---
2020-03-19T09:10:56.6504683Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-19T09:10:56.6510147Z ##[command]git config gc.auto 0
2020-03-19T09:10:56.6513711Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-19T09:10:56.6517385Z ##[command]git config --get-all http.proxy
2020-03-19T09:10:56.6524046Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70140/merge:refs/remotes/pull/70140/merge
---
2020-03-19T09:16:02.3129320Z     Checking core v0.0.0 (/checkout/src/libcore)
2020-03-19T09:16:05.8626653Z error[E0545]: `issue` must be a non-zero numeric string or "none"
2020-03-19T09:16:05.8628026Z     --> src/libcore/result.rs:1243:48
2020-03-19T09:16:05.8628944Z      |
2020-03-19T09:16:05.8630345Z 1243 |      #[unstable(feature = "result_flattening", issue = "0")]
2020-03-19T09:16:05.8631986Z      |                                                        |
2020-03-19T09:16:05.8631986Z      |                                                        |
2020-03-19T09:16:05.8632800Z      |                                                        `issue` must not be "0", use "none" instead
2020-03-19T09:16:09.9538076Z    Compiling libc v0.2.66
2020-03-19T09:16:10.8687187Z    Compiling autocfg v0.1.7
2020-03-19T09:16:12.1470056Z error: aborting due to previous error
2020-03-19T09:16:12.1470434Z 
---
2020-03-19T09:16:12.6248172Z   local time: Thu Mar 19 09:16:12 UTC 2020
2020-03-19T09:16:13.1604900Z   network time: Thu, 19 Mar 2020 09:16:13 GMT
2020-03-19T09:16:13.1608705Z == end clock drift check ==
2020-03-19T09:16:17.9386695Z 
2020-03-19T09:16:17.9454879Z ##[error]Bash exited with code '1'.
2020-03-19T09:16:17.9471404Z ##[section]Finishing: Run build
2020-03-19T09:16:17.9521878Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70140/merge to s
2020-03-19T09:16:17.9526171Z Task         : Get sources
2020-03-19T09:16:17.9526496Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-19T09:16:17.9526797Z Version      : 1.0.0
2020-03-19T09:16:17.9526996Z Author       : Microsoft
2020-03-19T09:16:17.9526996Z Author       : Microsoft
2020-03-19T09:16:17.9527324Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-19T09:16:17.9527852Z ==============================================================================
2020-03-19T09:16:18.2839146Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-19T09:16:18.2883229Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70140/merge to s
2020-03-19T09:16:18.2974657Z Cleaning up task key
2020-03-19T09:16:18.2975746Z Start cleaning up orphan processes.
2020-03-19T09:16:18.3265403Z Terminate orphan process: pid (4450) (python)
2020-03-19T09:16:18.3288359Z ##[section]Finishing: Finalize Job
