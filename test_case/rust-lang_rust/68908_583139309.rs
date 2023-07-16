plain
2020-02-06T22:00:43.0241734Z ========================== Starting Command Output ===========================
2020-02-06T22:00:43.0244088Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/e008cd75-a38d-4b26-9723-3ef98d2e9247.sh
2020-02-06T22:00:43.0244238Z 
2020-02-06T22:00:43.0247512Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-06T22:00:43.0253305Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68908/merge to s
2020-02-06T22:00:43.0254921Z Task         : Get sources
2020-02-06T22:00:43.0254998Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-06T22:00:43.0255033Z Version      : 1.0.0
2020-02-06T22:00:43.0255067Z Author       : Microsoft
---
2020-02-06T22:00:43.9946393Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-06T22:00:44.0024124Z ##[command]git config gc.auto 0
2020-02-06T22:00:44.0089903Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-06T22:00:44.0140275Z ##[command]git config --get-all http.proxy
2020-02-06T22:00:44.0303479Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68908/merge:refs/remotes/pull/68908/merge
---
2020-02-06T22:06:54.4916986Z    Compiling serde_json v1.0.40
2020-02-06T22:06:56.0445209Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-02-06T22:07:05.0677321Z     Finished release [optimized] target(s) in 1m 22s
2020-02-06T22:07:05.0768558Z tidy check
2020-02-06T22:07:05.5124082Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0637.md:3: trailing whitespace
2020-02-06T22:07:05.5124304Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0637.md: leading newline
2020-02-06T22:07:05.5124472Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0637.md: too many trailing newlines (2)
2020-02-06T22:07:07.5615899Z some tidy checks failed
2020-02-06T22:07:07.5621034Z Found 487 error codes
2020-02-06T22:07:07.5621462Z Found 0 error codes with no tests
2020-02-06T22:07:07.5621618Z Done!
2020-02-06T22:07:07.5621618Z Done!
2020-02-06T22:07:07.5621844Z 
2020-02-06T22:07:07.5621964Z 
2020-02-06T22:07:07.5623011Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-02-06T22:07:07.5623362Z 
2020-02-06T22:07:07.5623478Z 
2020-02-06T22:07:07.5630618Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-02-06T22:07:07.5630710Z Build completed unsuccessfully in 0:01:31
2020-02-06T22:07:07.5630710Z Build completed unsuccessfully in 0:01:31
2020-02-06T22:07:07.5686004Z == clock drift check ==
2020-02-06T22:07:07.5696231Z   local time: Thu Feb  6 22:07:07 UTC 2020
2020-02-06T22:07:07.7303875Z   network time: Thu, 06 Feb 2020 22:07:07 GMT
2020-02-06T22:07:07.7309503Z == end clock drift check ==
2020-02-06T22:07:08.4716778Z 
2020-02-06T22:07:08.4806296Z ##[error]Bash exited with code '1'.
2020-02-06T22:07:08.4817756Z ##[section]Finishing: Run build
2020-02-06T22:07:08.4831796Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68908/merge to s
2020-02-06T22:07:08.4833565Z Task         : Get sources
2020-02-06T22:07:08.4833615Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-06T22:07:08.4833665Z Version      : 1.0.0
2020-02-06T22:07:08.4833723Z Author       : Microsoft
2020-02-06T22:07:08.4833723Z Author       : Microsoft
2020-02-06T22:07:08.4833775Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-06T22:07:08.4833843Z ==============================================================================
2020-02-06T22:07:08.8715531Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-06T22:07:08.8763732Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68908/merge to s
2020-02-06T22:07:08.8868461Z Cleaning up task key
2020-02-06T22:07:08.8869511Z Start cleaning up orphan processes.
2020-02-06T22:07:08.8965857Z Terminate orphan process: pid (4155) (python)
2020-02-06T22:07:08.9144204Z ##[section]Finishing: Finalize Job
