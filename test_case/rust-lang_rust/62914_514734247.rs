plain
2019-07-24T15:45:52.1687574Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-24T15:45:52.1955316Z ##[command]git config gc.auto 0
2019-07-24T15:45:52.2017345Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-24T15:45:52.2065333Z ##[command]git config --get-all http.proxy
2019-07-24T15:45:52.2207116Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62914/merge:refs/remotes/pull/62914/merge
---
2019-07-24T15:46:26.9851853Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-24T15:46:26.9851885Z 
2019-07-24T15:46:26.9852124Z   git checkout -b <new-branch-name>
2019-07-24T15:46:26.9852181Z 
2019-07-24T15:46:26.9852232Z HEAD is now at a7fae211f Merge b8f92af7c6eb38f421dd5a1b26af4173051c917a into 27a6a304e2baaabca88059753f020377f2476978
2019-07-24T15:46:26.9980326Z ##[section]Finishing: Checkout
2019-07-24T15:46:26.9988888Z ##[section]Starting: Decide whether to run this job
2019-07-24T15:46:26.9992260Z Task         : Bash
2019-07-24T15:46:26.9992309Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-24T15:46:26.9992359Z Version      : 3.151.3
2019-07-24T15:46:26.9992426Z Author       : Microsoft Corporation
2019-07-24T15:46:26.9992426Z Author       : Microsoft Corporation
2019-07-24T15:46:26.9992479Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-07-24T15:46:26.9992534Z ==============================================================================
2019-07-24T15:46:27.1355865Z Generating script.
2019-07-24T15:46:27.1390015Z ========================== Starting Command Output ===========================
2019-07-24T15:46:27.1416358Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/951c00e4-cdc8-479a-965e-5ca4dc52d32e.sh
2019-07-24T15:46:27.5867210Z Executing the job since submodules are updated
2019-07-24T15:46:27.5955144Z ##[section]Finishing: Decide whether to run this job
2019-07-24T15:46:27.5966104Z ==============================================================================
2019-07-24T15:46:27.5966160Z Task         : Bash
2019-07-24T15:46:27.5966248Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-24T15:46:27.5966292Z Version      : 3.151.3
---
2019-07-24T15:50:17.0888136Z Removing intermediate container 95f740d64a0f
2019-07-24T15:50:17.0889399Z  ---> da118ce203f5
2019-07-24T15:50:17.0927685Z Successfully built da118ce203f5
2019-07-24T15:50:17.2846913Z Successfully tagged rust-ci:latest
2019-07-24T15:50:17.3261150Z Built container sha256:da118ce203f505afb27712a163a53cbd8336535f1f57498980db08cb13603385
2019-07-24T15:50:17.3281900Z Uploading finished image to https://rust-lang-ci-sccache2.s3.amazonaws.com/docker/4423d48e0ee10feb20310346fa56215bc110ee7537f42841f4b761300a16ec0cdfdc8014051ae325b14937fd2c4e9cfe4e90171cd786b357d6c3b904166fcdd9
2019-07-24T15:50:57.4278354Z upload failed: - to s3://rust-lang-ci-sccache2/docker/4423d48e0ee10feb20310346fa56215bc110ee7537f42841f4b761300a16ec0cdfdc8014051ae325b14937fd2c4e9cfe4e90171cd786b357d6c3b904166fcdd9 Unable to locate credentials
2019-07-24T15:50:58.4948336Z [CI_JOB_NAME=LinuxTools]
2019-07-24T15:50:58.4995349Z Starting sccache server...
2019-07-24T15:50:58.5917101Z configure: processing command line
2019-07-24T15:50:58.5917928Z configure: 
---
2019-07-24T17:52:38.7073666Z Cloning into 'rust-toolstate'...
2019-07-24T17:52:39.3747269Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-24T17:52:39.3992860Z [master 879666e] (linux CI update)
2019-07-24T17:52:39.3993020Z  1 file changed, 1 insertion(+)
2019-07-24T17:52:40.0333320Z remote: Invalid username or password.
2019-07-24T17:52:40.0334121Z fatal: Authentication failed for 'https://github.com/rust-lang-nursery/rust-toolstate.git/'
2019-07-24T17:52:40.3845370Z  * branch            master     -> FETCH_HEAD
2019-07-24T17:52:40.4017557Z HEAD is now at e6a5387 (windows CI update)
2019-07-24T17:52:40.4137256Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-24T17:52:40.4322426Z [master d8a6d28] (linux CI update)
2019-07-24T17:52:40.4322426Z [master d8a6d28] (linux CI update)
2019-07-24T17:52:40.4323507Z  1 file changed, 1 insertion(+)
2019-07-24T17:52:40.7316465Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-24T17:52:42.0751712Z  * branch            master     -> FETCH_HEAD
2019-07-24T17:52:42.0898391Z HEAD is now at e6a5387 (windows CI update)
2019-07-24T17:52:42.1006038Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-24T17:52:42.1188674Z [master 53b2e36] (linux CI update)
2019-07-24T17:52:42.1188674Z [master 53b2e36] (linux CI update)
2019-07-24T17:52:42.1188747Z  1 file changed, 1 insertion(+)
2019-07-24T17:52:42.4349907Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-24T17:52:43.7853860Z  * branch            master     -> FETCH_HEAD
2019-07-24T17:52:43.7999595Z HEAD is now at e6a5387 (windows CI update)
2019-07-24T17:52:43.8107772Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-24T17:52:43.8285623Z [master 9ffccd3] (linux CI update)
2019-07-24T17:52:43.8285623Z [master 9ffccd3] (linux CI update)
2019-07-24T17:52:43.8286077Z  1 file changed, 1 insertion(+)
2019-07-24T17:52:44.1199125Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-24T17:52:46.4312258Z  * branch            master     -> FETCH_HEAD
2019-07-24T17:52:46.4474296Z HEAD is now at e6a5387 (windows CI update)
2019-07-24T17:52:46.4590659Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-24T17:52:46.4771479Z [master cc45c26] (linux CI update)
2019-07-24T17:52:46.4771479Z [master cc45c26] (linux CI update)
2019-07-24T17:52:46.4771760Z  1 file changed, 1 insertion(+)
2019-07-24T17:52:46.7791066Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-24T17:52:49.1066900Z  * branch            master     -> FETCH_HEAD
2019-07-24T17:52:49.1224231Z HEAD is now at e6a5387 (windows CI update)
2019-07-24T17:52:49.1224231Z HEAD is now at e6a5387 (windows CI update)
2019-07-24T17:52:50.2045781Z ##[error]Bash exited with code '1'.
2019-07-24T17:52:50.2118472Z ##[section]Starting: Checkout
2019-07-24T17:52:50.2120416Z ==============================================================================
2019-07-24T17:52:50.2120474Z Task         : Get sources
2019-07-24T17:52:50.2120540Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
