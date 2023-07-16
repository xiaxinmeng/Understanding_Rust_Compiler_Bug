plain
2019-07-20T21:24:09.2688011Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-20T21:24:09.2886454Z ##[command]git config gc.auto 0
2019-07-20T21:24:09.2974720Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-20T21:24:09.3046262Z ##[command]git config --get-all http.proxy
2019-07-20T21:24:09.3190191Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62823/merge:refs/remotes/pull/62823/merge
---
2019-07-20T21:24:43.8359729Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-20T21:24:43.8360245Z 
2019-07-20T21:24:43.8360757Z   git checkout -b <new-branch-name>
2019-07-20T21:24:43.8361024Z 
2019-07-20T21:24:43.8361249Z HEAD is now at 32552cad8 Merge da4f4f6116e035205453faf405ca5637dbfd0e95 into 95b1fe560d2bd8472f250fb8cfd2168520a58405
2019-07-20T21:24:43.8511028Z ##[section]Finishing: Checkout
2019-07-20T21:24:43.8518412Z ##[section]Starting: Decide whether to run this job
2019-07-20T21:24:43.8521727Z Task         : Bash
2019-07-20T21:24:43.8521774Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-20T21:24:43.8521866Z Version      : 3.151.2
2019-07-20T21:24:43.8521909Z Author       : Microsoft Corporation
2019-07-20T21:24:43.8521909Z Author       : Microsoft Corporation
2019-07-20T21:24:43.8521958Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-07-20T21:24:43.8522031Z ==============================================================================
2019-07-20T21:24:43.9919807Z Generating script.
2019-07-20T21:24:43.9955952Z ========================== Starting Command Output ===========================
2019-07-20T21:24:43.9981326Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/4ac8eea6-5275-495d-bf2e-393dce17cabb.sh
2019-07-20T21:24:44.3475515Z Executing the job since submodules are updated
2019-07-20T21:24:44.3566948Z ##[section]Finishing: Decide whether to run this job
2019-07-20T21:24:44.3578515Z ==============================================================================
2019-07-20T21:24:44.3578617Z Task         : Bash
2019-07-20T21:24:44.3578664Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-20T21:24:44.3578708Z Version      : 3.151.2
---
2019-07-20T23:42:42.9853205Z Verifying status of rustc-guide...
2019-07-20T23:42:42.9952747Z Cloning into 'rust-toolstate'...
2019-07-20T23:42:43.7122441Z The state of "miri" has changed from "build-fail" to "test-pass"
2019-07-20T23:42:43.7122744Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-20T23:42:43.7351144Z [master c1b4cf2] (linux CI update)
2019-07-20T23:42:44.4465321Z remote: Invalid username or password.
2019-07-20T23:42:44.4465321Z remote: Invalid username or password.
2019-07-20T23:42:44.4467349Z fatal: Authentication failed for 'https://github.com/rust-lang-nursery/rust-toolstate.git/'
2019-07-20T23:42:46.8130614Z  * branch            master     -> FETCH_HEAD
2019-07-20T23:42:46.8301386Z HEAD is now at 9477056 (windows CI update)
2019-07-20T23:42:46.8410063Z The state of "miri" has changed from "build-fail" to "test-pass"
2019-07-20T23:42:46.8410494Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-20T23:42:46.8410494Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-20T23:42:46.8596545Z [master ba6108e] (linux CI update)
2019-07-20T23:42:46.8596614Z  1 file changed, 1 insertion(+)
2019-07-20T23:42:47.1675813Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-20T23:42:49.5473006Z  * branch            master     -> FETCH_HEAD
2019-07-20T23:42:49.5633200Z HEAD is now at 9477056 (windows CI update)
2019-07-20T23:42:49.5759027Z The state of "miri" has changed from "build-fail" to "test-pass"
2019-07-20T23:42:49.5759914Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-20T23:42:49.5759914Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-20T23:42:49.5937715Z [master f9a0f6f] (linux CI update)
2019-07-20T23:42:49.5938599Z  1 file changed, 1 insertion(+)
2019-07-20T23:42:49.9100297Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-20T23:42:53.2473689Z  * branch            master     -> FETCH_HEAD
2019-07-20T23:42:53.2638561Z HEAD is now at 9477056 (windows CI update)
2019-07-20T23:42:53.2766930Z The state of "miri" has changed from "build-fail" to "test-pass"
2019-07-20T23:42:53.2767320Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-20T23:42:53.2767320Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-20T23:42:53.2967963Z [master ad82b69] (linux CI update)
2019-07-20T23:42:53.2968032Z  1 file changed, 1 insertion(+)
2019-07-20T23:42:53.6138835Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-20T23:42:55.9616421Z  * branch            master     -> FETCH_HEAD
2019-07-20T23:42:55.9710788Z HEAD is now at 9477056 (windows CI update)
2019-07-20T23:42:55.9824853Z The state of "miri" has changed from "build-fail" to "test-pass"
2019-07-20T23:42:55.9825152Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-20T23:42:55.9825152Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-20T23:42:56.0001523Z [master f2e0b02] (linux CI update)
2019-07-20T23:42:56.0001608Z  1 file changed, 1 insertion(+)
2019-07-20T23:42:56.3228365Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-20T23:42:59.6634785Z  * branch            master     -> FETCH_HEAD
2019-07-20T23:42:59.6795110Z HEAD is now at 9477056 (windows CI update)
2019-07-20T23:42:59.6795110Z HEAD is now at 9477056 (windows CI update)
2019-07-20T23:43:00.2369204Z ##[error]Bash exited with code '1'.
2019-07-20T23:43:00.2407139Z ##[section]Starting: Checkout
2019-07-20T23:43:00.2409105Z ==============================================================================
2019-07-20T23:43:00.2409152Z Task         : Get sources
2019-07-20T23:43:00.2409221Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
