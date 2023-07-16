plain
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/66e1628c-7333-4ac9-af5c-a4f733ef96d9.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/71696/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/71696/merge:refs/remotes/pull/71696/merge
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
  local time: Wed Apr 29 22:16:17 UTC 2020
  network time: Wed, 29 Apr 2020 22:16:17 GMT
Starting sccache server...
configure: processing command line
configure: 
configure: rust.parallel-compiler := True
---
    Checking unicode-width v0.1.6
    Checking getopts v0.2.21
    Checking test v0.0.0 (/checkout/src/libtest)
    Finished release [optimized] target(s) in 27.20s
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
    Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
    Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/src/librustc_expand)
error[E0308]: mismatched types
    --> /checkout/src/librustc_data_structures/macros.rs:17:32
     |
15   | / macro_rules! static_assert_size {
16   | |     ($ty:ty, $size:expr) => {
17   | |         const _: [(); $size] = [(); ::std::mem::size_of::<$ty>()];
     | |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected an array with a fixed size of 16 elements, found one with 24 elements
19   | | }
     | |_- in this expansion of `static_assert_size!`
     | 
    ::: src/librustc_middle/mir/mod.rs:1864:1
    ::: src/librustc_middle/mir/mod.rs:1864:1
     |
1864 |   static_assert_size!(PlaceElem<'_>, 16);

    Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
error[E0308]: mismatched types
    --> src/librustc_middle/mir/mod.rs:2546:52
    --> src/librustc_middle/mir/mod.rs:2546:52
     |
2546 |         self.projs.push(ProjectionElem::Subslice { from, to, from_end: true });
     |                                                    |
     |                                                    expected `u64`, found `u32`
     |                                                    expected `u64`, found `u32`
     |                                                    help: you can convert an `u32` to `u64`: `from: from.into()`
error[E0308]: mismatched types
    --> src/librustc_middle/mir/mod.rs:2546:58
     |
     |
2546 |         self.projs.push(ProjectionElem::Subslice { from, to, from_end: true });
     |                                                          |
     |                                                          expected `u64`, found `u32`
     |                                                          expected `u64`, found `u32`
     |                                                          help: you can convert an `u32` to `u64`: `to: to.into()`
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0308`.
For more information about this error, try `rustc --explain E0308`.
{"reason":"build-finished","success":false}
error: could not compile `rustc_middle`.
To learn more, run the command again with --verbose.
To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:03:47
== clock drift check ==
  local time: Wed Apr 29 22:20:05 UTC 2020
  local time: Wed Apr 29 22:20:05 UTC 2020
  network time: Wed, 29 Apr 2020 22:20:05 GMT


##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/71696/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/71696/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (4087) (python)
##[section]Finishing: Finalize Job
