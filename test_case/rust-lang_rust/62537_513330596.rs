plain
2019-07-19T16:03:08.9008100Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-19T16:03:08.9192037Z ##[command]git config gc.auto 0
2019-07-19T16:03:08.9266271Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-19T16:03:08.9322028Z ##[command]git config --get-all http.proxy
2019-07-19T16:03:08.9469889Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62537/merge:refs/remotes/pull/62537/merge
---
2019-07-19T16:03:44.7396103Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-19T16:03:44.7396983Z 
2019-07-19T16:03:44.7397891Z   git checkout -b <new-branch-name>
2019-07-19T16:03:44.7398547Z 
2019-07-19T16:03:44.7398873Z HEAD is now at cacbca212 Merge 6492b107f61a032ed016058004e2bb1920f99dec into 527dce7137f7a3c7bf47d9a503abf25f88ea22de
2019-07-19T16:03:44.7522272Z ##[section]Finishing: Checkout
2019-07-19T16:03:44.7533199Z ##[section]Starting: Decide whether to run this job
2019-07-19T16:03:44.7536074Z Task         : Bash
2019-07-19T16:03:44.7536119Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-19T16:03:44.7536180Z Version      : 3.151.2
2019-07-19T16:03:44.7536223Z Author       : Microsoft Corporation
2019-07-19T16:03:44.7536223Z Author       : Microsoft Corporation
2019-07-19T16:03:44.7536272Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-07-19T16:03:44.7536339Z ==============================================================================
2019-07-19T16:03:44.8848607Z Generating script.
2019-07-19T16:03:44.8886896Z ========================== Starting Command Output ===========================
2019-07-19T16:03:44.8916347Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/a667b782-aefa-4f74-8647-345e23813d7a.sh
2019-07-19T16:03:45.2126840Z Executing the job since submodules are updated
2019-07-19T16:03:45.2183605Z ##[section]Finishing: Decide whether to run this job
2019-07-19T16:03:45.2193250Z ==============================================================================
2019-07-19T16:03:45.2193370Z Task         : Bash
2019-07-19T16:03:45.2193419Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-19T16:03:45.2193464Z Version      : 3.151.2
---
2019-07-19T18:27:29.5696491Z Verifying status of rustc-guide...
2019-07-19T18:27:29.5814395Z Cloning into 'rust-toolstate'...
2019-07-19T18:27:30.3459570Z The state of "rls" has changed from "build-fail" to "test-pass"
2019-07-19T18:27:30.3461055Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-19T18:27:30.3704714Z [master a716fa2] (linux CI update)
2019-07-19T18:27:31.1133333Z remote: Invalid username or password.
2019-07-19T18:27:31.1133333Z remote: Invalid username or password.
2019-07-19T18:27:31.1134180Z fatal: Authentication failed for 'https://github.com/rust-lang-nursery/rust-toolstate.git/'
2019-07-19T18:27:34.4690449Z  * branch            master     -> FETCH_HEAD
2019-07-19T18:27:34.4863710Z HEAD is now at 8ce5dbc (linux CI update)
2019-07-19T18:27:34.4977280Z The state of "rls" has changed from "build-fail" to "test-pass"
2019-07-19T18:27:34.4977699Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-19T18:27:34.4977699Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-19T18:27:34.5184510Z [master 0c0a6bb] (linux CI update)
2019-07-19T18:27:34.5184606Z  1 file changed, 1 insertion(+)
2019-07-19T18:27:34.8360218Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-19T18:27:36.1749581Z  * branch            master     -> FETCH_HEAD
2019-07-19T18:27:36.1917239Z HEAD is now at 8ce5dbc (linux CI update)
2019-07-19T18:27:36.2036576Z The state of "rls" has changed from "build-fail" to "test-pass"
2019-07-19T18:27:36.2037466Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-19T18:27:36.2037466Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-19T18:27:36.2223775Z [master 2335739] (linux CI update)
2019-07-19T18:27:36.2224311Z  1 file changed, 1 insertion(+)
2019-07-19T18:27:36.5413581Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-19T18:27:36.9323894Z  * branch            master     -> FETCH_HEAD
2019-07-19T18:27:36.9486493Z HEAD is now at 8ce5dbc (linux CI update)
2019-07-19T18:27:36.9600155Z The state of "rls" has changed from "build-fail" to "test-pass"
2019-07-19T18:27:36.9600871Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-19T18:27:36.9600871Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-19T18:27:36.9789136Z [master 2335739] (linux CI update)
2019-07-19T18:27:36.9789217Z  1 file changed, 1 insertion(+)
2019-07-19T18:27:37.2937883Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-19T18:27:39.6350573Z  * branch            master     -> FETCH_HEAD
2019-07-19T18:27:39.6504550Z HEAD is now at 8ce5dbc (linux CI update)
2019-07-19T18:27:39.6624277Z The state of "rls" has changed from "build-fail" to "test-pass"
2019-07-19T18:27:39.6625123Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-19T18:27:39.6625123Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-19T18:27:39.6820710Z [master c68151b] (linux CI update)
2019-07-19T18:27:39.6821101Z  1 file changed, 1 insertion(+)
2019-07-19T18:27:39.9934893Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-19T18:27:42.3322944Z  * branch            master     -> FETCH_HEAD
2019-07-19T18:27:42.3478416Z HEAD is now at 8ce5dbc (linux CI update)
2019-07-19T18:27:42.3478416Z HEAD is now at 8ce5dbc (linux CI update)
2019-07-19T18:27:43.2461332Z ##[error]Bash exited with code '1'.
2019-07-19T18:27:43.2499203Z ##[section]Starting: Checkout
2019-07-19T18:27:43.2501013Z ==============================================================================
2019-07-19T18:27:43.2501082Z Task         : Get sources
2019-07-19T18:27:43.2501144Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
