plain
2020-03-07T22:28:48.7282416Z ========================== Starting Command Output ===========================
2020-03-07T22:28:48.7284780Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/303460c3-aa49-40e3-a130-494e327300aa.sh
2020-03-07T22:28:48.7285475Z 
2020-03-07T22:28:48.7288989Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-07T22:28:48.7310996Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69799/merge to s
2020-03-07T22:28:48.7314768Z Task         : Get sources
2020-03-07T22:28:48.7315599Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-07T22:28:48.7315989Z Version      : 1.0.0
2020-03-07T22:28:48.7316281Z Author       : Microsoft
---
2020-03-07T22:28:49.8999286Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-07T22:28:49.9008581Z ##[command]git config gc.auto 0
2020-03-07T22:28:49.9015032Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-07T22:28:49.9018924Z ##[command]git config --get-all http.proxy
2020-03-07T22:28:49.9029777Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69799/merge:refs/remotes/pull/69799/merge
---
2020-03-07T22:34:26.4074017Z     Checking alloc v0.0.0 (/checkout/src/liballoc)
2020-03-07T22:34:27.1449621Z error[E0599]: no method named `dangling` found for struct `core::alloc::Layout` in the current scope
2020-03-07T22:34:27.1451082Z    --> src/liballoc/alloc.rs:170:24
2020-03-07T22:34:27.1451877Z     |
2020-03-07T22:34:27.1452852Z 170 |             Ok((layout.dangling(), 0))
2020-03-07T22:34:27.1459393Z 
2020-03-07T22:34:27.1524559Z error[E0599]: no method named `dangling` found for struct `core::alloc::Layout` in the current scope
2020-03-07T22:34:27.1525600Z    --> src/liballoc/alloc.rs:191:34
2020-03-07T22:34:27.1549906Z     |
2020-03-07T22:34:27.1549906Z     |
2020-03-07T22:34:27.1550618Z 191 |             (0, 0) => Ok((layout.dangling(), 0)),
2020-03-07T22:34:27.1552482Z 
2020-03-07T22:34:27.1583228Z error[E0599]: no method named `dangling` found for struct `core::alloc::Layout` in the current scope
2020-03-07T22:34:27.1584491Z    --> src/liballoc/alloc.rs:195:28
2020-03-07T22:34:27.1585098Z     |
2020-03-07T22:34:27.1585098Z     |
2020-03-07T22:34:27.1585731Z 195 |                 Ok((layout.dangling(), 0))
2020-03-07T22:34:27.1587236Z 
2020-03-07T22:34:27.1638442Z error[E0599]: no method named `dangling` found for struct `core::alloc::Layout` in the current scope
2020-03-07T22:34:27.1639258Z    --> src/liballoc/alloc.rs:206:24
2020-03-07T22:34:27.1639811Z     |
2020-03-07T22:34:27.1639811Z     |
2020-03-07T22:34:27.1640431Z 206 |             Ok((layout.dangling(), 0))
2020-03-07T22:34:27.1641907Z 
2020-03-07T22:34:27.2963333Z     Checking rustc-demangle v0.1.16
2020-03-07T22:34:27.5951125Z     Checking panic_abort v0.0.0 (/checkout/src/libpanic_abort)
2020-03-07T22:34:27.7645153Z     Checking backtrace v0.3.44
---
2020-03-07T22:34:28.0196094Z   local time: Sat Mar  7 22:34:28 UTC 2020
2020-03-07T22:34:28.3079831Z   network time: Sat, 07 Mar 2020 22:34:28 GMT
2020-03-07T22:34:28.3084757Z == end clock drift check ==
2020-03-07T22:34:29.2900679Z 
2020-03-07T22:34:29.2984698Z ##[error]Bash exited with code '1'.
2020-03-07T22:34:29.3000387Z ##[section]Finishing: Run build
2020-03-07T22:34:29.3052521Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69799/merge to s
2020-03-07T22:34:29.3057624Z Task         : Get sources
2020-03-07T22:34:29.3057989Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-07T22:34:29.3058333Z Version      : 1.0.0
2020-03-07T22:34:29.3058585Z Author       : Microsoft
2020-03-07T22:34:29.3058585Z Author       : Microsoft
2020-03-07T22:34:29.3058953Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-07T22:34:29.3059391Z ==============================================================================
2020-03-07T22:34:29.6787958Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-07T22:34:29.6834033Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69799/merge to s
2020-03-07T22:34:29.6930577Z Cleaning up task key
2020-03-07T22:34:29.6931880Z Start cleaning up orphan processes.
2020-03-07T22:34:29.7132741Z Terminate orphan process: pid (3711) (python)
2020-03-07T22:34:29.7282246Z ##[section]Finishing: Finalize Job
