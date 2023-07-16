plain
##[section]Starting: Linux x86_64-gnu-llvm-8
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 9'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/e0b0b29e-b311-4f13-9708-14657c061558.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72675/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72675/merge:refs/remotes/pull/72675/merge
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
   Compiling compiler_builtins v0.1.28
   Compiling unwind v0.0.0 (/checkout/src/libunwind)
   Compiling backtrace-sys v0.1.37
   Compiling hashbrown v0.6.2
error[E0004]: non-exhaustive patterns: `(true, _)` and `(false, _)` not covered
    --> src/libcore/cmp.rs:1091:27
1085 | /     macro_rules! partial_ord_impl {
1085 | /     macro_rules! partial_ord_impl {
1086 | |         ($($t:ty)*) => ($(
1087 | |             #[stable(feature = "rust1", since = "1.0.0")]
1088 | |             impl PartialOrd for $t {
...    |
1091 | |                     match (self <= other, self >= other) {
     | |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ patterns `(true, _)` and `(false, _)` not covered
1107 | |         )*)
1108 | |     }
     | |_____- in this expansion of `partial_ord_impl!`
...
...
1126 |       partial_ord_impl! { f32 f64 }
     |
     = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
     = note: the matched value is of type `(bool, bool)`


error[E0004]: non-exhaustive patterns: `Ok(true)` and `Ok(false)` not covered
     |
     |
2284 |           self.try_fold((), move |(), x| match f(&x).into_result() {
     |                                                ^^^^^^^^^^^^^^^^^^^ patterns `Ok(true)` and `Ok(false)` not covered
    ::: src/libcore/result.rs:247:1
     |
247  | / pub enum Result<T, E> {
248  | |     /// Contains the success value
248  | |     /// Contains the success value
249  | |     #[stable(feature = "rust1", since = "1.0.0")]
250  | |     Ok(#[stable(feature = "rust1", since = "1.0.0")] T),
...    |
...    |
254  | |     Err(#[stable(feature = "rust1", since = "1.0.0")] E),
255  | | }
     | |_- `result::Result<bool, E>` defined here
     = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
     = note: the matched value is of type `result::Result<bool, E>`


error[E0004]: non-exhaustive patterns: `(true, _)` and `(false, _)` not covered
  --> src/libcore/fmt/float.rs:79:22
   |
79 |     let sign = match (force_sign, negative_zero) {
   |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^ patterns `(true, _)` and `(false, _)` not covered
   = help: ensure that all possible cases are being handled, possibly by adding wildcards or more match arms
   = note: the matched value is of type `(bool, bool)`

error[E0004]: non-exhaustive patterns: `true` and `false` not covered
---
  local time: Wed May 27 22:03:15 UTC 2020
  network time: Wed, 27 May 2020 22:03:15 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72675/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72675/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3555) (python)
##[section]Finishing: Finalize Job
