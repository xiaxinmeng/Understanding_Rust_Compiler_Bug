plain
##[section]Starting: Linux mingw-check
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 9'
Agent machine name: 'fv-az578'
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
Downloading task: Bash (3.171.1)
Checking job knob settings.
   Knob: AgentToolsDirectory = /opt/hostedtoolcache Source: ${AGENT_TOOLSDIRECTORY} 
   Knob: AgentPerflog = /home/vsts/perflog Source: ${VSTS_AGENT_PERFLOG} 
Start tracking orphan processes.
##[section]Finishing: Initialize job
##[section]Starting: Configure Job Name
==============================================================================
---
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/55a61f3b-1b23-4d30-a6c2-db9e9ad70c90.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72559/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72559/merge:refs/remotes/pull/72559/merge
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
    Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
    Checking rustc_parse_format v0.0.0 (/checkout/src/librustc_parse_format)
    Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
    Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
    Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
    Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
    Checking chalk-engine v0.11.0
    Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
    Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---

error[E0433]: failed to resolve: use of undeclared type or module `ItemKind`
  --> src/librustc_middle/hir/mod.rs:94:21
   |
94 |                     ItemKind::Impl { ref of_trait, .. } => of_trait.is_some(),

error[E0433]: failed to resolve: use of undeclared type or module `Target`
  --> src/librustc_middle/hir/mod.rs:98:21
   |
   |
98 |                     Target::Method(MethodKind::Trait { body: true })

error[E0433]: failed to resolve: use of undeclared type or module `MethodKind`
  --> src/librustc_middle/hir/mod.rs:98:36
   |
   |
98 |                     Target::Method(MethodKind::Trait { body: true })
   |                                    ^^^^^^^^^^ use of undeclared type or module `MethodKind`
error[E0433]: failed to resolve: use of undeclared type or module `Target`
   --> src/librustc_middle/hir/mod.rs:100:21
    |
100 |                     Target::Method(MethodKind::Inherent)
---

error[E0412]: cannot find type `ImplItem` in this scope
  --> src/librustc_middle/hir/mod.rs:87:60
   |
87 |     fn from_impl_item<'tcx>(tcx: TyCtxt<'tcx>, impl_item: &ImplItem<'_>) -> Target {
   |
help: consider importing this struct
   |
8  | use rustc_hir::ImplItem;
8  | use rustc_hir::ImplItem;
   |

error[E0412]: cannot find type `Target` in this scope
  --> src/librustc_middle/hir/mod.rs:87:77
   |
87 |     fn from_impl_item<'tcx>(tcx: TyCtxt<'tcx>, impl_item: &ImplItem<'_>) -> Target {
   |
help: consider importing one of these items
   |
8  | use rustc_hir::Target;
---
    |
8   | use rustc_target::spec::Target;
    |

thread 'rustc' panicked at 'Failed to get parent for DefId(0:652 ~ rustc_middle[4e85]::hir[0]::{{impl}}[3])', src/librustc_middle/traits/specialization_graph.rs:46:52

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.45.0-beta.2 (1dc0f6d8e 2020-06-15) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z unstable-options -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C debug-assertions=n --crate-type lib
note: some of the compiler flags provided by cargo are hidden

error: aborting due to 13 previous errors


Some errors have detailed explanations: E0412, E0433.
For more information about an error, try `rustc --explain E0412`.
error: could not compile `rustc_middle`.

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:03:38
== clock drift check ==
  local time: Wed Jun 24 22:57:51 UTC 2020
  local time: Wed Jun 24 22:57:51 UTC 2020
  network time: Wed, 24 Jun 2020 22:57:51 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72559/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72559/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3627) (python)
##[section]Finishing: Finalize Job
