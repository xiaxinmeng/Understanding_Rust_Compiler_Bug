plain
2020-03-07T14:49:28.9973336Z ========================== Starting Command Output ===========================
2020-03-07T14:49:28.9980007Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/41cf9404-9b99-4063-86ca-6497e6a523be.sh
2020-03-07T14:49:28.9980589Z 
2020-03-07T14:49:28.9986440Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-07T14:49:29.0012963Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69799/merge to s
2020-03-07T14:49:29.0017045Z Task         : Get sources
2020-03-07T14:49:29.0017414Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-07T14:49:29.0017731Z Version      : 1.0.0
2020-03-07T14:49:29.0017948Z Author       : Microsoft
---
2020-03-07T14:49:29.9877892Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-07T14:49:29.9883889Z ##[command]git config gc.auto 0
2020-03-07T14:49:29.9889231Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-07T14:49:29.9892903Z ##[command]git config --get-all http.proxy
2020-03-07T14:49:29.9899813Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69799/merge:refs/remotes/pull/69799/merge
---
2020-03-07T14:54:21.7454936Z     Checking alloc v0.0.0 (/checkout/src/liballoc)
2020-03-07T14:54:22.4682786Z error[E0599]: no method named `dangling` found for struct `core::alloc::Layout` in the current scope
2020-03-07T14:54:22.4683664Z    --> src/liballoc/alloc.rs:170:24
2020-03-07T14:54:22.4684112Z     |
2020-03-07T14:54:22.4684662Z 170 |             Ok((layout.dangling(), 0))
2020-03-07T14:54:22.4686253Z 
2020-03-07T14:54:22.4735553Z error[E0599]: no method named `dangling` found for struct `core::alloc::Layout` in the current scope
2020-03-07T14:54:22.4736283Z    --> src/liballoc/alloc.rs:191:34
2020-03-07T14:54:22.4736756Z     |
2020-03-07T14:54:22.4736756Z     |
2020-03-07T14:54:22.4737323Z 191 |             (0, 0) => Ok((layout.dangling(), 0)),
2020-03-07T14:54:22.4738672Z 
2020-03-07T14:54:22.4785847Z error[E0599]: no method named `dangling` found for struct `core::alloc::Layout` in the current scope
2020-03-07T14:54:22.4786570Z    --> src/liballoc/alloc.rs:195:28
2020-03-07T14:54:22.4787052Z     |
2020-03-07T14:54:22.4787052Z     |
2020-03-07T14:54:22.4787593Z 195 |                 Ok((layout.dangling(), 0))
2020-03-07T14:54:22.4788895Z 
2020-03-07T14:54:22.4830433Z error[E0599]: no method named `dangling` found for struct `core::alloc::Layout` in the current scope
2020-03-07T14:54:22.4831165Z    --> src/liballoc/alloc.rs:206:24
2020-03-07T14:54:22.4831606Z     |
2020-03-07T14:54:22.4831606Z     |
2020-03-07T14:54:22.4832137Z 206 |             Ok((layout.dangling(), 0))
2020-03-07T14:54:22.4833426Z 
2020-03-07T14:54:22.5990693Z     Checking rustc-demangle v0.1.16
2020-03-07T14:54:22.8845959Z     Checking panic_abort v0.0.0 (/checkout/src/libpanic_abort)
2020-03-07T14:54:23.0474766Z     Checking backtrace v0.3.44
---
2020-03-07T14:54:23.3395503Z   local time: Sat Mar  7 14:54:23 UTC 2020
2020-03-07T14:54:23.6286240Z   network time: Sat, 07 Mar 2020 14:54:23 GMT
2020-03-07T14:54:23.6291812Z == end clock drift check ==
2020-03-07T14:54:24.6682653Z 
2020-03-07T14:54:24.6749851Z ##[error]Bash exited with code '1'.
2020-03-07T14:54:24.6767793Z ##[section]Finishing: Run build
2020-03-07T14:54:24.6838782Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69799/merge to s
2020-03-07T14:54:24.6845781Z Task         : Get sources
2020-03-07T14:54:24.6846181Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-07T14:54:24.6846548Z Version      : 1.0.0
2020-03-07T14:54:24.6846826Z Author       : Microsoft
2020-03-07T14:54:24.6846826Z Author       : Microsoft
2020-03-07T14:54:24.6847233Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-07T14:54:24.6847700Z ==============================================================================
2020-03-07T14:54:25.0365141Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-07T14:54:25.0425128Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69799/merge to s
2020-03-07T14:54:25.0525947Z Cleaning up task key
2020-03-07T14:54:25.0527679Z Start cleaning up orphan processes.
2020-03-07T14:54:25.0791647Z Terminate orphan process: pid (3680) (python)
2020-03-07T14:54:25.0963036Z ##[section]Finishing: Finalize Job
