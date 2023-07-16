plain
2019-07-24T18:48:06.0893586Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-24T18:48:06.1110818Z ##[command]git config gc.auto 0
2019-07-24T18:48:06.1188779Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-24T18:48:06.1244657Z ##[command]git config --get-all http.proxy
2019-07-24T18:48:06.1382155Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62944/merge:refs/remotes/pull/62944/merge
---
2019-07-24T18:48:42.4047975Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-24T18:48:42.4048005Z 
2019-07-24T18:48:42.4048537Z   git checkout -b <new-branch-name>
2019-07-24T18:48:42.4048571Z 
2019-07-24T18:48:42.4048643Z HEAD is now at 3e91cb0d2 Merge f2900b0b411f655dba96d278cf21d2face3811f6 into 27a6a304e2baaabca88059753f020377f2476978
2019-07-24T18:48:42.4220979Z ##[section]Finishing: Checkout
2019-07-24T18:48:42.4228994Z ##[section]Starting: Decide whether to run this job
2019-07-24T18:48:42.4231992Z Task         : Bash
2019-07-24T18:48:42.4232034Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-24T18:48:42.4232106Z Version      : 3.151.3
2019-07-24T18:48:42.4232146Z Author       : Microsoft Corporation
2019-07-24T18:48:42.4232146Z Author       : Microsoft Corporation
2019-07-24T18:48:42.4232363Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-07-24T18:48:42.4232490Z ==============================================================================
2019-07-24T18:48:42.5597688Z Generating script.
2019-07-24T18:48:42.5634699Z ========================== Starting Command Output ===========================
2019-07-24T18:48:42.5661074Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/27761e79-f34b-4cd7-a37e-914fcc6c1238.sh
2019-07-24T18:48:42.7343510Z Executing the job since submodules are updated
2019-07-24T18:48:42.7439123Z ##[section]Finishing: Decide whether to run this job
2019-07-24T18:48:42.7449677Z ==============================================================================
2019-07-24T18:48:42.7449773Z Task         : Bash
2019-07-24T18:48:42.7449862Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-24T18:48:42.7449907Z Version      : 3.151.3
---
2019-07-24T21:02:55.1014099Z The state of "miri" has changed from "build-fail" to "test-pass"
2019-07-24T21:02:55.1014852Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-24T21:02:55.1272007Z [master 8ab0977] (linux CI update)
2019-07-24T21:02:55.1272154Z  1 file changed, 1 insertion(+)
2019-07-24T21:02:55.8215011Z remote: Invalid username or password.
2019-07-24T21:02:55.8216053Z fatal: Authentication failed for 'https://github.com/rust-lang-nursery/rust-toolstate.git/'
2019-07-24T21:02:57.3760057Z  * branch            master     -> FETCH_HEAD
2019-07-24T21:02:57.3936028Z HEAD is now at 2a2a469 (windows CI update)
2019-07-24T21:02:57.4052159Z The state of "miri" has changed from "build-fail" to "test-pass"
2019-07-24T21:02:57.4052491Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-24T21:02:57.4052491Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-24T21:02:57.4240680Z [master 41500b7] (linux CI update)
2019-07-24T21:02:57.4240971Z  1 file changed, 1 insertion(+)
2019-07-24T21:02:57.7301856Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-24T21:02:58.0647607Z  * branch            master     -> FETCH_HEAD
2019-07-24T21:02:58.0812709Z HEAD is now at 2a2a469 (windows CI update)
2019-07-24T21:02:58.0933183Z The state of "miri" has changed from "build-fail" to "test-pass"
2019-07-24T21:02:58.0933504Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-24T21:02:58.0933504Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-24T21:02:58.1121957Z [master 75d7c89] (linux CI update)
2019-07-24T21:02:58.1122099Z  1 file changed, 1 insertion(+)
2019-07-24T21:02:58.4176893Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-24T21:03:02.0579342Z  * branch            master     -> FETCH_HEAD
2019-07-24T21:03:02.0747210Z HEAD is now at 2a2a469 (windows CI update)
2019-07-24T21:03:02.0867097Z The state of "miri" has changed from "build-fail" to "test-pass"
2019-07-24T21:03:02.0868086Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-24T21:03:02.0868086Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-24T21:03:02.1056564Z [master fd0a754] (linux CI update)
2019-07-24T21:03:02.1057245Z  1 file changed, 1 insertion(+)
2019-07-24T21:03:02.9184831Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-24T21:03:06.2624259Z  * branch            master     -> FETCH_HEAD
2019-07-24T21:03:06.2784697Z HEAD is now at 2a2a469 (windows CI update)
2019-07-24T21:03:06.2912447Z The state of "miri" has changed from "build-fail" to "test-pass"
2019-07-24T21:03:06.2913593Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-24T21:03:06.2913593Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-24T21:03:06.3104181Z [master 83e496b] (linux CI update)
2019-07-24T21:03:06.3104817Z  1 file changed, 1 insertion(+)
2019-07-24T21:03:06.6031644Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-24T21:03:06.9272171Z  * branch            master     -> FETCH_HEAD
2019-07-24T21:03:06.9438739Z HEAD is now at 2a2a469 (windows CI update)
2019-07-24T21:03:06.9438739Z HEAD is now at 2a2a469 (windows CI update)
2019-07-24T21:03:07.5410820Z ##[error]Bash exited with code '1'.
2019-07-24T21:03:07.5472760Z ##[section]Starting: Checkout
2019-07-24T21:03:07.5474623Z ==============================================================================
2019-07-24T21:03:07.5474678Z Task         : Get sources
2019-07-24T21:03:07.5474726Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
