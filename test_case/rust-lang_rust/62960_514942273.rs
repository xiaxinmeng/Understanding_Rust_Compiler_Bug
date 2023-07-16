plain
2019-07-25T02:05:25.4200059Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-25T02:05:25.4439102Z ##[command]git config gc.auto 0
2019-07-25T02:05:25.4506758Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-25T02:05:25.4560065Z ##[command]git config --get-all http.proxy
2019-07-25T02:05:25.4698000Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62960/merge:refs/remotes/pull/62960/merge
---
2019-07-25T02:05:59.5753472Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-25T02:05:59.5753740Z 
2019-07-25T02:05:59.5754161Z   git checkout -b <new-branch-name>
2019-07-25T02:05:59.5754330Z 
2019-07-25T02:05:59.5754535Z HEAD is now at 848e06b3b Merge 5647078b3181a1f54bc80a727b2c00a433d37bf5 into 03f19f7ff128a3b01eeab3f87f04cce22883f006
2019-07-25T02:05:59.5886710Z ##[section]Finishing: Checkout
2019-07-25T02:05:59.5892068Z ##[section]Starting: Decide whether to run this job
2019-07-25T02:05:59.5895842Z Task         : Bash
2019-07-25T02:05:59.5895888Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-25T02:05:59.5895934Z Version      : 3.151.3
2019-07-25T02:05:59.5895996Z Author       : Microsoft Corporation
2019-07-25T02:05:59.5895996Z Author       : Microsoft Corporation
2019-07-25T02:05:59.5896047Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-07-25T02:05:59.5896097Z ==============================================================================
2019-07-25T02:05:59.7112108Z Generating script.
2019-07-25T02:05:59.7142353Z ========================== Starting Command Output ===========================
2019-07-25T02:05:59.7170921Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/8c27c8ce-1081-4580-8407-ba26aabcd7e2.sh
2019-07-25T02:06:00.0265006Z Executing the job since submodules are updated
2019-07-25T02:06:00.0346921Z ##[section]Finishing: Decide whether to run this job
2019-07-25T02:06:00.0356024Z ==============================================================================
2019-07-25T02:06:00.0356134Z Task         : Bash
2019-07-25T02:06:00.0356180Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-25T02:06:00.0356229Z Version      : 3.151.3
---
2019-07-25T04:05:22.1981936Z Cloning into 'rust-toolstate'...
2019-07-25T04:05:22.8520030Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-25T04:05:22.8747787Z [master 0c78f61] (linux CI update)
2019-07-25T04:05:22.8748114Z  1 file changed, 1 insertion(+)
2019-07-25T04:05:23.5032009Z remote: Invalid username or password.
2019-07-25T04:05:23.5033798Z fatal: Authentication failed for 'https://github.com/rust-lang-nursery/rust-toolstate.git/'
2019-07-25T04:05:25.8148324Z  * branch            master     -> FETCH_HEAD
2019-07-25T04:05:25.8297685Z HEAD is now at c8d66de (windows CI update)
2019-07-25T04:05:25.8399473Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-25T04:05:25.8569580Z [master 63cd501] (linux CI update)
2019-07-25T04:05:25.8569580Z [master 63cd501] (linux CI update)
2019-07-25T04:05:25.8569826Z  1 file changed, 1 insertion(+)
2019-07-25T04:05:26.1676440Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-25T04:05:26.4794900Z  * branch            master     -> FETCH_HEAD
2019-07-25T04:05:26.4927622Z HEAD is now at c8d66de (windows CI update)
2019-07-25T04:05:26.5032433Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-25T04:05:26.5202207Z [master 904f54b] (linux CI update)
2019-07-25T04:05:26.5202207Z [master 904f54b] (linux CI update)
2019-07-25T04:05:26.5202805Z  1 file changed, 1 insertion(+)
2019-07-25T04:05:26.7982007Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-25T04:05:27.1111554Z  * branch            master     -> FETCH_HEAD
2019-07-25T04:05:27.1262988Z HEAD is now at c8d66de (windows CI update)
2019-07-25T04:05:27.1372924Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-25T04:05:27.1558066Z [master 4baa60e] (linux CI update)
2019-07-25T04:05:27.1558066Z [master 4baa60e] (linux CI update)
2019-07-25T04:05:27.1558140Z  1 file changed, 1 insertion(+)
2019-07-25T04:05:27.4460481Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-25T04:05:28.9160186Z  * branch            master     -> FETCH_HEAD
2019-07-25T04:05:28.9297017Z HEAD is now at c8d66de (windows CI update)
2019-07-25T04:05:28.9403378Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-25T04:05:28.9572688Z [master 3aa50b7] (linux CI update)
2019-07-25T04:05:28.9572688Z [master 3aa50b7] (linux CI update)
2019-07-25T04:05:28.9573306Z  1 file changed, 1 insertion(+)
2019-07-25T04:05:29.2581547Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-25T04:05:32.5690880Z  * branch            master     -> FETCH_HEAD
2019-07-25T04:05:32.5811812Z HEAD is now at c8d66de (windows CI update)
2019-07-25T04:05:32.5811812Z HEAD is now at c8d66de (windows CI update)
2019-07-25T04:05:33.6410980Z ##[error]Bash exited with code '1'.
2019-07-25T04:05:33.6446152Z ##[section]Starting: Checkout
2019-07-25T04:05:33.6448072Z ==============================================================================
2019-07-25T04:05:33.6448114Z Task         : Get sources
2019-07-25T04:05:33.6448163Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
