plain
##[section]Starting: Linux mingw-check
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 8'
Agent machine name: 'fv-az578'
Current agent version: '2.168.2'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/a08b00ae-f225-4f55-bf8b-38ff823b3fc7.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72692/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72692/merge:refs/remotes/pull/72692/merge
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
   Compiling bootstrap v0.0.0 (/checkout/src/bootstrap)
error: unused variable: `initial_sysroot`
   --> src/bootstrap/lib.rs:347:13
    |
347 |         let initial_sysroot = config.initial_rustc.parent().unwrap().parent().unwrap();
    |             ^^^^^^^^^^^^^^^ help: if this is intentional, prefix it with an underscore: `_initial_sysroot`
    = note: `-D unused-variables` implied by `-D warnings`

error[E0716]: temporary value dropped while borrowed
   --> src/bootstrap/lib.rs:348:48
   --> src/bootstrap/lib.rs:348:48
    |
348 |           let initial_target_libdir = Path::new(&output(Command::new(&config.initial_rustc)
    |  ________________________________________________^
349 | |                 .arg("--target")
350 | |                 .arg(config.build)
351 | |                 .arg("--print")
352 | |                 .arg("target-libdir")));
    | |                                      ^ - temporary value is freed at the end of this statement
    |                                        creates a temporary which is freed while still in use
    |                                        creates a temporary which is freed while still in use
353 |           let initial_lld = initial_target_libdir.parent().unwrap()
    |                             --------------------- borrow later used here
    = note: consider using a `let` binding to create a longer lived value

error: aborting due to 2 previous errors

---
  local time: Thu May 28 14:27:26 UTC 2020
  network time: Thu, 28 May 2020 14:27:26 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72692/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72692/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3477) (python)
##[section]Finishing: Finalize Job
