plain
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/70b18c1f-02c5-4f35-a42f-634ccaf2dc92.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/69171/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/69171/merge:refs/remotes/pull/69171/merge
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
---
    Checking rustc_span v0.0.0 (/checkout/src/librustc_span)
    Checking rustc_errors v0.0.0 (/checkout/src/librustc_errors)
    Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
    Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
    Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
    Checking rustc_session v0.0.0 (/checkout/src/librustc_session)
    Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
    Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
    Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
---
    Checking rustc_mir v0.0.0 (/checkout/src/librustc_mir)
    Checking rustc_passes v0.0.0 (/checkout/src/librustc_passes)
    Checking rustc_ty v0.0.0 (/checkout/src/librustc_ty)
    Checking rustc_plugin_impl v0.0.0 (/checkout/src/librustc_plugin_impl)
error[E0004]: non-exhaustive patterns: `InlineAsm { .. }` not covered
    --> src/librustc_mir/dataflow/framework/direction.rs:438:15
     |
438  |         match bb_data.terminator().kind {
     |               ^^^^^^^^^^^^^^^^^^^^^^^^^ pattern `InlineAsm { .. }` not covered
    ::: /checkout/src/librustc_middle/mir/mod.rs:1189:5
     |
1189 |     InlineAsm {
     |     --------- not covered
---
For more information about this error, try `rustc --explain E0004`.
error: could not compile `rustc_mir`.

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:04:31
== clock drift check ==
  local time: Wed May  6 11:13:20 UTC 2020
  local time: Wed May  6 11:13:20 UTC 2020
  network time: Wed, 06 May 2020 11:13:21 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/69171/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/69171/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3544) (python)
##[section]Finishing: Finalize Job
