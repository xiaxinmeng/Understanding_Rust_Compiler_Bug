plain
2020-02-08T20:40:15.0716933Z ========================== Starting Command Output ===========================
2020-02-08T20:40:15.0718628Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/5cce431a-7e25-4d14-a3bb-70225c86b910.sh
2020-02-08T20:40:15.0718664Z 
2020-02-08T20:40:15.0720873Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-08T20:40:15.0726345Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68835/merge to s
2020-02-08T20:40:15.0727833Z Task         : Get sources
2020-02-08T20:40:15.0727862Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-08T20:40:15.0727934Z Version      : 1.0.0
2020-02-08T20:40:15.0727963Z Author       : Microsoft
---
2020-02-08T20:40:16.9149186Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-08T20:40:16.9161220Z ##[command]git config gc.auto 0
2020-02-08T20:40:16.9163522Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-08T20:40:16.9167533Z ##[command]git config --get-all http.proxy
2020-02-08T20:40:16.9176940Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68835/merge:refs/remotes/pull/68835/merge
---
2020-02-08T20:45:08.0706164Z     Checking core v0.0.0 (/checkout/src/libcore)
2020-02-08T20:45:15.0447979Z error[E0609]: no field `is_empty` on type `&ops::range::RangeInclusive<Idx>`
2020-02-08T20:45:15.0450759Z    --> src/libcore/ops/range.rs:475:28
2020-02-08T20:45:15.0451004Z     |
2020-02-08T20:45:15.0451313Z 475 |         Some(true) == self.is_empty && self.start == self.end
2020-02-08T20:45:15.0451819Z     |
2020-02-08T20:45:15.0451819Z     |
2020-02-08T20:45:15.0452102Z     = note: available fields are: `start`, `end`, `exhausted`
2020-02-08T20:45:17.6286205Z    Compiling libc v0.2.66
2020-02-08T20:45:18.1582455Z error: aborting due to previous error
2020-02-08T20:45:18.1583411Z 
2020-02-08T20:45:18.1587686Z For more information about this error, try `rustc --explain E0609`.
---
2020-02-08T20:45:18.4690947Z   local time: Sat Feb  8 20:45:18 UTC 2020
2020-02-08T20:45:18.7575697Z   network time: Sat, 08 Feb 2020 20:45:18 GMT
2020-02-08T20:45:18.7575804Z == end clock drift check ==
2020-02-08T20:45:24.1623597Z 
2020-02-08T20:45:24.1708205Z ##[error]Bash exited with code '1'.
2020-02-08T20:45:24.1720666Z ##[section]Finishing: Run build
2020-02-08T20:45:24.1734284Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68835/merge to s
2020-02-08T20:45:24.1736093Z Task         : Get sources
2020-02-08T20:45:24.1736134Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-08T20:45:24.1736192Z Version      : 1.0.0
2020-02-08T20:45:24.1736228Z Author       : Microsoft
2020-02-08T20:45:24.1736228Z Author       : Microsoft
2020-02-08T20:45:24.1736410Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-08T20:45:24.1736470Z ==============================================================================
2020-02-08T20:45:25.5088077Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-08T20:45:25.5096718Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68835/merge to s
2020-02-08T20:45:25.5256449Z Cleaning up task key
2020-02-08T20:45:25.5257172Z Start cleaning up orphan processes.
2020-02-08T20:45:25.5428407Z Terminate orphan process: pid (4823) (python)
2020-02-08T20:45:25.5618554Z ##[section]Finishing: Finalize Job
