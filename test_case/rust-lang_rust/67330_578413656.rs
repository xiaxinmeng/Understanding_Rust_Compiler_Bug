plain
2020-01-25T14:48:56.9264782Z ========================== Starting Command Output ===========================
2020-01-25T14:48:56.9266823Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/6dcc6433-73bb-48e8-a8ce-59c7b5721b16.sh
2020-01-25T14:48:56.9267037Z 
2020-01-25T14:48:56.9269920Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-25T14:48:56.9277300Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67330/merge to s
2020-01-25T14:48:56.9279113Z Task         : Get sources
2020-01-25T14:48:56.9279150Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-25T14:48:56.9279237Z Version      : 1.0.0
2020-01-25T14:48:56.9279273Z Author       : Microsoft
---
2020-01-25T14:48:57.9660813Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-25T14:48:57.9675455Z ##[command]git config gc.auto 0
2020-01-25T14:48:57.9678823Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-25T14:48:57.9723236Z ##[command]git config --get-all http.proxy
2020-01-25T14:48:57.9798445Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/67330/merge:refs/remotes/pull/67330/merge
---
2020-01-25T14:54:53.6867558Z tidy error: /checkout/src/libcore/str/mod.rs:1212: undocumented unsafe
2020-01-25T14:54:53.6867899Z tidy error: /checkout/src/libcore/str/mod.rs:1217: undocumented unsafe
2020-01-25T14:54:54.2778191Z thread 'main' panicked at 'assertion failed: `(left != right)`
2020-01-25T14:54:54.2778372Z   left: `0`,
2020-01-25T14:54:54.2778752Z  right: `0`: "none" should be used when there is no issue, not "0"', src/tools/tidy/src/features.rs:417:21
2020-01-25T14:54:54.2778964Z 
2020-01-25T14:54:54.2778998Z 
2020-01-25T14:54:54.2778998Z 
2020-01-25T14:54:54.2779607Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-01-25T14:54:54.2779741Z 
2020-01-25T14:54:54.2779771Z 
2020-01-25T14:54:54.2787657Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-01-25T14:54:54.2787774Z Build completed unsuccessfully in 0:01:33
2020-01-25T14:54:54.2787774Z Build completed unsuccessfully in 0:01:33
2020-01-25T14:54:54.2841067Z == clock drift check ==
2020-01-25T14:54:54.2847922Z   local time: Sat Jan 25 14:54:54 UTC 2020
2020-01-25T14:54:54.5818513Z   network time: Sat, 25 Jan 2020 14:54:54 GMT
2020-01-25T14:54:54.5818624Z == end clock drift check ==
2020-01-25T14:54:55.3337488Z 
2020-01-25T14:54:55.3454773Z ##[error]Bash exited with code '1'.
2020-01-25T14:54:55.3469274Z ##[section]Finishing: Run build
2020-01-25T14:54:55.3485852Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/67330/merge to s
2020-01-25T14:54:55.3488309Z Task         : Get sources
2020-01-25T14:54:55.3488362Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-25T14:54:55.3488413Z Version      : 1.0.0
2020-01-25T14:54:55.3488475Z Author       : Microsoft
2020-01-25T14:54:55.3488475Z Author       : Microsoft
2020-01-25T14:54:55.3488527Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-25T14:54:55.3488730Z ==============================================================================
2020-01-25T14:54:55.8142132Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-25T14:54:55.8187669Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/67330/merge to s
2020-01-25T14:54:55.8312848Z Cleaning up task key
2020-01-25T14:54:55.8313581Z Start cleaning up orphan processes.
2020-01-25T14:54:55.8436100Z Terminate orphan process: pid (3808) (python)
2020-01-25T14:54:55.8665375Z ##[section]Finishing: Finalize Job
