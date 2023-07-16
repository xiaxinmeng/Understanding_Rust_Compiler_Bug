plain
##[section]Starting: Linux mingw-check
##[section]Starting: Initialize job
Agent name: 'Hosted Agent'
Agent machine name: 'fv-az578'
Current agent version: '2.171.1'
##[group]Operating System
16.04.6
LTS
LTS
##[endgroup]
##[group]Virtual Environment
Environment: ubuntu-16.04
Version: 20200621.1
Included Software: https://github.com/actions/virtual-environments/blob/ubuntu16/20200621.1/images/linux/Ubuntu1604-README.md
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/c15f487b-6082-4598-b754-269ca11d3ec5.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/73876/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/73876/merge:refs/remotes/pull/73876/merge
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
    Checking rustc_span v0.0.0 (/checkout/src/librustc_span)
    Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
    Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
    Checking rustc_parse_format v0.0.0 (/checkout/src/librustc_parse_format)
    Checking tracing v0.1.15
    Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
    Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
    Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
    Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
    Checking chalk-solve v0.14.0
---
    Checking rustc_mir v0.0.0 (/checkout/src/librustc_mir)
error[E0053]: method `relate_with_variance` has an incompatible type for trait
  --> src/librustc_mir/transform/validate.rs:74:9
   |
74 | /         fn relate_with_variance<T: Relate<'tcx>>(
76 | |             _: ty::Variance,
77 | |             a: &T,
...  |
...  |
81 | |             self.relate(a, b)
   | |_________^ expected type parameter `T`, found `&T`
   |
   |
   = note: expected fn pointer `fn(&mut transform::validate::equal_up_to_regions::LifetimeIgnoreRelation<'tcx>, rustc_middle::ty::Variance, T, T) -> std::result::Result<_, _>`
              found fn pointer `fn(&mut transform::validate::equal_up_to_regions::LifetimeIgnoreRelation<'tcx>, rustc_middle::ty::Variance, &T, &T) -> std::result::Result<_, _>`
error[E0053]: method `binders` has an incompatible type for trait
   --> src/librustc_mir/transform/validate.rs:109:9
    |
109 | /         fn binders<T>(
---
118 | |             Ok(a.clone())
119 | |         }
    | |_________^ expected struct `rustc_middle::ty::Binder`, found reference
    |
    = note: expected fn pointer `fn(&mut transform::validate::equal_up_to_regions::LifetimeIgnoreRelation<'tcx>, rustc_middle::ty::Binder<T>, rustc_middle::ty::Binder<T>) -> std::result::Result<_, _>`
               found fn pointer `fn(&mut transform::validate::equal_up_to_regions::LifetimeIgnoreRelation<'tcx>, &rustc_middle::ty::Binder<T>, &rustc_middle::ty::Binder<T>) -> std::result::Result<_, _>`
    Checking rustc_passes v0.0.0 (/checkout/src/librustc_passes)
    Checking rustc_passes v0.0.0 (/checkout/src/librustc_passes)
error[E0277]: the trait bound `&&rustc_middle::ty::TyS<'_>: rustc_middle::ty::relate::Relate<'_>` is not satisfied
   --> src/librustc_mir/transform/validate.rs:124:13
    |
124 |     relator.relate(&src, &dest).is_ok()
    |             ^^^^^^ the trait `rustc_middle::ty::relate::Relate<'_>` is not implemented for `&&rustc_middle::ty::TyS<'_>`
    = help: the following implementations were found:
    = help: the following implementations were found:
              <&'tcx rustc_middle::ty::TyS<'tcx> as rustc_middle::ty::relate::Relate<'tcx>>
error[E0308]: mismatched types
  --> src/librustc_mir/transform/validate.rs:81:25
   |
   |
74 |         fn relate_with_variance<T: Relate<'tcx>>(
...
...
81 |             self.relate(a, b)
   |                         |
   |                         expected type parameter `T`, found `&T`
   |                         help: consider dereferencing the borrow: `*a`
   |
   |
   = note: expected type parameter `T`
                   found reference `&T`

error[E0308]: mismatched types
  --> src/librustc_mir/transform/validate.rs:81:28
   |
74 |         fn relate_with_variance<T: Relate<'tcx>>(
...
...
81 |             self.relate(a, b)
   |                            |
   |                            expected type parameter `T`, found `&T`
   |                            help: consider dereferencing the borrow: `*b`
   |
---

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:05:56
== clock drift check ==
  local time: Mon Jun 29 23:57:30 UTC 2020
  local time: Mon Jun 29 23:57:30 UTC 2020
  network time: Mon, 29 Jun 2020 23:57:30 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/73876/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/73876/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (4048) (python)
##[section]Finishing: Finalize Job
