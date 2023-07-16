plain
2020-01-21T23:11:56.1184654Z ========================== Starting Command Output ===========================
2020-01-21T23:11:56.1187464Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/a3ac8a1c-1c0b-45ae-a72c-3169516ffc07.sh
2020-01-21T23:11:56.1187627Z 
2020-01-21T23:11:56.1191330Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-21T23:11:56.1196322Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68434/merge to s
2020-01-21T23:11:56.1197901Z Task         : Get sources
2020-01-21T23:11:56.1197930Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-21T23:11:56.1197966Z Version      : 1.0.0
2020-01-21T23:11:56.1197994Z Author       : Microsoft
---
2020-01-21T23:11:56.9168959Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-21T23:11:56.9257079Z ##[command]git config gc.auto 0
2020-01-21T23:11:56.9310430Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-21T23:11:56.9357922Z ##[command]git config --get-all http.proxy
2020-01-21T23:11:56.9507255Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68434/merge:refs/remotes/pull/68434/merge
---
2020-01-21T23:17:12.4893650Z    Compiling serde_json v1.0.40
2020-01-21T23:17:13.9663480Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-01-21T23:17:22.9877832Z     Finished release [optimized] target(s) in 1m 16s
2020-01-21T23:17:22.9976715Z tidy check
2020-01-21T23:17:23.5154947Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0747.md:1: line longer than 80 chars
2020-01-21T23:17:23.5155052Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0747.md:9: line longer than 80 chars
2020-01-21T23:17:25.5252912Z Found 488 error codes
2020-01-21T23:17:25.5253011Z Found 0 error codes with no tests
2020-01-21T23:17:25.5253057Z Done!
2020-01-21T23:17:25.5253136Z some tidy checks failed
2020-01-21T23:17:25.5253136Z some tidy checks failed
2020-01-21T23:17:25.5257118Z 
2020-01-21T23:17:25.5257184Z 
2020-01-21T23:17:25.5258097Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-01-21T23:17:25.5258258Z 
2020-01-21T23:17:25.5258283Z 
2020-01-21T23:17:25.5267254Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-01-21T23:17:25.5267319Z Build completed unsuccessfully in 0:01:26
2020-01-21T23:17:25.5267319Z Build completed unsuccessfully in 0:01:26
2020-01-21T23:17:25.5313415Z == clock drift check ==
2020-01-21T23:17:25.5323311Z   local time: Tue Jan 21 23:17:25 UTC 2020
2020-01-21T23:17:25.6946370Z   network time: Tue, 21 Jan 2020 23:17:25 GMT
2020-01-21T23:17:25.6956027Z == end clock drift check ==
2020-01-21T23:17:26.3853952Z 
2020-01-21T23:17:26.3956496Z ##[error]Bash exited with code '1'.
2020-01-21T23:17:26.3971596Z ##[section]Finishing: Run build
2020-01-21T23:17:26.3987887Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68434/merge to s
2020-01-21T23:17:26.3989863Z Task         : Get sources
2020-01-21T23:17:26.3989959Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-21T23:17:26.3990129Z Version      : 1.0.0
2020-01-21T23:17:26.3990166Z Author       : Microsoft
2020-01-21T23:17:26.3990166Z Author       : Microsoft
2020-01-21T23:17:26.3990225Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-21T23:17:26.3990270Z ==============================================================================
2020-01-21T23:17:26.7706649Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-21T23:17:26.7748375Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68434/merge to s
2020-01-21T23:17:26.7857429Z Cleaning up task key
2020-01-21T23:17:26.7858475Z Start cleaning up orphan processes.
2020-01-21T23:17:26.7955913Z Terminate orphan process: pid (4037) (python)
2020-01-21T23:17:26.8137367Z ##[section]Finishing: Finalize Job
