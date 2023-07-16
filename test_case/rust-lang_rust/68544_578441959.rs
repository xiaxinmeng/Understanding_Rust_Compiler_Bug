plain
2020-01-25T20:36:06.6972935Z ========================== Starting Command Output ===========================
2020-01-25T20:36:06.6975049Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/765d8a0c-9d29-4c94-bfdf-232222b3385e.sh
2020-01-25T20:36:06.6975092Z 
2020-01-25T20:36:06.6979513Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-25T20:36:06.6986405Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68544/merge to s
2020-01-25T20:36:06.6988172Z Task         : Get sources
2020-01-25T20:36:06.6988263Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-25T20:36:06.6988299Z Version      : 1.0.0
2020-01-25T20:36:06.6988333Z Author       : Microsoft
---
2020-01-25T20:36:07.7338835Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-25T20:36:07.7353550Z ##[command]git config gc.auto 0
2020-01-25T20:36:07.7357904Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-25T20:36:07.7362856Z ##[command]git config --get-all http.proxy
2020-01-25T20:36:07.7373037Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68544/merge:refs/remotes/pull/68544/merge
---
2020-01-25T20:41:47.4446438Z     Finished release [optimized] target(s) in 1m 21s
2020-01-25T20:41:47.4613564Z tidy check
2020-01-25T20:41:48.9048385Z * 589 error codes
2020-01-25T20:41:48.9048989Z * highest error code: E0746
2020-01-25T20:41:48.9557450Z tidy error: /checkout/src/librustc_feature/removed.rs:112: feature overlapping_marker_traits is not sorted by "since" (version number)
2020-01-25T20:41:50.0351950Z Stray file with UI testing output: "/checkout/src/test/ui/overlap-marker-trait.stderr"
2020-01-25T20:41:50.0626201Z Stray file with UI testing output: "/checkout/src/test/ui/traits/overlap-permitted-for-marker-traits-neg.stderr"
2020-01-25T20:41:50.2888849Z some tidy checks failed
2020-01-25T20:41:50.2889801Z Found 487 error codes
2020-01-25T20:41:50.2890096Z Found 0 error codes with no tests
2020-01-25T20:41:50.2890380Z Done!
2020-01-25T20:41:50.2890380Z Done!
2020-01-25T20:41:50.2895259Z 
2020-01-25T20:41:50.2895599Z 
2020-01-25T20:41:50.2896851Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-01-25T20:41:50.2897593Z 
2020-01-25T20:41:50.2897859Z 
2020-01-25T20:41:50.2905508Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-01-25T20:41:50.2905951Z Build completed unsuccessfully in 0:01:33
2020-01-25T20:41:50.2905951Z Build completed unsuccessfully in 0:01:33
2020-01-25T20:41:50.2958456Z == clock drift check ==
2020-01-25T20:41:50.2967607Z   local time: Sat Jan 25 20:41:50 UTC 2020
2020-01-25T20:41:50.5898119Z   network time: Sat, 25 Jan 2020 20:41:50 GMT
2020-01-25T20:41:50.5898229Z == end clock drift check ==
2020-01-25T20:41:51.3366179Z 
2020-01-25T20:41:51.3452403Z ##[error]Bash exited with code '1'.
2020-01-25T20:41:51.3464194Z ##[section]Finishing: Run build
2020-01-25T20:41:51.3479674Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68544/merge to s
2020-01-25T20:41:51.3481371Z Task         : Get sources
2020-01-25T20:41:51.3481413Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-25T20:41:51.3481454Z Version      : 1.0.0
2020-01-25T20:41:51.3481519Z Author       : Microsoft
2020-01-25T20:41:51.3481519Z Author       : Microsoft
2020-01-25T20:41:51.3481561Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-25T20:41:51.3481605Z ==============================================================================
2020-01-25T20:41:51.7860045Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-25T20:41:51.7908695Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68544/merge to s
2020-01-25T20:41:51.8018632Z Cleaning up task key
2020-01-25T20:41:51.8019359Z Start cleaning up orphan processes.
2020-01-25T20:41:51.8129865Z Terminate orphan process: pid (3567) (python)
2020-01-25T20:41:51.8365529Z ##[section]Finishing: Finalize Job
