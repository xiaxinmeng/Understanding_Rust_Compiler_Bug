plain
========================== Starting Command Output ===========================
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/2c4d3c32-4dcc-49d9-b70c-755ffb9aef11.sh

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
    Checking rustc_mir v0.0.0 (/checkout/src/librustc_mir)
error: unnecessary parentheses around method argument
   --> src/librustc_mir/borrow_check/type_check/mod.rs:648:45
    |
648 |                         tcx.mk_array(inner, (to - from))
    |
    = note: `-D unused-parens` implied by `-D warnings`

error[E0053]: method `array_subpath` has an incompatible type for trait
error[E0053]: method `array_subpath` has an incompatible type for trait
   --> src/librustc_mir/shim.rs:310:56
    |
310 |     fn array_subpath(&self, _path: Self::Path, _index: u32, _size: u32) -> Option<Self::Path> {
    | 
   ::: src/librustc_mir/util/elaborate_drops.rs:91:54
    |
    |
91  |     fn array_subpath(&self, path: Self::Path, index: u64, size: u64) -> Option<Self::Path>;
    |                                                      --- type in trait
    |
    = note: expected fn pointer `fn(&shim::DropShimElaborator<'a, 'tcx>, (), u64, u64) -> std::option::Option<_>`
               found fn pointer `fn(&shim::DropShimElaborator<'a, 'tcx>, (), u32, u32) -> std::option::Option<_>`
error[E0308]: mismatched types
  --> src/librustc_mir/util/aggregate.rs:61:37
   |
61 |                         min_length: offset + 1,
61 |                         min_length: offset + 1,
   |                                     ^^^^^^^^^^ expected `u32`, found `u64`

error[E0308]: mismatched types
   --> src/librustc_mir/borrow_check/places_conflict.rs:452:52
    |
452 |             if *offset_from_begin >= *min_length - *offset_from_end {

error[E0308]: mismatched types
   --> src/librustc_mir/borrow_check/places_conflict.rs:452:38
    |
    |
452 |             if *offset_from_begin >= *min_length - *offset_from_end {
    |                                      |
    |                                      expected `u64`, found `u32`
    |                                      expected `u64`, found `u32`
    |                                      help: you can convert an `u32` to `u64`: `(*min_length - *offset_from_end).into()`

error[E0277]: cannot subtract `u64` from `u32`
   --> src/librustc_mir/borrow_check/places_conflict.rs:452:50
    |
452 |             if *offset_from_begin >= *min_length - *offset_from_end {
    |                                                  ^ no implementation for `u32 - u64`
    |
    = help: the trait `std::ops::Sub<u64>` is not implemented for `u32`
error[E0308]: mismatched types
   --> src/librustc_mir/interpret/place.rs:549:53
    |
    |
549 |                     assert!(0 < offset && offset <= min_length);
    |                                                     |
    |                                                     expected `u64`, found `u32`
    |                                                     expected `u64`, found `u32`
    |                                                     help: you can convert an `u32` to `u64`: `min_length.into()`
error[E0308]: mismatched types
   --> src/librustc_mir/interpret/place.rs:552:38
    |
    |
552 |                     assert!(offset < min_length);
    |                                      |
    |                                      expected `u64`, found `u32`
    |                                      expected `u64`, found `u32`
    |                                      help: you can convert an `u32` to `u64`: `min_length.into()`
    Checking rustc_passes v0.0.0 (/checkout/src/librustc_passes)
error[E0308]: mismatched types
   --> src/librustc_mir/util/elaborate_drops.rs:688:45
    |
---
warning: build failed, waiting for other jobs to finish...
error[E0308]: mismatched types
   --> src/librustc_mir_build/build/matches/mod.rs:598:86
    |
598 |                     self.visit_bindings(subpattern, pattern_user_ty.clone().subslice(from, to), f);
    |                                                                                      |
    |                                                                                      expected `u64`, found `u32`
    |                                                                                      expected `u64`, found `u32`
    |                                                                                      help: you can convert an `u32` to `u64`: `from.into()`
error[E0308]: mismatched types
   --> src/librustc_mir_build/build/matches/mod.rs:598:92
    |
    |
598 |                     self.visit_bindings(subpattern, pattern_user_ty.clone().subslice(from, to), f);
    |                                                                                            |
    |                                                                                            expected `u64`, found `u32`
    |                                                                                            expected `u64`, found `u32`
    |                                                                                            help: you can convert an `u32` to `u64`: `to.into()`
error[E0308]: mismatched types
  --> src/librustc_mir_build/build/matches/util.rs:43:57
   |
   |
43 |                 ProjectionElem::ConstantIndex { offset: idx as u32, min_length, from_end: false };

error[E0308]: mismatched types
  --> src/librustc_mir_build/build/matches/util.rs:53:27
   |
   |
53 |                     from: prefix.len() as u32,
   |                           ^^^^^^^^^^^^^^^^^^^ expected `u64`, found `u32`

error[E0308]: mismatched types
  --> src/librustc_mir_build/build/matches/util.rs:54:41
   |
54 |                     to: if exact_size { min_length - suffix_len } else { suffix_len },
   |                                         |
   |                                         expected `u64`, found `u32`
   |                                         expected `u64`, found `u32`
   |                                         help: you can convert an `u32` to `u64`: `(min_length - suffix_len).into()`
error[E0308]: mismatched types
  --> src/librustc_mir_build/build/matches/util.rs:54:74
   |
   |
54 |                     to: if exact_size { min_length - suffix_len } else { suffix_len },
   |                                                                          |
   |                                                                          expected `u64`, found `u32`
   |                                                                          expected `u64`, found `u32`
   |                                                                          help: you can convert an `u32` to `u64`: `suffix_len.into()`
error[E0308]: mismatched types
  --> src/librustc_mir_build/build/matches/util.rs:64:41
   |
   |
64 |                 offset: if exact_size { min_length - end_offset } else { end_offset },
   |                                         |
   |                                         expected `u64`, found `u32`
   |                                         expected `u64`, found `u32`
   |                                         help: you can convert an `u32` to `u64`: `(min_length - end_offset).into()`
error[E0308]: mismatched types
  --> src/librustc_mir_build/build/matches/util.rs:64:74
   |
   |
64 |                 offset: if exact_size { min_length - end_offset } else { end_offset },
   |                                                                          |
   |                                                                          expected `u64`, found `u32`
   |                                                                          expected `u64`, found `u32`
   |                                                                          help: you can convert an `u32` to `u64`: `end_offset.into()`
error: aborting due to 8 previous errors

For more information about this error, try `rustc --explain E0308`.
error: could not compile `rustc_mir_build`.
error: could not compile `rustc_mir_build`.

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:05:05
== clock drift check ==
  local time: Sat May  2 13:25:42 UTC 2020
  local time: Sat May  2 13:25:42 UTC 2020
  network time: Sat, 02 May 2020 13:25:43 GMT
== end clock drift check ==

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
Terminate orphan process: pid (4758) (python)
##[section]Finishing: Finalize Job
