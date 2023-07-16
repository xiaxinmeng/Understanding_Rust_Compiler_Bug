plain
2019-07-23T22:44:06.8572517Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-23T22:44:06.8752865Z ##[command]git config gc.auto 0
2019-07-23T22:44:06.8821636Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-23T22:44:06.8875035Z ##[command]git config --get-all http.proxy
2019-07-23T22:44:06.9007308Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62914/merge:refs/remotes/pull/62914/merge
---
2019-07-23T22:44:42.2089560Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-23T22:44:42.2089606Z 
2019-07-23T22:44:42.2090065Z   git checkout -b <new-branch-name>
2019-07-23T22:44:42.2090094Z 
2019-07-23T22:44:42.2090142Z HEAD is now at 6f01c6d19 Merge d9b0d0f881ba6ac600a518d222320e42c493a089 into 299ef86e1f8b3e53154f834115752c719b611fa1
2019-07-23T22:44:42.2223579Z ##[section]Finishing: Checkout
2019-07-23T22:44:42.2230450Z ##[section]Starting: Decide whether to run this job
2019-07-23T22:44:42.2233214Z Task         : Bash
2019-07-23T22:44:42.2233260Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-23T22:44:42.2233320Z Version      : 3.151.3
2019-07-23T22:44:42.2233367Z Author       : Microsoft Corporation
2019-07-23T22:44:42.2233367Z Author       : Microsoft Corporation
2019-07-23T22:44:42.2233418Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-07-23T22:44:42.2233473Z ==============================================================================
2019-07-23T22:44:42.3648529Z Generating script.
2019-07-23T22:44:42.3678297Z ========================== Starting Command Output ===========================
2019-07-23T22:44:42.3681782Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/4617d4ce-a754-43a5-a9a2-a12096520c2c.sh
2019-07-23T22:44:42.4865831Z Executing the job since submodules are updated
2019-07-23T22:44:42.4953622Z ##[section]Finishing: Decide whether to run this job
2019-07-23T22:44:42.4963655Z ==============================================================================
2019-07-23T22:44:42.4963714Z Task         : Bash
2019-07-23T22:44:42.4963760Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-23T22:44:42.4963846Z Version      : 3.151.3
---
2019-07-24T00:51:40.2614408Z The state of "miri" has changed from "build-fail" to "test-pass"
2019-07-24T00:51:40.2615515Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-24T00:51:40.2862682Z [master 5bc135d] (linux CI update)
2019-07-24T00:51:40.2863182Z  1 file changed, 1 insertion(+)
2019-07-24T00:51:40.9198529Z remote: Invalid username or password.
2019-07-24T00:51:40.9199952Z fatal: Authentication failed for 'https://github.com/rust-lang-nursery/rust-toolstate.git/'
2019-07-24T00:51:41.2359549Z  * branch            master     -> FETCH_HEAD
2019-07-24T00:51:41.2359549Z  * branch            master     -> FETCH_HEAD
2019-07-24T00:51:41.2552857Z HEAD is now at eee16eb 📣 Toolstate changed by rust-lang/rust#62902!
2019-07-24T00:51:41.2656302Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-24T00:51:41.2838299Z [master d95d8e7] (linux CI update)
2019-07-24T00:51:41.2838364Z  1 file changed, 1 insertion(+)
2019-07-24T00:51:41.2838364Z  1 file changed, 1 insertion(+)
2019-07-24T00:51:41.5677988Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-24T00:51:42.8776253Z  * branch            master     -> FETCH_HEAD
2019-07-24T00:51:42.8776253Z  * branch            master     -> FETCH_HEAD
2019-07-24T00:51:42.8925962Z HEAD is now at eee16eb 📣 Toolstate changed by rust-lang/rust#62902!
2019-07-24T00:51:42.9042284Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-24T00:51:42.9226849Z [master 9a8cea3] (linux CI update)
2019-07-24T00:51:42.9233499Z  1 file changed, 1 insertion(+)
2019-07-24T00:51:42.9233499Z  1 file changed, 1 insertion(+)
2019-07-24T00:51:43.2106726Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-24T00:51:46.5364846Z  * branch            master     -> FETCH_HEAD
2019-07-24T00:51:46.5364846Z  * branch            master     -> FETCH_HEAD
2019-07-24T00:51:46.5513516Z HEAD is now at eee16eb 📣 Toolstate changed by rust-lang/rust#62902!
2019-07-24T00:51:46.5619444Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-24T00:51:46.5797868Z [master 74e8b59] (linux CI update)
2019-07-24T00:51:46.5798103Z  1 file changed, 1 insertion(+)
2019-07-24T00:51:46.5798103Z  1 file changed, 1 insertion(+)
2019-07-24T00:51:46.9000560Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-24T00:51:50.2110553Z  * branch            master     -> FETCH_HEAD
2019-07-24T00:51:50.2110553Z  * branch            master     -> FETCH_HEAD
2019-07-24T00:51:50.2255509Z HEAD is now at eee16eb 📣 Toolstate changed by rust-lang/rust#62902!
2019-07-24T00:51:50.2365590Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-24T00:51:50.2544569Z [master 271073f] (linux CI update)
2019-07-24T00:51:50.2544669Z  1 file changed, 1 insertion(+)
2019-07-24T00:51:50.2544669Z  1 file changed, 1 insertion(+)
2019-07-24T00:51:50.5397052Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-24T00:51:50.8614833Z  * branch            master     -> FETCH_HEAD
2019-07-24T00:51:50.8614833Z  * branch            master     -> FETCH_HEAD
2019-07-24T00:51:50.8778178Z HEAD is now at eee16eb 📣 Toolstate changed by rust-lang/rust#62902!
2019-07-24T00:51:51.4998399Z ##[error]Bash exited with code '1'.
2019-07-24T00:51:51.5033581Z ##[section]Starting: Checkout
2019-07-24T00:51:51.5035382Z ==============================================================================
2019-07-24T00:51:51.5035437Z Task         : Get sources
2019-07-24T00:51:51.5035735Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
