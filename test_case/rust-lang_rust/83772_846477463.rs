plain
    Checking rustc_type_ir v0.0.0 (/checkout/compiler/rustc_type_ir)
error[E0658]: use of unstable library feature 'trusted_step'
   --> /checkout/compiler/rustc_index/src/vec.rs:208:21
    |
69  |      / macro_rules! newtype_index {
70  |      |     // ---- public rules ----
72  |      |     // Use default constants
...        |
86  | /    |         $crate::newtype_index!(
86  | /    |         $crate::newtype_index!(
87  | |    |             // Leave out derives marker so we can use its absence to ensure it comes first
88  | |    |             @attrs        [$(#[$attrs])*]
89  | |    |             @type         [$name]
...   |    |
93  | |    |             @debug_format ["{}"]
94  | |    |                           $($tokens)+);
    | |____|_______________________________________- in this macro invocation (#2)
...        |
208 |      |         unsafe impl ::std::iter::TrustedStep for $type {}
...        |
365 | /    |         $crate::newtype_index!(
365 | /    |         $crate::newtype_index!(
366 | |    |             @derives      []
367 | |    |             @attrs        [$(#[$attrs])*]
368 | |    |             @type         [$type]
...   |    |
371 | |    |             @debug_format [$debug_format]
372 | |    |                           $($tokens)*);
    | |____|_______________________________________- in this macro invocation (#3)
454 | /    |         $crate::newtype_index!(
454 | /    |         $crate::newtype_index!(
455 | |    |             @derives      [$($derives,)*]
456 | |    |             @attrs        [$(#[$attrs])*]
457 | |    |             @type         [$type]
...   |    |
460 | |    |             @debug_format [$debug_format]
461 | |    |                           $($tokens)*);
    | |____|_______________________________________- in this macro invocation (#4)
476 | /    |         $crate::newtype_index!(
476 | /    |         $crate::newtype_index!(
477 | |    |             @derives      [$($derives,)*]
478 | |    |             @attrs        [$(#[$attrs])*]
479 | |    |             @type         [$type]
...   |    |
482 | |    |             @debug_format [$debug_format]
483 | |    |                           $($tokens)*);
    | |____|_______________________________________- in this macro invocation (#5)
485 |      | }
    |      | -
    |      | |
    |      | |
    |      | in this expansion of `rustc_index::newtype_index!` (#1)
    |      | in this expansion of `$crate::newtype_index!` (#2)
    |      | in this expansion of `$crate::newtype_index!` (#3)
    |      |_in this expansion of `$crate::newtype_index!` (#4)
    |        in this expansion of `$crate::newtype_index!` (#5)
   ::: compiler/rustc_type_ir/src/lib.rs:111:1
    |
111 | /      rustc_index::newtype_index! {
111 | /      rustc_index::newtype_index! {
112 | |          /// A [De Bruijn index][dbi] is a standard means of representing
113 | |          /// regions (and perhaps later types) in a higher-ranked setting. In
114 | |          /// particular, imagine a type like this:
154 | |          }
155 | |      }
    | |______- in this macro invocation (#1)
    |
    |
    = help: add `#![feature(trusted_step)]` to the crate attributes to enable

error: implementing `rustc_specialization_trait` traits is unstable
   --> /checkout/compiler/rustc_index/src/vec.rs:208:9
69  |      / macro_rules! newtype_index {
69  |      / macro_rules! newtype_index {
70  |      |     // ---- public rules ----
72  |      |     // Use default constants
...        |
86  | /    |         $crate::newtype_index!(
86  | /    |         $crate::newtype_index!(
87  | |    |             // Leave out derives marker so we can use its absence to ensure it comes first
88  | |    |             @attrs        [$(#[$attrs])*]
89  | |    |             @type         [$name]
...   |    |
93  | |    |             @debug_format ["{}"]
94  | |    |                           $($tokens)+);
    | |____|_______________________________________- in this macro invocation (#2)
...        |
208 |      |         unsafe impl ::std::iter::TrustedStep for $type {}
...        |
365 | /    |         $crate::newtype_index!(
365 | /    |         $crate::newtype_index!(
366 | |    |             @derives      []
367 | |    |             @attrs        [$(#[$attrs])*]
368 | |    |             @type         [$type]
...   |    |
371 | |    |             @debug_format [$debug_format]
372 | |    |                           $($tokens)*);
    | |____|_______________________________________- in this macro invocation (#3)
454 | /    |         $crate::newtype_index!(
454 | /    |         $crate::newtype_index!(
455 | |    |             @derives      [$($derives,)*]
456 | |    |             @attrs        [$(#[$attrs])*]
457 | |    |             @type         [$type]
...   |    |
460 | |    |             @debug_format [$debug_format]
461 | |    |                           $($tokens)*);
    | |____|_______________________________________- in this macro invocation (#4)
476 | /    |         $crate::newtype_index!(
476 | /    |         $crate::newtype_index!(
477 | |    |             @derives      [$($derives,)*]
478 | |    |             @attrs        [$(#[$attrs])*]
479 | |    |             @type         [$type]
...   |    |
482 | |    |             @debug_format [$debug_format]
483 | |    |                           $($tokens)*);
    | |____|_______________________________________- in this macro invocation (#5)
485 |      | }
    |      | -
    |      | |
    |      | |
    |      | in this expansion of `rustc_index::newtype_index!` (#1)
    |      | in this expansion of `$crate::newtype_index!` (#2)
    |      | in this expansion of `$crate::newtype_index!` (#3)
    |      |_in this expansion of `$crate::newtype_index!` (#4)
    |        in this expansion of `$crate::newtype_index!` (#5)
   ::: compiler/rustc_type_ir/src/lib.rs:111:1
    |
111 | /      rustc_index::newtype_index! {
111 | /      rustc_index::newtype_index! {
112 | |          /// A [De Bruijn index][dbi] is a standard means of representing
113 | |          /// regions (and perhaps later types) in a higher-ranked setting. In
114 | |          /// particular, imagine a type like this:
154 | |          }
155 | |      }
    | |______- in this macro invocation (#1)
    |
    |
    = help: add `#![feature(min_specialization)]` to the crate attributes to enable
    Checking rustc_span v0.0.0 (/checkout/compiler/rustc_span)
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0658`.
---
    |  _-
    | |_|
    | |_|
    | |
70  | |     // ---- public rules ----
72  | |     // Use default constants
...   |
86  | /         $crate::newtype_index!(
86  | /         $crate::newtype_index!(
87  |               // Leave out derives marker so we can use its absence to ensure it comes first
88  |               @attrs        [$(#[$attrs])*]
89  |               @type         [$name]
...
93  |               @debug_format ["{}"]
94  | |                           $($tokens)+);
    | |_______________________________________- in this macro invocation (#2)
...
208 | |         unsafe impl ::std::iter::TrustedStep for $type {}
...   |
348 | /         $crate::newtype_index!(
348 | /         $crate::newtype_index!(
349 |               @derives      []
350 |               @attrs        [$(#[$attrs])*]
351 |               @type         [$type]
...
354 |               @debug_format [$debug_format]
355 | |                           $($tokens)*);
    | |_______________________________________- in this macro invocation (#3)
484 | |     );
485 | | }
    | | -
    | |_|
    | |_|
    | |_in this expansion of `rustc_index::newtype_index!` (#1)
    | |_in this expansion of `$crate::newtype_index!` (#2)
    |   in this expansion of `$crate::newtype_index!` (#3)
   ::: compiler/rustc_span/src/def_id.rs:12:1
    |
    |
12  | / rustc_index::newtype_index! {
13  | |     pub struct CrateId {
14  | |         ENCODABLE = custom
16  | | }
    | |_- in this macro invocation (#1)
    |
    |
    = help: add `#![feature(trusted_step)]` to the crate attributes to enable
error: aborting due to previous error

For more information about this error, try `rustc --explain E0658`.
error: build failed
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "check" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--color" "always" "--features" " llvm max_level_info" "--manifest-path" "/checkout/compiler/rustc/Cargo.toml" "-p" "rustc-main" "-p" "rustc_driver" "-p" "rustc_target" "-p" "rustc_index" "-p" "rustc_macros" "-p" "rustc_save_analysis" "-p" "rustc_lexer" "-p" "rustc_data_structures" "-p" "rustc_graphviz" "-p" "rustc_span" "-p" "rustc_arena" "-p" "rustc_error_codes" "-p" "rustc_feature" "-p" "rustc_parse" "-p" "rustc_metadata" "-p" "rustc_expand" "-p" "rustc_lint_defs" "-p" "rustc_ast_passes" "-p" "rustc_attr" "-p" "rustc_ast_pretty" "-p" "rustc_hir" "-p" "rustc_interface" "-p" "rustc_trait_selection" "-p" "rustc_infer" "-p" "rustc_parse_format" "-p" "rustc_symbol_mangling" "-p" "rustc_ast_lowering" "-p" "rustc_resolve" "-p" "rustc_query_impl" "-p" "rustc_query_system" "-p" "rustc_builtin_macros" "-p" "rustc_codegen_llvm" "-p" "rustc_fs_util" "-p" "rustc_llvm" "-p" "rustc_passes" "-p" "rustc_incremental" "-p" "rustc_privacy" "-p" "rustc_traits" "-p" "rustc_ty_utils" "-p" "rustc_mir" "-p" "coverage_test_macros" "-p" "rustc_apfloat" "-p" "rustc_session" "-p" "rustc_errors" "-p" "rustc_serialize" "-p" "rustc_plugin_impl" "-p" "rustc_lint" "-p" "rustc_mir_build" "-p" "rustc_hir_pretty" "-p" "rustc_ast" "-p" "rustc_typeck" "-p" "rustc_middle" "-p" "rustc_type_ir" "-p" "rustc_codegen_ssa" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap check
Build completed unsuccessfully in 0:01:34
