plain
2020-01-24T21:43:48.3705936Z ========================== Starting Command Output ===========================
2020-01-24T21:43:48.3708304Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/485d0d2f-2e2e-4090-a49d-b19aa5c328b7.sh
2020-01-24T21:43:48.3708344Z 
2020-01-24T21:43:48.3711141Z ##[section]Finishing: Disable git automatic line ending conversion
2020-01-24T21:43:48.3716855Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68524/merge to s
2020-01-24T21:43:48.3718371Z Task         : Get sources
2020-01-24T21:43:48.3718444Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-24T21:43:48.3718480Z Version      : 1.0.0
2020-01-24T21:43:48.3718517Z Author       : Microsoft
---
2020-01-24T21:43:49.1073640Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-01-24T21:43:49.1179369Z ##[command]git config gc.auto 0
2020-01-24T21:43:49.1280255Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-01-24T21:43:49.1352252Z ##[command]git config --get-all http.proxy
2020-01-24T21:43:49.1482368Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68524/merge:refs/remotes/pull/68524/merge
---
2020-01-24T21:48:47.8631386Z    Compiling serde_json v1.0.40
2020-01-24T21:48:49.2310243Z    Compiling tidy v0.1.0 (/checkout/src/tools/tidy)
2020-01-24T21:48:57.3531078Z     Finished release [optimized] target(s) in 1m 08s
2020-01-24T21:48:57.3642273Z tidy check
2020-01-24T21:48:58.2955833Z tidy error: /checkout/src/test/ui/generator/issue-44197.rs:31: line longer than 100 chars
2020-01-24T21:48:59.8878136Z Found 487 error codes
2020-01-24T21:48:59.8878921Z Found 1 error codes with no tests
2020-01-24T21:48:59.8881918Z Done!
2020-01-24T21:48:59.8882392Z Error code E0628 needs to have at least one UI test!
2020-01-24T21:48:59.8882392Z Error code E0628 needs to have at least one UI test!
2020-01-24T21:48:59.8882671Z some tidy checks failed
2020-01-24T21:48:59.8882959Z 
2020-01-24T21:48:59.8883184Z 
2020-01-24T21:48:59.8884684Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-01-24T21:48:59.8885249Z 
2020-01-24T21:48:59.8885454Z 
2020-01-24T21:48:59.8890814Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-01-24T21:48:59.8891161Z Build completed unsuccessfully in 0:01:18
2020-01-24T21:48:59.8891161Z Build completed unsuccessfully in 0:01:18
2020-01-24T21:48:59.8943600Z == clock drift check ==
2020-01-24T21:48:59.8950767Z   local time: Fri Jan 24 21:48:59 UTC 2020
2020-01-24T21:49:00.1824835Z   network time: Fri, 24 Jan 2020 21:49:00 GMT
2020-01-24T21:49:00.1829125Z == end clock drift check ==
2020-01-24T21:49:00.9283555Z 
2020-01-24T21:49:00.9338549Z ##[error]Bash exited with code '1'.
2020-01-24T21:49:00.9349650Z ##[section]Finishing: Run build
2020-01-24T21:49:00.9362232Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/68524/merge to s
2020-01-24T21:49:00.9364134Z Task         : Get sources
2020-01-24T21:49:00.9364203Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-01-24T21:49:00.9364254Z Version      : 1.0.0
2020-01-24T21:49:00.9364300Z Author       : Microsoft
2020-01-24T21:49:00.9364300Z Author       : Microsoft
2020-01-24T21:49:00.9364366Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-01-24T21:49:00.9364419Z ==============================================================================
2020-01-24T21:49:01.3187854Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-01-24T21:49:01.3227071Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/68524/merge to s
2020-01-24T21:49:01.3333408Z Cleaning up task key
2020-01-24T21:49:01.3334278Z Start cleaning up orphan processes.
2020-01-24T21:49:01.3491820Z Terminate orphan process: pid (3761) (python)
2020-01-24T21:49:01.3669825Z ##[section]Finishing: Finalize Job
