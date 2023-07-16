plain
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/7c07ff92-e211-487c-9d2f-b209a7a653db.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/68362/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/68362/merge:refs/remotes/pull/68362/merge
---
 ---> f7353ccad5b1
Step 6/7 : ENV RUN_CHECK_WITH_PARALLEL_QUERIES 1
 ---> Using cache
 ---> ed38efbaa060
Step 7/7 : ENV SCRIPT python3 ../x.py test src/tools/expand-yaml-anchors &&            python3 ../x.py check --target=i686-pc-windows-gnu --host=i686-pc-windows-gnu &&            python3 ../x.py build --stage 0 src/tools/build-manifest &&            python3 ../x.py test --stage 0 src/tools/compiletest &&            python3 ../x.py test src/tools/tidy &&            /scripts/validate-toolstate.sh
 ---> c5008ef7ae8e
Successfully built c5008ef7ae8e
Successfully tagged rust-ci:latest
Built container sha256:c5008ef7ae8e94d7ef502e3cef26e61208e14ebdb36913f3a8bb86291bd6430b
Built container sha256:c5008ef7ae8e94d7ef502e3cef26e61208e14ebdb36913f3a8bb86291bd6430b
Looks like docker image is the same as before, not uploading
[CI_JOB_NAME=mingw-check]
[CI_JOB_NAME=mingw-check]
== clock drift check ==
  local time: Thu Apr 30 04:35:31 UTC 2020
  network time: Thu, 30 Apr 2020 04:35:31 GMT
Starting sccache server...
configure: processing command line
configure: 
configure: rust.parallel-compiler := True
---
    Checking unicode-width v0.1.6
    Checking getopts v0.2.21
    Checking test v0.0.0 (/checkout/src/libtest)
    Finished release [optimized] target(s) in 20.86s
{"reason":"build-finished","success":true}
    Checking cfg-if v0.1.10
   Compiling libc v0.2.69
   Compiling semver-parser v0.7.0
    Checking scopeguard v1.0.0
---
    Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
    Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
    Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
    Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
    Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
    Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
    Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
    Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
    Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
---
    Checking rustc_ty v0.0.0 (/checkout/src/librustc_ty)
error[E0023]: this pattern has 1 field, but the corresponding tuple variant has 2 fields
   --> src/librustc_ty/needs_drop.rs:110:29
    |
110 |   ...                   ty::GeneratorWitness(tys) => tcx.erase_late_bound_regions(tys),
    | 
    | 
   ::: /checkout/src/librustc_middle/ty/sty.rs:171:5
171 | /     GeneratorWitness(
172 | |         Binder<&'tcx List<Ty<'tcx>>>,
172 | |         Binder<&'tcx List<Ty<'tcx>>>,
173 | |         Binder<&'tcx List<ty::RegionOutlivesPredicate<'tcx>>>,
    | |_____- tuple variant defined here

error: aborting due to previous error


For more information about this error, try `rustc --explain E0023`.
error: could not compile `rustc_ty`.

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
{"reason":"build-finished","success":false}
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:03:33
== clock drift check ==
== clock drift check ==
  local time: Thu Apr 30 04:39:05 UTC 2020
  network time: Thu, 30 Apr 2020 04:39:05 GMT


##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/68362/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/68362/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3500) (python)
##[section]Finishing: Finalize Job
