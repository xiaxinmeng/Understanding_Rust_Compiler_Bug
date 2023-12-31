plain
##[section]Starting: Linux mingw-check
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 67'
Agent machine name: 'fv-az619'
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
Downloading task: Bash (3.163.2)
Checking job knob settings.
   Knob: AgentToolsDirectory = /opt/hostedtoolcache Source: ${AGENT_TOOLSDIRECTORY} 
   Knob: AgentPerflog = /home/vsts/perflog Source: ${VSTS_AGENT_PERFLOG} 
Start tracking orphan processes.
##[section]Finishing: Initialize job
##[section]Starting: Configure Job Name
==============================================================================
---
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/4394de8a-9282-46f3-aad5-d25ebf058c0b.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72746/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72746/merge:refs/remotes/pull/72746/merge
---
 ---> 3adb0605cc65
Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
 ---> Using cache
 ---> 28dbc326cb7f
Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            python3 ../x.py doc --stage 0 src/libstd &&            /scripts/validate-toolstate.sh
 ---> 537a01811900
Successfully built 537a01811900
Successfully tagged rust-ci:latest
Built container sha256:537a018119009dc218456238dec90b5530050db1e2a1e166550c218003f6159d
---
   Compiling autocfg v0.1.7
   Compiling std v0.0.0 (/checkout/src/libstd)
   Compiling compiler_builtins v0.1.28
   Compiling unwind v0.0.0 (/checkout/src/libunwind)
error: expected one of `.`, `;`, `?`, `}`, or an operator, found `cfg`
   |
   |
93 |             cfg.flag_if_supported("-fvisibility-global-new-delete-hidden")
   |                                                                           - expected one of `.`, `;`, `?`, `}`, or an operator
94 |             cfg.define("_LIBUNWIND_DISABLE_VISIBILITY_ANNOTATIONS", None);

error: aborting due to previous error

error: could not compile `unwind`.
---
  local time: Fri May 29 18:23:53 UTC 2020
  network time: Fri, 29 May 2020 18:23:53 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72746/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72746/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (8327) (python)
##[section]Finishing: Finalize Job
