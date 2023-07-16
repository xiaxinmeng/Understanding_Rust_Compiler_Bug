plain
##[section]Starting: Linux mingw-check
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 14'
Agent machine name: 'fv-az619'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/cac80ae1-0bd2-4e39-aa9b-b0e9ee41a7ae.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/73529/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/73529/merge:refs/remotes/pull/73529/merge
---
 ---> a9ec21d337b3
Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
 ---> Using cache
 ---> 5ff2c13d8dba
Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            python3 ../x.py doc --stage 0 src/libstd &&            /scripts/validate-toolstate.sh
 ---> 6b931e755c7e
Successfully built 6b931e755c7e
Successfully tagged rust-ci:latest
Built container sha256:6b931e755c7ea1f69816640eb9df74fafd40a545d0e5aa8341d35009dabb0f3c
---
    Checking alloc v0.0.0 (/checkout/src/liballoc)
error[E0053]: method `from_elem` has an incompatible type for trait
    --> src/liballoc/vec.rs:1808:41
     |
1795 |     fn from_elem(elem: Self, n: usize) -> Vec<Self>;
     |                                           --------- type in trait
...
1808 |     fn from_elem(elem: i8, n: usize) -> Vec<u8> {
     |                                         ^^^^^^^ expected `i8`, found `u8`
     = note: expected fn pointer `fn(_, _) -> vec::Vec<i8>`
                found fn pointer `fn(_, _) -> vec::Vec<u8>`

    Checking cfg-if v0.1.10
---
  local time: Sat Jun 20 03:51:35 UTC 2020
  network time: Sat, 20 Jun 2020 03:51:35 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/73529/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/73529/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (7018) (python)
##[section]Finishing: Finalize Job
