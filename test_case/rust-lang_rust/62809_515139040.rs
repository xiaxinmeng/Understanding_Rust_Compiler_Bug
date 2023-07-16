plain
2019-07-25T14:09:36.5726176Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-25T14:09:36.5947699Z ##[command]git config gc.auto 0
2019-07-25T14:09:36.6012524Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-25T14:09:36.6077400Z ##[command]git config --get-all http.proxy
2019-07-25T14:09:36.6215606Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62809/merge:refs/remotes/pull/62809/merge
---
2019-07-25T14:10:11.8590916Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-25T14:10:11.8590949Z 
2019-07-25T14:10:11.8591178Z   git checkout -b <new-branch-name>
2019-07-25T14:10:11.8591207Z 
2019-07-25T14:10:11.8591270Z HEAD is now at 93928a558 Merge c9529d8c4b614afc19334753b5425c34fb0290fd into eedf6ce4ef54bb03818ab21d714f1b9f13a6b31c
2019-07-25T14:10:11.8750325Z ##[section]Finishing: Checkout
2019-07-25T14:10:11.8757714Z ##[section]Starting: Decide whether to run this job
2019-07-25T14:10:11.8761398Z Task         : Bash
2019-07-25T14:10:11.8761446Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-25T14:10:11.8761494Z Version      : 3.151.3
2019-07-25T14:10:11.8761557Z Author       : Microsoft Corporation
2019-07-25T14:10:11.8761557Z Author       : Microsoft Corporation
2019-07-25T14:10:11.8761611Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-07-25T14:10:11.8761667Z ==============================================================================
2019-07-25T14:10:12.0115681Z Generating script.
2019-07-25T14:10:12.0148899Z ========================== Starting Command Output ===========================
2019-07-25T14:10:12.0167188Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/6cd29aef-c367-4b15-ba71-1cfa0661d968.sh
2019-07-25T14:10:12.3826865Z Executing the job since submodules are updated
2019-07-25T14:10:12.3921795Z ##[section]Finishing: Decide whether to run this job
2019-07-25T14:10:12.3931749Z ==============================================================================
2019-07-25T14:10:12.3931808Z Task         : Bash
2019-07-25T14:10:12.3931855Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-25T14:10:12.3931940Z Version      : 3.151.3
---
2019-07-25T15:35:31.2704754Z [ 69%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/MipsISelDAGToDAG.cpp.o
2019-07-25T15:35:32.6893994Z [ 69%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64SelectionDAGInfo.cpp.o
2019-07-25T15:35:38.9473415Z [ 69%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/MipsISelLowering.cpp.o
2019-07-25T15:35:39.1505693Z [ 69%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64SpeculationHardening.cpp.o
2019-07-25T15:35:46.2742733Z [ 69%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64StackTagging.cpp.o
2019-07-25T15:35:53.9503084Z [ 69%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64StorePairSuppress.cpp.o
2019-07-25T15:35:57.8673491Z [ 69%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/MipsLegalizerInfo.cpp.o
2019-07-25T15:35:59.3118880Z [ 69%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64Subtarget.cpp.o
2019-07-25T15:36:03.9771189Z [ 69%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/MipsBranchExpansion.cpp.o
---
2019-07-25T17:33:20.6412532Z Cloning into 'rust-toolstate'...
2019-07-25T17:33:21.2818483Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-25T17:33:21.3069389Z [master 1ff2519] (linux CI update)
2019-07-25T17:33:21.3069526Z  1 file changed, 1 insertion(+)
2019-07-25T17:33:21.9780238Z remote: Invalid username or password.
2019-07-25T17:33:21.9780993Z fatal: Authentication failed for 'https://github.com/rust-lang-nursery/rust-toolstate.git/'
2019-07-25T17:33:25.3077553Z  * branch            master     -> FETCH_HEAD
2019-07-25T17:33:25.3250437Z HEAD is now at a587986 (windows CI update)
2019-07-25T17:33:25.3364444Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-25T17:33:25.3548282Z [master 257d30b] (linux CI update)
2019-07-25T17:33:25.3548282Z [master 257d30b] (linux CI update)
2019-07-25T17:33:25.3549002Z  1 file changed, 1 insertion(+)
2019-07-25T17:33:25.6422950Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-25T17:33:26.9868164Z  * branch            master     -> FETCH_HEAD
2019-07-25T17:33:27.0016113Z HEAD is now at a587986 (windows CI update)
2019-07-25T17:33:27.0130139Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-25T17:33:27.0317047Z [master 6c62b31] (linux CI update)
2019-07-25T17:33:27.0317047Z [master 6c62b31] (linux CI update)
2019-07-25T17:33:27.0317346Z  1 file changed, 1 insertion(+)
2019-07-25T17:33:27.3149768Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-25T17:33:27.6386131Z  * branch            master     -> FETCH_HEAD
2019-07-25T17:33:27.6538402Z HEAD is now at a587986 (windows CI update)
2019-07-25T17:33:27.6651251Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-25T17:33:27.6840843Z [master 6c62b31] (linux CI update)
2019-07-25T17:33:27.6840843Z [master 6c62b31] (linux CI update)
2019-07-25T17:33:27.6841277Z  1 file changed, 1 insertion(+)
2019-07-25T17:33:27.9772865Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-25T17:33:28.2959469Z  * branch            master     -> FETCH_HEAD
2019-07-25T17:33:28.3107686Z HEAD is now at a587986 (windows CI update)
2019-07-25T17:33:28.3216445Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-25T17:33:28.3401236Z [master 01c3425] (linux CI update)
2019-07-25T17:33:28.3401236Z [master 01c3425] (linux CI update)
2019-07-25T17:33:28.3401369Z  1 file changed, 1 insertion(+)
2019-07-25T17:33:28.6577396Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-25T17:33:31.9969104Z  * branch            master     -> FETCH_HEAD
2019-07-25T17:33:32.0118621Z HEAD is now at a587986 (windows CI update)
2019-07-25T17:33:32.0118621Z HEAD is now at a587986 (windows CI update)
2019-07-25T17:33:32.5614929Z ##[error]Bash exited with code '1'.
2019-07-25T17:33:32.5657329Z ##[section]Starting: Checkout
2019-07-25T17:33:32.5659075Z ==============================================================================
2019-07-25T17:33:32.5659144Z Task         : Get sources
2019-07-25T17:33:32.5659191Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
