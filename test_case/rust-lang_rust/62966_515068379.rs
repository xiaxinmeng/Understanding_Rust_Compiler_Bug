plain
2019-07-25T08:32:51.8609381Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-25T08:32:51.8814073Z ##[command]git config gc.auto 0
2019-07-25T08:32:51.8884580Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-25T08:32:51.8950146Z ##[command]git config --get-all http.proxy
2019-07-25T08:32:51.9100144Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62966/merge:refs/remotes/pull/62966/merge
---
2019-07-25T08:33:29.6991403Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-25T08:33:29.6991438Z 
2019-07-25T08:33:29.6991675Z   git checkout -b <new-branch-name>
2019-07-25T08:33:29.6991706Z 
2019-07-25T08:33:29.6991773Z HEAD is now at 422a724be Merge d4c96f296cc51a1f1ab96789c29c1c896e768770 into 185b9acb66438894596f3c40d2ae4c6f7deeb8ab
2019-07-25T08:33:29.7121950Z ##[section]Finishing: Checkout
2019-07-25T08:33:29.7129915Z ##[section]Starting: Decide whether to run this job
2019-07-25T08:33:29.7133460Z Task         : Bash
2019-07-25T08:33:29.7133512Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-25T08:33:29.7133564Z Version      : 3.151.3
2019-07-25T08:33:29.7133630Z Author       : Microsoft Corporation
2019-07-25T08:33:29.7133630Z Author       : Microsoft Corporation
2019-07-25T08:33:29.7133689Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-07-25T08:33:29.7133749Z ==============================================================================
2019-07-25T08:33:29.8431906Z Generating script.
2019-07-25T08:33:29.8480812Z ========================== Starting Command Output ===========================
2019-07-25T08:33:29.8501734Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/b0dd3b19-ce4c-489c-82c3-d1d2ea8f6773.sh
2019-07-25T08:33:30.2167778Z Executing the job since submodules are updated
2019-07-25T08:33:30.2210390Z ##[section]Finishing: Decide whether to run this job
2019-07-25T08:33:30.2220641Z ==============================================================================
2019-07-25T08:33:30.2220732Z Task         : Bash
2019-07-25T08:33:30.2220811Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-25T08:33:30.2220851Z Version      : 3.151.3
---
2019-07-25T10:42:32.6532961Z The state of "miri" has changed from "test-pass" to "build-fail"
2019-07-25T10:42:32.6533515Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-25T10:42:32.6767505Z [master f3deed7] (linux CI update)
2019-07-25T10:42:32.6767635Z  1 file changed, 1 insertion(+)
2019-07-25T10:42:33.3093073Z remote: Invalid username or password.
2019-07-25T10:42:33.3093937Z fatal: Authentication failed for 'https://github.com/rust-lang-nursery/rust-toolstate.git/'
2019-07-25T10:42:35.6329346Z  * branch            master     -> FETCH_HEAD
2019-07-25T10:42:35.6329346Z  * branch            master     -> FETCH_HEAD
2019-07-25T10:42:35.6520197Z HEAD is now at cccca90 📣 Toolstate changed by rust-lang/rust#62944!
2019-07-25T10:42:35.6650903Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-25T10:42:35.6848051Z [master 6d475ce] (linux CI update)
2019-07-25T10:42:35.6848343Z  1 file changed, 1 insertion(+)
2019-07-25T10:42:35.6848343Z  1 file changed, 1 insertion(+)
2019-07-25T10:42:35.9801045Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-25T10:42:36.3147574Z  * branch            master     -> FETCH_HEAD
2019-07-25T10:42:36.3147574Z  * branch            master     -> FETCH_HEAD
2019-07-25T10:42:36.3317793Z HEAD is now at cccca90 📣 Toolstate changed by rust-lang/rust#62944!
2019-07-25T10:42:36.3451136Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-25T10:42:36.3630940Z [master b07b5d2] (linux CI update)
2019-07-25T10:42:36.3631458Z  1 file changed, 1 insertion(+)
2019-07-25T10:42:36.3631458Z  1 file changed, 1 insertion(+)
2019-07-25T10:42:36.6689050Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-25T10:42:39.0008447Z  * branch            master     -> FETCH_HEAD
2019-07-25T10:42:39.0008447Z  * branch            master     -> FETCH_HEAD
2019-07-25T10:42:39.0202247Z HEAD is now at cccca90 📣 Toolstate changed by rust-lang/rust#62944!
2019-07-25T10:42:39.0328310Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-25T10:42:39.0544246Z [master bcd8c6b] (linux CI update)
2019-07-25T10:42:39.0544816Z  1 file changed, 1 insertion(+)
2019-07-25T10:42:39.0544816Z  1 file changed, 1 insertion(+)
2019-07-25T10:42:39.3614557Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-25T10:42:40.6982886Z  * branch            master     -> FETCH_HEAD
2019-07-25T10:42:40.6982886Z  * branch            master     -> FETCH_HEAD
2019-07-25T10:42:40.7149313Z HEAD is now at cccca90 📣 Toolstate changed by rust-lang/rust#62944!
2019-07-25T10:42:40.7275741Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-25T10:42:40.7476771Z [master f6edb82] (linux CI update)
2019-07-25T10:42:40.7477339Z  1 file changed, 1 insertion(+)
2019-07-25T10:42:40.7477339Z  1 file changed, 1 insertion(+)
2019-07-25T10:42:41.0605477Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-25T10:42:44.4095303Z  * branch            master     -> FETCH_HEAD
2019-07-25T10:42:44.4095303Z  * branch            master     -> FETCH_HEAD
2019-07-25T10:42:44.4280103Z HEAD is now at cccca90 📣 Toolstate changed by rust-lang/rust#62944!
2019-07-25T10:42:45.3693168Z ##[error]Bash exited with code '1'.
2019-07-25T10:42:45.3737449Z ##[section]Starting: Checkout
2019-07-25T10:42:45.3739729Z ==============================================================================
2019-07-25T10:42:45.3739789Z Task         : Get sources
2019-07-25T10:42:45.3739862Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
