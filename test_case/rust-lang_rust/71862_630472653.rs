plain
##[section]Starting: Linux mingw-check
##[section]Starting: Initialize job
Agent name: 'Hosted Agent'
Agent machine name: 'fv-az578'
Current agent version: '2.168.2'
##[group]Operating System
16.04.6
LTS
LTS
##[endgroup]
##[group]Virtual Environment
Environment: ubuntu-16.04
Version: 20200512.2
Included Software: https://github.com/actions/virtual-environments/blob/ubuntu16/20200512.2/images/linux/Ubuntu1604-README.md
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/5a3b0df0-a46e-497b-9e8b-297d9f999430.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/71862/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71862/merge:refs/remotes/pull/71862/merge
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
    Checking rustc_mir v0.0.0 (/checkout/src/librustc_mir)
error[E0425]: cannot find value `id` in this scope
   --> src/librustc_mir/transform/check_unsafety.rs:730:65
    |
730 |                     tcx.lint_level_at_node(SAFE_PACKED_BORROWS, id).0,
    |
help: possible candidate is found in another module, you can import it into scope
    |
1   | use std::process::id;
1   | use std::process::id;
    |

error[E0425]: cannot find value `id` in this scope
   --> src/librustc_mir/transform/check_unsafety.rs:731:68
    |
731 |                     tcx.lint_level_at_node(UNSAFE_OP_IN_UNSAFE_FN, id).0,
    |
help: possible candidate is found in another module, you can import it into scope
    |
1   | use std::process::id;
1   | use std::process::id;
    |

    Checking rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
    Checking rustc_privacy v0.0.0 (/checkout/src/librustc_privacy)
error[E0618]: expected function, found enum variant `UnsafetyViolationKind::BorrowPacked`
    |
    |
212 |                     UnsafetyViolationKind::BorrowPacked(lint_root),
    |                     |
    |                     call expression requires function
    |
    |
help: `UnsafetyViolationKind::BorrowPacked` is a unit variant, you need to write it without the parenthesis
    |
212 |                     UnsafetyViolationKind::BorrowPacked,

error: aborting due to 3 previous errors

Some errors have detailed explanations: E0425, E0618.
Some errors have detailed explanations: E0425, E0618.
For more information about an error, try `rustc --explain E0425`.
error: could not compile `rustc_mir`.

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:05:26
== clock drift check ==
  local time: Mon May 18 22:29:17 UTC 2020
  local time: Mon May 18 22:29:17 UTC 2020
  network time: Mon, 18 May 2020 22:29:17 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/71862/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/71862/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3553) (python)
##[section]Finishing: Finalize Job
