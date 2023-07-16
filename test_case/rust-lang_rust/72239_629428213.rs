plain
##[section]Starting: Linux mingw-check
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 43'
Agent machine name: 'fv-az578'
Current agent version: '2.168.2'
##[group]Operating System
16.04.6
LTS
LTS
##[endgroup]
##[group]Virtual Environment
Environment: ubuntu-16.04
Version: 20200430.2
Included Software: https://github.com/actions/virtual-environments/blob/ubuntu16/20200430.2/images/linux/Ubuntu1604-README.md
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/610cc602-0c37-4337-96aa-5f643b8dcd64.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72239/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72239/merge:refs/remotes/pull/72239/merge
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
    Checking panic_unwind v0.0.0 (/checkout/src/libpanic_unwind)
error[E0053]: method `eq` has an incompatible type for trait
   --> src/libstd/net/addr.rs:677:5
    |
677 |     fn eq(&self, other: &SocketAddrV4) -> bool {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `net::addr::SocketAddr`, found struct `net::addr::SocketAddrV4`
    |
    = note: expected fn pointer `fn(&net::addr::SocketAddrV4, &net::addr::SocketAddr) -> _`
               found fn pointer `fn(&net::addr::SocketAddrV4, &net::addr::SocketAddrV4) -> _`
error[E0053]: method `eq` has an incompatible type for trait
   --> src/libstd/net/addr.rs:686:5
    |
    |
686 |     fn eq(&self, other: &SocketAddrV6) -> bool {
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected enum `net::addr::SocketAddr`, found struct `net::addr::SocketAddrV6`
    |
    = note: expected fn pointer `fn(&net::addr::SocketAddrV6, &net::addr::SocketAddr) -> _`
               found fn pointer `fn(&net::addr::SocketAddrV6, &net::addr::SocketAddrV6) -> _`
error[E0308]: mismatched types
   --> src/libstd/net/addr.rs:679:13
    |
678 |         match other {
678 |         match other {
    |               ----- this expression has type `&net::addr::SocketAddrV4`
679 |             SocketAddr::V4(v4) => self == v4,
    |             ^^^^^^^^^^^^^^^^^^ expected struct `net::addr::SocketAddrV4`, found enum `net::addr::SocketAddr`
error[E0308]: mismatched types
   --> src/libstd/net/addr.rs:680:13
    |
678 |         match other {
678 |         match other {
    |               ----- this expression has type `&net::addr::SocketAddrV4`
679 |             SocketAddr::V4(v4) => self == v4,
680 |             SocketAddr::V6(_) => false,
    |             ^^^^^^^^^^^^^^^^^ expected struct `net::addr::SocketAddrV4`, found enum `net::addr::SocketAddr`
error[E0308]: mismatched types
   --> src/libstd/net/addr.rs:688:13
    |
687 |         match other {
687 |         match other {
    |               ----- this expression has type `&net::addr::SocketAddrV6`
688 |             SocketAddr::V4(_) => false,
    |             ^^^^^^^^^^^^^^^^^ expected struct `net::addr::SocketAddrV6`, found enum `net::addr::SocketAddr`
error[E0308]: mismatched types
   --> src/libstd/net/addr.rs:689:13
    |
687 |         match other {
687 |         match other {
    |               ----- this expression has type `&net::addr::SocketAddrV6`
688 |             SocketAddr::V4(_) => false,
689 |             SocketAddr::V6(v6) => self == v6,
    |             ^^^^^^^^^^^^^^^^^^ expected struct `net::addr::SocketAddrV6`, found enum `net::addr::SocketAddr`
error: aborting due to 6 previous errors

Some errors have detailed explanations: E0053, E0308.
For more information about an error, try `rustc --explain E0053`.
---
  local time: Fri May 15 19:04:22 UTC 2020
  network time: Fri, 15 May 2020 19:04:22 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72239/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72239/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3383) (python)
##[section]Finishing: Finalize Job
