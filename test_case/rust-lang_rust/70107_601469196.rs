plain
2020-03-19T23:28:16.9773598Z ========================== Starting Command Output ===========================
2020-03-19T23:28:16.9777041Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/00bebd83-c190-4a2e-9500-4f4f050c13c6.sh
2020-03-19T23:28:16.9777456Z 
2020-03-19T23:28:16.9786398Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-19T23:28:16.9805785Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70107/merge to s
2020-03-19T23:28:16.9808784Z Task         : Get sources
2020-03-19T23:28:16.9809034Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-19T23:28:16.9809276Z Version      : 1.0.0
2020-03-19T23:28:16.9809769Z Author       : Microsoft
---
2020-03-19T23:28:17.9743213Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-19T23:28:17.9749872Z ##[command]git config gc.auto 0
2020-03-19T23:28:17.9756322Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-19T23:28:17.9761786Z ##[command]git config --get-all http.proxy
2020-03-19T23:28:17.9773020Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70107/merge:refs/remotes/pull/70107/merge
---
2020-03-19T23:55:52.1862202Z    Compiling compiler_builtins v0.1.25
2020-03-19T23:55:52.9576332Z error[E0282]: type annotations needed
2020-03-19T23:55:52.9576939Z    --> src/libcore/array/iter.rs:193:37
2020-03-19T23:55:52.9577341Z     |
2020-03-19T23:55:52.9577893Z 193 |         unsafe { ptr::drop_in_place(self.as_mut_slice()) }
2020-03-19T23:55:52.9579066Z 
2020-03-19T23:55:53.8075291Z    Compiling backtrace-sys v0.1.32
2020-03-19T23:55:54.6012933Z    Compiling unwind v0.0.0 (/checkout/src/libunwind)
2020-03-19T23:55:55.4627770Z    Compiling hashbrown v0.6.2
---
2020-03-19T23:56:00.6784335Z   local time: Thu Mar 19 23:56:00 UTC 2020
2020-03-19T23:56:00.8413328Z   network time: Thu, 19 Mar 2020 23:56:00 GMT
2020-03-19T23:56:00.8418904Z == end clock drift check ==
2020-03-19T23:56:02.5400410Z 
2020-03-19T23:56:02.5466639Z ##[error]Bash exited with code '1'.
2020-03-19T23:56:02.5482786Z ##[section]Finishing: Run build
2020-03-19T23:56:02.5529056Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70107/merge to s
2020-03-19T23:56:02.5534120Z Task         : Get sources
2020-03-19T23:56:02.5534400Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-19T23:56:02.5534660Z Version      : 1.0.0
2020-03-19T23:56:02.5534856Z Author       : Microsoft
2020-03-19T23:56:02.5534856Z Author       : Microsoft
2020-03-19T23:56:02.5535147Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-19T23:56:02.5535478Z ==============================================================================
2020-03-19T23:56:02.8993856Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-19T23:56:02.9041905Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70107/merge to s
2020-03-19T23:56:02.9126622Z Cleaning up task key
2020-03-19T23:56:02.9127863Z Start cleaning up orphan processes.
2020-03-19T23:56:02.9322913Z Terminate orphan process: pid (3815) (python)
2020-03-19T23:56:02.9514726Z ##[section]Finishing: Finalize Job
