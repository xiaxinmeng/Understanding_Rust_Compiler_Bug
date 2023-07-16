plain
2019-07-24T17:51:59.2640384Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-24T17:51:59.2855053Z ##[command]git config gc.auto 0
2019-07-24T17:51:59.2953070Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-24T17:51:59.3036207Z ##[command]git config --get-all http.proxy
2019-07-24T17:51:59.3179539Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62943/merge:refs/remotes/pull/62943/merge
---
2019-07-24T17:52:35.8084690Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-24T17:52:35.8084724Z 
2019-07-24T17:52:35.8085359Z   git checkout -b <new-branch-name>
2019-07-24T17:52:35.8085401Z 
2019-07-24T17:52:35.8085497Z HEAD is now at 45493b41d Merge 1bf17ba95500eb3afd15d437b89d1b07344202d4 into 27a6a304e2baaabca88059753f020377f2476978
2019-07-24T17:52:35.8225455Z ##[section]Finishing: Checkout
2019-07-24T17:52:35.8233031Z ##[section]Starting: Decide whether to run this job
2019-07-24T17:52:35.8236130Z Task         : Bash
2019-07-24T17:52:35.8236217Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-24T17:52:35.8236268Z Version      : 3.151.3
2019-07-24T17:52:35.8236317Z Author       : Microsoft Corporation
2019-07-24T17:52:35.8236317Z Author       : Microsoft Corporation
2019-07-24T17:52:35.8236395Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-07-24T17:52:35.8236453Z ==============================================================================
2019-07-24T17:52:35.9663313Z Generating script.
2019-07-24T17:52:35.9692048Z ========================== Starting Command Output ===========================
2019-07-24T17:52:35.9712173Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/4b5404bb-a480-4bf4-91e8-cadbeb4157d5.sh
2019-07-24T17:52:36.3096830Z Executing the job since submodules are updated
2019-07-24T17:52:36.3201462Z ##[section]Finishing: Decide whether to run this job
2019-07-24T17:52:36.3211887Z ==============================================================================
2019-07-24T17:52:36.3212045Z Task         : Bash
2019-07-24T17:52:36.3212093Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-24T17:52:36.3212139Z Version      : 3.151.3
---
2019-07-24T20:09:59.4263418Z The state of "clippy-driver" has changed from "build-fail" to "test-pass"
2019-07-24T20:09:59.4263812Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-24T20:09:59.4566507Z [master 2e4521b] (linux CI update)
2019-07-24T20:09:59.4566750Z  1 file changed, 1 insertion(+)
2019-07-24T20:10:00.1422241Z remote: Invalid username or password.
2019-07-24T20:10:00.1424100Z fatal: Authentication failed for 'https://github.com/rust-lang-nursery/rust-toolstate.git/'
2019-07-24T20:10:00.4787502Z  * branch            master     -> FETCH_HEAD
2019-07-24T20:10:00.4998472Z HEAD is now at 2a2a469 (windows CI update)
2019-07-24T20:10:00.5138637Z The state of "clippy-driver" has changed from "build-fail" to "test-pass"
2019-07-24T20:10:00.5139136Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-24T20:10:00.5139136Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-24T20:10:00.5358355Z [master d754d27] (linux CI update)
2019-07-24T20:10:00.5358550Z  1 file changed, 1 insertion(+)
2019-07-24T20:10:00.8512397Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-24T20:10:02.7012764Z  * branch            master     -> FETCH_HEAD
2019-07-24T20:10:02.7173586Z HEAD is now at 2a2a469 (windows CI update)
2019-07-24T20:10:02.7306800Z The state of "clippy-driver" has changed from "build-fail" to "test-pass"
2019-07-24T20:10:02.7307231Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-24T20:10:02.7307231Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-24T20:10:02.7518963Z [master 1df5082] (linux CI update)
2019-07-24T20:10:02.7519182Z  1 file changed, 1 insertion(+)
2019-07-24T20:10:03.1089951Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-24T20:10:04.1649389Z  * branch            master     -> FETCH_HEAD
2019-07-24T20:10:04.1751853Z HEAD is now at 2a2a469 (windows CI update)
2019-07-24T20:10:04.1889245Z The state of "clippy-driver" has changed from "build-fail" to "test-pass"
2019-07-24T20:10:04.1889592Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-24T20:10:04.1889592Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-24T20:10:04.2097342Z [master 916498a] (linux CI update)
2019-07-24T20:10:04.2097495Z  1 file changed, 1 insertion(+)
2019-07-24T20:10:05.1758532Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-24T20:10:06.0618531Z  * branch            master     -> FETCH_HEAD
2019-07-24T20:10:06.0784926Z HEAD is now at 2a2a469 (windows CI update)
2019-07-24T20:10:06.0918137Z The state of "clippy-driver" has changed from "build-fail" to "test-pass"
2019-07-24T20:10:06.0918978Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-24T20:10:06.0918978Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-24T20:10:06.1117904Z [master 3a465de] (linux CI update)
2019-07-24T20:10:06.1117999Z  1 file changed, 1 insertion(+)
2019-07-24T20:10:06.7886643Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-24T20:10:07.4809065Z  * branch            master     -> FETCH_HEAD
2019-07-24T20:10:07.4974662Z HEAD is now at 2a2a469 (windows CI update)
2019-07-24T20:10:07.4974662Z HEAD is now at 2a2a469 (windows CI update)
2019-07-24T20:10:08.5722626Z ##[error]Bash exited with code '1'.
2019-07-24T20:10:08.5764723Z ##[section]Starting: Checkout
2019-07-24T20:10:08.5766780Z ==============================================================================
2019-07-24T20:10:08.5766844Z Task         : Get sources
2019-07-24T20:10:08.5766897Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
