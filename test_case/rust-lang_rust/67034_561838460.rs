plain
2019-12-04T20:59:39.8396224Z ========================== Starting Command Output ===========================
2019-12-04T20:59:39.8400574Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/0f91aee5-11b0-49cb-9930-0656d360b9e1.sh
2019-12-04T20:59:39.8400811Z 
2019-12-04T20:59:39.8405775Z ##[section]Finishing: Disable git automatic line ending conversion
2019-12-04T20:59:39.8413129Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67034/merge to s
2019-12-04T20:59:39.8415217Z Task         : Get sources
2019-12-04T20:59:39.8415328Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2019-12-04T20:59:39.8415367Z Version      : 1.0.0
2019-12-04T20:59:39.8415406Z Author       : Microsoft
---
2019-12-04T20:59:42.4933558Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-04T20:59:43.3266667Z ##[command]git config gc.auto 0
2019-12-04T20:59:43.3272422Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-04T20:59:43.3274506Z ##[command]git config --get-all http.proxy
2019-12-04T20:59:43.3282535Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67034/merge:refs/remotes/pull/67034/merge
---
2019-12-04T21:04:40.4913535Z Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-04T21:04:40.4940557Z Checking std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
2019-12-04T21:04:40.9001566Z    Compiling cc v1.0.47
2019-12-04T21:04:40.9002190Z     Checking core v0.0.0 (/checkout/src/libcore)
2019-12-04T21:04:41.0042558Z error: expected one of `.`, `;`, `?`, `}`, or an operator, found keyword `if`
2019-12-04T21:04:41.0043874Z     |
2019-12-04T21:04:41.0044207Z 135 |         self.location.fmt(formatter)
2019-12-04T21:04:41.0044207Z 135 |         self.location.fmt(formatter)
2019-12-04T21:04:41.0044658Z     |                                     - expected one of `.`, `;`, `?`, `}`, or an operator here
2019-12-04T21:04:41.0045009Z 136 |         if let Some(message) = self.message {
2019-12-04T21:04:41.0045474Z 
2019-12-04T21:04:49.5082437Z    Compiling build_helper v0.1.0 (/checkout/src/build_helper)
2019-12-04T21:04:51.1987799Z    Compiling libc v0.2.64
2019-12-04T21:04:52.1231313Z    Compiling autocfg v0.1.6
---
2019-12-04T21:04:53.3617259Z   local time: Wed Dec  4 21:04:53 UTC 2019
2019-12-04T21:04:53.6547299Z   network time: Wed, 04 Dec 2019 21:04:53 GMT
2019-12-04T21:04:53.6549492Z == end clock drift check ==
2019-12-04T21:05:07.1536235Z 
2019-12-04T21:05:07.1683162Z ##[error]Bash exited with code '1'.
2019-12-04T21:05:07.1697672Z ##[section]Finishing: Run build
2019-12-04T21:05:07.1717465Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67034/merge to s
2019-12-04T21:05:07.1719898Z Task         : Get sources
2019-12-04T21:05:07.1719975Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2019-12-04T21:05:07.1720041Z Version      : 1.0.0
2019-12-04T21:05:07.1720083Z Author       : Microsoft
2019-12-04T21:05:07.1720083Z Author       : Microsoft
2019-12-04T21:05:07.1720156Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2019-12-04T21:05:07.1720209Z ==============================================================================
2019-12-04T21:05:07.6140328Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2019-12-04T21:05:07.6202816Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/67034/merge to s
2019-12-04T21:05:07.6318268Z Start cleaning up orphan processes.
2019-12-04T21:05:07.6446859Z Terminate orphan process: pid (3538) (python)
2019-12-04T21:05:07.6793387Z ##[section]Finishing: Finalize Job
2019-12-04T21:05:07.6858452Z ##[section]Finishing: Linux mingw-check
