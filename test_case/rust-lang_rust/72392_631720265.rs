plain
##[section]Starting: Linux mingw-check
##[section]Starting: Initialize job
Agent name: 'Azure Pipelines 27'
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
[command]/bin/bash --noprofile --norc /home/vsts/work/_temp/f59f519d-873d-4cf4-b851-10d5d4ddfc58.sh

##[section]Finishing: Disable git automatic line ending conversion
##[section]Starting: Checkout rust-lang/rust@refs/pull/72392/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
---
##[command]git remote add origin https://github.com/rust-lang/rust
##[command]git config gc.auto 0
##[command]git config --get-all http.https://github.com/rust-lang/rust.extraheader
##[command]git config --get-all http.proxy
##[command]git -c http.extraheader="AUTHORIZATION: basic ***" fetch --force --tags --prune --progress --no-recurse-submodules --depth=2 origin +refs/heads/*:refs/remotes/origin/* +refs/pull/72392/merge:refs/remotes/pull/72392/merge
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
    Checking rustc_ast_lowering v0.0.0 (/checkout/src/librustc_ast_lowering)
    Checking chalk-solve v0.10.0
    Checking rustc_ast_passes v0.0.0 (/checkout/src/librustc_ast_passes)
    Checking rustc_expand v0.0.0 (/checkout/src/librustc_expand)
    Checking rustc_builtin_macros v0.0.0 (/checkout/src/librustc_builtin_macros)
error[E0277]: the trait bound `rustc_ast::ast::InlineAsmTemplatePiece: std::marker::Copy` is not satisfied
   --> src/librustc_middle/ty/codec.rs:320:17
    |
138 |    pub fn decode_arena_allocable<D, T: ArenaAllocatable + Decodable>(
    |                                        ---------------- required by this bound in `ty::codec::decode_arena_allocable`
313 | /  macro_rules! impl_arena_allocatable_decoder {
313 | /  macro_rules! impl_arena_allocatable_decoder {
314 | |      ([]$args:tt) => {};
315 | |      ([decode $(, $attrs:ident)*]
316 | |       [[$DecoderName:ident [$($typaram:tt),*]], [$name:ident: $ty:ty], $tcx:lifetime]) => {
320 | |                  decode_arena_allocable(self)
320 | |                  decode_arena_allocable(self)
    | |                  ^^^^^^^^^^^^^^^^^^^^^^ the trait `std::marker::Copy` is not implemented for `rustc_ast::ast::InlineAsmTemplatePiece`
333 | |      };
334 | |  }
334 | |  }
    | |__- in this expansion of `impl_arena_allocatable_decoder!` (#4)
337 | /  macro_rules! impl_arena_allocatable_decoders {
337 | /  macro_rules! impl_arena_allocatable_decoders {
338 | |      ($args:tt, [$($a:tt $name:ident: $ty:ty,)*], $tcx:lifetime) => {
339 | |          $(
340 | |              impl_arena_allocatable_decoder!($a [$args, [$name: $ty], $tcx]);
341 | |          )*
342 | |      }
343 | |  }
343 | |  }
    | |__- in this expansion of `impl_arena_allocatable_decoders!` (#3)
346 | /  macro_rules! implement_ty_decoder {
346 | /  macro_rules! implement_ty_decoder {
347 | |      ($DecoderName:ident <$($typaram:tt),*>) => {
348 | |          mod __ty_decoder_impl {
349 | |              use std::borrow::Cow;
...   |
399 | |              rustc_hir::arena_types!(impl_arena_allocatable_decoders, [$DecoderName [$($typaram),*]], 'tcx);
...   |
494 | |      }
495 | |  }
495 | |  }
    | |__- in this expansion of `implement_ty_decoder!` (#1)
    | 
   ::: src/librustc_middle/ty/query/on_disk_cache.rs:558:1
    |
558 |    implement_ty_decoder!(CacheDecoder<'a, 'tcx>);
    | 
   ::: /checkout/src/librustc_hir/arena.rs:11:1
    |
    |
11  |  / macro_rules! arena_types {
12  |  |     ($macro:path, $args:tt, $tcx:lifetime) => (
13  |  |         $macro!($args, [
14  | ||             // HIR types
14  | ||             // HIR types
15  | ||             [few] hir_krate: rustc_hir::Crate<$tcx>,
16  | ||             [] arm: rustc_hir::Arm<$tcx>,
...   ||
49  | ||             [] where_predicate: rustc_hir::WherePredicate<$tcx>,
50  | ||         ], $tcx);
    | ||_________________- in this macro invocation (#3)
52  |  | }
52  |  | }
    |  |_- in this expansion of `rustc_hir::arena_types!` (#2)
    |
    = note: required because of the requirements on the impl of `arena::ArenaAllocatable` for `rustc_ast::ast::InlineAsmTemplatePiece`

error[E0277]: the trait bound `rustc_ast::ast::InlineAsmTemplatePiece: std::marker::Copy` is not satisfied
   --> src/librustc_middle/ty/codec.rs:327:17
    |
148 |    pub fn decode_arena_allocable_slice<D, T: ArenaAllocatable + Decodable>(
    |                                              ---------------- required by this bound in `ty::codec::decode_arena_allocable_slice`
313 | /  macro_rules! impl_arena_allocatable_decoder {
313 | /  macro_rules! impl_arena_allocatable_decoder {
314 | |      ([]$args:tt) => {};
315 | |      ([decode $(, $attrs:ident)*]
316 | |       [[$DecoderName:ident [$($typaram:tt),*]], [$name:ident: $ty:ty], $tcx:lifetime]) => {
327 | |                  decode_arena_allocable_slice(self)
327 | |                  decode_arena_allocable_slice(self)
    | |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the trait `std::marker::Copy` is not implemented for `rustc_ast::ast::InlineAsmTemplatePiece`
333 | |      };
334 | |  }
334 | |  }
    | |__- in this expansion of `impl_arena_allocatable_decoder!` (#4)
337 | /  macro_rules! impl_arena_allocatable_decoders {
337 | /  macro_rules! impl_arena_allocatable_decoders {
338 | |      ($args:tt, [$($a:tt $name:ident: $ty:ty,)*], $tcx:lifetime) => {
339 | |          $(
340 | |              impl_arena_allocatable_decoder!($a [$args, [$name: $ty], $tcx]);
341 | |          )*
342 | |      }
343 | |  }
343 | |  }
    | |__- in this expansion of `impl_arena_allocatable_decoders!` (#3)
346 | /  macro_rules! implement_ty_decoder {
346 | /  macro_rules! implement_ty_decoder {
347 | |      ($DecoderName:ident <$($typaram:tt),*>) => {
348 | |          mod __ty_decoder_impl {
349 | |              use std::borrow::Cow;
...   |
399 | |              rustc_hir::arena_types!(impl_arena_allocatable_decoders, [$DecoderName [$($typaram),*]], 'tcx);
...   |
494 | |      }
495 | |  }
495 | |  }
    | |__- in this expansion of `implement_ty_decoder!` (#1)
    | 
   ::: src/librustc_middle/ty/query/on_disk_cache.rs:558:1
    |
558 |    implement_ty_decoder!(CacheDecoder<'a, 'tcx>);
    | 
   ::: /checkout/src/librustc_hir/arena.rs:11:1
    |
    |
11  |  / macro_rules! arena_types {
12  |  |     ($macro:path, $args:tt, $tcx:lifetime) => (
13  |  |         $macro!($args, [
14  | ||             // HIR types
14  | ||             // HIR types
15  | ||             [few] hir_krate: rustc_hir::Crate<$tcx>,
16  | ||             [] arm: rustc_hir::Arm<$tcx>,
...   ||
49  | ||             [] where_predicate: rustc_hir::WherePredicate<$tcx>,
50  | ||         ], $tcx);
    | ||_________________- in this macro invocation (#3)
52  |  | }
52  |  | }
    |  |_- in this expansion of `rustc_hir::arena_types!` (#2)
    |
    = note: required because of the requirements on the impl of `arena::ArenaAllocatable` for `rustc_ast::ast::InlineAsmTemplatePiece`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.
error: could not compile `rustc_middle`.
error: could not compile `rustc_middle`.

To learn more, run the command again with --verbose.
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "2" "--release" "--color" "always" "--features" " llvm" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:04:26
== clock drift check ==
  local time: Wed May 20 20:50:47 UTC 2020
  local time: Wed May 20 20:50:47 UTC 2020
  network time: Wed, 20 May 2020 20:50:47 GMT
== end clock drift check ==

##[error]Bash exited with code '1'.
##[section]Finishing: Run build
##[section]Starting: Checkout rust-lang/rust@refs/pull/72392/merge to s
Task         : Get sources
Description  : Get sources from a repository. Supports Git, TfsVC, and SVN repositories.
Version      : 1.0.0
Author       : Microsoft
Author       : Microsoft
Help         : [More Information](https://go.microsoft.com/fwlink/?LinkId=798199)
==============================================================================
Cleaning any cached credential from repository: rust-lang/rust (GitHub)
##[section]Finishing: Checkout rust-lang/rust@refs/pull/72392/merge to s
Cleaning up task key
Start cleaning up orphan processes.
Terminate orphan process: pid (3498) (python)
##[section]Finishing: Finalize Job
