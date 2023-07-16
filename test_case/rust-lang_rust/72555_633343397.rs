plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 3'
Agent machine name: 'fv-az578'
Current agent version: '2.169.0'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/1f052108-e196-4f72-bfea-788a0f606c33.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72555/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72555/merge:refs/remotes/pull/72555/merge
---
 ---> cb2676f08729
Step 5/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --llvm-root=/usr/lib/llvm-8       --enable-llvm-link-shared       --set rust.thin-lto-import-instr-limit=10
 ---> Using cache
 ---> df25ce111862
Step 6/8 : ENV SCRIPT python2.7 ../x.py test --exclude src/tools/tidy &&            python2.7 ../x.py test src/test/mir-opt --pass=build                                   --target=armv5te-unknown-linux-gnueabi &&            python2.7 ../x.py test src/tools/tidy
 ---> 599b9ac96b27
Step 7/8 : ENV NO_DEBUG_ASSERTIONS=1
 ---> Using cache
 ---> 091087e35a36
---
   Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
   Compiling chalk-rust-ir v0.10.0
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling chalk-solve v0.10.0
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
   Compiling fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
   Compiling chalk-rust-ir v0.10.0
   Compiling rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
   Compiling rustc_hir v0.0.0 (/checkout/src/librustc_hir)
   Compiling rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
   Compiling chalk-solve v0.10.0
   Compiling rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
   Compiling rustc_parse v0.0.0 (/checkout/src/librustc_parse)
   Compiling rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
...........................................................................i........................ 1800/10222
.................................................................................................... 1900/10222
..............................................................................................i..i.. 2000/10222
.................................................................................................... 2100/10222
....................................................................................iiiii........... 2200/10222
.................................................................................................... 2400/10222
.................................................................................................... 2500/10222
.................................................................................................... 2600/10222
.................................................................................................... 2700/10222
---
...........i...............i........................................................................ 5200/10222
.................................................................................................... 5300/10222
..........................................................i......................................... 5400/10222
...................................................i................................................ 5500/10222
..............................................................ii.ii........i...i.....F.............. 5600/10222
....i............................................................................................... 5800/10222
............i....................................................................................... 5900/10222
................................................................ii.................................. 6000/10222
...i................................................................................................ 6100/10222
...i................................................................................................ 6100/10222
.................................................................................................... 6200/10222
.................................................................................................... 6300/10222
.........................ii...i..ii...........i..................................................... 6400/10222
.................................................................................................... 6600/10222
.................................................................................................... 6700/10222
.................................................................................................... 6700/10222
..........................................................i..ii..................................... 6800/10222
.................................................................................................... 7000/10222
.................................................................................................... 7100/10222
............i....................................................................................... 7200/10222
.................................................................................................... 7300/10222
---
.................................................................................................... 8100/10222
.................................................................................................... 8200/10222
.................................................................................................... 8300/10222
......................................i............................................................. 8400/10222
............................................................................................iiiiii.i 8500/10222
iiiii.i............................................................................................. 8600/10222
.................................................................................................... 8800/10222
.................................................................................................... 8900/10222
.................................................................................................... 9000/10222
.................................................................................................... 9100/10222
---

---- [ui] ui/json-short.rs stdout ----
diff of stderr:

- {"message":"`main` function not found in crate `json_short`","code":{"code":"E0601","explanation":"No `main` function was found in a binary crate. To fix this error, add a
- `main` function. For example:
+ {"message":"`main` function not found in crate `json_short`","code":{"code":"E0601","explanation":"No `main` function was found in a binary crate.
+ To fix this error, add a `main` function:
3 
4 