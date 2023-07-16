plain
2020-02-25T16:51:54.5581801Z ========================== Starting Command Output ===========================
2020-02-25T16:51:54.5586161Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/a628b7a4-f95a-420e-b4f9-e0dfc4568beb.sh
2020-02-25T16:51:54.5586676Z 
2020-02-25T16:51:54.5592066Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-25T16:51:54.5613909Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69464/merge to s
2020-02-25T16:51:54.5617614Z Task         : Get sources
2020-02-25T16:51:54.5617954Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-25T16:51:54.5618282Z Version      : 1.0.0
2020-02-25T16:51:54.5618531Z Author       : Microsoft
---
2020-02-25T16:51:55.5415009Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-25T16:51:55.5436748Z ##[command]git config gc.auto 0
2020-02-25T16:51:55.5442017Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-25T16:51:55.5445942Z ##[command]git config --get-all http.proxy
2020-02-25T16:51:55.5453056Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69464/merge:refs/remotes/pull/69464/merge
---
2020-02-25T16:54:50.1037001Z 
2020-02-25T16:54:50.1302019Z ##########################################                                59.1%
2020-02-25T16:54:50.1304899Z ######################################################################## 100.0%
2020-02-25T16:54:50.4398454Z extracting /checkout/obj/build/cache/2020-01-31/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.gz
2020-02-25T16:54:50.5041017Z     Updating git repository `https://github.com/Marwes/ena`
2020-02-25T16:54:50.9078092Z error: failed to resolve patches for `https://github.com/rust-lang/crates.io-index`
2020-02-25T16:54:50.9078619Z Caused by:
2020-02-25T16:54:50.9078969Z   failed to load source for a dependency on `ena`
2020-02-25T16:54:50.9079180Z 
2020-02-25T16:54:50.9079315Z Caused by:
2020-02-25T16:54:50.9079315Z Caused by:
2020-02-25T16:54:50.9079603Z   Unable to update https://github.com/Marwes/ena?branch=detach_undo_log#da974876
2020-02-25T16:54:50.9080006Z Caused by:
2020-02-25T16:54:50.9080006Z Caused by:
2020-02-25T16:54:50.9080646Z   revspec 'da974876317c95a0fb797e45530cacf383fa873b' not found; class=Reference (4); code=NotFound (-3)
2020-02-25T16:54:50.9087905Z Build completed unsuccessfully in 0:00:12
2020-02-25T16:54:50.9152280Z == clock drift check ==
2020-02-25T16:54:50.9192886Z   local time: Tue Feb 25 16:54:50 UTC 2020
2020-02-25T16:54:51.0737367Z   network time: Tue, 25 Feb 2020 16:54:51 GMT
2020-02-25T16:54:51.0737367Z   network time: Tue, 25 Feb 2020 16:54:51 GMT
2020-02-25T16:54:51.0741587Z == end clock drift check ==
2020-02-25T16:54:58.6049682Z 
2020-02-25T16:54:58.6131197Z ##[error]Bash exited with code '1'.
2020-02-25T16:54:58.6150648Z ##[section]Finishing: Run build
2020-02-25T16:54:58.6198174Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69464/merge to s
2020-02-25T16:54:58.6203407Z Task         : Get sources
2020-02-25T16:54:58.6203740Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-25T16:54:58.6204062Z Version      : 1.0.0
2020-02-25T16:54:58.6204276Z Author       : Microsoft
2020-02-25T16:54:58.6204276Z Author       : Microsoft
2020-02-25T16:54:58.6204618Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-25T16:54:58.6205029Z ==============================================================================
2020-02-25T16:54:58.9871388Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-25T16:54:58.9917788Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69464/merge to s
2020-02-25T16:54:59.0004434Z Cleaning up task key
2020-02-25T16:54:59.0005633Z Start cleaning up orphan processes.
2020-02-25T16:54:59.0196655Z Terminate orphan process: pid (3706) (python)
2020-02-25T16:54:59.0326721Z ##[section]Finishing: Finalize Job
