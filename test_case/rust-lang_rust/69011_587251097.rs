plain
2020-02-18T02:46:38.6554615Z ========================== Starting Command Output ===========================
2020-02-18T02:46:38.6557639Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/e1f55b3f-f673-42c7-a7b8-407a4e04c40b.sh
2020-02-18T02:46:38.6557814Z 
2020-02-18T02:46:38.6560720Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-18T02:46:38.6567463Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69011/merge to s
2020-02-18T02:46:38.6569016Z Task         : Get sources
2020-02-18T02:46:38.6569045Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-18T02:46:38.6569073Z Version      : 1.0.0
2020-02-18T02:46:38.6569118Z Author       : Microsoft
---
2020-02-18T02:46:39.5557548Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-18T02:46:39.5639253Z ##[command]git config gc.auto 0
2020-02-18T02:46:39.5716505Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-18T02:46:39.5774122Z ##[command]git config --get-all http.proxy
2020-02-18T02:46:39.5947524Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69011/merge:refs/remotes/pull/69011/merge
---
2020-02-18T02:52:51.7687788Z    Compiling serde_json v1.0.40
2020-02-18T02:52:53.4113845Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-02-18T02:53:04.0637519Z     Finished release [optimized] target(s) in 1m 36s
2020-02-18T02:53:04.0725525Z tidy check
2020-02-18T02:53:04.4594737Z tidy error: /checkout/src/libcore/fmt/num.rs:330: undocumented unsafe
2020-02-18T02:53:04.4594818Z tidy error: /checkout/src/libcore/fmt/num.rs:341: undocumented unsafe
2020-02-18T02:53:04.4595296Z tidy error: /checkout/src/libcore/fmt/num.rs:350: undocumented unsafe
2020-02-18T02:53:04.4595436Z tidy error: /checkout/src/libcore/fmt/num.rs:355: undocumented unsafe
2020-02-18T02:53:04.4595480Z tidy error: /checkout/src/libcore/fmt/num.rs:367: undocumented unsafe
2020-02-18T02:53:06.5656466Z some tidy checks failed
2020-02-18T02:53:06.5656558Z Found 487 error codes
2020-02-18T02:53:06.5656603Z Found 0 error codes with no tests
2020-02-18T02:53:06.5656888Z Done!
2020-02-18T02:53:06.5656888Z Done!
2020-02-18T02:53:06.5656915Z 
2020-02-18T02:53:06.5657116Z 
2020-02-18T02:53:06.5658455Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-02-18T02:53:06.5658582Z 
2020-02-18T02:53:06.5658607Z 
2020-02-18T02:53:06.5667489Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-02-18T02:53:06.5667602Z Build completed unsuccessfully in 0:01:47
2020-02-18T02:53:06.5667602Z Build completed unsuccessfully in 0:01:47
2020-02-18T02:53:06.5715611Z == clock drift check ==
2020-02-18T02:53:06.5724244Z   local time: Tue Feb 18 02:53:06 UTC 2020
2020-02-18T02:53:06.8571491Z   network time: Tue, 18 Feb 2020 02:53:06 GMT
2020-02-18T02:53:06.8576351Z == end clock drift check ==
2020-02-18T02:53:07.6202638Z 
2020-02-18T02:53:07.6306625Z ##[error]Bash exited with code '1'.
2020-02-18T02:53:07.6319775Z ##[section]Finishing: Run build
2020-02-18T02:53:07.6336259Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69011/merge to s
2020-02-18T02:53:07.6337976Z Task         : Get sources
2020-02-18T02:53:07.6338036Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-18T02:53:07.6338079Z Version      : 1.0.0
2020-02-18T02:53:07.6338117Z Author       : Microsoft
2020-02-18T02:53:07.6338117Z Author       : Microsoft
2020-02-18T02:53:07.6338178Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-18T02:53:07.6338241Z ==============================================================================
2020-02-18T02:53:08.0489727Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-18T02:53:08.0531349Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69011/merge to s
2020-02-18T02:53:08.0641724Z Cleaning up task key
2020-02-18T02:53:08.0642975Z Start cleaning up orphan processes.
2020-02-18T02:53:08.0747635Z Terminate orphan process: pid (4892) (python)
2020-02-18T02:53:08.0944435Z ##[section]Finishing: Finalize Job
