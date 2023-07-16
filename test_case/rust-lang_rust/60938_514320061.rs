plain
2019-07-23T15:58:45.3762271Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-23T15:58:45.3940204Z ##[command]git config gc.auto 0
2019-07-23T15:58:45.4005435Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-23T15:58:45.4056259Z ##[command]git config --get-all http.proxy
2019-07-23T15:58:45.4178017Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/60938/merge:refs/remotes/pull/60938/merge
---
2019-07-23T15:59:23.8198819Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-23T15:59:23.8198865Z 
2019-07-23T15:59:23.8215711Z   git checkout -b <new-branch-name>
2019-07-23T15:59:23.8215943Z 
2019-07-23T15:59:23.8216052Z HEAD is now at e748b6bf8 Merge 218ab4cd7fdf145a0870c582a23ad5fd85cd80e5 into 3ebca72a11869f946b31f900faffb75c2bb2473a
2019-07-23T15:59:23.8347903Z ##[section]Finishing: Checkout
2019-07-23T15:59:23.8355754Z ##[section]Starting: Decide whether to run this job
2019-07-23T15:59:23.8359211Z Task         : Bash
2019-07-23T15:59:23.8359249Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-23T15:59:23.8359305Z Version      : 3.151.3
2019-07-23T15:59:23.8359344Z Author       : Microsoft Corporation
2019-07-23T15:59:23.8359344Z Author       : Microsoft Corporation
2019-07-23T15:59:23.8359386Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-07-23T15:59:23.8359446Z ==============================================================================
2019-07-23T15:59:23.9568438Z Generating script.
2019-07-23T15:59:23.9597680Z ========================== Starting Command Output ===========================
2019-07-23T15:59:23.9616202Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/ca66f7ee-aa33-4513-bae6-071dfc489b4c.sh
2019-07-23T15:59:24.2662572Z Executing the job since submodules are updated
2019-07-23T15:59:24.2745517Z ##[section]Finishing: Decide whether to run this job
2019-07-23T15:59:24.2755527Z ==============================================================================
2019-07-23T15:59:24.2755571Z Task         : Bash
2019-07-23T15:59:24.2755644Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-23T15:59:24.2755679Z Version      : 3.151.3
---
2019-07-23T18:03:40.6240604Z Verifying status of embedded-book...
2019-07-23T18:03:40.6252710Z Verifying status of rustc-guide...
2019-07-23T18:03:40.6401253Z Cloning into 'rust-toolstate'...
2019-07-23T18:03:41.3424352Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-23T18:03:41.3656464Z [master 4bda9b2] (linux CI update)
2019-07-23T18:03:42.1237555Z remote: Invalid username or password.
2019-07-23T18:03:42.1237555Z remote: Invalid username or password.
2019-07-23T18:03:42.1239021Z fatal: Authentication failed for 'https://github.com/rust-lang-nursery/rust-toolstate.git/'
2019-07-23T18:03:45.4603565Z  * branch            master     -> FETCH_HEAD
2019-07-23T18:03:45.4776213Z HEAD is now at 1c20fe8 (windows CI update)
2019-07-23T18:03:45.4888251Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-23T18:03:45.5069710Z [master 0a8c1ae] (linux CI update)
2019-07-23T18:03:45.5069710Z [master 0a8c1ae] (linux CI update)
2019-07-23T18:03:45.5069954Z  1 file changed, 1 insertion(+)
2019-07-23T18:03:45.8449153Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-23T18:03:47.1867686Z  * branch            master     -> FETCH_HEAD
2019-07-23T18:03:47.2021479Z HEAD is now at 1c20fe8 (windows CI update)
2019-07-23T18:03:47.2132706Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-23T18:03:47.2312601Z [master 3156f8c] (linux CI update)
2019-07-23T18:03:47.2312601Z [master 3156f8c] (linux CI update)
2019-07-23T18:03:47.2313021Z  1 file changed, 1 insertion(+)
2019-07-23T18:03:47.5543270Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-23T18:03:48.9067342Z  * branch            master     -> FETCH_HEAD
2019-07-23T18:03:48.9223899Z HEAD is now at 1c20fe8 (windows CI update)
2019-07-23T18:03:48.9340868Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-23T18:03:48.9510181Z [master 1348238] (linux CI update)
2019-07-23T18:03:48.9510181Z [master 1348238] (linux CI update)
2019-07-23T18:03:48.9510530Z  1 file changed, 1 insertion(+)
2019-07-23T18:03:49.2657359Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-23T18:03:50.6056035Z  * branch            master     -> FETCH_HEAD
2019-07-23T18:03:50.6211911Z HEAD is now at 1c20fe8 (windows CI update)
2019-07-23T18:03:50.6328061Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-23T18:03:50.6509274Z [master aeea6b5] (linux CI update)
2019-07-23T18:03:50.6509274Z [master aeea6b5] (linux CI update)
2019-07-23T18:03:50.6510048Z  1 file changed, 1 insertion(+)
2019-07-23T18:03:50.9790517Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-23T18:03:53.3125414Z  * branch            master     -> FETCH_HEAD
2019-07-23T18:03:53.3271937Z HEAD is now at 1c20fe8 (windows CI update)
2019-07-23T18:03:53.3271937Z HEAD is now at 1c20fe8 (windows CI update)
2019-07-23T18:03:54.2829337Z ##[error]Bash exited with code '1'.
2019-07-23T18:03:54.2864874Z ##[section]Starting: Checkout
2019-07-23T18:03:54.2866915Z ==============================================================================
2019-07-23T18:03:54.2866992Z Task         : Get sources
2019-07-23T18:03:54.2867040Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
