plain
##[section]Starting: Linux x86_64-gnu-tools
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 3'
Agent machine name: 'fv-az578'
Current agent version: '2.169.1'
##[group]Operating System
16.04.6
LTS
LTS
##[endgroup]
##[group]Virtual Environment
Environment: ubuntu-16.04
Version: 20200517.1
Included Software: https://github.com/actions/virtual-environments/blob/ubuntu16/20200517.1/images/linux/Ubuntu1604-README.md
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/73f75eef-bdf4-43db-9593-3ef7ece9731a.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/73085/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/73085/merge:refs/remotes/pull/73085/merge
fatal: couldn't find remote ref refs/pull/73085/merge
##[warning]Git fetch failed with exit code 128, back off 7.399 seconds before retry.
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/73085/merge:refs/remotes/pull/73085/merge
fatal: couldn't find remote ref refs/pull/73085/merge
##[warning]Git fetch failed with exit code 128, back off 2.304 seconds before retry.
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/73085/merge:refs/remotes/pull/73085/merge
fatal: couldn't find remote ref refs/pull/73085/merge
##[error]Git fetch failed with exit code: 128
##[section]Finishing: Checkout rust-lang/rust@refs/pull/73085/merge to s
##[section]Starting: Checkout rust-lang/rust@refs/pull/73085/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
##[section]Finishing: Checkout rust-lang/rust@refs/pull/73085/merge to s
Cleaning up task key
Start cleaning up orphan processes.
##[section]Finishing: Finalize Job
##[section]Finishing: Linux x86_64-gnu-tools
