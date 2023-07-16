plain
2019-12-04T17:40:40.5404322Z ========================== Starting Command Output ===========================
2019-12-04T17:40:40.5408776Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/0314807d-fecc-427b-b2a0-90da981d737b.sh
2019-12-04T17:40:40.5409019Z 
2019-12-04T17:40:40.5412166Z ##[section]Finishing: Disable git automatic line ending conversion
2019-12-04T17:40:40.5419014Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67029/merge to s
2019-12-04T17:40:40.5420704Z Task         : Get sources
2019-12-04T17:40:40.5420782Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2019-12-04T17:40:40.5420811Z Version      : 1.0.0
2019-12-04T17:40:40.5420838Z Author       : Microsoft
---
2019-12-04T17:40:43.2331289Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-12-04T17:40:43.2533036Z ##[command]git config gc.auto 0
2019-12-04T17:40:43.2626950Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-12-04T17:40:43.2692962Z ##[command]git config --get-all http.proxy
2019-12-04T17:40:43.2863066Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67029/merge:refs/remotes/pull/67029/merge
---
2019-12-04T17:44:04.6157599Z #############################################################             85.2%
2019-12-04T17:44:04.6158648Z ######################################################################## 100.0%
2019-12-04T17:44:05.0153951Z extracting /checkout/obj/build/cache/2019-11-06/cargo-beta-x86_64-unknown-linux-gnu.tar.gz
2019-12-04T17:44:05.0978721Z     Updating crates.io index
2019-12-04T17:44:12.9819264Z     Updating git repository `https://github.com/spastorino/rustc-rayon`
---
2019-12-04T17:47:03.6544523Z * highest error code: E0745
2019-12-04T17:47:04.0442383Z * 273 features
2019-12-04T17:47:04.7397390Z Dependencies not on the whitelist:
2019-12-04T17:47:04.7398236Z * crossbeam-channel 
2019-12-04T17:47:04.7404996Z invalid source: "git+https://github.com/spastorino/rustc-rayon?branch=latch-target-thread-rustc#dd85f978633ae564515b7d1c82ba3d1cff76b1f1"
2019-12-04T17:47:04.7405381Z invalid source: "git+https://github.com/spastorino/rustc-rayon?branch=latch-target-thread-rustc#dd85f978633ae564515b7d1c82ba3d1cff76b1f1"
2019-12-04T17:47:05.0093289Z some tidy checks failed
2019-12-04T17:47:05.0093952Z Found 486 error codes
2019-12-04T17:47:05.0094455Z Found 0 error codes with no tests
2019-12-04T17:47:05.0094534Z Done!
2019-12-04T17:47:05.0094534Z Done!
2019-12-04T17:47:05.0099547Z 
2019-12-04T17:47:05.0099824Z 
2019-12-04T17:47:05.0101456Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2019-12-04T17:47:05.0106712Z 
2019-12-04T17:47:05.0106775Z 
2019-12-04T17:47:05.0117387Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2019-12-04T17:47:05.0117791Z Build completed unsuccessfully in 0:01:33
2019-12-04T17:47:05.0117791Z Build completed unsuccessfully in 0:01:33
2019-12-04T17:47:05.0176582Z == clock drift check ==
2019-12-04T17:47:05.0188541Z   local time: Wed Dec  4 17:47:05 UTC 2019
2019-12-04T17:47:05.2993760Z   network time: Wed, 04 Dec 2019 17:47:05 GMT
2019-12-04T17:47:05.2998152Z == end clock drift check ==
2019-12-04T17:47:06.6781634Z 
2019-12-04T17:47:06.6900382Z ##[error]Bash exited with code '1'.
2019-12-04T17:47:06.6913686Z ##[section]Finishing: Run build
2019-12-04T17:47:06.6929814Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67029/merge to s
2019-12-04T17:47:06.6931755Z Task         : Get sources
2019-12-04T17:47:06.6931796Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2019-12-04T17:47:06.6931850Z Version      : 1.0.0
2019-12-04T17:47:06.6931885Z Author       : Microsoft
2019-12-04T17:47:06.6931885Z Author       : Microsoft
2019-12-04T17:47:06.6931928Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2019-12-04T17:47:06.6931984Z ==============================================================================
2019-12-04T17:47:07.1380317Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2019-12-04T17:47:07.1438623Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/67029/merge to s
2019-12-04T17:47:07.1558388Z Start cleaning up orphan processes.
2019-12-04T17:47:07.1688085Z Terminate orphan process: pid (3831) (python)
2019-12-04T17:47:07.2054217Z ##[section]Finishing: Finalize Job
2019-12-04T17:47:07.2120055Z ##[section]Finishing: Linux x86_64-gnu-llvm-7
