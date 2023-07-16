plain
2020-01-31T13:37:19.7465639Z ========================== Starting Command Output ===========================
2020-01-31T13:37:19.7469338Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/8f7619dc-e4ce-40b3-814d-bc886f8689ef.sh
2020-01-31T13:37:19.7469452Z 
2020-01-31T13:37:19.7474049Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-31T13:37:19.7481127Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68700/merge to s
2020-01-31T13:37:19.7483103Z Task         : Get sources
2020-01-31T13:37:19.7483141Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-31T13:37:19.7483179Z Version      : 1.0.0
2020-01-31T13:37:19.7483277Z Author       : Microsoft
---
2020-01-31T13:37:20.7622183Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-31T13:37:20.7720875Z ##[command]git config gc.auto 0
2020-01-31T13:37:20.7803007Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-31T13:37:20.7863980Z ##[command]git config --get-all http.proxy
2020-01-31T13:37:20.8002090Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68700/merge:refs/remotes/pull/68700/merge
---
2020-01-31T13:42:29.8322883Z     Checking panic_abort v0.0.0 (/checkout/src/libpanic_abort)
2020-01-31T13:42:29.9898649Z     Checking backtrace v0.3.40
2020-01-31T13:42:31.1017403Z     Checking rustc-std-workspace-alloc v1.99.0 (/checkout/src/tools/rustc-std-workspace-alloc)
2020-01-31T13:42:31.1018368Z     Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
2020-01-31T13:42:32.5862517Z error[E0658]: use of unstable library feature 'wake_trait'
2020-01-31T13:42:32.5864437Z     |
2020-01-31T13:42:32.5865385Z 479 |     pub use alloc::task::*;
2020-01-31T13:42:32.5866158Z     |             ^^^^^^^^^^^
2020-01-31T13:42:32.5866792Z     |
2020-01-31T13:42:32.5866792Z     |
2020-01-31T13:42:32.5867488Z     = help: add `#![feature(wake_trait)]` to the crate attributes to enable
2020-01-31T13:42:34.3183378Z error: aborting due to previous error
2020-01-31T13:42:34.3183536Z 
2020-01-31T13:42:34.3184007Z For more information about this error, try `rustc --explain E0658`.
2020-01-31T13:42:34.3286838Z error: could not compile `std`.
---
2020-01-31T13:42:34.3369593Z   local time: Fri Jan 31 13:42:34 UTC 2020
2020-01-31T13:42:34.6259037Z   network time: Fri, 31 Jan 2020 13:42:34 GMT
2020-01-31T13:42:34.6260389Z == end clock drift check ==
2020-01-31T13:42:35.6921663Z 
2020-01-31T13:42:35.6979058Z ##[error]Bash exited with code '1'.
2020-01-31T13:42:35.7001634Z ##[section]Finishing: Run build
2020-01-31T13:42:35.7020068Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68700/merge to s
2020-01-31T13:42:35.7022598Z Task         : Get sources
2020-01-31T13:42:35.7022644Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-31T13:42:35.7022714Z Version      : 1.0.0
2020-01-31T13:42:35.7022869Z Author       : Microsoft
2020-01-31T13:42:35.7022869Z Author       : Microsoft
2020-01-31T13:42:35.7022915Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-31T13:42:35.7022988Z ==============================================================================
2020-01-31T13:42:36.1698244Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-31T13:42:36.1736680Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68700/merge to s
2020-01-31T13:42:36.1848064Z Cleaning up task key
2020-01-31T13:42:36.1849172Z Start cleaning up orphan processes.
2020-01-31T13:42:36.1951436Z Terminate orphan process: pid (4404) (python)
2020-01-31T13:42:36.2173497Z ##[section]Finishing: Finalize Job
