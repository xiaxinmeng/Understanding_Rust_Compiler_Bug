plain
2019-07-24T21:46:33.8058253Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-24T21:46:33.8258023Z ##[command]git config gc.auto 0
2019-07-24T21:46:34.4579320Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-24T21:46:34.4582687Z ##[command]git config --get-all http.proxy
2019-07-24T21:46:34.4585363Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62766/merge:refs/remotes/pull/62766/merge
---
2019-07-24T21:47:08.8215754Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-24T21:47:08.8215788Z 
2019-07-24T21:47:08.8216051Z   git checkout -b <new-branch-name>
2019-07-24T21:47:08.8216083Z 
2019-07-24T21:47:08.8216136Z HEAD is now at 821014f96 Merge 2ad13354f4f7c1e6a78ee2d9b808cc766ddaf259 into 03f19f7ff128a3b01eeab3f87f04cce22883f006
2019-07-24T21:47:08.8367787Z ##[section]Finishing: Checkout
2019-07-24T21:47:08.8374227Z ##[section]Starting: Decide whether to run this job
2019-07-24T21:47:08.8377289Z Task         : Bash
2019-07-24T21:47:08.8377351Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-24T21:47:08.8377395Z Version      : 3.151.3
2019-07-24T21:47:08.8377453Z Author       : Microsoft Corporation
2019-07-24T21:47:08.8377453Z Author       : Microsoft Corporation
2019-07-24T21:47:08.8377516Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-07-24T21:47:08.8377568Z ==============================================================================
2019-07-24T21:47:08.9688340Z Generating script.
2019-07-24T21:47:08.9728479Z ========================== Starting Command Output ===========================
2019-07-24T21:47:08.9755397Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/0a4e69ad-2f83-4d49-a5d1-764a6014ecf9.sh
2019-07-24T21:47:09.3644984Z Executing the job since submodules are updated
2019-07-24T21:47:09.3707157Z ##[section]Finishing: Decide whether to run this job
2019-07-24T21:47:09.3719511Z ==============================================================================
2019-07-24T21:47:09.3719575Z Task         : Bash
2019-07-24T21:47:09.3719671Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-24T21:47:09.3719743Z Version      : 3.151.3
---
2019-07-24T23:53:36.6615622Z Cloning into 'rust-toolstate'...
2019-07-24T23:53:37.2899249Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-24T23:53:37.3173260Z [master 0e061cf] (linux CI update)
2019-07-24T23:53:37.3173371Z  1 file changed, 1 insertion(+)
2019-07-24T23:53:37.9326563Z remote: Invalid username or password.
2019-07-24T23:53:37.9328185Z fatal: Authentication failed for 'https://github.com/rust-lang-nursery/rust-toolstate.git/'
2019-07-24T23:53:38.2600850Z  * branch            master     -> FETCH_HEAD
2019-07-24T23:53:38.2786598Z HEAD is now at 2a2a469 (windows CI update)
2019-07-24T23:53:38.2917952Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-24T23:53:38.3109878Z [master 4d324d3] (linux CI update)
2019-07-24T23:53:38.3109878Z [master 4d324d3] (linux CI update)
2019-07-24T23:53:38.3109950Z  1 file changed, 1 insertion(+)
2019-07-24T23:53:38.6262052Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-24T23:53:39.9538034Z  * branch            master     -> FETCH_HEAD
2019-07-24T23:53:39.9697090Z HEAD is now at 2a2a469 (windows CI update)
2019-07-24T23:53:39.9822609Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-24T23:53:39.9999815Z [master 47d4d3c] (linux CI update)
2019-07-24T23:53:39.9999815Z [master 47d4d3c] (linux CI update)
2019-07-24T23:53:40.0001185Z  1 file changed, 1 insertion(+)
2019-07-24T23:53:40.2991140Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-24T23:53:42.6318789Z  * branch            master     -> FETCH_HEAD
2019-07-24T23:53:42.6496175Z HEAD is now at 2a2a469 (windows CI update)
2019-07-24T23:53:42.6628130Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-24T23:53:42.6844944Z [master 151d16c] (linux CI update)
2019-07-24T23:53:42.6844944Z [master 151d16c] (linux CI update)
2019-07-24T23:53:42.6845061Z  1 file changed, 1 insertion(+)
2019-07-24T23:53:42.9805837Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-24T23:53:45.3055528Z  * branch            master     -> FETCH_HEAD
2019-07-24T23:53:45.3237625Z HEAD is now at 2a2a469 (windows CI update)
2019-07-24T23:53:45.3380977Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-24T23:53:45.3589895Z [master e210219] (linux CI update)
2019-07-24T23:53:45.3589895Z [master e210219] (linux CI update)
2019-07-24T23:53:45.3590404Z  1 file changed, 1 insertion(+)
2019-07-24T23:53:45.6525670Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-24T23:53:48.9763125Z  * branch            master     -> FETCH_HEAD
2019-07-24T23:53:48.9951820Z HEAD is now at 2a2a469 (windows CI update)
2019-07-24T23:53:48.9951820Z HEAD is now at 2a2a469 (windows CI update)
2019-07-24T23:53:50.0317759Z ##[error]Bash exited with code '1'.
2019-07-24T23:53:50.0355551Z ##[section]Starting: Checkout
2019-07-24T23:53:50.0357602Z ==============================================================================
2019-07-24T23:53:50.0357660Z Task         : Get sources
2019-07-24T23:53:50.0357723Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
