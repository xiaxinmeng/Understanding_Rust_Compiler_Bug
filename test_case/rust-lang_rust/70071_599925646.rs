plain
2020-03-17T07:31:27.2154332Z ========================== Starting Command Output ===========================
2020-03-17T07:31:27.2156839Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/930ab6df-6791-4925-938c-93fd99bf4c08.sh
2020-03-17T07:31:27.2157101Z 
2020-03-17T07:31:27.2161439Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-17T07:31:27.2191222Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70071/merge to s
2020-03-17T07:31:27.2194488Z Task         : Get sources
2020-03-17T07:31:27.2194788Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-17T07:31:27.2195065Z Version      : 1.0.0
2020-03-17T07:31:27.2195251Z Author       : Microsoft
---
2020-03-17T07:31:30.5113102Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-17T07:31:30.5120389Z ##[command]git config gc.auto 0
2020-03-17T07:31:30.5125014Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-17T07:31:30.5128437Z ##[command]git config --get-all http.proxy
2020-03-17T07:31:30.5136066Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/70071/merge:refs/remotes/pull/70071/merge
---
2020-03-17T07:38:26.1089751Z    Compiling serde_json v1.0.40
2020-03-17T07:38:27.6759475Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-03-17T07:38:37.5663419Z     Finished release [optimized] target(s) in 1m 20s
2020-03-17T07:38:37.5761809Z tidy check
2020-03-17T07:38:38.4511656Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0308.md:28: tab character
2020-03-17T07:38:38.4512202Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0308.md:30: tab character
2020-03-17T07:38:38.4512719Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0308.md:31: line longer than 80 chars
2020-03-17T07:38:38.4513276Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0308.md:31: tab character
2020-03-17T07:38:38.4513764Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0308.md:54: tab character
2020-03-17T07:38:38.4514251Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0308.md:55: tab character
2020-03-17T07:38:38.4514755Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0308.md:57: tab character
2020-03-17T07:38:38.4515494Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0308.md:58: tab character
2020-03-17T07:38:40.3460239Z Found 489 error codes
2020-03-17T07:38:40.3460553Z Found 0 error codes with no tests
2020-03-17T07:38:40.3460814Z Done!
2020-03-17T07:38:40.3460999Z some tidy checks failed
2020-03-17T07:38:40.3460999Z some tidy checks failed
2020-03-17T07:38:40.3461156Z 
2020-03-17T07:38:40.3461266Z 
2020-03-17T07:38:40.3462956Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-03-17T07:38:40.3464288Z 
2020-03-17T07:38:40.3464418Z 
2020-03-17T07:38:40.3470791Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-03-17T07:38:40.3471208Z Build completed unsuccessfully in 0:01:31
2020-03-17T07:38:40.3471208Z Build completed unsuccessfully in 0:01:31
2020-03-17T07:38:40.3520768Z == clock drift check ==
2020-03-17T07:38:40.3547163Z   local time: Tue Mar 17 07:38:40 UTC 2020
2020-03-17T07:38:40.6423572Z   network time: Tue, 17 Mar 2020 07:38:40 GMT
2020-03-17T07:38:40.6431020Z == end clock drift check ==
2020-03-17T07:38:41.4188617Z 
2020-03-17T07:38:41.4282488Z ##[error]Bash exited with code '1'.
2020-03-17T07:38:41.4296753Z ##[section]Finishing: Run build
2020-03-17T07:38:41.4370679Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/70071/merge to s
2020-03-17T07:38:41.4376614Z Task         : Get sources
2020-03-17T07:38:41.4377101Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-17T07:38:41.4377554Z Version      : 1.0.0
2020-03-17T07:38:41.4377871Z Author       : Microsoft
2020-03-17T07:38:41.4377871Z Author       : Microsoft
2020-03-17T07:38:41.4378355Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-17T07:38:41.4378951Z ==============================================================================
2020-03-17T07:38:41.7895297Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-17T07:38:41.7952651Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/70071/merge to s
2020-03-17T07:38:41.8049591Z Cleaning up task key
2020-03-17T07:38:41.8051158Z Start cleaning up orphan processes.
2020-03-17T07:38:41.8395702Z Terminate orphan process: pid (3948) (python)
2020-03-17T07:38:41.8433568Z ##[section]Finishing: Finalize Job
