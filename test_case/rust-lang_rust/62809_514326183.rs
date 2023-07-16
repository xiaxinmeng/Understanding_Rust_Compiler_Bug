plain
2019-07-23T15:00:18.2852018Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-23T15:00:18.3015209Z ##[command]git config gc.auto 0
2019-07-23T15:00:18.3086564Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-23T15:00:18.3130258Z ##[command]git config --get-all http.proxy
2019-07-23T15:00:18.3262657Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62809/merge:refs/remotes/pull/62809/merge
---
2019-07-23T15:00:54.7786475Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-23T15:00:54.7786789Z 
2019-07-23T15:00:54.7787530Z   git checkout -b <new-branch-name>
2019-07-23T15:00:54.7787806Z 
2019-07-23T15:00:54.7788196Z HEAD is now at 0a4e77443 Merge c696d5701c10f3de4e25802649f05595dfcdcc23 into 3ebca72a11869f946b31f900faffb75c2bb2473a
2019-07-23T15:00:54.7921011Z ##[section]Finishing: Checkout
2019-07-23T15:00:54.8015310Z ##[section]Starting: Decide whether to run this job
2019-07-23T15:00:54.8018130Z Task         : Bash
2019-07-23T15:00:54.8018174Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-23T15:00:54.8018224Z Version      : 3.151.3
2019-07-23T15:00:54.8018263Z Author       : Microsoft Corporation
2019-07-23T15:00:54.8018263Z Author       : Microsoft Corporation
2019-07-23T15:00:54.8018440Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-07-23T15:00:54.8018497Z ==============================================================================
2019-07-23T15:00:54.9302916Z Generating script.
2019-07-23T15:00:54.9336169Z ========================== Starting Command Output ===========================
2019-07-23T15:00:54.9355686Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/669c1dd9-3c95-4ebb-9791-ecdd4f1f4ae9.sh
2019-07-23T15:00:55.2814618Z Executing the job since submodules are updated
2019-07-23T15:00:55.2842504Z ##[section]Finishing: Decide whether to run this job
2019-07-23T15:00:55.2852168Z ==============================================================================
2019-07-23T15:00:55.2852261Z Task         : Bash
2019-07-23T15:00:55.2852303Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-23T15:00:55.2852378Z Version      : 3.151.3
---
2019-07-23T16:27:06.2975349Z [ 69%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/MipsISelLowering.cpp.o
2019-07-23T16:27:07.7337699Z [ 69%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64SelectionDAGInfo.cpp.o
2019-07-23T16:27:14.1936169Z [ 69%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64SpeculationHardening.cpp.o
2019-07-23T16:27:17.9869442Z [ 69%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/MipsFrameLowering.cpp.o
2019-07-23T16:27:21.5271755Z [ 69%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64StackTagging.cpp.o
2019-07-23T16:27:29.3455658Z [ 69%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64StorePairSuppress.cpp.o
2019-07-23T16:27:31.6137002Z [ 69%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/MipsBranchExpansion.cpp.o
2019-07-23T16:27:34.8029296Z [ 69%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64Subtarget.cpp.o
2019-07-23T16:27:39.6413335Z [ 69%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/MipsMCInstLower.cpp.o
---
2019-07-23T18:20:48.8454811Z Cloning into 'rust-toolstate'...
2019-07-23T18:20:49.4856614Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-23T18:20:49.5113306Z [master 4145817] (linux CI update)
2019-07-23T18:20:49.5113443Z  1 file changed, 1 insertion(+)
2019-07-23T18:20:50.2054252Z remote: Invalid username or password.
2019-07-23T18:20:50.2055049Z fatal: Authentication failed for 'https://github.com/rust-lang-nursery/rust-toolstate.git/'
2019-07-23T18:20:53.5650443Z  * branch            master     -> FETCH_HEAD
2019-07-23T18:20:53.5816549Z HEAD is now at 8007779 (linux CI update)
2019-07-23T18:20:53.5926094Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-23T18:20:53.6120982Z [master 3248da4] (linux CI update)
2019-07-23T18:20:53.6120982Z [master 3248da4] (linux CI update)
2019-07-23T18:20:53.6121687Z  1 file changed, 1 insertion(+)
2019-07-23T18:20:53.9347338Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-23T18:20:57.3021823Z  * branch            master     -> FETCH_HEAD
2019-07-23T18:20:57.3170039Z HEAD is now at 8007779 (linux CI update)
2019-07-23T18:20:57.3276717Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-23T18:20:57.3453298Z [master 99ccde3] (linux CI update)
2019-07-23T18:20:57.3453298Z [master 99ccde3] (linux CI update)
2019-07-23T18:20:57.3454078Z  1 file changed, 1 insertion(+)
2019-07-23T18:20:57.6566247Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-23T18:20:59.0370527Z  * branch            master     -> FETCH_HEAD
2019-07-23T18:20:59.0517907Z HEAD is now at 8007779 (linux CI update)
2019-07-23T18:20:59.0626785Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-23T18:20:59.0804604Z [master 5718258] (linux CI update)
2019-07-23T18:20:59.0804604Z [master 5718258] (linux CI update)
2019-07-23T18:20:59.0805578Z  1 file changed, 1 insertion(+)
2019-07-23T18:20:59.3957231Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-23T18:21:03.1844800Z  * branch            master     -> FETCH_HEAD
2019-07-23T18:21:03.1995751Z HEAD is now at 8007779 (linux CI update)
2019-07-23T18:21:03.2106913Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-23T18:21:03.2288538Z [master 8aec8a5] (linux CI update)
2019-07-23T18:21:03.2288538Z [master 8aec8a5] (linux CI update)
2019-07-23T18:21:03.2288627Z  1 file changed, 1 insertion(+)
2019-07-23T18:21:03.9514644Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-23T18:21:05.5661419Z  * branch            master     -> FETCH_HEAD
2019-07-23T18:21:05.5809460Z HEAD is now at 8007779 (linux CI update)
2019-07-23T18:21:05.5809460Z HEAD is now at 8007779 (linux CI update)
2019-07-23T18:21:06.4651410Z ##[error]Bash exited with code '1'.
2019-07-23T18:21:06.4705526Z ##[section]Starting: Checkout
2019-07-23T18:21:06.4707868Z ==============================================================================
2019-07-23T18:21:06.4707927Z Task         : Get sources
2019-07-23T18:21:06.4707974Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
