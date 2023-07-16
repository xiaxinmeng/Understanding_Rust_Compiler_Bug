plain
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/c6c76f01-387c-48a1-8214-ac4d205a9ee0.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/71269/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71269/merge:refs/remotes/pull/71269/merge
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
error[E0308]: mismatched types
    --> src/librustc_session/options.rs:290:24
     |
158  | / macro_rules! options {
158  | / macro_rules! options {
159  | |     ($struct_name:ident, $setter_name:ident, $defaultfn:ident,
160  | |      $buildfn:ident, $prefix:expr, $outputname:expr,
161  | |      $stat:ident, $mod_desc:ident, $mod_set:ident,
...    |
290  | |                 $parse(&mut redirect_field!(cg.$opt), v)
     | |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected `bool`, found enum `std::option::Option`
639  | |     }
640  | | ) }
     | |___- in this expansion of `options!`
...
...
743  | / options! {DebuggingOptions, DebuggingSetter, basic_debugging_options,
744  | |           build_debugging_options, "Z", "debugging",
745  | |           DB_OPTIONS, db_type_desc, dbsetters,
...    |
1017 | |     // - src/librustc_interface/tests.rs
1018 | | }
     | |_- in this macro invocation
---

To learn more, run the command again with --verbose.
warning: build failed, waiting for other jobs to finish...
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:03:41
== clock drift check ==
  local time: Sun May  3 14:55:14 UTC 2020
  local time: Sun May  3 14:55:14 UTC 2020
  network time: Sun, 03 May 2020 14:55:14 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/71269/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/71269/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3630) (python)
##[section]Finishing: Finalize Job
