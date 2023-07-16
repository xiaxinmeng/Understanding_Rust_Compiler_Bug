plain
##[section]Starting: Linux mingw-check
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 8'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/a3c31a08-1579-43c4-bd56-6b70bb5d22e4.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/73343/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/73343/merge:refs/remotes/pull/73343/merge
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
    Checking rustc_parse_format v0.0.0 (/checkout/src/librustc_parse_format)
    Checking chalk-rust-ir v0.10.0
    Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
    Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
    Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
    Checking chalk-solve v0.10.0
    Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
    Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
    Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
configure: rust.debug-assertions := True
configure: dist.missing-tools   := True
configure: llvm.ccache          := sccache
configure: rust.channel         := nightly
configure: build.configure-args := ['--enable-sccache', '--disable-manage-submodu ...
configure: writing `config.toml` in current directory
configure: 
configure: run `python /checkout/x.py --help`
configure: 
---
Hugepagesize:       2048 kB
DirectMap4k:      133056 kB
DirectMap2M:     4061184 kB
DirectMap1G:     5242880 kB
+ python3 ../x.py test src/tools/expand-yaml-anchors
Ensuring the YAML anchors in the GitHub Actions config were expanded
Ensuring the YAML anchors in the GitHub Actions config were expanded
Building stage0 tool expand-yaml-anchors (x86_64-unknown-linux-gnu)
   Compiling unicode-xid v0.2.0
   Compiling syn v1.0.11
   Compiling linked-hash-map v0.5.2
   Compiling lazy_static v1.4.0
   Compiling lazy_static v1.4.0
   Compiling yaml-rust v0.4.3
   Compiling quote v1.0.2
   Compiling thiserror-impl v1.0.5
   Compiling thiserror v1.0.5
   Compiling yaml-merge-keys v0.4.0
   Compiling expand-yaml-anchors v0.1.0 (/checkout/src/tools/expand-yaml-anchors)
Build completed successfully in 0:00:32
+ python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu
    Finished dev [unoptimized] target(s) in 0.20s
Checking rustdoc artifacts (x86_64-unknown-linux-gnu -> i686-pc-windows-gnu)
---
    Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
error[E0425]: cannot find value `ENABLE_LINE_INPUT` in module `c`
   --> src/libstd/sys/windows/stdio.rs:147:24
    |
147 |             mode ^= c::ENABLE_LINE_INPUT;
    |                        ^^^^^^^^^^^^^^^^^ not found in `c`
error[E0425]: cannot find value `ENABLE_LINE_INPUT` in module `c`
   --> src/libstd/sys/windows/stdio.rs:149:24
    |
    |
149 |             mode |= c::ENABLE_LINE_INPUT;
    |                        ^^^^^^^^^^^^^^^^^ not found in `c`
error[E0425]: cannot find function, tuple struct or tuple variant `SetConsoleMode` in module `c`
   --> src/libstd/sys/windows/stdio.rs:151:16
    |
    |
151 |           if !c::SetConsoleMode(handle, mode) {
    |                  ^^^^^^^^^^^^^^ help: a function with a similar name exists: `GetConsoleMode`
   ::: src/libstd/sys/windows/c.rs:689:9
    |
    |
689 | /         pub fn GetConsoleMode(hConsoleHandle: HANDLE,
690 | |                               lpMode: LPDWORD) -> BOOL;
    | |_______________________________________________________- similarly named function `GetConsoleMode` defined here
error[E0308]: mismatched types
   --> src/libstd/sys/windows/stdio.rs:140:12
    |
    |
140 |         if !c::GetConsoleMode(handle, &mut mode) {
    |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `bool`, found `i32`
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0308, E0425.
For more information about an error, try `rustc --explain E0308`.
---
  local time: Sun Jun 14 14:51:49 UTC 2020
  network time: Sun, 14 Jun 2020 14:51:49 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/73343/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/73343/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (4087) (python)
##[section]Finishing: Finalize Job
