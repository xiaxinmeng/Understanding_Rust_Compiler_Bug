plain
Copying stage2 rustc from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Check compiletest suite=ui-fulldeps mode=ui (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 71 tests
......FF....F..F.....F.i..F.....FF.F..FFF.FFFFFF.F.F...................

---- [ui] src/test/ui-fulldeps/internal-lints/default_hash_types.rs stdout ----
diff of stderr:


4 LL |     let _map: HashMap<String, String> = HashMap::default();
6    |
6    |
+    = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary
8   --> $DIR/default_hash_types.rs:4:9
9    |


10 LL | #![deny(rustc::default_hash_types)]
11    |         ^^^^^^^^^^^^^^^^^^^^^^^^^
-    = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary
13 
14 error: prefer `FxHashMap` over `HashMap`, it has better performance


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/default_hash_types/default_hash_types.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/default_hash_types/default_hash_types.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args internal-lints/default_hash_types.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/internal-lints/default_hash_types.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/default_hash_types" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/default_hash_types/auxiliary"
stdout: none
--- stderr -------------------------------
error: prefer `FxHashMap` over `HashMap`, it has better performance
   |
   |
LL |     let _map: HashMap<String, String> = HashMap::default();
   |
   |
   = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary
  --> /checkout/src/test/ui-fulldeps/internal-lints/default_hash_types.rs:4:9
   |
   |
LL | #![deny(rustc::default_hash_types)]


error: prefer `FxHashMap` over `HashMap`, it has better performance
   |
   |
LL |     let _map: HashMap<String, String> = HashMap::default();
   |
   |
   = note: a `use rustc_data_structures::fx::FxHashMap` may be necessary

error: prefer `FxHashSet` over `HashSet`, it has better performance
   |
   |
LL |     let _set: HashSet<String> = HashSet::default();
   |
   |
   = note: a `use rustc_data_structures::fx::FxHashSet` may be necessary

error: prefer `FxHashSet` over `HashSet`, it has better performance
   |
   |
LL |     let _set: HashSet<String> = HashSet::default();
   |
   |
   = note: a `use rustc_data_structures::fx::FxHashSet` may be necessary
error: aborting due to 4 previous errors
------------------------------------------



---- [ui] src/test/ui-fulldeps/internal-lints/existing_doc_keyword.rs stdout ----
diff of stderr:

4 LL | #[doc(keyword = "tadam")]
6    |
+    = help: only existing keywords are allowed in core/std
7 note: the lint level is defined here
8   --> $DIR/existing_doc_keyword.rs:8:9
8   --> $DIR/existing_doc_keyword.rs:8:9
9    |

10 LL | #![deny(rustc::existing_doc_keyword)]
-    = help: only existing keywords are allowed in core/std
13 
14 error: aborting due to previous error
15 
---
To only update this specific test, also pass `--test-args internal-lints/existing_doc_keyword.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/internal-lints/existing_doc_keyword.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/existing_doc_keyword" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/existing_doc_keyword/auxiliary"
stdout: none
--- stderr -------------------------------
error: found non-existing keyword `tadam` used in `#[doc(keyword = \"...\")]`
   |
   |
LL | #[doc(keyword = "tadam")] //~ ERROR
   |
   = help: only existing keywords are allowed in core/std
note: the lint level is defined here
  --> /checkout/src/test/ui-fulldeps/internal-lints/existing_doc_keyword.rs:8:9
  --> /checkout/src/test/ui-fulldeps/internal-lints/existing_doc_keyword.rs:8:9
   |
LL | #![deny(rustc::existing_doc_keyword)]

error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui-fulldeps/internal-lints/query_stability.rs stdout ----
diff of stderr:

4 LL |     for _ in x.drain() {}
6    |
6    |
+    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale
8   --> $DIR/query_stability.rs:4:9
9    |


10 LL | #![deny(rustc::potential_query_instability)]
11    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-    = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale
14 error: using `iter` can result in unstable query results
15   --> $DIR/query_stability.rs:16:16



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/query_stability/query_stability.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args internal-lints/query_stability.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/internal-lints/query_stability.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/query_stability" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/query_stability/auxiliary"
stdout: none
--- stderr -------------------------------
error: using `drain` can result in unstable query results
   |
   |
LL |     for _ in x.drain() {}
   |
   |
   = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale
  --> /checkout/src/test/ui-fulldeps/internal-lints/query_stability.rs:4:9
   |
   |
LL | #![deny(rustc::potential_query_instability)]

error: using `iter` can result in unstable query results
  --> /checkout/src/test/ui-fulldeps/internal-lints/query_stability.rs:16:16
   |
   |
LL |     for _ in x.iter() {}
   |
   |
   = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale
error: using `iter_mut` can result in unstable query results
  --> /checkout/src/test/ui-fulldeps/internal-lints/query_stability.rs:19:36
   |
   |
LL |     for _ in Some(&mut x).unwrap().iter_mut() {}
   |
   |
   = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale
error: using `into_iter` can result in unstable query results
  --> /checkout/src/test/ui-fulldeps/internal-lints/query_stability.rs:22:14
   |
LL |     for _ in x {}
LL |     for _ in x {}
   |              ^
   |
   = note: if you believe this case to be fine, allow this lint and add a comment explaining your rationale
error: aborting due to 4 previous errors
------------------------------------------



---- [ui] src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs stdout ----
diff of stderr:

4 LL | impl LintPass for Foo {
6    |
6    |
+    = help: try using `declare_lint_pass!` or `impl_lint_pass!` instead
8   --> $DIR/lint_pass_impl_without_macro.rs:4:9
9    |

10 LL | #![deny(rustc::lint_pass_impl_without_macro)]
10 LL | #![deny(rustc::lint_pass_impl_without_macro)]
11    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
-    = help: try using `declare_lint_pass!` or `impl_lint_pass!` instead
13 
14 error: implementing `LintPass` by hand


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro/lint_pass_impl_without_macro.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro/lint_pass_impl_without_macro.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args internal-lints/lint_pass_impl_without_macro.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro/auxiliary"
stdout: none
--- stderr -------------------------------
error: implementing `LintPass` by hand
   |
   |
LL | impl LintPass for Foo { //~ERROR implementing `LintPass` by hand
   |
   |
   = help: try using `declare_lint_pass!` or `impl_lint_pass!` instead
  --> /checkout/src/test/ui-fulldeps/internal-lints/lint_pass_impl_without_macro.rs:4:9
   |
LL | #![deny(rustc::lint_pass_impl_without_macro)]
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: implementing `LintPass` by hand
   |
   |
LL |         impl LintPass for Custom { //~ERROR implementing `LintPass` by hand
...
...
LL | custom_lint_pass_macro!();
   |
   |
   = help: try using `declare_lint_pass!` or `impl_lint_pass!` instead
   = note: this error originates in the macro `custom_lint_pass_macro` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/ty_tykind_usage" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unstable-options" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/internal-lints/ty_tykind_usage/auxiliary"
stdout: none
--- stderr -------------------------------
error: usage of `ty::TyKind::<kind>`
   |
   |
LL |     let kind = TyKind::Bool; //~ ERROR usage of `ty::TyKind::<kind>`
   |                ^^^^^^ help: try using `ty::<kind>` directly: `ty`
note: the lint level is defined here
  --> /checkout/src/test/ui-fulldeps/internal-lints/ty_tykind_usage.rs:11:8
   |
   |
LL | #[deny(rustc::usage_of_ty_tykind)]


error: usage of `ty::TyKind::<kind>`
   |
   |
LL |         TyKind::Bool => (),                 //~ ERROR usage of `ty::TyKind::<kind>`
   |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`

error: usage of `ty::TyKind::<kind>`
   |
   |
LL |         TyKind::Char => (),                 //~ ERROR usage of `ty::TyKind::<kind>`
   |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`

error: usage of `ty::TyKind::<kind>`
   |
   |
LL |         TyKind::Int(..) => (),              //~ ERROR usage of `ty::TyKind::<kind>`
   |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`

error: usage of `ty::TyKind::<kind>`
   |
   |
LL |         TyKind::Uint(..) => (),             //~ ERROR usage of `ty::TyKind::<kind>`
   |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`

error: usage of `ty::TyKind::<kind>`
   |
   |
LL |         TyKind::Float(..) => (),            //~ ERROR usage of `ty::TyKind::<kind>`
   |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`

error: usage of `ty::TyKind::<kind>`
   |
   |
LL |         TyKind::Adt(..) => (),              //~ ERROR usage of `ty::TyKind::<kind>`
   |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`

error: usage of `ty::TyKind::<kind>`
   |
   |
LL |         TyKind::Foreign(..) => (),          //~ ERROR usage of `ty::TyKind::<kind>`
   |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`

error: usage of `ty::TyKind::<kind>`
   |
   |
LL |         TyKind::Str => (),                  //~ ERROR usage of `ty::TyKind::<kind>`
   |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`

error: usage of `ty::TyKind::<kind>`
   |
   |
LL |         TyKind::Array(..) => (),            //~ ERROR usage of `ty::TyKind::<kind>`
   |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`

error: usage of `ty::TyKind::<kind>`
   |
   |
LL |         TyKind::Slice(..) => (),            //~ ERROR usage of `ty::TyKind::<kind>`
   |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`

error: usage of `ty::TyKind::<kind>`
   |
   |
LL |         TyKind::RawPtr(..) => (),           //~ ERROR usage of `ty::TyKind::<kind>`
   |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`

error: usage of `ty::TyKind::<kind>`
   |
   |
LL |         TyKind::Ref(..) => (),              //~ ERROR usage of `ty::TyKind::<kind>`
   |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`

error: usage of `ty::TyKind::<kind>`
   |
   |
LL |         TyKind::FnDef(..) => (),            //~ ERROR usage of `ty::TyKind::<kind>`
   |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`

error: usage of `ty::TyKind::<kind>`
   |
   |
LL |         TyKind::FnPtr(..) => (),            //~ ERROR usage of `ty::TyKind::<kind>`
   |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`

error: usage of `ty::TyKind::<kind>`
   |
   |
LL |         TyKind::Dynamic(..) => (),          //~ ERROR usage of `ty::TyKind::<kind>`
   |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`

error: usage of `ty::TyKind::<kind>`
   |
   |
LL |         TyKind::Closure(..) => (),          //~ ERROR usage of `ty::TyKind::<kind>`
   |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`

error: usage of `ty::TyKind::<kind>`
   |
   |
LL |         TyKind::Generator(..) => (),        //~ ERROR usage of `ty::TyKind::<kind>`
   |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`

error: usage of `ty::TyKind::<kind>`
   |
   |
LL |         TyKind::GeneratorWitness(..) => (), //~ ERROR usage of `ty::TyKind::<kind>`
   |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`

error: usage of `ty::TyKind::<kind>`
   |
   |
LL |         TyKind::Never => (),                //~ ERROR usage of `ty::TyKind::<kind>`
   |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`

error: usage of `ty::TyKind::<kind>`
   |
   |
LL |         TyKind::Tuple(..) => (),            //~ ERROR usage of `ty::TyKind::<kind>`
   |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`

error: usage of `ty::TyKind::<kind>`
   |
   |
LL |         TyKind::Projection(..) => (),       //~ ERROR usage of `ty::TyKind::<kind>`
   |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`

error: usage of `ty::TyKind::<kind>`
   |
   |
LL |         TyKind::Opaque(..) => (),           //~ ERROR usage of `ty::TyKind::<kind>`
   |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`

error: usage of `ty::TyKind::<kind>`
   |
   |
LL |         TyKind::Param(..) => (),            //~ ERROR usage of `ty::TyKind::<kind>`
   |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`

error: usage of `ty::TyKind::<kind>`
   |
   |
LL |         TyKind::Bound(..) => (),            //~ ERROR usage of `ty::TyKind::<kind>`
   |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`

error: usage of `ty::TyKind::<kind>`
   |
   |
LL |         TyKind::Placeholder(..) => (),      //~ ERROR usage of `ty::TyKind::<kind>`
   |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`

error: usage of `ty::TyKind::<kind>`
   |
   |
LL |         TyKind::Infer(..) => (),            //~ ERROR usage of `ty::TyKind::<kind>`
   |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`

error: usage of `ty::TyKind::<kind>`
   |
   |
LL |         TyKind::Error(_) => (),             //~ ERROR usage of `ty::TyKind::<kind>`
   |         ^^^^^^ help: try using `ty::<kind>` directly: `ty`

error: usage of `ty::TyKind::<kind>`
   |
   |
LL |     if let TyKind::Int(int_ty) = kind {} //~ ERROR usage of `ty::TyKind::<kind>`
   |            ^^^^^^ help: try using `ty::<kind>` directly: `ty`

thread 'rustc' panicked at 'failed to find message in primary or fallback fluent bundles', compiler/rustc_errors/src/translation.rs:64:9
   0:     0x7f157456852e - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha08bd098848a1e25
   1:     0x7f15745d1308 - core::fmt::write::h3d197bd7fec8130a
   1:     0x7f15745d1308 - core::fmt::write::h3d197bd7fec8130a
   2:     0x7f1574559311 - std::io::Write::write_fmt::hf15e2fceb8e3636f
   3:     0x7f157456b4fe - std::panicking::default_hook::{{closure}}::h8207cc32005b7c5e
   4:     0x7f157456b1be - std::panicking::default_hook::h879f28d5d2e17408
   5:     0x7f1574f1b554 - rustc_driver[73c8e760204621f2]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f157456bcb1 - std::panicking::rust_panic_with_hook::h74e91077c26fced7
   7:     0x7f157456bad7 - std::panicking::begin_panic_handler::{{closure}}::h84c034404a7ed57d
   8:     0x7f1574568a64 - std::sys_common::backtrace::__rust_end_short_backtrace::he943d20a8a84c288
   9:     0x7f157456b7a2 - rust_begin_unwind
  10:     0x7f157451af03 - core::panicking::panic_fmt::h03b56255b8de9ef3
Some tests failed in compiletest suite=ui-fulldeps mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
  11:     0x7f15745cdaa1 - core::panicking::panic_display::h6d13b5d501f6f339
  12:     0x7f15745cda4b - core::panicking::panic_str::h6b3012f06e34142d
  13:     0x7f157451ad76 - core::option::expect_failed::h802740aa15e620bc
  14:     0x7f1577c74c43 - <rustc_errors[4639b3fdf06aee6f]::emitter::EmitterWriter as rustc_errors[4639b3fdf06aee6f]::translation::Translate>::translate_message
  15:     0x7f1577c69d84 - <rustc_errors[4639b3fdf06aee6f]::emitter::EmitterWriter>::msg_to_buffer
  16:     0x7f1577c6a74c - <rustc_errors[4639b3fdf06aee6f]::emitter::EmitterWriter>::emit_message_default
  17:     0x7f1577c6805c - <rustc_errors[4639b3fdf06aee6f]::emitter::EmitterWriter as rustc_errors[4639b3fdf06aee6f]::emitter::Emitter>::emit_diagnostic
  18:     0x7f1577c78bda - <rustc_errors[4639b3fdf06aee6f]::json::Diagnostic>::from_errors_diagnostic
  19:     0x7f1577c7770c - <rustc_errors[4639b3fdf06aee6f]::json::JsonEmitter as rustc_errors[4639b3fdf06aee6f]::emitter::Emitter>::emit_diagnostic
  20:     0x7f1577cc49eb - <rustc_errors[4639b3fdf06aee6f]::HandlerInner>::emit_diagnostic
  21:     0x7f1577cc32c1 - <rustc_errors[4639b3fdf06aee6f]::Handler>::emit_diagnostic
  22:     0x7f1577ccbc7d - <() as rustc_errors[4639b3fdf06aee6f]::diagnostic_builder::EmissionGuarantee>::diagnostic_builder_emit_producing_guarantee
  23:     0x7f1577432e2b - rustc_middle[5e283c10211e23e]::lint::struct_lint_level::struct_lint_level_impl::<rustc_error_messages[e64807f889885155]::DiagnosticMessage>
  24:     0x7f157742cbf2 - rustc_middle[5e283c10211e23e]::lint::struct_lint_level::<rustc_error_messages[e64807f889885155]::DiagnosticMessage, <rustc_lint[829080bd00995501]::internal::TyTyKind as rustc_lint[829080bd00995501]::passes::LateLintPass>::check_ty::{closure#0}>
  25:     0x7f15774f8e2a - <rustc_middle[5e283c10211e23e]::ty::context::TyCtxt>::struct_span_lint_hir::<rustc_span[eb07ab32e60c1cc]::span_encoding::Span, rustc_error_messages[e64807f889885155]::DiagnosticMessage, <rustc_lint[829080bd00995501]::internal::TyTyKind as rustc_lint[829080bd00995501]::passes::LateLintPass>::check_ty::{closure#0}>
  26:     0x7f157747d0cf - <rustc_lint[829080bd00995501]::context::LateContext as rustc_lint[829080bd00995501]::context::LintContext>::struct_span_lint::<rustc_span[eb07ab32e60c1cc]::span_encoding::Span, rustc_error_messages[e64807f889885155]::DiagnosticMessage, <rustc_lint[829080bd00995501]::internal::TyTyKind as rustc_lint[829080bd00995501]::passes::LateLintPass>::check_ty::{closure#0}>
  27:     0x7f15774449cd - <rustc_lint[829080bd00995501]::internal::TyTyKind as rustc_lint[829080bd00995501]::passes::LateLintPass>::check_ty
  28:     0x7f157749e803 - <rustc_lint[829080bd00995501]::late::LateLintPassObjects as rustc_lint[829080bd00995501]::passes::LateLintPass>::check_ty
  29:     0x7f157510ae4f - rustc_hir[6f6bcd12f609723c]::intravisit::walk_fn::<rustc_lint[829080bd00995501]::late::LateContextAndPass<rustc_lint[829080bd00995501]::late::LateLintPassObjects>>
  30:     0x7f15750a9bb2 - <rustc_lint[829080bd00995501]::late::LateContextAndPass<rustc_lint[829080bd00995501]::late::LateLintPassObjects> as rustc_hir[6f6bcd12f609723c]::intravisit::Visitor>::visit_fn
  31:     0x7f157510d3ba - rustc_hir[6f6bcd12f609723c]::intravisit::walk_item::<rustc_lint[829080bd00995501]::late::LateContextAndPass<rustc_lint[829080bd00995501]::late::LateLintPassObjects>>
  32:     0x7f15750ac054 - <rustc_lint[829080bd00995501]::late::LateContextAndPass<rustc_lint[829080bd00995501]::late::LateLintPassObjects> as rustc_hir[6f6bcd12f609723c]::intravisit::Visitor>::visit_nested_item
  33:     0x7f15750a8406 - <rustc_lint[829080bd00995501]::late::LateContextAndPass<rustc_lint[829080bd00995501]::late::LateLintPassObjects> as rustc_hir[6f6bcd12f609723c]::intravisit::Visitor>::visit_stmt
  34:     0x7f157510caac - rustc_hir[6f6bcd12f609723c]::intravisit::walk_expr::<rustc_lint[829080bd00995501]::late::LateContextAndPass<rustc_lint[829080bd00995501]::late::LateLintPassObjects>>
  35:     0x7f15750a7f1f - <rustc_lint[829080bd00995501]::late::LateContextAndPass<rustc_lint[829080bd00995501]::late::LateLintPassObjects> as rustc_hir[6f6bcd12f609723c]::intravisit::Visitor>::visit_expr
  36:     0x7f15750a99d1 - <rustc_lint[829080bd00995501]::late::LateContextAndPass<rustc_lint[829080bd00995501]::late::LateLintPassObjects> as rustc_hir[6f6bcd12f609723c]::intravisit::Visitor>::visit_nested_body
  37:     0x7f15750a9bb2 - <rustc_lint[829080bd00995501]::late::LateContextAndPass<rustc_lint[829080bd00995501]::late::LateLintPassObjects> as rustc_hir[6f6bcd12f609723c]::intravisit::Visitor>::visit_fn
  38:     0x7f157510d3ba - rustc_hir[6f6bcd12f609723c]::intravisit::walk_item::<rustc_lint[829080bd00995501]::late::LateContextAndPass<rustc_lint[829080bd00995501]::late::LateLintPassObjects>>
  39:     0x7f15750ac054 - <rustc_lint[829080bd00995501]::late::LateContextAndPass<rustc_lint[829080bd00995501]::late::LateLintPassObjects> as rustc_hir[6f6bcd12f609723c]::intravisit::Visitor>::visit_nested_item
  40:     0x7f157510bdbc - rustc_hir[6f6bcd12f609723c]::intravisit::walk_mod::<rustc_lint[829080bd00995501]::late::LateContextAndPass<rustc_lint[829080bd00995501]::late::LateLintPassObjects>>
  41:     0x7f15750a7a62 - rustc_lint[829080bd00995501]::late::late_lint_pass_crate::<rustc_lint[829080bd00995501]::late::LateLintPassObjects>
  42:     0x7f15750a6e6f - rustc_lint[829080bd00995501]::late::late_lint_crate::<rustc_lint[829080bd00995501]::BuiltinCombinedLateLintPass>
  43:     0x7f1575052698 - <rustc_session[9c2fcbfb9c0e7a50]::session::Session>::time::<(), rustc_lint[829080bd00995501]::late::check_crate<rustc_lint[829080bd00995501]::BuiltinCombinedLateLintPass, rustc_interface[93851a8ad3afe8d4]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#0}::{closure#0}>
  44:     0x7f157502b6b2 - rustc_data_structures[88cf5a3801a79fbc]::sync::join::<rustc_lint[829080bd00995501]::late::check_crate<rustc_lint[829080bd00995501]::BuiltinCombinedLateLintPass, rustc_interface[93851a8ad3afe8d4]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#0}, rustc_lint[829080bd00995501]::late::check_crate<rustc_lint[829080bd00995501]::BuiltinCombinedLateLintPass, rustc_interface[93851a8ad3afe8d4]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}, (), ()>
  45:     0x7f1575052a20 - <rustc_session[9c2fcbfb9c0e7a50]::session::Session>::time::<(), rustc_interface[93851a8ad3afe8d4]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}>
  46:     0x7f157502c708 - std[cbb8deaedd3917d0]::panic::catch_unwind::<core[8207c3258571ca0e]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[93851a8ad3afe8d4]::passes::analysis::{closure#5}::{closure#1}::{closure#2}>, ()>
  47:     0x7f15750d0ba5 - <core[8207c3258571ca0e]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[93851a8ad3afe8d4]::passes::analysis::{closure#5}::{closure#1}> as core[8207c3258571ca0e]::ops::function::FnOnce<()>>::call_once
  48:     0x7f157502c829 - std[cbb8deaedd3917d0]::panic::catch_unwind::<core[8207c3258571ca0e]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[93851a8ad3afe8d4]::passes::analysis::{closure#5}::{closure#1}>, ()>
  49:     0x7f1575054976 - <rustc_session[9c2fcbfb9c0e7a50]::session::Session>::time::<(), rustc_interface[93851a8ad3afe8d4]::passes::analysis::{closure#5}>
  50:     0x7f15750759ec - rustc_interface[93851a8ad3afe8d4]::passes::analysis
  51:     0x7f1576b37dc8 - rustc_query_system[32dc960d8a53c9e3]::query::plumbing::try_execute_query::<rustc_query_impl[37ed453da7d29069]::plumbing::QueryCtxt, rustc_query_system[32dc960d8a53c9e3]::query::caches::DefaultCache<(), core[8207c3258571ca0e]::result::Result<(), rustc_errors[4639b3fdf06aee6f]::ErrorGuaranteed>>>
  52:     0x7f1576c188bb - rustc_query_system[32dc960d8a53c9e3]::query::plumbing::get_query::<rustc_query_impl[37ed453da7d29069]::queries::analysis, rustc_query_impl[37ed453da7d29069]::plumbing::QueryCtxt>
  53:     0x7f1576a16a6a - <rustc_query_impl[37ed453da7d29069]::Queries as rustc_middle[5e283c10211e23e]::ty::query::QueryEngine>::analysis
  54:     0x7f1574f7c446 - <rustc_interface[93851a8ad3afe8d4]::passes::QueryContext>::enter::<rustc_driver[73c8e760204621f2]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[8207c3258571ca0e]::result::Result<(), rustc_errors[4639b3fdf06aee6f]::ErrorGuaranteed>>
  55:     0x7f1574f31e3c - rustc_interface[93851a8ad3afe8d4]::interface::create_compiler_and_run::<core[8207c3258571ca0e]::result::Result<(), rustc_errors[4639b3fdf06aee6f]::ErrorGuaranteed>, rustc_driver[73c8e760204621f2]::run_compiler::{closure#1}>
  56:     0x7f1574f960bf - <scoped_tls[dcda34d3adf18490]::ScopedKey<rustc_span[eb07ab32e60c1cc]::SessionGlobals>>::set::<rustc_interface[93851a8ad3afe8d4]::interface::run_compiler<core[8207c3258571ca0e]::result::Result<(), rustc_errors[4639b3fdf06aee6f]::ErrorGuaranteed>, rustc_driver[73c8e760204621f2]::run_compiler::{closure#1}>::{closure#0}, core[8207c3258571ca0e]::result::Result<(), rustc_errors[4639b3fdf06aee6f]::ErrorGuaranteed>>
  57:     0x7f1574f8008f - std[cbb8deaedd3917d0]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[93851a8ad3afe8d4]::util::run_in_thread_pool_with_globals<rustc_interface[93851a8ad3afe8d4]::interface::run_compiler<core[8207c3258571ca0e]::result::Result<(), rustc_errors[4639b3fdf06aee6f]::ErrorGuaranteed>, rustc_driver[73c8e760204621f2]::run_compiler::{closure#1}>::{closure#0}, core[8207c3258571ca0e]::result::Result<(), rustc_errors[4639b3fdf06aee6f]::ErrorGuaranteed>>::{closure#0}, core[8207c3258571ca0e]::result::Result<(), rustc_errors[4639b3fdf06aee6f]::ErrorGuaranteed>>
  58:     0x7f1574f333b1 - std[cbb8deaedd3917d0]::panic::catch_unwind::<core[8207c3258571ca0e]::panic::unwind_safe::AssertUnwindSafe<<std[cbb8deaedd3917d0]::thread::Builder>::spawn_unchecked_<rustc_interface[93851a8ad3afe8d4]::util::run_in_thread_pool_with_globals<rustc_interface[93851a8ad3afe8d4]::interface::run_compiler<core[8207c3258571ca0e]::result::Result<(), rustc_errors[4639b3fdf06aee6f]::ErrorGuaranteed>, rustc_driver[73c8e760204621f2]::run_compiler::{closure#1}>::{closure#0}, core[8207c3258571ca0e]::result::Result<(), rustc_errors[4639b3fdf06aee6f]::ErrorGuaranteed>>::{closure#0}, core[8207c3258571ca0e]::result::Result<(), rustc_errors[4639b3fdf06aee6f]::ErrorGuaranteed>>::{closure#1}::{closure#0}>, core[8207c3258571ca0e]::result::Result<(), rustc_errors[4639b3fdf06aee6f]::ErrorGuaranteed>>
  59:     0x7f1574f83c60 - <<std[cbb8deaedd3917d0]::thread::Builder>::spawn_unchecked_<rustc_interface[93851a8ad3afe8d4]::util::run_in_thread_pool_with_globals<rustc_interface[93851a8ad3afe8d4]::interface::run_compiler<core[8207c3258571ca0e]::result::Result<(), rustc_errors[4639b3fdf06aee6f]::ErrorGuaranteed>, rustc_driver[73c8e760204621f2]::run_compiler::{closure#1}>::{closure#0}, core[8207c3258571ca0e]::result::Result<(), rustc_errors[4639b3fdf06aee6f]::ErrorGuaranteed>>::{closure#0}, core[8207c3258571ca0e]::result::Result<(), rustc_errors[4639b3fdf06aee6f]::ErrorGuaranteed>>::{closure#1} as core[8207c3258571ca0e]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  60:     0x7f1574578aa5 - std::sys::unix::thread::Thread::new::thread_start::ha7e48239067192e4
  61:     0x7f1574313b43 - <unknown>
  62:     0x7f15743a5a00 - <unknown>
  63:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (6d203eb71 2022-09-18) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0 -Z unstable-options
query stack during panic:
query stack during panic:
#0 [analysis] running analysis passes on this crate
error: aborting due to 29 previous errors
------------------------------------------



---- [ui] src/test/ui-fulldeps/issue-15778-fail.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-fail/auxiliary" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/issue-15778-fail/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs:7:1
   |
LL | #[macro_use]
   | ^^^^^^^^^^^^
   | ^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

error[E0282]: type annotations needed
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs:32:38
   |
LL |             cx.lint(CRATE_NOT_OKAY, |lint| {
   |                                      ^^^^
LL |                 lint.build("crate is not marked with #![crate_okay]").set_span(span).emit();
   |                 ---- type must be known at this point
   |
help: consider giving this closure parameter an explicit type
   |
LL |             cx.lint(CRATE_NOT_OKAY, |lint: _| {

error[E0061]: this function takes 3 arguments but 2 arguments were supplied
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-for-crate.rs:32:16
   |
   |
LL |               cx.lint(CRATE_NOT_OKAY, |lint| {
   |  ________________^^^^-
LL | |                 lint.build("crate is not marked with #![crate_okay]").set_span(span).emit();
LL | |             });
   | |______________- an argument is missing
note: associated function defined here
  --> /checkout/compiler/rustc_lint/src/context.rs:917:8
   |
LL |     fn lint(
LL |     fn lint(
   |        ^^^^
help: provide the argument
   |
LL ~             cx.lint(CRATE_NOT_OKAY, |lint| {
LL +                 lint.build("crate is not marked with #![crate_okay]").set_span(span).emit();
LL ~             }, /* value */);

error: aborting due to 2 previous errors; 1 warning emitted

Some errors have detailed explanations: E0061, E0282.
Some errors have detailed explanations: E0061, E0282.
For more information about an error, try `rustc --explain E0061`.
------------------------------------------


---- [ui] src/test/ui-fulldeps/lint-group-denied-lint-allowed.rs stdout ----

error: auxiliary build of "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" failed to compile: 
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-denied-lint-allowed/auxiliary" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--crate-type" "dylib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui-fulldeps/lint-group-denied-lint-allowed/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui-fulldeps/auxiliary/lint-group-plugin-test.rs:8:1
   |
LL | #[macro_use]
   | ^^^^^^^^^^^^
   | ^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: unused imports: `LintArray`, `LintPass`
   |
   |
LL | use rustc_lint::{LateContext, LateLintPass, LintArray, LintContext, LintId, LintPass};

