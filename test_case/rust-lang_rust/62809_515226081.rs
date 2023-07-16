plain
2019-07-25T18:26:24.2431296Z ##[command]git remote add origin https://github.com/rust-lang/rust
2019-07-25T18:26:24.2633298Z ##[command]git config gc.auto 0
2019-07-25T18:26:24.2709658Z ##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
2019-07-25T18:26:24.2756043Z ##[command]git config --get-all http.proxy
2019-07-25T18:26:24.2894515Z ##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/62809/merge:refs/remotes/pull/62809/merge
---
2019-07-25T18:26:57.7362235Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-25T18:26:57.7362262Z 
2019-07-25T18:26:57.7362646Z   git checkout -b <new-branch-name>
2019-07-25T18:26:57.7362673Z 
2019-07-25T18:26:57.7362715Z HEAD is now at 593ba213f Merge dc50a633f3260a3aeb79a4ca9800587be7f732e7 into eedf6ce4ef54bb03818ab21d714f1b9f13a6b31c
2019-07-25T18:26:57.7510806Z ##[section]Finishing: Checkout
2019-07-25T18:26:57.7518402Z ##[section]Starting: Decide whether to run this job
2019-07-25T18:26:57.7520877Z Task         : Bash
2019-07-25T18:26:57.7520935Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-25T18:26:57.7521102Z Version      : 3.151.3
2019-07-25T18:26:57.7521140Z Author       : Microsoft Corporation
2019-07-25T18:26:57.7521140Z Author       : Microsoft Corporation
2019-07-25T18:26:57.7521200Z Help         : https://docs.microsoft.com/azure/devops/pipelines/tasks/utility/bash
2019-07-25T18:26:57.7521244Z ==============================================================================
2019-07-25T18:26:57.8849822Z Generating script.
2019-07-25T18:26:57.8881342Z ========================== Starting Command Output ===========================
2019-07-25T18:26:57.8903554Z [command]/bin/bash --noprofile --norc /home/vsts/work/_temp/6dcdbbcd-ac3f-4bed-9fea-8c929086552c.sh
2019-07-25T18:26:58.2097137Z Executing the job since submodules are updated
2019-07-25T18:26:58.2197431Z ##[section]Finishing: Decide whether to run this job
2019-07-25T18:26:58.2208298Z ==============================================================================
2019-07-25T18:26:58.2208353Z Task         : Bash
2019-07-25T18:26:58.2208417Z Description  : Run a Bash script on macOS, Linux, or Windows
2019-07-25T18:26:58.2208592Z Version      : 3.151.3
---
2019-07-25T19:53:04.9763051Z [ 69%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/MipsISelDAGToDAG.cpp.o
2019-07-25T19:53:10.2308378Z [ 69%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64SelectionDAGInfo.cpp.o
2019-07-25T19:53:12.4585443Z [ 69%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/MipsISelLowering.cpp.o
2019-07-25T19:53:16.5634735Z [ 69%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64SpeculationHardening.cpp.o
2019-07-25T19:53:23.5003120Z [ 69%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64StackTagging.cpp.o
2019-07-25T19:53:30.2891621Z [ 69%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/MipsLegalizerInfo.cpp.o
2019-07-25T19:53:31.2317445Z [ 69%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64StorePairSuppress.cpp.o
2019-07-25T19:53:36.3869472Z [ 69%] Building CXX object lib/Target/AArch64/CMakeFiles/LLVMAArch64CodeGen.dir/AArch64Subtarget.cpp.o
2019-07-25T19:53:36.6958225Z [ 69%] Building CXX object lib/Target/Mips/CMakeFiles/LLVMMipsCodeGen.dir/MipsBranchExpansion.cpp.o
---
2019-07-25T21:43:11.6581966Z Cloning into 'rust-toolstate'...
2019-07-25T21:43:12.3090009Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-25T21:43:12.3307706Z [master 6ffac73] (linux CI update)
2019-07-25T21:43:12.3309326Z  1 file changed, 1 insertion(+)
2019-07-25T21:43:12.9438737Z remote: Invalid username or password.
2019-07-25T21:43:12.9439619Z fatal: Authentication failed for 'https://github.com/rust-lang-nursery/rust-toolstate.git/'
2019-07-25T21:43:16.2460484Z  * branch            master     -> FETCH_HEAD
2019-07-25T21:43:16.2630045Z HEAD is now at 994f84a (windows CI update)
2019-07-25T21:43:16.2749892Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-25T21:43:16.2906775Z [master 4e0c1f5] (linux CI update)
2019-07-25T21:43:16.2906775Z [master 4e0c1f5] (linux CI update)
2019-07-25T21:43:16.2907769Z  1 file changed, 1 insertion(+)
2019-07-25T21:43:16.5796432Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-25T21:43:18.9191678Z  * branch            master     -> FETCH_HEAD
2019-07-25T21:43:18.9332104Z HEAD is now at 994f84a (windows CI update)
2019-07-25T21:43:18.9435096Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-25T21:43:18.9591663Z [master 666c8ef] (linux CI update)
2019-07-25T21:43:18.9591663Z [master 666c8ef] (linux CI update)
2019-07-25T21:43:18.9591741Z  1 file changed, 1 insertion(+)
2019-07-25T21:43:19.2355184Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-25T21:43:19.5427166Z  * branch            master     -> FETCH_HEAD
2019-07-25T21:43:19.5565835Z HEAD is now at 994f84a (windows CI update)
2019-07-25T21:43:19.5667530Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-25T21:43:19.5828562Z [master 12cfd22] (linux CI update)
2019-07-25T21:43:19.5828562Z [master 12cfd22] (linux CI update)
2019-07-25T21:43:19.5829090Z  1 file changed, 1 insertion(+)
2019-07-25T21:43:19.8607832Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-25T21:43:22.1643726Z  * branch            master     -> FETCH_HEAD
2019-07-25T21:43:22.1776511Z HEAD is now at 994f84a (windows CI update)
2019-07-25T21:43:22.1876844Z The state of "rustc-guide" has changed from "test-pass" to ""
2019-07-25T21:43:22.2040255Z [master e40a318] (linux CI update)
2019-07-25T21:43:22.2040255Z [master e40a318] (linux CI update)
2019-07-25T21:43:22.2040679Z  1 file changed, 1 insertion(+)
2019-07-25T21:43:22.4798321Z fatal: could not read Username for 'https://github.com': No such device or address
2019-07-25T21:43:24.7865019Z  * branch            master     -> FETCH_HEAD
2019-07-25T21:43:24.8041671Z HEAD is now at 994f84a (windows CI update)
2019-07-25T21:43:24.8041671Z HEAD is now at 994f84a (windows CI update)
2019-07-25T21:43:25.4853774Z ##[error]Bash exited with code '1'.
2019-07-25T21:43:25.4892815Z ##[section]Starting: Checkout
2019-07-25T21:43:25.4894881Z ==============================================================================
2019-07-25T21:43:25.4894943Z Task         : Get sources
2019-07-25T21:43:25.4894995Z Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
