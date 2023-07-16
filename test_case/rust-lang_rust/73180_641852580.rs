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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/52b740e2-2c28-4f03-8a30-5865bda9a124.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/73180/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/73180/merge:refs/remotes/pull/73180/merge
---
Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
 ---> Running in 599d61ee3e25
Removing intermediate container 599d61ee3e25
 ---> 1d3a613089a3
Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            python3 ../x.py doc --stage 0 src/libstd &&            /scripts/validate-toolstate.sh
Removing intermediate container 74335fc9646e
 ---> 4eb8f365f7f5
Successfully built 4eb8f365f7f5
Successfully tagged rust-ci:latest
Successfully tagged rust-ci:latest
Built container sha256:4eb8f365f7f51883ed7793c3b2ead391c9b59df18922394b674fc9deea87995d
Uploading finished image to https://ci-caches.rust-lang.org/docker/d3f0b748e51c65823b26e5c9aa3312fcd51003139be9a43290b3a342a55aa1a19cca0eefe8cd2813f66ed4bd2ebafe54507cc7aa11fb4b3d0cb6de76d4374fee
upload failed: - to s3://rust-lang-ci-sccache2/docker/d3f0b748e51c65823b26e5c9aa3312fcd51003139be9a43290b3a342a55aa1a19cca0eefe8cd2813f66ed4bd2ebafe54507cc7aa11fb4b3d0cb6de76d4374fee An error occurred (InvalidAccessKeyId) when calling the CreateMultipartUpload operation: The AWS Access Key Id you provided does not exist in our records.
[CI_JOB_NAME=mingw-check]
== clock drift check ==
  local time: Wed Jun 10 08:44:58 UTC 2020
  network time: Wed, 10 Jun 2020 08:44:58 GMT
---
    Checking rustc_span v0.0.0 (/checkout/src/librustc_span)
    Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
    Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
    Checking rustc_parse_format v0.0.0 (/checkout/src/librustc_parse_format)
    Checking chalk-rust-ir v0.10.0
    Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
    Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
    Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
    Checking chalk-solve v0.10.0
    Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
    Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
    Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
    Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
    Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/src/librustc_expand)
error[E0412]: cannot find type `TypeFlags` in this scope
    --> src/librustc_middle/ty/structural_impls.rs:1006:37
     |
1006 |     fn has_type_flags(&self, flags: TypeFlags) -> bool {
     |
help: consider importing this struct
     |
5    | use crate::ty::TypeFlags;
5    | use crate::ty::TypeFlags;
     |

    Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
error[E0599]: no method named `add_binder` found for mutable reference `&mut ty::flags::FlagComputation` in the current scope
   --> src/librustc_middle/ty/flags.rs:102:22
    |
102 |                 self.add_binder(debruijn);
    |                      ^^^^^^^^^^ help: there is an associated function with a similar name: `add_kind`
error[E0599]: no method named `add_binder` found for mutable reference `&mut ty::flags::FlagComputation` in the current scope
error[E0599]: no method named `add_binder` found for mutable reference `&mut ty::flags::FlagComputation` in the current scope
   --> src/librustc_middle/ty/flags.rs:282:22
    |
282 |                 self.add_binder(debruijn);
    |                      ^^^^^^^^^^ help: there is an associated function with a similar name: `add_kind`
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0412, E0599.
For more information about an error, try `rustc --explain E0412`.
For more information about an error, try `rustc --explain E0412`.
error: could not compile `rustc_middle`.

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:04:48
== clock drift check ==
  local time: Wed Jun 10 08:49:47 UTC 2020
  local time: Wed Jun 10 08:49:47 UTC 2020
  network time: Wed, 10 Jun 2020 08:49:48 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/73180/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/73180/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (4819) (python)
##[section]Finishing: Finalize Job
