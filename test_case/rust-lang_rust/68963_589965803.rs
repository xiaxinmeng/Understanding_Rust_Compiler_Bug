plain
2020-02-22T15:02:52.8628321Z ========================== Starting Command Output ===========================
2020-02-22T15:02:52.8647707Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/f5319de8-513c-4bf5-a9f1-f732c4aee6dc.sh
2020-02-22T15:02:52.8839084Z 
2020-02-22T15:02:52.8893504Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-22T15:02:52.8914065Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68963/merge to s
2020-02-22T15:02:52.8917174Z Task         : Get sources
2020-02-22T15:02:52.8917416Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-22T15:02:52.8917650Z Version      : 1.0.0
2020-02-22T15:02:52.8917853Z Author       : Microsoft
---
2020-02-22T15:02:53.6897410Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-22T15:02:53.6903863Z ##[command]git config gc.auto 0
2020-02-22T15:02:53.6906888Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-22T15:02:53.6909970Z ##[command]git config --get-all http.proxy
2020-02-22T15:02:53.6915517Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68963/merge:refs/remotes/pull/68963/merge
---
2020-02-22T15:08:37.0927637Z Done!
2020-02-22T15:08:37.0930363Z some tidy checks failed
2020-02-22T15:08:37.0933669Z 
2020-02-22T15:08:37.0933811Z 
2020-02-22T15:08:37.0937348Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-02-22T15:08:37.0941442Z 
2020-02-22T15:08:37.0941524Z 
2020-02-22T15:08:37.0954071Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-02-22T15:08:37.0954552Z Build completed unsuccessfully in 0:01:28
2020-02-22T15:08:37.0954552Z Build completed unsuccessfully in 0:01:28
2020-02-22T15:08:37.0999027Z == clock drift check ==
2020-02-22T15:08:37.1007617Z   local time: Sat Feb 22 15:08:37 UTC 2020
2020-02-22T15:08:37.6225480Z   network time: Sat, 22 Feb 2020 15:08:37 GMT
2020-02-22T15:08:37.6226445Z == end clock drift check ==
2020-02-22T15:08:38.3558780Z 
2020-02-22T15:08:38.3622084Z ##[error]Bash exited with code '1'.
2020-02-22T15:08:38.3642856Z ##[section]Finishing: Run build
2020-02-22T15:08:38.3679804Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68963/merge to s
2020-02-22T15:08:38.3683656Z Task         : Get sources
2020-02-22T15:08:38.3683925Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-22T15:08:38.3684176Z Version      : 1.0.0
2020-02-22T15:08:38.3684339Z Author       : Microsoft
2020-02-22T15:08:38.3684339Z Author       : Microsoft
2020-02-22T15:08:38.3684599Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-22T15:08:38.3684917Z ==============================================================================
2020-02-22T15:08:38.6561217Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-22T15:08:38.6609245Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68963/merge to s
2020-02-22T15:08:38.6706093Z Cleaning up task key
2020-02-22T15:08:38.6707159Z Start cleaning up orphan processes.
2020-02-22T15:08:38.6875024Z Terminate orphan process: pid (3490) (python)
2020-02-22T15:08:38.7010458Z ##[section]Finishing: Finalize Job
