plain
2019-07-22T01:17:03.6381279Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-22T01:17:03.6563527Z ##[command]git config gc.auto 0
2019-07-22T01:17:03.6651477Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-22T01:17:03.6714272Z ##[command]git config --get-all http.proxy
2019-07-22T01:17:03.6855945Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62861/merge:refs/remotes/pull/62861/merge
---
2019-07-22T01:17:36.9941694Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-22T01:17:36.9941886Z 
2019-07-22T01:17:36.9942103Z   git checkout -b <new-branch-name>
2019-07-22T01:17:36.9942130Z 
2019-07-22T01:17:36.9942174Z HEAD is now at a8c499b7f Merge c5810baacf2c6e1d8cf64b94ab18484c44b6ce3f into 273f42b5964c29dda2c5a349dd4655529767b07f
2019-07-22T01:17:37.0086459Z ##[section]Finishing: Checkout
2019-07-22T01:17:37.0093041Z ##[section]Starting: Decide whether to run this job
2019-07-22T01:17:37.0096516Z Task         : Bash
2019-07-22T01:17:37.0096563Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-22T01:17:37.0096609Z Version      : 3.151.2
2019-07-22T01:17:37.0096671Z Author       : Microsoft Corporation
2019-07-22T01:17:37.0096671Z Author       : Microsoft Corporation
2019-07-22T01:17:37.0096721Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-07-22T01:17:37.0096771Z ==============================================================================
2019-07-22T01:17:37.1572146Z Generating script.
2019-07-22T01:17:37.1607632Z ========================== Starting Command Output ===========================
2019-07-22T01:17:37.1624667Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/d2d9a217-e8af-44a9-b387-a76286e8e737.sh
2019-07-22T01:17:37.5940751Z Executing the job since submodules are updated
2019-07-22T01:17:37.6062104Z ##[section]Finishing: Decide whether to run this job
2019-07-22T01:17:37.6074046Z ==============================================================================
2019-07-22T01:17:37.6074109Z Task         : Bash
2019-07-22T01:17:37.6074155Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-22T01:17:37.6074241Z Version      : 3.151.2
---
2019-07-22T03:42:20.0583822Z The state of "clippy-driver" has changed from "build-fail" to "test-pass"
2019-07-22T03:42:20.0584247Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-22T03:42:20.0838714Z [master fa6115d] (linux CI update)
2019-07-22T03:42:20.0838807Z  1 file changed, 1 insertion(+)
2019-07-22T03:42:20.7915696Z remote: Invalid username or password.
2019-07-22T03:42:20.7916663Z fatal: Authentication failed for 'https://github.com/rust-lang-nursery/rust-toolstate.git/'
2019-07-22T03:42:24.1612810Z  * branch            master     -> FETCH_HEAD
2019-07-22T03:42:24.1784930Z HEAD is now at 854a323 (windows CI update)
2019-07-22T03:42:24.1900841Z The state of "clippy-driver" has changed from "build-fail" to "test-pass"
2019-07-22T03:42:24.1902764Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-22T03:42:24.1902764Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-22T03:42:24.2081791Z [master 1788851] (linux CI update)
2019-07-22T03:42:24.2081864Z  1 file changed, 1 insertion(+)
2019-07-22T03:42:24.5378504Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-22T03:42:24.8885625Z  * branch            master     -> FETCH_HEAD
2019-07-22T03:42:24.9042935Z HEAD is now at 854a323 (windows CI update)
2019-07-22T03:42:24.9156432Z The state of "clippy-driver" has changed from "build-fail" to "test-pass"
2019-07-22T03:42:24.9157212Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-22T03:42:24.9157212Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-22T03:42:24.9343465Z [master 1788851] (linux CI update)
2019-07-22T03:42:24.9343588Z  1 file changed, 1 insertion(+)
2019-07-22T03:42:25.2497763Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-22T03:42:27.5928025Z  * branch            master     -> FETCH_HEAD
2019-07-22T03:42:27.6107641Z HEAD is now at 854a323 (windows CI update)
2019-07-22T03:42:27.6223012Z The state of "clippy-driver" has changed from "build-fail" to "test-pass"
2019-07-22T03:42:27.6223306Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-22T03:42:27.6223306Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-22T03:42:27.6403871Z [master cb29f52] (linux CI update)
2019-07-22T03:42:27.6405937Z  1 file changed, 1 insertion(+)
2019-07-22T03:42:27.9630229Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-22T03:42:28.3120106Z  * branch            master     -> FETCH_HEAD
2019-07-22T03:42:28.3276824Z HEAD is now at 854a323 (windows CI update)
2019-07-22T03:42:28.3397943Z The state of "clippy-driver" has changed from "build-fail" to "test-pass"
2019-07-22T03:42:28.3398839Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-22T03:42:28.3398839Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-22T03:42:28.3585257Z [master d1b78c4] (linux CI update)
2019-07-22T03:42:28.3585332Z  1 file changed, 1 insertion(+)
2019-07-22T03:42:28.6827189Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-22T03:42:30.0367660Z  * branch            master     -> FETCH_HEAD
2019-07-22T03:42:30.0525909Z HEAD is now at 854a323 (windows CI update)
2019-07-22T03:42:30.0525909Z HEAD is now at 854a323 (windows CI update)
2019-07-22T03:42:31.1529107Z ##[error]Bash exited with code '1'.
2019-07-22T03:42:31.1573156Z ##[section]Starting: Checkout
2019-07-22T03:42:31.1575677Z ==============================================================================
2019-07-22T03:42:31.1575747Z Task         : Get sources
2019-07-22T03:42:31.1575792Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
