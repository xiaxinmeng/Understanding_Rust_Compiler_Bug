plain
##[section]Starting: Windows x86_64-mingw-1
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 6'
Agent machine name: 'fv-az379'
Current agent version: '2.168.2'
##[group]Operating System
10.0.14393
Datacenter
Datacenter
##[endgroup]
##[group]Virtual Environment
Environment: windows-2016
Version: 20200505.1
Included Software: https://github.com/actions/virtual-environments/blob/win16/20200505.1/images/win/Windows2016-Readme.md
##[endgroup]
Agent running as: 'VssAdministrator'
Prepare build directory.
Set build variables.
Download all required tasks.
Download all required tasks.
Downloading task: Bash (3.163.2)
Checking job knob settings.
   Knob: AgentToolsDirectory = C:/hostedtoolcache/windows Source: ${AGENT_TOOLSDIRECTORY} 
   Knob: AgentPerflog = c:\vsts\perflog Source: ${VSTS_AGENT_PERFLOG} 
Start tracking orphan processes.
##[section]Finishing: Initialize job
##[section]Starting: Configure Job Name
==============================================================================
---
========================== Starting Command Output ===========================
[command]"C:\Program Files\Git\bin\bash.exe" --noprofile --norc /d/a/_temp/61a0c17e-ae35-4517-8aca-6f3515860a86.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72384/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72384/merge:refs/remotes/pull/72384/merge
