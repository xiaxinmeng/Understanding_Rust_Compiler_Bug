plain
##[section]Starting: Linux x86_64-gnu-tools
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 11'
Agent machine name: 'fv-az578'
Current agent version: '2.170.1'
##[group]Operating System
16.04.6
LTS
LTS
##[endgroup]
##[group]Virtual Environment
Environment: ubuntu-16.04
Version: 20200614.1
Included Software: https://github.com/actions/virtual-environments/blob/ubuntu16/20200614.1/images/linux/Ubuntu1604-README.md
##[endgroup]
Agent running as: 'vsts'
Prepare build directory.
Set build variables.
Download all required tasks.
Download all required tasks.
Downloading task: Bash (3.171.1)
Checking job knob settings.
   Knob: AgentToolsDirectory = /opt/hostedtoolcache Source: ${AGENT_TOOLSDIRECTORY} 
   Knob: AgentPerflog = /home/vsts/perflog Source: ${VSTS_AGENT_PERFLOG} 
Start tracking orphan processes.
##[section]Finishing: Initialize job
##[section]Starting: Configure Job Name
==============================================================================
---
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/6852ec49-2e97-4b22-aeb2-73067fad6a4f.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/73418/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/73418/merge:refs/remotes/pull/73418/merge
---

######################################################################## 100.0%
extracting /checkout/obj/build/cache/2020-04-22/rustfmt-nightly-x86_64-unknown-linux-gnu.tar.xz
    Updating crates.io index
error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
If you want to try to generate the lock file without accessing the network, use the --offline flag.
Build completed unsuccessfully in 0:00:40
Makefile:60: recipe for target 'prepare' failed
make: *** [prepare] Error 1
Command failed. Attempt 2/5:
Command failed. Attempt 2/5:
    Updating crates.io index
error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
If you want to try to generate the lock file without accessing the network, use the --offline flag.
Build completed unsuccessfully in 0:00:00
Makefile:60: recipe for target 'prepare' failed
make: *** [prepare] Error 1
Command failed. Attempt 3/5:
Command failed. Attempt 3/5:
    Updating crates.io index
error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
If you want to try to generate the lock file without accessing the network, use the --offline flag.
Build completed unsuccessfully in 0:00:00
Makefile:60: recipe for target 'prepare' failed
make: *** [prepare] Error 1
Command failed. Attempt 4/5:
Command failed. Attempt 4/5:
    Updating crates.io index
error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
If you want to try to generate the lock file without accessing the network, use the --offline flag.
Build completed unsuccessfully in 0:00:00
Makefile:60: recipe for target 'prepare' failed
make: *** [prepare] Error 1
Command failed. Attempt 5/5:
Command failed. Attempt 5/5:
    Updating crates.io index
error: the lock file /checkout/Cargo.lock needs to be updated but --locked was passed to prevent this
If you want to try to generate the lock file without accessing the network, use the --offline flag.
Build completed unsuccessfully in 0:00:00
Makefile:60: recipe for target 'prepare' failed
make: *** [prepare] Error 1
The command has failed after 5 attempts.
The command has failed after 5 attempts.
== clock drift check ==
  local time: Tue Jun 23 15:33:51 UTC 2020
  network time: Tue, 23 Jun 2020 15:33:51 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/73418/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/73418/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3688) (python)
##[section]Finishing: Finalize Job
