plain
2019-07-20T14:18:01.0162325Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-20T14:18:01.0336195Z ##[command]git config gc.auto 0
2019-07-20T14:18:01.0406358Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-20T14:18:01.0454239Z ##[command]git config --get-all http.proxy
2019-07-20T14:18:01.0570936Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62805/merge:refs/remotes/pull/62805/merge
---
2019-07-20T14:18:36.0749662Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-20T14:18:36.0749896Z 
2019-07-20T14:18:36.0750285Z   git checkout -b <new-branch-name>
2019-07-20T14:18:36.0750502Z 
2019-07-20T14:18:36.0750691Z HEAD is now at a499249ec Merge bfd033cea63c7e8749725d19831c38fdc3263788 into f69b07144a151f46aaee1b6230ba4160e9394562
2019-07-20T14:18:36.0868394Z ##[section]Finishing: Checkout
2019-07-20T14:18:36.0874184Z ##[section]Starting: Decide whether to run this job
2019-07-20T14:18:36.0876928Z Task         : Bash
2019-07-20T14:18:36.0876979Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-20T14:18:36.0877013Z Version      : 3.151.2
2019-07-20T14:18:36.0877046Z Author       : Microsoft Corporation
2019-07-20T14:18:36.0877046Z Author       : Microsoft Corporation
2019-07-20T14:18:36.0877100Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-07-20T14:18:36.0877137Z ==============================================================================
2019-07-20T14:18:36.2058425Z Generating script.
2019-07-20T14:18:36.2085685Z ========================== Starting Command Output ===========================
2019-07-20T14:18:36.2106517Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/d8f55ecf-aa59-4e63-8c39-dc12ae76a648.sh
2019-07-20T14:18:36.2617636Z Executing the job since submodules are updated
2019-07-20T14:18:36.2696478Z ##[section]Finishing: Decide whether to run this job
2019-07-20T14:18:36.2706309Z ==============================================================================
2019-07-20T14:18:36.2706359Z Task         : Bash
2019-07-20T14:18:36.2706571Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-20T14:18:36.2706611Z Version      : 3.151.2
---
2019-07-20T16:30:20.4122633Z The state of "rls" has changed from "build-fail" to "test-pass"
2019-07-20T16:30:21.2215358Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-20T16:30:21.2215433Z [master 7927d4c] (linux CI update)
2019-07-20T16:30:21.2215473Z  1 file changed, 1 insertion(+)
2019-07-20T16:30:21.2215528Z remote: Invalid username or password.
2019-07-20T16:30:21.2215787Z fatal: Authentication failed for 'https://github.com/rust-lang-nursery/rust-toolstate.git/'
2019-07-20T16:30:22.4716069Z  * branch            master     -> FETCH_HEAD
2019-07-20T16:30:22.4894030Z HEAD is now at c880e6c (windows CI update)
2019-07-20T16:30:22.5009236Z The state of "rls" has changed from "build-fail" to "test-pass"
2019-07-20T16:30:22.5009559Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-20T16:30:22.5009559Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-20T16:30:22.5187275Z [master f6c92e1] (linux CI update)
2019-07-20T16:30:22.5187341Z  1 file changed, 1 insertion(+)
2019-07-20T16:30:22.8357969Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-20T16:30:25.1804313Z  * branch            master     -> FETCH_HEAD
2019-07-20T16:30:25.1944184Z HEAD is now at c880e6c (windows CI update)
2019-07-20T16:30:25.2049280Z The state of "rls" has changed from "build-fail" to "test-pass"
2019-07-20T16:30:25.2050176Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-20T16:30:25.2050176Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-20T16:30:25.2205896Z [master 88634d8] (linux CI update)
2019-07-20T16:30:25.2206433Z  1 file changed, 1 insertion(+)
2019-07-20T16:30:25.5303628Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-20T16:30:28.8965854Z  * branch            master     -> FETCH_HEAD
2019-07-20T16:30:28.9097100Z HEAD is now at c880e6c (windows CI update)
2019-07-20T16:30:28.9198140Z The state of "rls" has changed from "build-fail" to "test-pass"
2019-07-20T16:30:28.9200031Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-20T16:30:28.9200031Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-20T16:30:28.9353089Z [master e9cb9c5] (linux CI update)
2019-07-20T16:30:28.9353520Z  1 file changed, 1 insertion(+)
2019-07-20T16:30:29.2422642Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-20T16:30:29.5814457Z  * branch            master     -> FETCH_HEAD
2019-07-20T16:30:29.5954385Z HEAD is now at c880e6c (windows CI update)
2019-07-20T16:30:29.6056141Z The state of "rls" has changed from "build-fail" to "test-pass"
2019-07-20T16:30:29.6056425Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-20T16:30:29.6056425Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-20T16:30:29.6220421Z [master 1d7dd6d] (linux CI update)
2019-07-20T16:30:29.6220523Z  1 file changed, 1 insertion(+)
2019-07-20T16:30:29.9448112Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-20T16:30:31.2790279Z  * branch            master     -> FETCH_HEAD
2019-07-20T16:30:31.2923865Z HEAD is now at c880e6c (windows CI update)
2019-07-20T16:30:31.2923865Z HEAD is now at c880e6c (windows CI update)
2019-07-20T16:30:32.2531117Z ##[error]Bash exited with code '1'.
2019-07-20T16:30:32.2587459Z ##[section]Starting: Checkout
2019-07-20T16:30:32.2589553Z ==============================================================================
2019-07-20T16:30:32.2589609Z Task         : Get sources
2019-07-20T16:30:32.2589675Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
