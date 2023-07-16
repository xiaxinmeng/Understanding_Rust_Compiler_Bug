plain
##[section]Starting: Linux mingw-check
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 2'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/5c30c3ef-eaf7-4c5d-aa9a-1a8469ca14dc.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/71948/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71948/merge:refs/remotes/pull/71948/merge
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
    Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
    Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
    Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
    Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
    Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
    Checking chalk-rust-ir v0.10.0
    Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
    Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
    Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
84  | |                             ($op:expr,
85  | |                              $cx:expr,
86  | |                              $ty:expr,
...   |
105 | |                                         implements_trait($cx, $ty, trait_id, &[$rty])
    | |                                         |                                     |
    | |                                         |                                     creates a temporary which is freed while still in use
    | |                                         argument requires that borrow lasts for `'1`
106 | |                                     },)*
---
    |
51  | impl<'a, 'tcx> LateLintPass<'a, 'tcx> for EqOp {
    |          ---- lifetime `'tcx` defined here
...
107 |                         if (requires_ref || (lcpy && rcpy)) && implements_trait(cx, lty, trait_id, &[rty.into()]) {
    |                                                                |                                    |           |
    |                                                                |                                    |           temporary value is freed at the end of this statement
    |                                                                |                                    creates a temporary which is freed while still in use
    |                                                                argument requires that borrow lasts for `'tcx`
    |                                                                argument requires that borrow lasts for `'tcx`

error[E0716]: temporary value dropped while borrowed
   --> src/tools/clippy/clippy_lints/src/eq_op.rs:125:69
    |
51  | impl<'a, 'tcx> LateLintPass<'a, 'tcx> for EqOp {
    |          ---- lifetime `'tcx` defined here
...
125 |                             && implements_trait(cx, lty, trait_id, &[cx.tables.expr_ty(right).into()])
    |                                |                                    |                                |
    |                                |                                    |                                temporary value is freed at the end of this statement
    |                                |                                    creates a temporary which is freed while still in use
    |                                argument requires that borrow lasts for `'tcx`
    |                                argument requires that borrow lasts for `'tcx`

error[E0716]: temporary value dropped while borrowed
   --> src/tools/clippy/clippy_lints/src/eq_op.rs:144:89
    |
51  | impl<'a, 'tcx> LateLintPass<'a, 'tcx> for EqOp {
    |          ---- lifetime `'tcx` defined here
...
144 |                             && implements_trait(cx, cx.tables.expr_ty(left), trait_id, &[rty.into()])
    |                                |                                                        |           |
    |                                |                                                        |           temporary value is freed at the end of this statement
    |                                |                                                        creates a temporary which is freed while still in use
    |                                argument requires that borrow lasts for `'tcx`
    |                                argument requires that borrow lasts for `'tcx`

error[E0716]: temporary value dropped while borrowed
   --> src/tools/clippy/clippy_lints/src/eq_op.rs:168:69
    |
51  | impl<'a, 'tcx> LateLintPass<'a, 'tcx> for EqOp {
    |          ---- lifetime `'tcx` defined here
...
168 |                             && implements_trait(cx, lty, trait_id, &[cx.tables.expr_ty(right).into()])
    |                                |                                    |                                |
    |                                |                                    |                                temporary value is freed at the end of this statement
    |                                |                                    creates a temporary which is freed while still in use
    |                                argument requires that borrow lasts for `'tcx`
    |                                argument requires that borrow lasts for `'tcx`

error[E0716]: temporary value dropped while borrowed
   --> src/tools/clippy/clippy_lints/src/eq_op.rs:192:89
    |
51  | impl<'a, 'tcx> LateLintPass<'a, 'tcx> for EqOp {
    |          ---- lifetime `'tcx` defined here
...
192 |                             && implements_trait(cx, cx.tables.expr_ty(left), trait_id, &[rty.into()])
    |                                |                                                        |           |
    |                                |                                                        |           temporary value is freed at the end of this statement
    |                                |                                                        creates a temporary which is freed while still in use
    |                                argument requires that borrow lasts for `'tcx`
    |                                argument requires that borrow lasts for `'tcx`

error[E0716]: temporary value dropped while borrowed
    --> src/tools/clippy/clippy_lints/src/methods/mod.rs:3508:53
     |
3489 |         fn matches_ref<'a>(
     |                        -- lifetime `'a` defined here
...
3508 |             implements_trait(cx, ty, trait_def_id, &[parent_ty.into()])
     |             |                                       |
     |             |                                       creates a temporary which is freed while still in use
     |             argument requires that borrow lasts for `'a`
3509 |         }
---
    |
549 | fn check_to_owned(cx: &LateContext<'_, '_>, expr: &Expr<'_>, other: &Expr<'_>) {
    |                   -- lifetime `'1` appears in the type of `cx`
...
579 |         implements_trait(cx, tam.ty, partial_eq_trait_id, &[other_ty.into()])
    |         |                                                  |
    |         |                                                  creates a temporary which is freed while still in use
    |         argument requires that borrow lasts for `'1`
580 |     });
---
    |
549 | fn check_to_owned(cx: &LateContext<'_, '_>, expr: &Expr<'_>, other: &Expr<'_>) {
    |                   -- lifetime `'1` appears in the type of `cx`
...
582 |         implements_trait(cx, arg_ty, partial_eq_trait_id, &[tam.ty.into()])
    |         |                                                  |
    |         |                                                  creates a temporary which is freed while still in use
    |         argument requires that borrow lasts for `'1`
583 |     });
---
    |
549 | fn check_to_owned(cx: &LateContext<'_, '_>, expr: &Expr<'_>, other: &Expr<'_>) {
    |                                        -- let's call this `'1`
...
584 |     let arg_impl_partial_eq_other = implements_trait(cx, arg_ty, partial_eq_trait_id, &[other_ty.into()]);
    |                                     ---------------------------------------------------^^^^^^^^^^^^^^^^^-- temporary value is freed at the end of this statement
    |                                     |                                                  creates a temporary which is freed while still in use
    |                                     argument requires that borrow lasts for `'1`

error[E0716]: temporary value dropped while borrowed
---
  local time: Fri May 15 03:42:38 UTC 2020
  network time: Fri, 15 May 2020 03:42:39 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/71948/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/71948/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3552) (python)
##[section]Finishing: Finalize Job
