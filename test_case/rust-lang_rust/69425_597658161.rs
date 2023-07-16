plain
2020-03-11T13:54:26.2287895Z ========================== Starting Command Output ===========================
2020-03-11T13:54:26.2290699Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/2f13d116-6e69-4187-916c-b26d2d4b64ff.sh
2020-03-11T13:54:26.2290992Z 
2020-03-11T13:54:26.2295454Z ##[section]Finishing: Disable git automatic line ending conversion
2020-03-11T13:54:26.2315705Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69425/merge to s
2020-03-11T13:54:26.2319459Z Task         : Get sources
2020-03-11T13:54:26.2319803Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-11T13:54:26.2320113Z Version      : 1.0.0
2020-03-11T13:54:26.2320532Z Author       : Microsoft
---
2020-03-11T13:54:27.2228346Z ##[command]git remote add origin https://github.com/rust-lang/rust
2020-03-11T13:54:27.2236827Z ##[command]git config gc.auto 0
2020-03-11T13:54:27.2244480Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2020-03-11T13:54:27.2251158Z ##[command]git config --get-all http.proxy
2020-03-11T13:54:27.2260653Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69425/merge:refs/remotes/pull/69425/merge
---
2020-03-11T14:00:10.4866497Z Found 0 error codes with no tests
2020-03-11T14:00:10.4867053Z Done!
2020-03-11T14:00:10.4867494Z 
2020-03-11T14:00:10.4867850Z 
2020-03-11T14:00:10.4869680Z command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor"
2020-03-11T14:00:10.4877108Z 
2020-03-11T14:00:10.4877627Z 
2020-03-11T14:00:10.4878098Z failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
2020-03-11T14:00:10.4878682Z Build completed unsuccessfully in 0:01:32
2020-03-11T14:00:10.4878682Z Build completed unsuccessfully in 0:01:32
2020-03-11T14:00:10.4931218Z == clock drift check ==
2020-03-11T14:00:10.4939291Z   local time: Wed Mar 11 14:00:10 UTC 2020
2020-03-11T14:00:10.7842470Z   network time: Wed, 11 Mar 2020 14:00:10 GMT
2020-03-11T14:00:10.7843278Z == end clock drift check ==
2020-03-11T14:00:11.5953454Z 
2020-03-11T14:00:11.6030800Z ##[error]Bash exited with code '1'.
2020-03-11T14:00:11.6056706Z ##[section]Finishing: Run build
2020-03-11T14:00:11.6114144Z ##[section]Starting: Checkout rust-lang/rust@refs/pull/69425/merge to s
2020-03-11T14:00:11.6119332Z Task         : Get sources
2020-03-11T14:00:11.6119740Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
2020-03-11T14:00:11.6120100Z Version      : 1.0.0
2020-03-11T14:00:11.6120542Z Author       : Microsoft
2020-03-11T14:00:11.6120542Z Author       : Microsoft
2020-03-11T14:00:11.6120958Z Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
2020-03-11T14:00:11.6121421Z ==============================================================================
2020-03-11T14:00:11.9727852Z Cleaning any cached credential from repository: rust-lang/rust (GitHub)
2020-03-11T14:00:11.9772684Z ##[section]Finishing: Checkout rust-lang/rust@refs/pull/69425/merge to s
2020-03-11T14:00:11.9874353Z Cleaning up task key
2020-03-11T14:00:11.9875846Z Start cleaning up orphan processes.
2020-03-11T14:00:12.0203612Z Terminate orphan process: pid (3848) (python)
2020-03-11T14:00:12.0237529Z ##[section]Finishing: Finalize Job
