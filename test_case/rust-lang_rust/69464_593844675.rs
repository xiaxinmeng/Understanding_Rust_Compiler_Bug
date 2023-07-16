plain
2020-03-03T09:08:37.7187851Z ========================== Starting Command Output ===========================
2020-03-03T09:08:37.7190458Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/d8177096-2b59-4285-be6e-0df8f07bd4fe.sh
2020-03-03T09:08:37.7191052Z 
2020-03-03T09:08:37.7194486Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-03T09:08:37.7213948Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69464/merge to s
2020-03-03T09:08:37.7217317Z Task         : Get sources
2020-03-03T09:08:37.7217645Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-03T09:08:37.7217961Z Version      : 1.0.0
2020-03-03T09:08:37.7218170Z Author       : Microsoft
---
2020-03-03T09:08:38.7131737Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-03T09:08:38.7140512Z ##[command]git config gc.auto 0
2020-03-03T09:08:38.7146750Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-03T09:08:38.7152770Z ##[command]git config --get-all http.proxy
2020-03-03T09:08:38.7162763Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69464/merge:refs/remotes/pull/69464/merge
---
2020-03-03T09:11:44.1492676Z downloading https://static.rust-lang.org/dist/2020-01-31/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.gz
2020-03-03T09:11:44.3837898Z 
2020-03-03T09:11:44.3838798Z ######################################################################## 100.0%
2020-03-03T09:11:44.6937285Z extracting /checkout/obj/build/cache/2020-01-31/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.gz
2020-03-03T09:11:44.7656580Z     Updating git repository `https://github.com/Marwes/ena`
2020-03-03T09:12:03.2776873Z error: failed to write /checkout/Cargo.lock
2020-03-03T09:12:03.2777644Z 
2020-03-03T09:12:03.2777939Z Caused by:
2020-03-03T09:12:03.2778305Z   failed to open: /checkout/Cargo.lock
---
2020-03-03T09:12:03.2862868Z   local time: Tue Mar  3 09:12:03 UTC 2020
2020-03-03T09:12:03.8188833Z   network time: Tue, 03 Mar 2020 09:12:03 GMT
2020-03-03T09:12:03.8196554Z == end clock drift check ==
2020-03-03T09:12:04.3077084Z 
2020-03-03T09:12:04.3150054Z ##[error]Bash exited with code '1'.
2020-03-03T09:12:04.3163991Z ##[section]Finishing: Run build
2020-03-03T09:12:04.3209142Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69464/merge to s
2020-03-03T09:12:04.3214033Z Task         : Get sources
2020-03-03T09:12:04.3214415Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-03T09:12:04.3214751Z Version      : 1.0.0
2020-03-03T09:12:04.3214987Z Author       : Microsoft
2020-03-03T09:12:04.3214987Z Author       : Microsoft
2020-03-03T09:12:04.3215511Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-03T09:12:04.3215936Z ==============================================================================
2020-03-03T09:12:04.6459148Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-03T09:12:04.6463962Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69464/merge to s
2020-03-03T09:12:04.6550709Z Cleaning up task key
2020-03-03T09:12:04.6552292Z Start cleaning up orphan processes.
2020-03-03T09:12:04.6830707Z Terminate orphan process: pid (4428) (python)
2020-03-03T09:12:04.6851666Z ##[section]Finishing: Finalize Job
