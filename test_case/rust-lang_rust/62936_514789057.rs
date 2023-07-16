plain
2019-07-24T14:32:16.7905866Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-24T14:32:16.8130300Z ##[command]git config gc.auto 0
2019-07-24T14:32:16.8185070Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-24T14:32:16.8237612Z ##[command]git config --get-all http.proxy
2019-07-24T14:32:16.8394534Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62936/merge:refs/remotes/pull/62936/merge
---
2019-07-24T14:32:52.3607453Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-24T14:32:52.3607488Z 
2019-07-24T14:32:52.3607729Z   git checkout -b <new-branch-name>
2019-07-24T14:32:52.3607761Z 
2019-07-24T14:32:52.3607833Z HEAD is now at 9028d037b Merge c9cdfa110b543c307f3c89594fa7e6bdb48ac910 into 27a6a304e2baaabca88059753f020377f2476978
2019-07-24T14:32:52.3775655Z ##[section]Finishing: Checkout
2019-07-24T14:32:52.3783941Z ##[section]Starting: Decide whether to run this job
2019-07-24T14:32:52.3787569Z Task         : Bash
2019-07-24T14:32:52.3787622Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-24T14:32:52.3787714Z Version      : 3.151.3
2019-07-24T14:32:52.3787767Z Author       : Microsoft Corporation
2019-07-24T14:32:52.3787767Z Author       : Microsoft Corporation
2019-07-24T14:32:52.3787825Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-07-24T14:32:52.3787906Z ==============================================================================
2019-07-24T14:32:52.5105934Z Generating script.
2019-07-24T14:32:52.5134705Z ========================== Starting Command Output ===========================
2019-07-24T14:32:52.5153660Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/ad3ec21f-903f-4025-a251-9baccd55a3e7.sh
2019-07-24T14:32:52.7510219Z Executing the job since submodules are updated
2019-07-24T14:32:52.7612460Z ##[section]Finishing: Decide whether to run this job
2019-07-24T14:32:52.7624559Z ==============================================================================
2019-07-24T14:32:52.7624624Z Task         : Bash
2019-07-24T14:32:52.7624782Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-24T14:32:52.7624833Z Version      : 3.151.3
---
2019-07-24T16:41:05.8252425Z Cloning into 'rust-toolstate'...
2019-07-24T16:41:06.6545002Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-24T16:41:06.6824097Z [master b97cf8f] (linux CI update)
2019-07-24T16:41:06.6824280Z  1 file changed, 1 insertion(+)
2019-07-24T16:41:07.4877129Z remote: Invalid username or password.
2019-07-24T16:41:07.4879385Z fatal: Authentication failed for 'https://github.com/rust-lang-nursery/rust-toolstate.git/'
2019-07-24T16:41:09.8231744Z  * branch            master     -> FETCH_HEAD
2019-07-24T16:41:09.8448405Z HEAD is now at e6a5387 (windows CI update)
2019-07-24T16:41:09.8600502Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-24T16:41:09.8809397Z [master f8373b4] (linux CI update)
2019-07-24T16:41:09.8809397Z [master f8373b4] (linux CI update)
2019-07-24T16:41:09.8809943Z  1 file changed, 1 insertion(+)
2019-07-24T16:41:10.1999921Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-24T16:41:11.5489823Z  * branch            master     -> FETCH_HEAD
2019-07-24T16:41:11.5676323Z HEAD is now at e6a5387 (windows CI update)
2019-07-24T16:41:11.5800261Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-24T16:41:11.6006058Z [master e31501c] (linux CI update)
2019-07-24T16:41:11.6006058Z [master e31501c] (linux CI update)
2019-07-24T16:41:11.6006677Z  1 file changed, 1 insertion(+)
2019-07-24T16:41:11.8953379Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-24T16:41:14.2603161Z  * branch            master     -> FETCH_HEAD
2019-07-24T16:41:14.2763561Z HEAD is now at e6a5387 (windows CI update)
2019-07-24T16:41:14.2875595Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-24T16:41:14.3076313Z [master 5f03b4c] (linux CI update)
2019-07-24T16:41:14.3076313Z [master 5f03b4c] (linux CI update)
2019-07-24T16:41:14.3076761Z  1 file changed, 1 insertion(+)
2019-07-24T16:41:14.7278485Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-24T16:41:17.0898720Z  * branch            master     -> FETCH_HEAD
2019-07-24T16:41:17.1129730Z HEAD is now at e6a5387 (windows CI update)
2019-07-24T16:41:17.1286578Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-24T16:41:17.1487249Z [master 03439b9] (linux CI update)
2019-07-24T16:41:17.1487249Z [master 03439b9] (linux CI update)
2019-07-24T16:41:17.1487448Z  1 file changed, 1 insertion(+)
2019-07-24T16:41:17.4403577Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-24T16:41:20.7596734Z  * branch            master     -> FETCH_HEAD
2019-07-24T16:41:20.7766980Z HEAD is now at e6a5387 (windows CI update)
2019-07-24T16:41:20.7766980Z HEAD is now at e6a5387 (windows CI update)
2019-07-24T16:41:21.7031854Z ##[error]Bash exited with code '1'.
2019-07-24T16:41:21.7074927Z ##[section]Starting: Checkout
2019-07-24T16:41:21.7076893Z ==============================================================================
2019-07-24T16:41:21.7076975Z Task         : Get sources
2019-07-24T16:41:21.7077027Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
