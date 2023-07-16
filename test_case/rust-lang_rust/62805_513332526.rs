plain
2019-07-19T16:06:29.8932068Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-19T16:06:29.9132157Z ##[command]git config gc.auto 0
2019-07-19T16:06:29.9221621Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-19T16:06:29.9270338Z ##[command]git config --get-all http.proxy
2019-07-19T16:06:29.9400817Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62805/merge:refs/remotes/pull/62805/merge
---
2019-07-19T16:07:04.0395160Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-19T16:07:04.0395453Z 
2019-07-19T16:07:04.0395854Z   git checkout -b <new-branch-name>
2019-07-19T16:07:04.0396070Z 
2019-07-19T16:07:04.0396295Z HEAD is now at cef2936a6 Merge 6492b107f61a032ed016058004e2bb1920f99dec into 527dce7137f7a3c7bf47d9a503abf25f88ea22de
2019-07-19T16:07:04.0517685Z ##[section]Finishing: Checkout
2019-07-19T16:07:04.0525027Z ##[section]Starting: Decide whether to run this job
2019-07-19T16:07:04.0527513Z Task         : Bash
2019-07-19T16:07:04.0527676Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-19T16:07:04.0527721Z Version      : 3.151.2
2019-07-19T16:07:04.0527779Z Author       : Microsoft Corporation
2019-07-19T16:07:04.0527779Z Author       : Microsoft Corporation
2019-07-19T16:07:04.0527825Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-07-19T16:07:04.0527873Z ==============================================================================
2019-07-19T16:07:04.1867648Z Generating script.
2019-07-19T16:07:04.1897410Z ========================== Starting Command Output ===========================
2019-07-19T16:07:04.1918067Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/cbaf9c7f-666b-4487-abee-c1b45812e922.sh
2019-07-19T16:07:04.2237121Z Executing the job since submodules are updated
2019-07-19T16:07:04.2327419Z ##[section]Finishing: Decide whether to run this job
2019-07-19T16:07:04.2337125Z ==============================================================================
2019-07-19T16:07:04.2337194Z Task         : Bash
2019-07-19T16:07:04.2337264Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-19T16:07:04.2337301Z Version      : 3.151.2
---
2019-07-19T18:33:32.6398985Z The state of "rls" has changed from "build-fail" to "test-pass"
2019-07-19T18:33:32.6400353Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-19T18:33:32.6400433Z [master 1ab3f4c] (linux CI update)
2019-07-19T18:33:32.6401194Z  1 file changed, 1 insertion(+)
2019-07-19T18:33:33.3170195Z remote: Invalid username or password.
2019-07-19T18:33:33.3170979Z fatal: Authentication failed for 'https://github.com/rust-lang-nursery/rust-toolstate.git/'
2019-07-19T18:33:34.6766353Z  * branch            master     -> FETCH_HEAD
2019-07-19T18:33:34.6939253Z HEAD is now at 8ce5dbc (linux CI update)
2019-07-19T18:33:34.7074447Z The state of "rls" has changed from "build-fail" to "test-pass"
2019-07-19T18:33:34.7074791Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-19T18:33:34.7074791Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-19T18:33:34.7275535Z [master abd7bbf] (linux CI update)
2019-07-19T18:33:34.7275609Z  1 file changed, 1 insertion(+)
2019-07-19T18:33:35.0587706Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-19T18:33:38.4096957Z  * branch            master     -> FETCH_HEAD
2019-07-19T18:33:38.4249783Z HEAD is now at 8ce5dbc (linux CI update)
2019-07-19T18:33:38.4366244Z The state of "rls" has changed from "build-fail" to "test-pass"
2019-07-19T18:33:38.4366533Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-19T18:33:38.4366533Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-19T18:33:38.4577811Z [master 833ef43] (linux CI update)
2019-07-19T18:33:38.4577936Z  1 file changed, 1 insertion(+)
2019-07-19T18:33:38.7728799Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-19T18:33:40.1349689Z  * branch            master     -> FETCH_HEAD
2019-07-19T18:33:40.1514219Z HEAD is now at 8ce5dbc (linux CI update)
2019-07-19T18:33:40.1657451Z The state of "rls" has changed from "build-fail" to "test-pass"
2019-07-19T18:33:40.1658493Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-19T18:33:40.1658493Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-19T18:33:40.1842058Z [master 588ed74] (linux CI update)
2019-07-19T18:33:40.1842300Z  1 file changed, 1 insertion(+)
2019-07-19T18:33:40.5068402Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-19T18:33:40.8634987Z  * branch            master     -> FETCH_HEAD
2019-07-19T18:33:40.8803790Z HEAD is now at 8ce5dbc (linux CI update)
2019-07-19T18:33:40.8931077Z The state of "rls" has changed from "build-fail" to "test-pass"
2019-07-19T18:33:40.8931403Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-19T18:33:40.8931403Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-19T18:33:40.9104552Z [master 588ed74] (linux CI update)
2019-07-19T18:33:40.9104632Z  1 file changed, 1 insertion(+)
2019-07-19T18:33:41.2430692Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-19T18:33:43.5902402Z  * branch            master     -> FETCH_HEAD
2019-07-19T18:33:43.6102245Z HEAD is now at 8ce5dbc (linux CI update)
2019-07-19T18:33:43.6102245Z HEAD is now at 8ce5dbc (linux CI update)
2019-07-19T18:33:44.5704832Z ##[error]Bash exited with code '1'.
2019-07-19T18:33:44.5741474Z ##[section]Starting: Checkout
2019-07-19T18:33:44.5743150Z ==============================================================================
2019-07-19T18:33:44.5743218Z Task         : Get sources
2019-07-19T18:33:44.5743263Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
