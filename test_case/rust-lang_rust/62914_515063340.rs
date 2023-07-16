plain
2019-07-25T12:08:46.9125653Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-25T12:08:46.9322034Z ##[command]git config gc.auto 0
2019-07-25T12:08:46.9385671Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-25T12:08:46.9437202Z ##[command]git config --get-all http.proxy
2019-07-25T12:08:46.9559438Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62914/merge:refs/remotes/pull/62914/merge
---
2019-07-25T12:09:21.4113034Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-25T12:09:21.4113076Z 
2019-07-25T12:09:21.4113235Z   git checkout -b <new-branch-name>
2019-07-25T12:09:21.4113259Z 
2019-07-25T12:09:21.4113481Z HEAD is now at b65163ff4 Merge d9b0d0f881ba6ac600a518d222320e42c493a089 into eedf6ce4ef54bb03818ab21d714f1b9f13a6b31c
2019-07-25T12:09:21.4257617Z ##[section]Finishing: Checkout
2019-07-25T12:09:21.4264706Z ##[section]Starting: Decide whether to run this job
2019-07-25T12:09:21.4267208Z Task         : Bash
2019-07-25T12:09:21.4267246Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-25T12:09:21.4267285Z Version      : 3.151.3
2019-07-25T12:09:21.4267338Z Author       : Microsoft Corporation
2019-07-25T12:09:21.4267338Z Author       : Microsoft Corporation
2019-07-25T12:09:21.4267378Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-07-25T12:09:21.4267421Z ==============================================================================
2019-07-25T12:09:21.5576686Z Generating script.
2019-07-25T12:09:21.5610780Z ========================== Starting Command Output ===========================
2019-07-25T12:09:21.5628417Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/feebc0ff-5dfa-4225-992f-7ef2f889becc.sh
2019-07-25T12:09:21.7509659Z Executing the job since submodules are updated
2019-07-25T12:09:21.7583424Z ##[section]Finishing: Decide whether to run this job
2019-07-25T12:09:21.7593589Z ==============================================================================
2019-07-25T12:09:21.7593642Z Task         : Bash
2019-07-25T12:09:21.7593687Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-25T12:09:21.7593774Z Version      : 3.151.3
---
2019-07-25T14:20:13.5236057Z Cloning into 'rust-toolstate'...
2019-07-25T14:20:14.6276098Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-25T14:20:14.6493722Z [master d91c3f0] (linux CI update)
2019-07-25T14:20:14.6494816Z  1 file changed, 1 insertion(+)
2019-07-25T14:20:15.8670701Z remote: Invalid username or password.
2019-07-25T14:20:15.8673163Z fatal: Authentication failed for 'https://github.com/rust-lang-nursery/rust-toolstate.git/'
2019-07-25T14:20:17.3732087Z  * branch            master     -> FETCH_HEAD
2019-07-25T14:20:17.3912583Z HEAD is now at 5486804 (windows CI update)
2019-07-25T14:20:17.4030971Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-25T14:20:17.4208804Z [master eb45aaa] (linux CI update)
2019-07-25T14:20:17.4208804Z [master eb45aaa] (linux CI update)
2019-07-25T14:20:17.4209956Z  1 file changed, 1 insertion(+)
2019-07-25T14:20:17.7070209Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-25T14:20:20.0690790Z  * branch            master     -> FETCH_HEAD
2019-07-25T14:20:20.0843039Z HEAD is now at 5486804 (windows CI update)
2019-07-25T14:20:20.0965421Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-25T14:20:20.1137424Z [master dd29ec1] (linux CI update)
2019-07-25T14:20:20.1137424Z [master dd29ec1] (linux CI update)
2019-07-25T14:20:20.1137481Z  1 file changed, 1 insertion(+)
2019-07-25T14:20:20.4153805Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-25T14:20:22.7378389Z  * branch            master     -> FETCH_HEAD
2019-07-25T14:20:22.7526966Z HEAD is now at 5486804 (windows CI update)
2019-07-25T14:20:22.7638046Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-25T14:20:22.7821650Z [master 9d8f1b1] (linux CI update)
2019-07-25T14:20:22.7821650Z [master 9d8f1b1] (linux CI update)
2019-07-25T14:20:22.7822322Z  1 file changed, 1 insertion(+)
2019-07-25T14:20:23.0999479Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-25T14:20:23.4704537Z  * branch            master     -> FETCH_HEAD
2019-07-25T14:20:23.4848397Z HEAD is now at 5486804 (windows CI update)
2019-07-25T14:20:23.4979128Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-25T14:20:23.5183811Z [master cb363a2] (linux CI update)
2019-07-25T14:20:23.5183811Z [master cb363a2] (linux CI update)
2019-07-25T14:20:23.5183900Z  1 file changed, 1 insertion(+)
2019-07-25T14:20:24.0674325Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-25T14:20:26.4376050Z  * branch            master     -> FETCH_HEAD
2019-07-25T14:20:26.4525376Z HEAD is now at 5486804 (windows CI update)
2019-07-25T14:20:26.4525376Z HEAD is now at 5486804 (windows CI update)
2019-07-25T14:20:27.0041584Z ##[error]Bash exited with code '1'.
2019-07-25T14:20:27.0082784Z ##[section]Starting: Checkout
2019-07-25T14:20:27.0084822Z ==============================================================================
2019-07-25T14:20:27.0084871Z Task         : Get sources
2019-07-25T14:20:27.0084914Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
