plain
2020-02-23T11:45:45.2400749Z ========================== Starting Command Output ===========================
2020-02-23T11:45:45.2404025Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/1694fa09-d538-4a15-8894-db7f73615112.sh
2020-02-23T11:45:45.2404402Z 
2020-02-23T11:45:45.2408189Z ##[section]Finishing: Disable git automatic line ending conversion
2020-02-23T11:45:45.2428129Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69372/merge to s
2020-02-23T11:45:45.2431241Z Task         : Get sources
2020-02-23T11:45:45.2431472Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-23T11:45:45.2431696Z Version      : 1.0.0
2020-02-23T11:45:45.2431895Z Author       : Microsoft
---
2020-02-23T11:45:46.2628391Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-02-23T11:45:46.2633731Z ##[command]git config gc.auto 0
2020-02-23T11:45:46.2637950Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-02-23T11:45:46.2641228Z ##[command]git config --get-all http.proxy
2020-02-23T11:45:46.2647800Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69372/merge:refs/remotes/pull/69372/merge
---
2020-02-23T11:51:31.6969101Z    Compiling serde_json v1.0.40
2020-02-23T11:51:33.2800610Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-02-23T11:51:43.3172542Z     Finished release [optimized] target(s) in 1m 25s
2020-02-23T11:51:43.3173312Z tidy check
2020-02-23T11:51:43.3173894Z tidy error: /checkout/src/librustc_error_codes/error_codes/E0080.md:18: line longer than 80 chars
2020-02-23T11:51:44.9456834Z some tidy checks failed
2020-02-23T11:51:44.9457334Z Found 487 error codes
2020-02-23T11:51:44.9458072Z Found 0 error codes with no tests
2020-02-23T11:51:44.9458788Z Done!
2020-02-23T11:51:44.9458788Z Done!
2020-02-23T11:51:44.9458902Z 
2020-02-23T11:51:44.9461254Z 
2020-02-23T11:51:44.9463103Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-02-23T11:51:44.9464626Z 
2020-02-23T11:51:44.9464754Z 
2020-02-23T11:51:44.9474696Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-02-23T11:51:44.9475044Z Build completed unsuccessfully in 0:01:34
2020-02-23T11:51:44.9475044Z Build completed unsuccessfully in 0:01:34
2020-02-23T11:51:44.9523502Z == clock drift check ==
2020-02-23T11:51:44.9532258Z   local time: Sun Feb 23 11:51:44 UTC 2020
2020-02-23T11:51:45.4993264Z   network time: Sun, 23 Feb 2020 11:51:45 GMT
2020-02-23T11:51:45.4996801Z == end clock drift check ==
2020-02-23T11:51:46.3692743Z 
2020-02-23T11:51:46.3776617Z ##[error]Bash exited with code '1'.
2020-02-23T11:51:46.3789237Z ##[section]Finishing: Run build
2020-02-23T11:51:46.3838485Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69372/merge to s
2020-02-23T11:51:46.3842703Z Task         : Get sources
2020-02-23T11:51:46.3842999Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-02-23T11:51:46.3843304Z Version      : 1.0.0
2020-02-23T11:51:46.3843495Z Author       : Microsoft
2020-02-23T11:51:46.3843495Z Author       : Microsoft
2020-02-23T11:51:46.3843796Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-02-23T11:51:46.3844172Z ==============================================================================
2020-02-23T11:51:46.6991952Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-02-23T11:51:46.7034519Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69372/merge to s
2020-02-23T11:51:46.7135835Z Cleaning up task key
2020-02-23T11:51:46.7137811Z Start cleaning up orphan processes.
2020-02-23T11:51:46.7446559Z Terminate orphan process: pid (3912) (python)
2020-02-23T11:51:46.7481898Z ##[section]Finishing: Finalize Job
