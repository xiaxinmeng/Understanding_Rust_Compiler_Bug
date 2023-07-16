plain
[00:05:48]    Compiling syntax_pos v0.0.0 (file:///checkout/src/libsyntax_pos)
[00:05:51]    Compiling rustc_errors v0.0.0 (file:///checkout/src/librustc_errors)
[00:07:11]    Compiling proc_macro v0.0.0 (file:///checkout/src/libproc_macro)
[00:07:22]    Compiling syntax_ext v0.0.0 (file:///checkout/src/libsyntax_ext)
[00:07:22] error[E0658]: :vis fragment specifier is experimental and subject to change (see issue #41022)
[00:07:22]    --> librustc/lint/mod.rs:114:6
[00:07:22]     |
[00:07:22] 114 |     ($vis: vis $NAME: ident, $Level: ident, $desc: expr) => (
[00:07:22]     |
[00:07:22]     = help: add #![feature(macro_vis_matcher)] to the crate attributes to enable
[00:07:22] 
[00:07:22] 
[00:07:22] error[E0658]: :vis fragment specifier is experimental and subject to change (see issue #41022)
[00:07:22]    --> librustc/lint/mod.rs:117:6
[00:07:22]     |
[00:07:22] 117 |     ($vis: vis $NAME: ident, $Level: ident, $desc: expr, report_in_external_macro: $rep: expr) => (
[00:07:22]     |
[00:07:22]     = help: add #![feature(macro_vis_matcher)] to the crate attributes to enable
[00:07:22] 
[00:07:22] 
[00:07:22] error[E0658]: :vis fragment specifier is experimental and subject to change (see issue #41022)
[00:07:22]    --> librustc/lint/mod.rs:120:6
[00:07:22]     |
[00:07:22] 120 |     ($vis: vis $NAME: ident, $Level: ident, $desc: expr, $external: expr) => (
[00:07:22]     |
[00:07:22]     = help: add #![feature(macro_vis_matcher)] to the crate attributes to enable
[00:07:22] 
[00:07:22] 
[00:07:22] error[E0658]: :vis fragment specifier is experimental and subject to change (see issue #41022)
[00:07:22]    --> librustc/lint/mod.rs:129:6
[00:07:22]     |
[00:07:22] 129 |     ($vis: vis $NAME: ident, $Level: ident, $desc: expr,
[00:07:22]     |
[00:07:22]     = help: add #![feature(macro_vis_matcher)] to the crate attributes to enable
[00:07:22] 
[00:07:22] 
[00:07:22] error[E0658]: :vis fragment specifier is experimental and subject to change (see issue #41022)
[00:07:22]    --> librustc/lint/mod.rs:144:6
[00:07:22]     |
[00:07:22] 144 |     ($vis: vis $tool: ident ::$NAME: ident, $Level: ident, $desc: expr) => (
[00:07:22]     |
[00:07:22]     = help: add #![feature(macro_vis_matcher)] to the crate attributes to enable
[00:07:22] 
[00:07:22] 
[00:07:22] error[E0658]: :vis fragment specifier is experimental and subject to change (see issue #41022)
[00:07:22]    --> librustc/lint/mod.rs:147:6
[00:07:22]     |
[00:07:22] 147 |     ($vis: vis $tool: ident ::$NAME: ident, $Level: ident, $desc: expr,
[00:07:22]     |
[00:07:22]     = help: add #![feature(macro_vis_matcher)] to the crate attributes to enable
[00:07:22] 
[00:07:22] 
[00:07:22] error[E0658]: :vis fragment specifier is experimental and subject to change (see issue #41022)
[00:07:22]    --> librustc/lint/mod.rs:151:6
[00:07:22]     |
[00:07:22] 151 |     ($vis: vis $tool: ident ::$NAME: ident, $Level: ident, $desc: expr, $external: expr) => (
[00:07:22]     |
[00:07:22]     = help: add #![feature(macro_vis_matcher)] to the crate attributes to enable
[00:07:22] 
[00:07:22] 
[00:07:24] error[E0432]: unresolved import `lint::builtin::PARENTHESIZED_PARAMS_IN_TYPES_AND_MODULES`
[00:07:24]   --> librustc/hir/lowering.rs:50:27
[00:07:24]    |
[00:07:24] 50 | use lint::builtin::{self, PARENTHESIZED_PARAMS_IN_TYPES_AND_MODULES,
[00:07:24]    |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `PARENTHESIZED_PARAMS_IN_TYPES_AND_MODULES` in `lint::builtin`
[00:07:24] 
[00:07:24] error[E0432]: unresolved import `lint::builtin::ELIDED_LIFETIMES_IN_PATHS`
[00:07:24]   --> librustc/hir/lowering.rs:51:21
[00:07:24] 51 |                     ELIDED_LIFETIMES_IN_PATHS};
[00:07:24] 51 |                     ELIDED_LIFETIMES_IN_PATHS};
[00:07:24]    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^ no `ELIDED_LIFETIMES_IN_PATHS` in `lint::builtin`
[00:07:24] 
[00:07:24] error[E0432]: unresolved import `lint::builtin::parser::QUESTION_MARK_MACRO_SEP`
[00:07:24]   --> librustc/lint/mod.rs:41:5
[00:07:24]    |
[00:07:24] 41 | use lint::builtin::parser::QUESTION_MARK_MACRO_SEP;
[00:07:24]    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `QUESTION_MARK_MACRO_SEP` in `lint::builtin::parser`
[00:07:24] 
[00:07:29] error[E0425]: cannot find value `BARE_TRAIT_OBJECTS` in module `builtin`
[00:07:29]     --> librustc/hir/lowering.rs:4795:22
[00:07:29] 4795 |             builtin::BARE_TRAIT_OBJECTS,
[00:07:29]      |                      ^^^^^^^^^^^^^^^^^^ not found in `builtin`
[00:07:29] 
[00:07:29] 
[00:07:29] error[E0425]: cannot find value `ILLEGAL_FLOATING_POINT_LITERAL_PATTERN` in this scope
[00:07:29]    --> librustc/lint/builtin.rs:351:13
[00:07:29] 351 |             ILLEGAL_FLOATING_POINT_LITERAL_PATTERN,
[00:07:29]     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
[00:07:29] 
[00:07:29] error[E0425]: cannot find value `EXCEEDING_BITSHIFTS` in this scope
---
[00:07:29]     |
[00:07:29] 360 |             UNREACHABLE_CODE,
[00:07:29]     |             ^^^^^^^^^^^^^^^^ not found in this scope
[00:07:29] 
[00:07:29] error[E0425]: cannot find value `UNREACHABLE_PATTERNS` in this scope
[00:07:29]    --> librustc/lint/builtin.rs:361:13
[00:07:29] 361 |             UNREACHABLE_PATTERNS,
[00:07:29]     |             ^^^^^^^^^^^^^^^^^^^^ not found in this scope
[00:07:29] 
[00:07:29] error[E0425]: cannot find value `UNUSED_MACROS` in this scope
[00:07:29] error[E0425]: cannot find value `UNUSED_MACROS` in this scope
[00:07:29]    --> librustc/lint/builtin.rs:362:13
[00:07:29]     |
[00:07:29] 362 |             UNUSED_MACROS,
[00:07:29]     |             ^^^^^^^^^^^^^ not found in this scope
[00:07:29] 
[00:07:29] error[E0425]: cannot find value `WARNINGS` in this scope
[00:07:29]    --> librustc/lint/builtin.rs:363:13
[00:07:29] 363 |             WARNINGS,
[00:07:29]     |             ^^^^^^^^ not found in this scope
[00:07:29] 
[00:07:29] error[E0425]: cannot find value `UNUSED_FEATURES` in this scope
---
[00:07:29]     |
[00:07:29] 365 |             STABLE_FEATURES,
[00:07:29]     |             ^^^^^^^^^^^^^^^ not found in this scope
[00:07:29] 
[00:07:29] error[E0425]: cannot find value `UNKNOWN_CRATE_TYPES` in this scope
[00:07:29]    --> librustc/lint/builtin.rs:366:13
[00:07:29] 366 |             UNKNOWN_CRATE_TYPES,
[00:07:29]     |             ^^^^^^^^^^^^^^^^^^^ not found in this scope
[00:07:29] 
[00:07:29] 
[00:07:29] error[E0425]: cannot find value `TRIVIAL_CASTS` in this scope
[00:07:29]    --> librustc/lint/builtin.rs:367:13
[00:07:29] 367 |             TRIVIAL_CASTS,
[00:07:29]     |             ^^^^^^^^^^^^^ not found in this scope
[00:07:29] 
[00:07:29] 
[00:07:29] error[E0425]: cannot find value `TRIVIAL_NUMERIC_CASTS` in this scope
[00:07:29]    --> librustc/lint/builtin.rs:368:13
[00:07:29] 368 |             TRIVIAL_NUMERIC_CASTS,
[00:07:29]     |             ^^^^^^^^^^^^^^^^^^^^^ not found in this scope
[00:07:29] 
[00:07:29] error[E0425]: cannot find value `PRIVATE_IN_PUBLIC` in this scope
---
[00:07:29]     |
[00:07:29] 370 |             PUB_USE_OF_PRIVATE_EXTERN_CRATE,
[00:07:29]     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
[00:07:29] 
[00:07:29] error[E0425]: cannot find value `INVALID_TYPE_PARAM_DEFAULT` in this scope
[00:07:29]    --> librustc/lint/builtin.rs:371:13
[00:07:29] 371 |             INVALID_TYPE_PARAM_DEFAULT,
[00:07:29]     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
[00:07:29] 
[00:07:29] error[E0425]: cannot find value `CONST_ERR` in this scope
---
[00:07:29]     |
[00:07:29] 373 |             RENAMED_AND_REMOVED_LINTS,
[00:07:29]     |             ^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
[00:07:29] 
[00:07:29] error[E0425]: cannot find value `SAFE_EXTERN_STATICS` in this scope
[00:07:29]    --> librustc/lint/builtin.rs:374:13
[00:07:29] 374 |             SAFE_EXTERN_STATICS,
[00:07:29]     |             ^^^^^^^^^^^^^^^^^^^ not found in this scope
[00:07:29] 
[00:07:29] 
[00:07:29] error[E0425]: cannot find value `SAFE_PACKED_BORROWS` in this scope
[00:07:29]    --> librustc/lint/builtin.rs:375:13
[00:07:29] 375 |             SAFE_PACKED_BORROWS,
[00:07:29]     |             ^^^^^^^^^^^^^^^^^^^ not found in this scope
[00:07:29] 
[00:07:29] 
[00:07:29] error[E0425]: cannot find value `PATTERNS_IN_FNS_WITHOUT_BODY` in this scope
[00:07:29]    --> librustc/lint/builtin.rs:376:13
[00:07:29] 376 |             PATTERNS_IN_FNS_WITHOUT_BODY,
[00:07:29]     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
[00:07:29] 
[00:07:29] 
[00:07:29] error[E0425]: cannot find value `LEGACY_DIRECTORY_OWNERSHIP` in this scope
[00:07:29]    --> librustc/lint/builtin.rs:377:13
[00:07:29] 377 |             LEGACY_DIRECTORY_OWNERSHIP,
[00:07:29]     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
[00:07:29] 
[00:07:29] 
[00:07:29] error[E0425]: cannot find value `LEGACY_CONSTRUCTOR_VISIBILITY` in this scope
[00:07:29]    --> librustc/lint/builtin.rs:378:13
[00:07:29] 378 |             LEGACY_CONSTRUCTOR_VISIBILITY,
[00:07:29]     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
[00:07:29] 
[00:07:29] error[E0425]: cannot find value `MISSING_FRAGMENT_SPECIFIER` in this scope
---
[00:07:29]     |
[00:07:29] 381 |             LATE_BOUND_LIFETIME_ARGUMENTS,
[00:07:29]     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
[00:07:29] 
[00:07:29] error[E0425]: cannot find value `INCOHERENT_FUNDAMENTAL_IMPLS` in this scope
[00:07:29]    --> librustc/lint/builtin.rs:382:13
[00:07:29] 382 |             INCOHERENT_FUNDAMENTAL_IMPLS,
[00:07:29]     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
[00:07:29] 
[00:07:29] error[E0425]: cannot find value `DEPRECATED` in this scope
---
[00:07:29]     |
[00:07:29] 385 |             UNUSED_MUT,
[00:07:29]     |             ^^^^^^^^^^ not found in this scope
[00:07:29] 
[00:07:29] error[E0425]: cannot find value `SINGLE_USE_LIFETIMES` in this scope
[00:07:29]    --> librustc/lint/builtin.rs:386:13
[00:07:29] 386 |             SINGLE_USE_LIFETIMES,
[00:07:29]     |             ^^^^^^^^^^^^^^^^^^^^ not found in this scope
[00:07:29] 
[00:07:29] error[E0425]: cannot find value `UNUSED_LIFETIMES` in this scope
---
[00:07:29]     |
[00:07:29] 389 |             TYVAR_BEHIND_RAW_POINTER,
[00:07:29]     |             ^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
[00:07:29] 
[00:07:29] error[E0425]: cannot find value `ELIDED_LIFETIMES_IN_PATHS` in this scope
[00:07:29]    --> librustc/lint/builtin.rs:390:13
[00:07:29] 390 |             ELIDED_LIFETIMES_IN_PATHS,
[00:07:29]     |             ^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
[00:07:29] 
[00:07:29] 
[00:07:29] error[E0425]: cannot find value `BARE_TRAIT_OBJECTS` in this scope
[00:07:29]    --> librustc/lint/builtin.rs:391:13
[00:07:29] 391 |             BARE_TRAIT_OBJECTS,
[00:07:29]     |             ^^^^^^^^^^^^^^^^^^ not found in this scope
[00:07:29] 
[00:07:29] 
[00:07:29] error[E0425]: cannot find value `ABSOLUTE_PATHS_NOT_STARTING_WITH_CRATE` in this scope
[00:07:29]    --> librustc/lint/builtin.rs:392:13
[00:07:29] 392 |             ABSOLUTE_PATHS_NOT_STARTING_WITH_CRATE,
[00:07:29]     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
[00:07:29] 
[00:07:29] error[E0425]: cannot find value `UNSTABLE_NAME_COLLISIONS` in this scope
[00:07:29] error[E0425]: cannot find value `UNSTABLE_NAME_COLLISIONS` in this scope
[00:07:29]    --> librustc/lint/builtin.rs:393:13
[00:07:29]     |
[00:07:29] 393 |             UNSTABLE_NAME_COLLISIONS,
[00:07:29]     |             ^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
[00:07:29] 
[00:07:30] error[E0425]: cannot find value `IRREFUTABLE_LET_PATTERNS` in this scope
[00:07:30]    --> librustc/lint/builtin.rs:394:13
[00:07:30] 394 |             IRREFUTABLE_LET_PATTERNS,
[00:07:30]     |             ^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
[00:07:30] 
[00:07:30] 
[00:07:30] error[E0425]: cannot find value `DUPLICATE_ASSOCIATED_TYPE_BINDINGS` in this scope
[00:07:30]    --> librustc/lint/builtin.rs:395:13
[00:07:30] 395 |             DUPLICATE_ASSOCIATED_TYPE_BINDINGS,
[00:07:30]     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
[00:07:30] 
[00:07:30] 
[00:07:30] error[E0425]: cannot find value `DUPLICATE_MACRO_EXPORTS` in this scope
[00:07:30]    --> librustc/lint/builtin.rs:396:13
[00:07:30] 396 |             DUPLICATE_MACRO_EXPORTS,
[00:07:30]     |             ^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
[00:07:30] 
[00:07:30] 
[00:07:30] error[E0425]: cannot find value `INTRA_DOC_LINK_RESOLUTION_FAILURE` in this scope
[00:07:30]    --> librustc/lint/builtin.rs:397:13
[00:07:30] 397 |             INTRA_DOC_LINK_RESOLUTION_FAILURE,
[00:07:30]     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
[00:07:30] 
[00:07:30] 
[00:07:30] error[E0425]: cannot find value `WHERE_CLAUSES_OBJECT_SAFETY` in this scope
[00:07:30]    --> librustc/lint/builtin.rs:398:13
[00:07:30] 398 |             WHERE_CLAUSES_OBJECT_SAFETY,
[00:07:30]     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
[00:07:30] 
[00:07:30] error[E0425]: cannot find value `PROC_MACRO_DERIVE_RESOLUTION_FALLBACK` in this scope
---
[00:07:30]     |
[00:07:30] 400 |             MACRO_USE_EXTERN_CRATE,
[00:07:30]     |             ^^^^^^^^^^^^^^^^^^^^^^ not found in this scope
[00:07:30] 
[00:07:30] error[E0425]: cannot find value `QUESTION_MARK_MACRO_SEP` in module `parser`
[00:07:30]    --> librustc/lint/builtin.rs:401:21
[00:07:30]     |
[00:07:30] 401 |             parser::QUESTION_MARK_MACRO_SEP,
[00:07:30]     |                     ^^^^^^^^^^^^^^^^^^^^^^^ not found in `parser`
[00:07:30] 
[00:07:30] error[E0425]: cannot find value `WARNINGS` in module `lint::builtin`
[00:07:30]     |
[00:07:30]     |
[00:07:30] 108 |                 self.get_lint_id_level(LintId::of(lint::builtin::WARNINGS),
[00:07:30]     |                                                                  ^^^^^^^^ not found in `lint::builtin`
[00:07:30] 
[00:07:30] error[E0425]: cannot find value `RENAMED_AND_REMOVED_LINTS` in module `builtin`
[00:07:30]     |
[00:07:30] 285 |                         let lint = builtin::RENAMED_AND_REMOVED_LINTS;
[00:07:30]     |                                             ^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `builtin`
[00:07:30] 
[00:07:30] 
[00:07:30] error[E0425]: cannot find value `UNKNOWN_LINTS` in module `builtin`
[00:07:30]     |
[00:07:30]     |
[00:07:30] 307 |                         let lint = builtin::UNKNOWN_LINTS;
[00:07:30]     |                                             ^^^^^^^^^^^^^ not found in `builtin`
[00:07:30] 
[00:07:30] error[E0425]: cannot find value `UNSTABLE_NAME_COLLISIONS` in module `lint::builtin`
[00:07:30]    --> librustc/lint/mod.rs:604:69
[00:07:30]     |
[00:07:30] 604 |         let explanation = if lint_id == LintId::of(::lint::builtin::UNSTABLE_NAME_COLLISIONS) {
[00:07:30]     |                                                                     ^^^^^^^^^^^^^^^^^^^^^^^^ not found in `lint::builtin`
[00:07:30] 
[00:07:30] error[E0425]: cannot find value `DEAD_CODE` in module `lint::builtin`
[00:07:30]    --> librustc/middle/dead.rs:316:43
[00:07:30]     |
[00:07:30] 316 |     tcx.lint_level_at_node(lint::builtin::DEAD_CODE, id).0 == lint::Allow
[00:07:30]     |                                           ^^^^^^^^^ not found in `lint::builtin`
[00:07:30] 
[00:07:30] error[E0425]: cannot find value `DEAD_CODE` in module `lint::builtin`
[00:07:30]    --> librustc/middle/dead.rs:525:43
[00:07:30]     |
[00:07:30] 525 |                 .lint_node(lint::builtin::DEAD_CODE,
[00:07:30]     |                                           ^^^^^^^^^ not found in `lint::builtin`
[00:07:30] 
[00:07:30] error[E0425]: cannot find value `UNUSED_VARIABLES` in module `lint::builtin`
[00:07:30]     --> librustc/middle/liveness.rs:1541:55
[00:07:30]      |
[00:07:30] 1541 |                         .lint_hir_note(lint::builtin::UNUSED_VARIABLES, hir_id, sp,
[00:07:30]      |                                                       ^^^^^^^^^^^^^^^^ not found in `lint::builtin`
[00:07:30] 
[00:07:30] error[E0425]: cannot find value `UNUSED_VARIABLES` in module `lint::builtin`
[00:07:30]     --> librustc/middle/liveness.rs:1548:62
[00:07:30]      |
[00:07:30] 1548 |                         .struct_span_lint_hir(lint::builtin::UNUSED_VARIABLES, hir_id, sp, &msg);
[00:07:30]      |                                                              ^^^^^^^^^^^^^^^^ not found in `lint::builtin`
[00:07:30] 
[00:07:30] error[E0425]: cannot find value `UNUSED_ASSIGNMENTS` in module `lint::builtin`
[00:07:30]     --> librustc/middle/liveness.rs:1582:53
[00:07:30]      |
[00:07:30] 1582 |                 self.ir.tcx.lint_hir(lint::builtin::UNUSED_ASSIGNMENTS, hir_id, sp,
[00:07:30]      |                                                     ^^^^^^^^^^^^^^^^^^ not found in `lint::builtin`
[00:07:30] 
[00:07:30] error[E0425]: cannot find value `UNUSED_ASSIGNMENTS` in module `lint::builtin`
[00:07:30]     --> librustc/middle/liveness.rs:1585:53
[00:07:30]      |
[00:07:30] 1585 |                 self.ir.tcx.lint_hir(lint::builtin::UNUSED_ASSIGNMENTS, hir_id, sp,
[00:07:30]      |                                                     ^^^^^^^^^^^^^^^^^^ not found in `lint::builtin`
[00:07:30] 
[00:07:30] error[E0425]: cannot find value `SINGLE_USE_LIFETIMES` in module `lint::builtin`
[00:07:30]     --> librustc/middle/resolve_lifetime.rs:1457:44
[00:07:30] 1457 |                             lint::builtin::SINGLE_USE_LIFETIMES,
[00:07:30] 1457 |                             lint::builtin::SINGLE_USE_LIFETIMES,
[00:07:30]      |                                            ^^^^^^^^^^^^^^^^^^^^ not found in `lint::builtin`
[00:07:30] 
[00:07:30] error[E0425]: cannot find value `UNUSED_LIFETIMES` in module `lint::builtin`
[00:07:30]     --> librustc/middle/resolve_lifetime.rs:1483:44
[00:07:30] 1483 |                             lint::builtin::UNUSED_LIFETIMES,
[00:07:30] 1483 |                             lint::builtin::UNUSED_LIFETIMES,
[00:07:30]      |                                            ^^^^^^^^^^^^^^^^ not found in `lint::builtin`
[00:07:30] 
[00:07:30] error[E0425]: cannot find value `DEPRECATED` in module `lint::builtin`
[00:07:30]     |
[00:07:30]     |
[00:07:30] 577 |             self.lint_node(lint::builtin::DEPRECATED, id, span, &msg);
[00:07:30]     |                                           ^^^^^^^^^^ not found in `lint::builtin`
[00:07:30] 
[00:07:30] error[E0425]: cannot find value `STABLE_FEATURES` in module `lint::builtin`
[00:07:30]     |
[00:07:30]     |
[00:07:30] 873 |     tcx.lint_node(lint::builtin::STABLE_FEATURES,
[00:07:30]     |                                  ^^^^^^^^^^^^^^^ not found in `lint::builtin`
[00:07:30] error[E0425]: cannot find value `CONST_ERR` in module `rustc::lint::builtin`
[00:07:30]   --> librustc/mir/interpret/error.rs:96:41
[00:07:30]    |
[00:07:30]    |
[00:07:30] 96 |                 ::rustc::lint::builtin::CONST_ERR,
[00:07:30]    |                                         ^^^^^^^^^ not found in `rustc::lint::builtin`
[00:07:30] 
[00:07:30] error[E0425]: cannot find value `WHERE_CLAUSES_OBJECT_SAFETY` in module `lint::builtin`
[00:07:30]    --> librustc/traits/object_safety.rs:136:40
[00:07:30]     |
[00:07:30] 136 |                         lint::builtin::WHERE_CLAUSES_OBJECT_SAFETY,
[00:07:30]     |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `lint::builtin`
[00:07:30] 
[00:07:30] error[E0425]: cannot find value `INCOHERENT_FUNDAMENTAL_IMPLS` in module `lint::builtin`
[00:07:30]    --> librustc/traits/specialize/mod.rs:352:40
[00:07:30] 352 |                         lint::builtin::INCOHERENT_FUNDAMENTAL_IMPLS,
[00:07:30] 352 |                         lint::builtin::INCOHERENT_FUNDAMENTAL_IMPLS,
[00:07:30]     |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ not found in `lint::builtin`
[00:07:55] error: aborting due to 79 previous errors
[00:07:55] 
[00:07:55] Some errors occurred: E0425, E0432, E0658.
[00:07:55] For more information about an error, try `rustc --explain E0425`.
[00:07:55] For more information about an error, try `rustc --explain E0425`.
[00:07:55] error: Could not compile `rustc`.
[00:07:55] 
[00:07:55] Caused by:
[00:07:55]   process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name rustc librustc/lib.rs --color always --error-format json --crate-type dylib --emit=dep-info,link -C prefer-dynamic -C opt-level=2 -C metadata=0dae47d5f57c38d7 -C extra-filename=-0dae47d5f57c38d7 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/release/deps --extern arena=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libarena-b8f9e6fb5ae336d7.so --extern backtrace=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbacktrace-4df0a88f8df46534.rlib --extern bitflags=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbitflags-3907cba388d41ef0.rlib --extern byteorder=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libbyteorder-8246be02936c9b1b.rlib --extern chalk_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libchalk_engine-e5f2643de4c58f16.rlib --extern flate2=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libflate2-3ba4d83893108d52.rlib --extern fmt_macros=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libfmt_macros-2ce739823c85fd99.so --extern graphviz=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libgraphviz-45ae4394366d07fd.so --extern jobserver=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libjobserver-a35da6e99f628d82.rlib --extern lazy_static=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblazy_static-fab92f61cacce826.rlib --extern log=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/liblog-b6c566856a1e65b9.rlib --extern parking_lot=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libparking_lot-299596815de3c6ea.rlib --extern polonius_engine=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libpolonius_engine-c6e8cf18dad58451.rlib --extern proc_macro=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libproc_macro-147c4f5085805223.so --extern rustc_rayon=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon-54bf4ce7e3ad2c0c.rlib --extern rustc_rayon_core=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_rayon_core-d60152ee716e8998.rlib --extern rustc_apfloat=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_apfloat-165c205e2819b15f.rlib --extern rustc_data_structures=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_data_structures-8b624a6d6082b2ff.so --extern rustc_errors=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_errors-89eed8215142aadd.so --extern rustc_fs_util=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_fs_util-3f97c3ca5071d1e2.so --extern rustc_target=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_target-20eb47b9c402fee3.so --extern scoped_tls=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libscoped_tls-6a496b98b1d59ff3.rlib --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-8c9bc9ee6cc9592f.so --extern serialize=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libserialize-8c9bc9ee6cc9592f.rlib --extern syntax=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax-64bbe8e4870170a3.so --extern syntax_pos=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libsyntax_pos-3b51f50aecba154c.so --extern tempfile=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/libtempfile-7a923cd3dbf409d7.rlib -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/backtrace-sys-01a673445b66da02/out -L native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/build/miniz-sys-3c585aa15bfc4e69/out` (exit code: 1)
[00:07:55] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:07:55] expected success, got: exit code: 101
[00:07:55] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1119:9
[00:07:55] travis_fold:end:stage0-rustc

[00:07:55] travis_time:end:stage0-rustc:start=1534290018691604251,finish=1534290192041962735,duration=173350358484


[00:07:55] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:07:55] Build completed unsuccessfully in 0:03:50
[00:07:55] make: *** [all] Error 1
[00:07:55] Makefile:28: recipe for target 'all' failed

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:059647bc
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Aug 14 23:43:12 UTC 2018
ry corruption
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000] Base memory trampoline at [ffff880000099000] 99000 size 24576
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000] Using GB pages for direct mapping
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000] BRK [0x02211000, 0x02211fff] PGTABLE
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000] BRK [0x02212000, 0x02212fff] PGTABLE
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000] BRK [0x02213000, 0x02213fff] PGTABLE
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000] RAMDISK: [mem 0x35614000-0x36b01fff]
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000] ACPI: Early table checksum verification disabled
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000] ACPI: RSDP 0x00000000000F27C0 000014 (v00 Google)
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000] ACPI: RSDT 0x00000000BFFF3430 000038 (v01 Google GOOGRSDT 00000001 GOOG 00000001)
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000] ACPI: FACP 0x00000000BFFFCF60 0000F4 (v02 Google GOOGFACP 00000001 GOOG 00000001)
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000] ACPI: DSDT 0x00000000BFFF3470 0017B2 (v01 Google GOOGDSDT 00000001 GOOG 00000001)
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.006 kernel: [    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000]   Normal   [mem 0x0000000100000000-0x00000003ffffffff]
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000]   Device   empty
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000] Movable zone start for each node
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000] Early memory node ranges
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000]   node   0: [mem 0x0000000000001000-0x000000000009efff]
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000]   node   0: [mem 0x0000000000100000-0x00000000bfff2fff]
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000]   node   0: [mem 0x0000000100000000-0x00000003ffffffff]
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x00000003ffffffff]
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000] On node 0 totalpages: 3932049
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000]   DMA zone: 64 pages used for memmap
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000]   DMA zone: 21 pages reserved
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000]   DMA zone: 3998 pages, LIFO batch:0
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000]   DMA32 zone: 12224 pages used for memmap
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000]   DMA32 zone: 782323 pages, LIFO batch:31
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000]   Normal zone: 49152 pages used for memmap
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000]   Normal zone: 3145728 pages, LIFO batch:31
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000] ACPI: PM-Timer IO Port: 0xb008
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000] ACPI: Local APIC address 0xfee00000
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] dfl dfl lint[0x1])
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000] IOAPIC[0]: apic_id 0, version 17, address 0xfec00000, GSI 0-23
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 5 global_irq 5 high level)
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 9 global_irq 9 high level)
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 10 global_irq 10 high level)
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 11 global_irq 11 high level)
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000] ACPI: IRQ5 used by override.
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000] ACPI: IRQ9 used by override.
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000] ACPI: IRQ10 used by override.
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000] ACPI: IRQ11 used by override.
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000] Using ACPI (MADT) for SMP configuration information
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000] smpboot: Allowing 4 CPUs, 0 hotplug CPUs
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x00000000-0x00000fff]
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x0009f000-0x0009ffff]
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000a0000-0x000effff]
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000] PM: Registered nosave memory: [mem 0x000f0000-0x000fffff]
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xbfff3000-0xbfffffff]
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000] PM: Registered nosave memory: [mem 0xc0000000-0xfffbbfff]
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909690 console=ttyS0 console=ttyS0
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000] PID hash table entries: 4096 (order: 3, 32768 bytes)
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000] Calgary: detecting Calgary via BIOS EBDA area
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000] Calgary: Unable to locate Rio Grande table in EBDA - bailing!
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000] Memory: 15375492K/15728196K available (8272K kernel code, 1304K rwdata, 4004K rodata, 1496K init, 1316K bss, 352704K reserved, 0K cma-reserved)
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000] SLUB: HWalign=64, Order=0-3, MinObjects=0, CPUs=4, Nodes=1
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000] Hierarchical RCU implementation.
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000]  Build-time adjustment of leaf fanout to 64.
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000]  RCU restricting CPUs from NR_CPUS=512 to nr_cpu_ids=4.
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000] RCU: Adjusting geometry for rcu_fanout_leaf=64, nr_cpu_ids=4
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000] NR_IRQS:33024 nr_irqs:456 16
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.000000] Console: colour VGA+ 80x25
Aug 14 23:33:21 travis-job-80b87ac2-58aPUs:      #1
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.842267] kvm-clock: cpu 1, msr 3:ffff1041, secondary cpu clock
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.849506]  #2
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.850883] kvm-clock: cpu 2, msr 3:ffff1081, secondary cpu clock
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.857121]  #3
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.858690] kvm-clock: cpu 3, msr 3:ffff10c1, secondary cpu clock
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.865076] x86: Booted up 1 node, 4 CPUs
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.866666] smpboot: Total of 4 processors activated (18400.00 BogoMIPS)
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.872473] devtmpfs: initialized
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.878177] evm: security.selinux
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.879794] evm: security.SMACK64
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.881604] evm: security.SMACK64EXEC
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.883706] evm: security.SMACK64TRANSMUTE
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.885807] evm: security.SMACK64MMAP
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    0.8876800000:00:03.0: [1af4:1004] type 00 class 0x000000
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    1.134705] pci 0000:00:03.0: reg 0x10: [io  0xc000-0xc03f]
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    1.142158] pci 0000:00:03.0: reg 0x14: [mem 0xfebfe000-0xfebfe07f]
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    1.165457] pci 0000:00:04.0: [1af4:1000] type 00 class 0x020000
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    1.176335] pci 0000:00:04.0: reg 0x10: [io  0xc040-0xc07f]
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    1.183945] pci 0000:00:04.0: reg 0x14: [mem 0xfebff000-0xfebff0ff]
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    1.207723] ACPI: PCI Interrupt Link [LNKA] (IRQs 5 *10 11)
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    1.212197] ACPI: PCI Interrupt Link [LNKB] (IRQs 5 *10 11)
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    1.217595] ACPI: PCI Interrupt Link [LNKC] (IRQs 5 10 *11)
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    1.222695] ACPI: PCI Interrupt Link [LNKD] (IRQs 5 10 *11)
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    1.227087] ACPI: PCI Interrupt Link [LNKS] (IRQs *9)
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    1.250208] ACPI: Enabled 16 GPEs in block 00 to 0F
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    1.252675] vgaarb: loade
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    1.294706] pnp 00:06: Plug and Play ACPI device, IDs PNP0501 (active)
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    1.294865] pnp: PnP ACPI: found 7 devices
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    1.305178] clocksource: acpi_pm: mask: 0xffffff max_cycles: 0xffffff, max_idle_ns: 2085701024 ns
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    1.308559] pci_bus 0000:00: resource 4 [io  0x0000-0x0cf7 window]
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    1.308561] pci_bus 0000:00: resource 5 [io  0x0d00-0xffff window]
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    1.308563] pci_bus 0000:00: resource 6 [mem 0x000a0000-0x000bffff window]
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    1.308565] pci_bus 0000:00: resource 7 [mem 0xc0000000-0xfebfffff window]
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    1.308606] NET: Registered protocol family 2
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    1.310552] TCP established hash table entries: 131072 (order: 8, 1048576 bytes)
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    1.313444] TCP bind hash table entries: 65536 (order: 8, 1048576 bytes)
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    1.316201] TCP: Hash tables configured (established 131072 bind 65536)
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-9el: [    3.586195] input: Sleep Button as /devices/LNXSYSTM:00/LNXSLPBN:00/input/input1
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    3.589086] ACPI: Sleep Button [SLPF]
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    3.591918] GHES: HEST is not enabled!
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    3.596418] ACPI: PCI Interrupt Link [LNKC] enabled at IRQ 11
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    3.598687] virtio-pci 0000:00:03.0: virtio_pci: leaving for legacy driver
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    3.608135] ACPI: PCI Interrupt Link [LNKD] enabled at IRQ 10
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    3.611002] virtio-pci 0000:00:04.0: virtio_pci: leaving for legacy driver
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    3.620188] Serial: 8250/16550 driver, 32 ports, IRQ sharing enabled
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    3.644359] 00:03: ttyS0 at I/O 0x3f8 (irq = 4, base_baud = 115200) is a 16550A
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    3.668790] 00:04: ttyS1 at I/O 0x2f8 (irq = 3, base_baud = 115200) is a 16550A
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    3.693257] 00:05: ttyS2 at I/O 0x3e8 (irq = 6, base_baud = 115200) is a 16550A
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [    3.717643] 00:06: ttyS3 at I/O 0x2e8 (irq = 7, base_baud = 115200) iac2-58a0-4210-94ca-f06909699206 kernel: [   10.724122] ppdev: user-space parallel port driver
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [   10.936934] random: mktemp: uninitialized urandom read (6 bytes read, 55 bits of entropy available)
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [   11.004768] random: mktemp: uninitialized urandom read (6 bytes read, 55 bits of entropy available)
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [   11.077716] random: cloud-init: uninitialized urandom read (32 bytes read, 56 bits of entropy available)
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [   11.257914] random: cloud-init: uninitialized urandom read (32 bytes read, 56 bits of entropy available)
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [   11.551859] random: mktemp: uninitialized urandom read (12 bytes read, 59 bits of entropy available)
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [   11.643826] random: mktemp: uninitialized urandom read (6 bytes read, 60 bits of entropy available)
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [   11.733368] EXT4-fs (sda1): resizing filesystem from 3931904 to 7864064 blocks
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [   11.792147] EXT4-fs (sda1): resized filesystem to 7864064
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [   12.305434] init: failsafe main process (1095) killed by TERM signal
Aug 14 23:33:21 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 rsyslogd-2039: Could no open output pipe '/dev/xconsole': No such file or directory [try http://www.rsyslog.com/e/2039 ]
Aug 14 23:33:22 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 instance-setup: INFO Running set_multiqueue.
Aug 14 23:33:22 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 instance-setup: INFO Set channels for eth0 to 4.
Aug 14 23:33:22 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 instance-setup: INFO Setting /proc/irq/25/smp_affinity_list to 0 for device virtio1.
Aug 14 23:33:22 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 instance-setup: INFO /proc/irq/25/smp_affinity_list: real affinity 0
Aug 14 23:33:22 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 instance-setup: INFO Setting /proc/irq/26/smp_affinity_list to 0 for device virtio1.
Aug 14 23:33:22 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 instance-setup: INFO /proc/irq/26/smp_affinity_list: real affinity 0
Aug 14 23:33:22 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 instance-setup: INFO Setting /proc/irq/27/smp_affinity_list to 1 for device virtio1.
Aug 14 23:33:22 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 instance-setup: INFO /proc/irq/27/smp_affinity_list: real affinity 1
Aug 14 23:33:22 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 instance-setup: INFO Setting /proc/irq/28/smp_affinity_list to 1 for device virtio1.
Aug 14 23:33:22 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 instance-setup: INFO /proc/irq/28/smp_affinity_list: real affinity 1
Aug 14 23:33:22 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 instance-setup: INFO Setting /proc/irq/29/smp_affinity_list to 2 for device virtio1.
Aug 14 23:33:2oogle IP Forwarding daemon.
Aug 14 23:33:22 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 google-clock-skew: INFO Starting Google Clock Skew daemon.
Aug 14 23:33:22 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 google-clock-skew: INFO Clock drift token has changed: 0.
Aug 14 23:33:22 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 google-accounts: INFO Starting Google Accounts daemon.
Aug 14 23:33:22 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 google-accounts: INFO Creating a new user account for me.
Aug 14 23:33:22 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 google-accounts: INFO Created user account me.
Aug 14 23:33:22 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 google-accounts: INFO Creating a new user account for aj.
Aug 14 23:33:23 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 google-accounts: INFO Created user account aj.
Aug 14 23:33:23 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 google-accounts: INFO Creating a new user account for carmen.
Aug 14 23:33:23 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 google-accounts: INFO Created user account carmen.
Aug 14 23:33:23 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 google-accounts: INFO Removing user packer.
Aug 14 23:33:23 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [   13.484607] random: nonblocking pool is initialized
Aug 14 23:33:23 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [   13.652078] floppy0: no floppy controllers found
Aug 14 23:33:23 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 pollinate: system was previously seeded at [2017-12-05 19:31:29.715998981 +0000]
Aug 14 23:33:23 travis-job-80b87ac2-vis-job-80b87ac2-58a0-4210-94ca-f06909699206 ec2: 1024 b6:2f:ba:ff:f8:27:fb:06:c8:a7:4b:3a:6a:1d:27:09  root@travis-job-80b87ac2-58a0-4210-94ca-f06909699206 (DSA)
Aug 14 23:33:31 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 ec2: 256 92:33:8c:03:9a:46:82:78:64:63:56:2b:60:4c:bd:39  root@travis-job-80b87ac2-58a0-4210-94ca-f06909699206 (ECDSA)
Aug 14 23:33:31 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 ec2: 256 87:b4:db:46:e2:8b:20:a2:ef:c3:e8:4b:04:be:70:6f  root@travis-job-80b87ac2-58a0-4210-94ca-f06909699206 (ED25519)
Aug 14 23:33:31 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 ec2: 2048 04:ac:31:8e:09:10:af:0c:1b:00:52:48:34:a1:02:fb  root@travis-job-80b87ac2-58a0-4210-94ca-f06909699206 (RSA)
Aug 14 23:33:31 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 ec2: -----END SSH HOST KEY FINGERPRINTS-----
Aug 14 23:33:31 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 ec2: #############################################################
Aug 14 23:33:38 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 ntpdate[2148]: the NTP socket is in use, exiting
Aug 14 23:35:15 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [  125.798923] IPv6: ADDRCONF(NETDEV_UP): docker0: link is not ready
Aug 14 23:36:16 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [  186.687908] device veth463fde8 entered promiscuous mode
Aug 14 23:36:16 travis-job-80b87ac2-58a0-4210-94ca-f06909699206 kernel: [  186.790810] cgroup: docker-runc (4769) created nested cgroup for controller "memory" which has incomplete hierarchy support. Nested cgroups may change behavior in the future.
Aug 14 23:36:16 travis-job-80b87ac2-58a0-4210-94ca-f069ot/lib/rustlib/x86_64-unknown-linux-gnu/lib
49356 ./src/llvm/test/CodeGen/X86
43692 ./src/libcompiler_builtins
42624 ./src/libcompiler_builtins/compiler-rt
41112 ./obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu
