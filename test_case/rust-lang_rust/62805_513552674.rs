plain
2019-07-21T10:45:54.5629116Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-21T10:45:54.5808086Z ##[command]git config gc.auto 0
2019-07-21T10:45:54.5884019Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-21T10:45:54.5934361Z ##[command]git config --get-all http.proxy
2019-07-21T10:45:55.3407721Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62805/merge:refs/remotes/pull/62805/merge
---
2019-07-21T10:46:28.5795843Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-21T10:46:28.5795872Z 
2019-07-21T10:46:28.5796102Z   git checkout -b <new-branch-name>
2019-07-21T10:46:28.5796133Z 
2019-07-21T10:46:28.5796182Z HEAD is now at 1a33eec6e Merge 7c37c9b9722c1facd17cf9946db4cc843cbd9c56 into 1301422a6c2e8916560b8cc2f0564f38d8858a75
2019-07-21T10:46:28.5932966Z ##[section]Finishing: Checkout
2019-07-21T10:46:28.5940418Z ##[section]Starting: Decide whether to run this job
2019-07-21T10:46:28.5943457Z Task         : Bash
2019-07-21T10:46:28.5943504Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-21T10:46:28.5943549Z Version      : 3.151.2
2019-07-21T10:46:28.5943610Z Author       : Microsoft Corporation
2019-07-21T10:46:28.5943610Z Author       : Microsoft Corporation
2019-07-21T10:46:28.5943659Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-07-21T10:46:28.5943709Z ==============================================================================
2019-07-21T10:46:28.7336097Z Generating script.
2019-07-21T10:46:28.7367179Z ========================== Starting Command Output ===========================
2019-07-21T10:46:28.7387407Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/64009ae2-6db4-4fe1-aecd-ef35cc0d3251.sh
2019-07-21T10:46:29.0142929Z Executing the job since submodules are updated
2019-07-21T10:46:29.0233399Z ##[section]Finishing: Decide whether to run this job
2019-07-21T10:46:29.0243712Z ==============================================================================
2019-07-21T10:46:29.0243773Z Task         : Bash
2019-07-21T10:46:29.0243821Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-21T10:46:29.0243906Z Version      : 3.151.2
---
2019-07-21T13:03:32.5918871Z The state of "rls" has changed from "build-fail" to "test-pass"
2019-07-21T13:03:32.5919239Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-21T13:03:32.6157271Z [master d8e1ba5] (linux CI update)
2019-07-21T13:03:32.6157361Z  1 file changed, 1 insertion(+)
2019-07-21T13:03:33.3272128Z remote: Invalid username or password.
2019-07-21T13:03:33.3272943Z fatal: Authentication failed for 'https://github.com/rust-lang-nursery/rust-toolstate.git/'
2019-07-21T13:03:35.6691479Z  * branch            master     -> FETCH_HEAD
2019-07-21T13:03:35.6884439Z HEAD is now at ffda10e (windows CI update)
2019-07-21T13:03:35.6994046Z The state of "rls" has changed from "build-fail" to "test-pass"
2019-07-21T13:03:35.6995048Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-21T13:03:35.6995048Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-21T13:03:35.7177105Z [master 36f6a93] (linux CI update)
2019-07-21T13:03:35.7178292Z  1 file changed, 1 insertion(+)
2019-07-21T13:03:36.0479469Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-21T13:03:37.4060135Z  * branch            master     -> FETCH_HEAD
2019-07-21T13:03:37.4210264Z HEAD is now at ffda10e (windows CI update)
2019-07-21T13:03:37.4324438Z The state of "rls" has changed from "build-fail" to "test-pass"
2019-07-21T13:03:37.4324747Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-21T13:03:37.4324747Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-21T13:03:37.4509258Z [master 826114a] (linux CI update)
2019-07-21T13:03:37.4509454Z  1 file changed, 1 insertion(+)
2019-07-21T13:03:37.7652332Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-21T13:03:38.1063776Z  * branch            master     -> FETCH_HEAD
2019-07-21T13:03:38.1219321Z HEAD is now at ffda10e (windows CI update)
2019-07-21T13:03:38.1333706Z The state of "rls" has changed from "build-fail" to "test-pass"
2019-07-21T13:03:38.1334019Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-21T13:03:38.1334019Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-21T13:03:38.1513631Z [master 632a4cd] (linux CI update)
2019-07-21T13:03:38.1513742Z  1 file changed, 1 insertion(+)
2019-07-21T13:03:38.4698153Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-21T13:03:41.8102866Z  * branch            master     -> FETCH_HEAD
2019-07-21T13:03:41.8250901Z HEAD is now at ffda10e (windows CI update)
2019-07-21T13:03:41.8365097Z The state of "rls" has changed from "build-fail" to "test-pass"
2019-07-21T13:03:41.8365391Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-21T13:03:41.8365391Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-21T13:03:41.8538696Z [master 18d5336] (linux CI update)
2019-07-21T13:03:41.8539109Z  1 file changed, 1 insertion(+)
2019-07-21T13:03:42.1759722Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-21T13:03:45.5300662Z  * branch            master     -> FETCH_HEAD
2019-07-21T13:03:45.5462204Z HEAD is now at ffda10e (windows CI update)
2019-07-21T13:03:45.5462204Z HEAD is now at ffda10e (windows CI update)
2019-07-21T13:03:46.4554957Z ##[error]Bash exited with code '1'.
2019-07-21T13:03:46.4592474Z ##[section]Starting: Checkout
2019-07-21T13:03:46.4594338Z ==============================================================================
2019-07-21T13:03:46.4594392Z Task         : Get sources
2019-07-21T13:03:46.4594437Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
