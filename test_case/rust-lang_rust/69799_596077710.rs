plain
2020-03-07T11:13:27.8253265Z ========================== Starting Command Output ===========================
2020-03-07T11:13:27.8255760Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/3439abe8-1922-4590-afb0-ec26d3cc22d9.sh
2020-03-07T11:13:27.8255987Z 
2020-03-07T11:13:27.8259884Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-07T11:13:27.8275893Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69799/merge to s
2020-03-07T11:13:27.8278882Z Task         : Get sources
2020-03-07T11:13:27.8279140Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-07T11:13:27.8279387Z Version      : 1.0.0
2020-03-07T11:13:27.8279568Z Author       : Microsoft
---
2020-03-07T11:13:29.1460579Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-07T11:13:29.1482302Z ##[command]git config gc.auto 0
2020-03-07T11:13:29.1486699Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-07T11:13:29.1490178Z ##[command]git config --get-all http.proxy
2020-03-07T11:13:29.1499555Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69799/merge:refs/remotes/pull/69799/merge
---
2020-03-07T11:17:47.9834647Z     Checking alloc v0.0.0 (/checkout/src/liballoc)
2020-03-07T11:17:48.5797017Z error[E0599]: no method named `dangling` found for struct `core::alloc::Layout` in the current scope
2020-03-07T11:17:48.5797856Z    --> src/liballoc/alloc.rs:170:24
2020-03-07T11:17:48.5798309Z     |
2020-03-07T11:17:48.5798829Z 170 |             Ok((layout.dangling(), 0))
2020-03-07T11:17:48.5800519Z 
2020-03-07T11:17:48.5847514Z error[E0599]: no method named `dangling` found for struct `core::alloc::Layout` in the current scope
2020-03-07T11:17:48.5848259Z    --> src/liballoc/alloc.rs:191:34
2020-03-07T11:17:48.5848721Z     |
2020-03-07T11:17:48.5848721Z     |
2020-03-07T11:17:48.5849445Z 191 |             (0, 0) => Ok((layout.dangling(), 0)),
2020-03-07T11:17:48.5850857Z 
2020-03-07T11:17:48.5879787Z error[E0599]: no method named `dangling` found for struct `core::alloc::Layout` in the current scope
2020-03-07T11:17:48.5881146Z    --> src/liballoc/alloc.rs:195:28
2020-03-07T11:17:48.5881616Z     |
2020-03-07T11:17:48.5881616Z     |
2020-03-07T11:17:48.5882182Z 195 |                 Ok((layout.dangling(), 0))
2020-03-07T11:17:48.5883847Z 
2020-03-07T11:17:48.5916293Z error[E0599]: no method named `dangling` found for struct `core::alloc::Layout` in the current scope
2020-03-07T11:17:48.5917683Z    --> src/liballoc/alloc.rs:206:24
2020-03-07T11:17:48.5918153Z     |
2020-03-07T11:17:48.5918153Z     |
2020-03-07T11:17:48.5918685Z 206 |             Ok((layout.dangling(), 0))
2020-03-07T11:17:48.5919989Z 
2020-03-07T11:17:48.7002930Z     Checking rustc-demangle v0.1.16
2020-03-07T11:17:48.9594474Z     Checking panic_abort v0.0.0 (/checkout/src/libpanic_abort)
2020-03-07T11:17:49.1045509Z     Checking backtrace v0.3.44
---
2020-03-07T11:17:49.2896975Z   local time: Sat Mar  7 11:17:49 UTC 2020
2020-03-07T11:17:49.5679688Z   network time: Sat, 07 Mar 2020 11:17:49 GMT
2020-03-07T11:17:49.5682715Z == end clock drift check ==
2020-03-07T11:17:50.6940928Z 
2020-03-07T11:17:50.7014289Z ##[error]Bash exited with code '1'.
2020-03-07T11:17:50.7026457Z ##[section]Finishing: Run build
2020-03-07T11:17:50.7098973Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69799/merge to s
2020-03-07T11:17:50.7103492Z Task         : Get sources
2020-03-07T11:17:50.7103814Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-07T11:17:50.7104597Z Version      : 1.0.0
2020-03-07T11:17:50.7104848Z Author       : Microsoft
2020-03-07T11:17:50.7104848Z Author       : Microsoft
2020-03-07T11:17:50.7105179Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-07T11:17:50.7105563Z ==============================================================================
2020-03-07T11:17:51.0006916Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-07T11:17:51.0050418Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69799/merge to s
2020-03-07T11:17:51.0125076Z Cleaning up task key
2020-03-07T11:17:51.0126196Z Start cleaning up orphan processes.
2020-03-07T11:17:51.0285311Z Terminate orphan process: pid (4130) (python)
2020-03-07T11:17:51.0412150Z ##[section]Finishing: Finalize Job
