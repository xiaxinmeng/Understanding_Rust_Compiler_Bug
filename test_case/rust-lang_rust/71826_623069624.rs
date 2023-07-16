plain
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/4f7e8ad2-e788-435a-a51f-a443f94dc0ea.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/71826/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71826/merge:refs/remotes/pull/71826/merge
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
    Checking rustc_feature v0.0.0 (/checkout/src/librustc_feature)
    Checking fmt_macros v0.0.0 (/checkout/src/libfmt_macros)
    Checking rustc_ast_pretty v0.0.0 (/checkout/src/librustc_ast_pretty)
    Checking rustc_hir v0.0.0 (/checkout/src/librustc_hir)
    Checking rustc_query_system v0.0.0 (/checkout/src/librustc_query_system)
    Checking rustc_attr v0.0.0 (/checkout/src/librustc_attr)
    Checking rustc_parse v0.0.0 (/checkout/src/librustc_parse)
    Checking rustc_hir_pretty v0.0.0 (/checkout/src/librustc_hir_pretty)
    Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
    Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
    Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/src/librustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
error[E0308]: mismatched types
   --> src/librustc_middle/ty/error.rs:411:46
    |
411 | ...                   .as_local_hir_id(generics.type_param(p, self).def_id)

error[E0599]: no method named `and_then` found for struct `rustc_hir::hir_id::HirId` in the current scope
error[E0599]: no method named `and_then` found for struct `rustc_hir::hir_id::HirId` in the current scope
   --> src/librustc_middle/ty/error.rs:412:30
    |
412 | ...                   .and_then(|id| self.hir().find(self.hir().get_parent_node(id)))
    |                        ^^^^^^^^ method not found in `rustc_hir::hir_id::HirId`
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0308, E0599.
For more information about an error, try `rustc --explain E0308`.
For more information about an error, try `rustc --explain E0308`.
error: could not compile `rustc_middle`.

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:03:58
== clock drift check ==
  local time: Sun May  3 07:40:32 UTC 2020
  local time: Sun May  3 07:40:32 UTC 2020
  network time: Sun, 03 May 2020 07:40:32 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/71826/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/71826/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3606) (python)
##[section]Finishing: Finalize Job
