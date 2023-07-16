plain
2020-01-27T18:13:48.7645204Z ========================== Starting Command Output ===========================
2020-01-27T18:13:48.7647183Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/78cd13aa-bd0b-4273-8a6a-cd436fd5d96f.sh
2020-01-27T18:13:48.7647348Z 
2020-01-27T18:13:48.7650329Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-27T18:13:48.7656240Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68558/merge to s
2020-01-27T18:13:48.7658077Z Task         : Get sources
2020-01-27T18:13:48.7658113Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-27T18:13:48.7658148Z Version      : 1.0.0
2020-01-27T18:13:48.7658182Z Author       : Microsoft
---
2020-01-27T18:13:49.5764661Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-27T18:13:49.5863031Z ##[command]git config gc.auto 0
2020-01-27T18:13:49.5937764Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-27T18:13:49.5986491Z ##[command]git config --get-all http.proxy
2020-01-27T18:13:49.6121130Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68558/merge:refs/remotes/pull/68558/merge
---
2020-01-27T18:18:27.3040115Z     Checking panic_abort v0.0.0 (/checkout/src/libpanic_abort)
2020-01-27T18:18:27.4602453Z     Checking backtrace v0.3.40
2020-01-27T18:18:28.6163799Z     Checking rustc-std-workspace-alloc v1.99.0 (/checkout/src/tools/rustc-std-workspace-alloc)
2020-01-27T18:18:28.6168532Z     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2020-01-27T18:18:30.6035364Z error[E0599]: no method named `capacity` found for type `alloc_crate::boxed::Box<[u8]>` in the current scope
2020-01-27T18:18:30.6037974Z     |
2020-01-27T18:18:30.6038833Z 202 |         self.buf.capacity()
2020-01-27T18:18:30.6039856Z     |                  ^^^^^^^^ method not found in `alloc_crate::boxed::Box<[u8]>`
2020-01-27T18:18:30.6040245Z 
---
2020-01-27T18:18:31.4724600Z   local time: Mon Jan 27 18:18:31 UTC 2020
2020-01-27T18:18:31.7601459Z   network time: Mon, 27 Jan 2020 18:18:31 GMT
2020-01-27T18:18:31.7601601Z == end clock drift check ==
2020-01-27T18:18:32.7773303Z 
2020-01-27T18:18:32.7835219Z ##[error]Bash exited with code '1'.
2020-01-27T18:18:32.7846882Z ##[section]Finishing: Run build
2020-01-27T18:18:32.7862614Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68558/merge to s
2020-01-27T18:18:32.7864536Z Task         : Get sources
2020-01-27T18:18:32.7864587Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-27T18:18:32.7864638Z Version      : 1.0.0
2020-01-27T18:18:32.7864701Z Author       : Microsoft
2020-01-27T18:18:32.7864701Z Author       : Microsoft
2020-01-27T18:18:32.7864750Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-27T18:18:32.7864804Z ==============================================================================
2020-01-27T18:18:33.1708580Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-27T18:18:33.1756445Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68558/merge to s
2020-01-27T18:18:33.1866937Z Cleaning up task key
2020-01-27T18:18:33.1867863Z Start cleaning up orphan processes.
2020-01-27T18:18:33.1975570Z Terminate orphan process: pid (3455) (python)
2020-01-27T18:18:33.2162838Z ##[section]Finishing: Finalize Job
