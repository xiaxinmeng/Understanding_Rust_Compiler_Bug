plain
##[section]Starting: Linux mingw-check
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 12'
Agent machine name: 'fv-az578'
Current agent version: '2.170.1'
##[group]Operating System
16.04.6
LTS
LTS
##[endgroup]
##[group]Virtual Environment
Environment: ubuntu-16.04
Version: 20200604.1
Included Software: https://github.com/actions/virtual-environments/blob/ubuntu16/20200604.1/images/linux/Ubuntu1604-README.md
##[endgroup]
Agent running as: 'vsts'
Prepare build directory.
Set build variables.
Download all required tasks.
Download all required tasks.
Downloading task: Bash (3.163.3)
Checking job knob settings.
   Knob: AgentToolsDirectory = /opt/hostedtoolcache Source: ${AGENT_TOOLSDIRECTORY} 
   Knob: AgentPerflog = /home/vsts/perflog Source: ${VSTS_AGENT_PERFLOG} 
Start tracking orphan processes.
##[section]Finishing: Initialize job
##[section]Starting: Configure Job Name
==============================================================================
---
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/3eb35ef8-c582-4d10-9fa6-bda1b21e2099.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/73139/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/73139/merge:refs/remotes/pull/73139/merge
---
 ---> f883e675ad62
Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
 ---> Using cache
 ---> c0b156eb069c
Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            python3 ../x.py doc --stage 0 src/libstd &&            /scripts/validate-toolstate.sh
 ---> 8541bab6b38c
Successfully built 8541bab6b38c
Successfully tagged rust-ci:latest
Built container sha256:8541bab6b38c07f1b7eb787539b9cbe93daa6ac4458d3d7bd8a8921622a14ba1
---
  local time: Thu Jun 11 14:31:08 UTC 2020
  network time: Thu, 11 Jun 2020 14:31:08 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/73139/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/73139/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3750) (python)
##[section]Finishing: Finalize Job
