plain
2019-07-21T10:28:49.5371427Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-21T10:28:49.5610204Z ##[command]git config gc.auto 0
2019-07-21T10:28:49.5682165Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-21T10:28:49.5751850Z ##[command]git config --get-all http.proxy
2019-07-21T10:28:49.5874402Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62823/merge:refs/remotes/pull/62823/merge
---
2019-07-21T10:29:23.5110105Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-21T10:29:23.5110146Z 
2019-07-21T10:29:23.5110303Z   git checkout -b <new-branch-name>
2019-07-21T10:29:23.5110326Z 
2019-07-21T10:29:23.5110362Z HEAD is now at 07e8c24f9 Merge 10f877b5ea4ddbdfb2364a1ff82ad4fa37b76ae7 into 1301422a6c2e8916560b8cc2f0564f38d8858a75
2019-07-21T10:29:23.5255087Z ##[section]Finishing: Checkout
2019-07-21T10:29:23.5262603Z ##[section]Starting: Decide whether to run this job
2019-07-21T10:29:23.5265602Z Task         : Bash
2019-07-21T10:29:23.5265643Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-21T10:29:23.5265701Z Version      : 3.151.2
2019-07-21T10:29:23.5265737Z Author       : Microsoft Corporation
2019-07-21T10:29:23.5265737Z Author       : Microsoft Corporation
2019-07-21T10:29:23.5265779Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-07-21T10:29:23.5265974Z ==============================================================================
2019-07-21T10:29:23.6530267Z Generating script.
2019-07-21T10:29:23.6557426Z ========================== Starting Command Output ===========================
2019-07-21T10:29:23.6576446Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/ba72da03-734d-4fef-af10-bb4ff261d07b.sh
2019-07-21T10:29:23.8801258Z Executing the job since submodules are updated
2019-07-21T10:29:23.8838094Z ##[section]Finishing: Decide whether to run this job
2019-07-21T10:29:23.8848097Z ==============================================================================
2019-07-21T10:29:23.8848182Z Task         : Bash
2019-07-21T10:29:23.8848217Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-21T10:29:23.8848250Z Version      : 3.151.2
---
2019-07-21T12:33:13.1879134Z The state of "miri" has changed from "build-fail" to "test-pass"
2019-07-21T12:33:13.1879448Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-21T12:33:13.2113427Z [master 6f5f445] (linux CI update)
2019-07-21T12:33:13.2113505Z  1 file changed, 1 insertion(+)
2019-07-21T12:33:13.8759275Z remote: Invalid username or password.
2019-07-21T12:33:13.8760231Z fatal: Authentication failed for 'https://github.com/rust-lang-nursery/rust-toolstate.git/'
2019-07-21T12:33:17.2340079Z  * branch            master     -> FETCH_HEAD
2019-07-21T12:33:17.2501388Z HEAD is now at ffda10e (windows CI update)
2019-07-21T12:33:17.2617325Z The state of "miri" has changed from "build-fail" to "test-pass"
2019-07-21T12:33:17.2618048Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-21T12:33:17.2618048Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-21T12:33:17.2786960Z [master fb1a555] (linux CI update)
2019-07-21T12:33:17.2787028Z  1 file changed, 1 insertion(+)
2019-07-21T12:33:17.5803013Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-21T12:33:20.9186802Z  * branch            master     -> FETCH_HEAD
2019-07-21T12:33:20.9342093Z HEAD is now at ffda10e (windows CI update)
2019-07-21T12:33:20.9453066Z The state of "miri" has changed from "build-fail" to "test-pass"
2019-07-21T12:33:20.9453952Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-21T12:33:20.9453952Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-21T12:33:20.9643137Z [master e7420d2] (linux CI update)
2019-07-21T12:33:20.9643737Z  1 file changed, 1 insertion(+)
2019-07-21T12:33:21.2706464Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-21T12:33:24.6145310Z  * branch            master     -> FETCH_HEAD
2019-07-21T12:33:24.6292240Z HEAD is now at ffda10e (windows CI update)
2019-07-21T12:33:24.6406338Z The state of "miri" has changed from "build-fail" to "test-pass"
2019-07-21T12:33:24.6406629Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-21T12:33:24.6406629Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-21T12:33:24.6583940Z [master 6eab6bf] (linux CI update)
2019-07-21T12:33:24.6584367Z  1 file changed, 1 insertion(+)
2019-07-21T12:33:24.9788387Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-21T12:33:26.3463378Z  * branch            master     -> FETCH_HEAD
2019-07-21T12:33:26.3629603Z HEAD is now at ffda10e (windows CI update)
2019-07-21T12:33:26.3742920Z The state of "miri" has changed from "build-fail" to "test-pass"
2019-07-21T12:33:26.3744063Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-21T12:33:26.3744063Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-21T12:33:26.3938724Z [master 8dc0b40] (linux CI update)
2019-07-21T12:33:26.3939126Z  1 file changed, 1 insertion(+)
2019-07-21T12:33:26.7143257Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-21T12:33:27.0526636Z  * branch            master     -> FETCH_HEAD
2019-07-21T12:33:27.0662860Z HEAD is now at ffda10e (windows CI update)
2019-07-21T12:33:27.0662860Z HEAD is now at ffda10e (windows CI update)
2019-07-21T12:33:27.6313764Z ##[error]Bash exited with code '1'.
2019-07-21T12:33:27.6346066Z ##[section]Starting: Checkout
2019-07-21T12:33:27.6347543Z ==============================================================================
2019-07-21T12:33:27.6347584Z Task         : Get sources
2019-07-21T12:33:27.6347636Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
