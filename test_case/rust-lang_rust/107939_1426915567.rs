plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:c8a09107e1a7966f8c20565a263305ce8f62405f)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
........................................................................................ 2552/14444
........................................................................................ 2640/14444
........................................................................................ 2728/14444
........................................................................................ 2816/14444
...........................................F......................F..................... 2904/14444
........................................................................................ 3080/14444
........................................................................................ 3168/14444
........................................................................................ 3256/14444
...............................i.......................................................i 3344/14444
---
failures:

---- [ui] tests/ui/consts/const-eval/raw-bytes.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/const-eval/raw-bytes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/raw-bytes" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/raw-bytes/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:20:1
   |
LL | const BAD_ENUM: Enum = unsafe { mem::transmute(1usize) };
   | ^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-tag>: encountered 0x0000000000000001, but expected a valid enum tag
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
           }

error[E0080]: it is undefined behavior to use this value
error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:28:1
   |
LL | const BAD_ENUM2: Enum2 = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-tag>: encountered 0x0000000000000000, but expected a valid enum tag
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:42:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:42:1
   |
LL | const BAD_UNINHABITED_VARIANT1: UninhDiscriminant = unsafe { mem::transmute(1u8) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-variant(B)>.0: encountered a value of the never type `!`
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:44:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:44:1
   |
LL | const BAD_UNINHABITED_VARIANT2: UninhDiscriminant = unsafe { mem::transmute(3u8) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-variant(D)>.0: encountered a value of uninhabited type Never
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:50:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:50:1
   |
LL | const BAD_OPTION_CHAR: Option<(char, char)> = Some(('x', unsafe { mem::transmute(!0u32) }));
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value at .<enum-variant(Some)>.0.1: encountered 0xffffffff, but expected a valid unicode scalar value (in `0..=0x10FFFF` but not in `0xD800..=0xDFFF`)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 4) {
               78 00 00 00 ff ff ff ff                         │ x.......

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:54:1
   |
   |
LL | const NULL_PTR: NonNull<u8> = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered 0, but expected something greater or equal to 1
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:57:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:57:1
   |
LL | const NULL_U8: NonZeroU8 = unsafe { mem::transmute(0u8) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered 0, but expected something greater or equal to 1
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 1, align: 1) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:59:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:59:1
   |
LL | const NULL_USIZE: NonZeroUsize = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered 0, but expected something greater or equal to 1
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:65:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:65:1
   |
LL | const BAD_RANGE1: RestrictedRange1 = unsafe { RestrictedRange1(42) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered 42, but expected something in the range 10..=30
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
               2a 00 00 00                                     │ *...

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:71:1
   |
   |
LL | const BAD_RANGE2: RestrictedRange2 = unsafe { RestrictedRange2(20) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered 20, but expected something less or equal to 10, or greater or equal to 30
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:74:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:74:1
   |
LL | const NULL_FAT_PTR: NonNull<dyn Send> = unsafe {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered 0, but expected something greater or equal to 1
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               00 00 00 00 00 00 00 00 ╾───────alloc28───────╼ │ ........╾──────╼

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:82:1
   |
   |
LL | const UNALIGNED: &u16 = unsafe { mem::transmute(&[0u8; 4]) };
   | ^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered an unaligned reference (required 2 byte alignment but found 1)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:86:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:86:1
   |
LL | const UNALIGNED_BOX: Box<u16> = unsafe { mem::transmute(&[0u8; 4]) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered an unaligned box (required 2 byte alignment but found 1)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:90:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:90:1
   |
LL | const NULL: &u16 = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^ constructing invalid value: encountered a null reference
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:93:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:93:1
   |
LL | const NULL_BOX: Box<u16> = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a null box
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:96:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:96:1
   |
LL | const USIZE_AS_REF: &'static u8 = unsafe { mem::transmute(1337usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a dangling reference (address 0x539 is unallocated)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:99:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:99:1
   |
LL | const USIZE_AS_BOX: Box<u8> = unsafe { mem::transmute(1337usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a dangling box (address 0x539 is unallocated)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:102:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:102:1
   |
LL | const NULL_FN_PTR: fn() = unsafe { mem::transmute(0usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered null pointer, but expected a function pointer
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:104:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:104:1
   |
LL | const DANGLING_FN_PTR: fn() = unsafe { mem::transmute(13usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered 0xd[noalloc], but expected a function pointer
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
               0d 00 00 00 00 00 00 00                         │ ........

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:106:1
   |
   |
LL | const DATA_FN_PTR: fn() = unsafe { mem::transmute(&13) };
   | ^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered alloc55, but expected a function pointer
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:112:1
  --> fake-test-src-base/consts/const-eval/raw-bytes.rs:112:1
   |
LL | const BAD_BAD_REF: &Bar = unsafe { mem::transmute(1usize) };
   | ^^^^^^^^^^^^^^^^^^^^^^^ constructing invalid value: encountered a reference pointing to uninhabited type Bar
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }


error: internal compiler error: compiler/rustc_const_eval/src/interpret/validity.rs:924:17: Unexpected Undefined Behavior error during validation: memory access failed: alloc61 has size 1, so pointer to 999 bytes starting at offset 0 is out-of-bounds
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1644:9
stack backtrace:
   0:     0x7f5799b07f55 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hee4ced7286c978cb
   1:     0x7f5799b759e8 - core::fmt::write::h326231867062362c
   1:     0x7f5799b759e8 - core::fmt::write::h326231867062362c
   2:     0x7f5799af9ff1 - std::io::Write::write_fmt::h7265d5d1376b0963
   3:     0x7f5799b07d61 - std::sys_common::backtrace::print::h3b6669b79c9fba53
   4:     0x7f5799b0afc4 - std::panicking::default_hook::{{closure}}::hbbf2f88c1a5e6958
   5:     0x7f5799b0acaa - std::panicking::default_hook::h7891ec361a6eee32
   6:     0x7f579a5b5202 - rustc_driver_impl[4e6f8daf397a3442]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f5799b0b701 - std::panicking::rust_panic_with_hook::h6ee02897f5d1b46f
   8:     0x7f579d4d11b3 - std[c784e3df5bbe93e2]::panicking::begin_panic::<rustc_errors[91cc727be444362d]::ExplicitBug>::{closure#0}
   9:     0x7f579d4cc946 - std[c784e3df5bbe93e2]::sys_common::backtrace::__rust_end_short_backtrace::<std[c784e3df5bbe93e2]::panicking::begin_panic<rustc_errors[91cc727be444362d]::ExplicitBug>::{closure#0}, !>
  10:     0x7f579a55ec36 - std[c784e3df5bbe93e2]::panicking::begin_panic::<rustc_errors[91cc727be444362d]::ExplicitBug>
  11:     0x7f579d4961d6 - std[c784e3df5bbe93e2]::panic::panic_any::<rustc_errors[91cc727be444362d]::ExplicitBug>
  12:     0x7f579d4930a0 - <rustc_errors[91cc727be444362d]::HandlerInner>::bug::<&alloc[fcc2b1410145e313]::string::String>
  13:     0x7f579d492cb0 - <rustc_errors[91cc727be444362d]::Handler>::bug::<&alloc[fcc2b1410145e313]::string::String>
  14:     0x7f579d565ae5 - rustc_middle[8dad0eb348f48ec7]::util::bug::opt_span_bug_fmt::<rustc_span[7ffeea359ae9f249]::span_encoding::Span>::{closure#0}
  15:     0x7f579d5642cc - rustc_middle[8dad0eb348f48ec7]::ty::context::tls::with_opt::<rustc_middle[8dad0eb348f48ec7]::util::bug::opt_span_bug_fmt<rustc_span[7ffeea359ae9f249]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  16:     0x7f579d564276 - rustc_middle[8dad0eb348f48ec7]::ty::context::tls::with_context_opt::<rustc_middle[8dad0eb348f48ec7]::ty::context::tls::with_opt<rustc_middle[8dad0eb348f48ec7]::util::bug::opt_span_bug_fmt<rustc_span[7ffeea359ae9f249]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  17:     0x7f579d565a29 - rustc_middle[8dad0eb348f48ec7]::util::bug::opt_span_bug_fmt::<rustc_span[7ffeea359ae9f249]::span_encoding::Span>
  18:     0x7f579a569295 - rustc_middle[8dad0eb348f48ec7]::util::bug::bug_fmt
  19:     0x7f579be35637 - <rustc_const_eval[b78b8074de55c24b]::interpret::eval_context::InterpCx<rustc_const_eval[b78b8074de55c24b]::const_eval::machine::CompileTimeInterpreter>>::validate_operand_internal
  20:     0x7f579be6b16c - rustc_const_eval[b78b8074de55c24b]::const_eval::eval_queries::eval_to_allocation_raw_provider
  21:     0x7f579c49fe9f - rustc_query_system[9c97b1ee9c99d518]::query::plumbing::get_query::<rustc_query_impl[88efce5047ede0c6]::queries::eval_to_allocation_raw, rustc_query_impl[88efce5047ede0c6]::plumbing::QueryCtxt, rustc_middle[8dad0eb348f48ec7]::dep_graph::dep_node::DepKind>
  22:     0x7f579c0f929c - <rustc_query_impl[88efce5047ede0c6]::Queries as rustc_middle[8dad0eb348f48ec7]::ty::query::QueryEngine>::eval_to_allocation_raw
  23:     0x7f579be6830c - rustc_const_eval[b78b8074de55c24b]::const_eval::eval_queries::eval_to_const_value_raw_provider
  24:     0x7f579c4a5009 - rustc_query_system[9c97b1ee9c99d518]::query::plumbing::get_query::<rustc_query_impl[88efce5047ede0c6]::queries::eval_to_const_value_raw, rustc_query_impl[88efce5047ede0c6]::plumbing::QueryCtxt, rustc_middle[8dad0eb348f48ec7]::dep_graph::dep_node::DepKind>
  25:     0x7f579c0f9a5c - <rustc_query_impl[88efce5047ede0c6]::Queries as rustc_middle[8dad0eb348f48ec7]::ty::query::QueryEngine>::eval_to_const_value_raw
  26:     0x7f579be68044 - rustc_const_eval[b78b8074de55c24b]::const_eval::eval_queries::eval_to_const_value_raw_provider
  27:     0x7f579c4a5009 - rustc_query_system[9c97b1ee9c99d518]::query::plumbing::get_query::<rustc_query_impl[88efce5047ede0c6]::queries::eval_to_const_value_raw, rustc_query_impl[88efce5047ede0c6]::plumbing::QueryCtxt, rustc_middle[8dad0eb348f48ec7]::dep_graph::dep_node::DepKind>
  28:     0x7f579c0f9a5c - <rustc_query_impl[88efce5047ede0c6]::Queries as rustc_middle[8dad0eb348f48ec7]::ty::query::QueryEngine>::eval_to_const_value_raw
  29:     0x7f579d5519ef - <rustc_middle[8dad0eb348f48ec7]::ty::query::TyCtxtEnsure>::const_eval_poly
  30:     0x7f579ce95e7f - <rustc_lint[780114dc5e4bcf7b]::BuiltinCombinedModuleLateLintPass as rustc_lint[780114dc5e4bcf7b]::passes::LateLintPass>::check_item
  31:     0x7f579ce46c53 - <rustc_lint[780114dc5e4bcf7b]::late::LateContextAndPass<rustc_lint[780114dc5e4bcf7b]::BuiltinCombinedModuleLateLintPass> as rustc_hir[abf5e5c0653d55df]::intravisit::Visitor>::visit_nested_item
  32:     0x7f579cec031c - rustc_hir[abf5e5c0653d55df]::intravisit::walk_mod::<rustc_lint[780114dc5e4bcf7b]::late::LateContextAndPass<rustc_lint[780114dc5e4bcf7b]::BuiltinCombinedModuleLateLintPass>>
  33:     0x7f579ce4c660 - rustc_lint[780114dc5e4bcf7b]::late::late_lint_mod::<rustc_lint[780114dc5e4bcf7b]::BuiltinCombinedModuleLateLintPass>
  34:     0x7f579ce8f8ed - rustc_lint[780114dc5e4bcf7b]::lint_mod
  35:     0x7f579c411b1d - rustc_query_system[9c97b1ee9c99d518]::query::plumbing::try_execute_query::<rustc_query_impl[88efce5047ede0c6]::queries::lint_mod, rustc_query_impl[88efce5047ede0c6]::plumbing::QueryCtxt>
  36:     0x7f579c4c2da0 - rustc_query_system[9c97b1ee9c99d518]::query::plumbing::get_query::<rustc_query_impl[88efce5047ede0c6]::queries::lint_mod, rustc_query_impl[88efce5047ede0c6]::plumbing::QueryCtxt, rustc_middle[8dad0eb348f48ec7]::dep_graph::dep_node::DepKind>
  37:     0x7f579c0ebbd0 - <rustc_query_impl[88efce5047ede0c6]::Queries as rustc_middle[8dad0eb348f48ec7]::ty::query::QueryEngine>::lint_mod
  38:     0x7f579a7c35f0 - <core[1afb7b33f9e88180]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f0947719a8019974]::sync::par_for_each_in<&[rustc_hir[abf5e5c0653d55df]::hir_id::OwnerId], <rustc_middle[8dad0eb348f48ec7]::hir::map::Map>::par_for_each_module<rustc_lint[780114dc5e4bcf7b]::late::check_crate<rustc_lint[780114dc5e4bcf7b]::BuiltinCombinedLateLintPass, rustc_interface[2575fbd8bca5080e]::passes::analysis::{closure#6}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[1afb7b33f9e88180]::ops::function::FnOnce<()>>::call_once
  39:     0x7f579a73e006 - std[c784e3df5bbe93e2]::panicking::try::<(), core[1afb7b33f9e88180]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f0947719a8019974]::sync::par_for_each_in<&[rustc_hir[abf5e5c0653d55df]::hir_id::OwnerId], <rustc_middle[8dad0eb348f48ec7]::hir::map::Map>::par_for_each_module<rustc_lint[780114dc5e4bcf7b]::late::check_crate<rustc_lint[780114dc5e4bcf7b]::BuiltinCombinedLateLintPass, rustc_interface[2575fbd8bca5080e]::passes::analysis::{closure#6}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  40:     0x7f579a6f0bfd - rustc_data_structures[f0947719a8019974]::sync::par_for_each_in::<&[rustc_hir[abf5e5c0653d55df]::hir_id::OwnerId], <rustc_middle[8dad0eb348f48ec7]::hir::map::Map>::par_for_each_module<rustc_lint[780114dc5e4bcf7b]::late::check_crate<rustc_lint[780114dc5e4bcf7b]::BuiltinCombinedLateLintPass, rustc_interface[2575fbd8bca5080e]::passes::analysis::{closure#6}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}::{closure#0}>::{closure#0}>
  41:     0x7f579a72438d - <rustc_session[3c45338a8d9e3773]::session::Session>::time::<(), rustc_lint[780114dc5e4bcf7b]::late::check_crate<rustc_lint[780114dc5e4bcf7b]::BuiltinCombinedLateLintPass, rustc_interface[2575fbd8bca5080e]::passes::analysis::{closure#6}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}>
  42:     0x7f579a7244b0 - <rustc_session[3c45338a8d9e3773]::session::Session>::time::<(), rustc_interface[2575fbd8bca5080e]::passes::analysis::{closure#6}::{closure#1}::{closure#2}::{closure#0}>
  43:     0x7f579a73e0b5 - std[c784e3df5bbe93e2]::panicking::try::<(), core[1afb7b33f9e88180]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[2575fbd8bca5080e]::passes::analysis::{closure#6}::{closure#1}::{closure#2}>>
  44:     0x7f579a7c44a8 - <core[1afb7b33f9e88180]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[2575fbd8bca5080e]::passes::analysis::{closure#6}::{closure#1}> as core[1afb7b33f9e88180]::ops::function::FnOnce<()>>::call_once
  45:     0x7f579a725c14 - <rustc_session[3c45338a8d9e3773]::session::Session>::time::<(), rustc_interface[2575fbd8bca5080e]::passes::analysis::{closure#6}>
  46:     0x7f579a703a6c - rustc_interface[2575fbd8bca5080e]::passes::analysis
  47:     0x7f579c40ef8c - rustc_query_system[9c97b1ee9c99d518]::query::plumbing::try_execute_query::<rustc_query_impl[88efce5047ede0c6]::queries::analysis, rustc_query_impl[88efce5047ede0c6]::plumbing::QueryCtxt>
  48:     0x7f579c4c29f1 - rustc_query_system[9c97b1ee9c99d518]::query::plumbing::get_query::<rustc_query_impl[88efce5047ede0c6]::queries::analysis, rustc_query_impl[88efce5047ede0c6]::plumbing::QueryCtxt, rustc_middle[8dad0eb348f48ec7]::dep_graph::dep_node::DepKind>
  49:     0x7f579c0c924a - <rustc_query_impl[88efce5047ede0c6]::Queries as rustc_middle[8dad0eb348f48ec7]::ty::query::QueryEngine>::analysis
  50:     0x7f579a6420a2 - <rustc_middle[8dad0eb348f48ec7]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[4e6f8daf397a3442]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[1afb7b33f9e88180]::result::Result<(), rustc_errors[91cc727be444362d]::ErrorGuaranteed>>
  51:     0x7f579a623f25 - <rustc_interface[2575fbd8bca5080e]::interface::Compiler>::enter::<rustc_driver_impl[4e6f8daf397a3442]::run_compiler::{closure#1}::{closure#2}, core[1afb7b33f9e88180]::result::Result<core[1afb7b33f9e88180]::option::Option<rustc_interface[2575fbd8bca5080e]::queries::Linker>, rustc_errors[91cc727be444362d]::ErrorGuaranteed>>
  52:     0x7f579a5b6377 - rustc_span[7ffeea359ae9f249]::with_source_map::<core[1afb7b33f9e88180]::result::Result<(), rustc_errors[91cc727be444362d]::ErrorGuaranteed>, rustc_interface[2575fbd8bca5080e]::interface::run_compiler<core[1afb7b33f9e88180]::result::Result<(), rustc_errors[91cc727be444362d]::ErrorGuaranteed>, rustc_driver_impl[4e6f8daf397a3442]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  53:     0x7f579a624ba6 - <scoped_tls[34fa82c1b3c8f743]::ScopedKey<rustc_span[7ffeea359ae9f249]::SessionGlobals>>::set::<rustc_interface[2575fbd8bca5080e]::interface::run_compiler<core[1afb7b33f9e88180]::result::Result<(), rustc_errors[91cc727be444362d]::ErrorGuaranteed>, rustc_driver_impl[4e6f8daf397a3442]::run_compiler::{closure#1}>::{closure#0}, core[1afb7b33f9e88180]::result::Result<(), rustc_errors[91cc727be444362d]::ErrorGuaranteed>>
  54:     0x7f579a5c8a59 - std[c784e3df5bbe93e2]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[2575fbd8bca5080e]::util::run_in_thread_pool_with_globals<rustc_interface[2575fbd8bca5080e]::interface::run_compiler<core[1afb7b33f9e88180]::result::Result<(), rustc_errors[91cc727be444362d]::ErrorGuaranteed>, rustc_driver_impl[4e6f8daf397a3442]::run_compiler::{closure#1}>::{closure#0}, core[1afb7b33f9e88180]::result::Result<(), rustc_errors[91cc727be444362d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1afb7b33f9e88180]::result::Result<(), rustc_errors[91cc727be444362d]::ErrorGuaranteed>>
  55:     0x7f579a61ad48 - std[c784e3df5bbe93e2]::panicking::try::<core[1afb7b33f9e88180]::result::Result<(), rustc_errors[91cc727be444362d]::ErrorGuaranteed>, core[1afb7b33f9e88180]::panic::unwind_safe::AssertUnwindSafe<<std[c784e3df5bbe93e2]::thread::Builder>::spawn_unchecked_<rustc_interface[2575fbd8bca5080e]::util::run_in_thread_pool_with_globals<rustc_interface[2575fbd8bca5080e]::interface::run_compiler<core[1afb7b33f9e88180]::result::Result<(), rustc_errors[91cc727be444362d]::ErrorGuaranteed>, rustc_driver_impl[4e6f8daf397a3442]::run_compiler::{closure#1}>::{closure#0}, core[1afb7b33f9e88180]::result::Result<(), rustc_errors[91cc727be444362d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1afb7b33f9e88180]::result::Result<(), rustc_errors[91cc727be444362d]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  56:     0x7f579a5c2e37 - <<std[c784e3df5bbe93e2]::thread::Builder>::spawn_unchecked_<rustc_interface[2575fbd8bca5080e]::util::run_in_thread_pool_with_globals<rustc_interface[2575fbd8bca5080e]::interface::run_compiler<core[1afb7b33f9e88180]::result::Result<(), rustc_errors[91cc727be444362d]::ErrorGuaranteed>, rustc_driver_impl[4e6f8daf397a3442]::run_compiler::{closure#1}>::{closure#0}, core[1afb7b33f9e88180]::result::Result<(), rustc_errors[91cc727be444362d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1afb7b33f9e88180]::result::Result<(), rustc_errors[91cc727be444362d]::ErrorGuaranteed>>::{closure#1} as core[1afb7b33f9e88180]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  57:     0x7f5799b1851e - std::sys::unix::thread::Thread::new::thread_start::h88c05699f861c7c1
  58:     0x7f57998adb43 - <unknown>
  59:     0x7f579993fa00 - <unknown>
  60:                0x0 - <unknown>
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.69.0-nightly (7ee75ca7f 2023-02-11) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [eval_to_allocation_raw] const-evaluating + checking `STR_TOO_LONG`
#1 [eval_to_const_value_raw] simplifying constant for the type system `STR_TOO_LONG`
#2 [eval_to_const_value_raw] simplifying constant for the type system `STR_TOO_LONG`
#3 [lint_mod] linting top-level module
#4 [analysis] running analysis passes on this crate
error: aborting due to 22 previous errors

For more information about this error, try `rustc --explain E0080`.
------------------------------------------
------------------------------------------


---- [ui] tests/ui/consts/const-eval/ub-wide-ptr.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/const-eval/ub-wide-ptr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-wide-ptr" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-wide-ptr/auxiliary"
stdout: none
--- stderr -------------------------------
error: internal compiler error: compiler/rustc_const_eval/src/interpret/validity.rs:924:17: Unexpected Undefined Behavior error during validation: memory access failed: alloc8 has size 1, so pointer to 999 bytes starting at offset 0 is out-of-bounds
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1644:9
stack backtrace:
   0:     0x7f3a09be3f55 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hee4ced7286c978cb
   1:     0x7f3a09c519e8 - core::fmt::write::h326231867062362c
   1:     0x7f3a09c519e8 - core::fmt::write::h326231867062362c
   2:     0x7f3a09bd5ff1 - std::io::Write::write_fmt::h7265d5d1376b0963
   3:     0x7f3a09be3d61 - std::sys_common::backtrace::print::h3b6669b79c9fba53
   4:     0x7f3a09be6fc4 - std::panicking::default_hook::{{closure}}::hbbf2f88c1a5e6958
   5:     0x7f3a09be6caa - std::panicking::default_hook::h7891ec361a6eee32
   6:     0x7f3a0a691202 - rustc_driver_impl[4e6f8daf397a3442]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f3a09be7701 - std::panicking::rust_panic_with_hook::h6ee02897f5d1b46f
   8:     0x7f3a0d5ad1b3 - std[c784e3df5bbe93e2]::panicking::begin_panic::<rustc_errors[91cc727be444362d]::ExplicitBug>::{closure#0}
   9:     0x7f3a0d5a8946 - std[c784e3df5bbe93e2]::sys_common::backtrace::__rust_end_short_backtrace::<std[c784e3df5bbe93e2]::panicking::begin_panic<rustc_errors[91cc727be444362d]::ExplicitBug>::{closure#0}, !>
  10:     0x7f3a0a63ac36 - std[c784e3df5bbe93e2]::panicking::begin_panic::<rustc_errors[91cc727be444362d]::ExplicitBug>
  11:     0x7f3a0d5721d6 - std[c784e3df5bbe93e2]::panic::panic_any::<rustc_errors[91cc727be444362d]::ExplicitBug>
  12:     0x7f3a0d56f0a0 - <rustc_errors[91cc727be444362d]::HandlerInner>::bug::<&alloc[fcc2b1410145e313]::string::String>
  13:     0x7f3a0d56ecb0 - <rustc_errors[91cc727be444362d]::Handler>::bug::<&alloc[fcc2b1410145e313]::string::String>
  14:     0x7f3a0d641ae5 - rustc_middle[8dad0eb348f48ec7]::util::bug::opt_span_bug_fmt::<rustc_span[7ffeea359ae9f249]::span_encoding::Span>::{closure#0}
  15:     0x7f3a0d6402cc - rustc_middle[8dad0eb348f48ec7]::ty::context::tls::with_opt::<rustc_middle[8dad0eb348f48ec7]::util::bug::opt_span_bug_fmt<rustc_span[7ffeea359ae9f249]::span_encoding::Span>::{closure#0}, !>::{closure#0}
  16:     0x7f3a0d640276 - rustc_middle[8dad0eb348f48ec7]::ty::context::tls::with_context_opt::<rustc_middle[8dad0eb348f48ec7]::ty::context::tls::with_opt<rustc_middle[8dad0eb348f48ec7]::util::bug::opt_span_bug_fmt<rustc_span[7ffeea359ae9f249]::span_encoding::Span>::{closure#0}, !>::{closure#0}, !>
  17:     0x7f3a0d641a29 - rustc_middle[8dad0eb348f48ec7]::util::bug::opt_span_bug_fmt::<rustc_span[7ffeea359ae9f249]::span_encoding::Span>
  18:     0x7f3a0a645295 - rustc_middle[8dad0eb348f48ec7]::util::bug::bug_fmt
  19:     0x7f3a0bf11637 - <rustc_const_eval[b78b8074de55c24b]::interpret::eval_context::InterpCx<rustc_const_eval[b78b8074de55c24b]::const_eval::machine::CompileTimeInterpreter>>::validate_operand_internal
  20:     0x7f3a0bf4716c - rustc_const_eval[b78b8074de55c24b]::const_eval::eval_queries::eval_to_allocation_raw_provider
  21:     0x7f3a0c57be9f - rustc_query_system[9c97b1ee9c99d518]::query::plumbing::get_query::<rustc_query_impl[88efce5047ede0c6]::queries::eval_to_allocation_raw, rustc_query_impl[88efce5047ede0c6]::plumbing::QueryCtxt, rustc_middle[8dad0eb348f48ec7]::dep_graph::dep_node::DepKind>
  22:     0x7f3a0c1d529c - <rustc_query_impl[88efce5047ede0c6]::Queries as rustc_middle[8dad0eb348f48ec7]::ty::query::QueryEngine>::eval_to_allocation_raw
  23:     0x7f3a0bf4430c - rustc_const_eval[b78b8074de55c24b]::const_eval::eval_queries::eval_to_const_value_raw_provider
  24:     0x7f3a0c581009 - rustc_query_system[9c97b1ee9c99d518]::query::plumbing::get_query::<rustc_query_impl[88efce5047ede0c6]::queries::eval_to_const_value_raw, rustc_query_impl[88efce5047ede0c6]::plumbing::QueryCtxt, rustc_middle[8dad0eb348f48ec7]::dep_graph::dep_node::DepKind>
  25:     0x7f3a0c1d5a5c - <rustc_query_impl[88efce5047ede0c6]::Queries as rustc_middle[8dad0eb348f48ec7]::ty::query::QueryEngine>::eval_to_const_value_raw
  26:     0x7f3a0bf44044 - rustc_const_eval[b78b8074de55c24b]::const_eval::eval_queries::eval_to_const_value_raw_provider
  27:     0x7f3a0c581009 - rustc_query_system[9c97b1ee9c99d518]::query::plumbing::get_query::<rustc_query_impl[88efce5047ede0c6]::queries::eval_to_const_value_raw, rustc_query_impl[88efce5047ede0c6]::plumbing::QueryCtxt, rustc_middle[8dad0eb348f48ec7]::dep_graph::dep_node::DepKind>
  28:     0x7f3a0c1d5a5c - <rustc_query_impl[88efce5047ede0c6]::Queries as rustc_middle[8dad0eb348f48ec7]::ty::query::QueryEngine>::eval_to_const_value_raw
  29:     0x7f3a0d62d9ef - <rustc_middle[8dad0eb348f48ec7]::ty::query::TyCtxtEnsure>::const_eval_poly
  30:     0x7f3a0cf71e7f - <rustc_lint[780114dc5e4bcf7b]::BuiltinCombinedModuleLateLintPass as rustc_lint[780114dc5e4bcf7b]::passes::LateLintPass>::check_item
  31:     0x7f3a0cf22c53 - <rustc_lint[780114dc5e4bcf7b]::late::LateContextAndPass<rustc_lint[780114dc5e4bcf7b]::BuiltinCombinedModuleLateLintPass> as rustc_hir[abf5e5c0653d55df]::intravisit::Visitor>::visit_nested_item
  32:     0x7f3a0cf9c31c - rustc_hir[abf5e5c0653d55df]::intravisit::walk_mod::<rustc_lint[780114dc5e4bcf7b]::late::LateContextAndPass<rustc_lint[780114dc5e4bcf7b]::BuiltinCombinedModuleLateLintPass>>
  33:     0x7f3a0cf28660 - rustc_lint[780114dc5e4bcf7b]::late::late_lint_mod::<rustc_lint[780114dc5e4bcf7b]::BuiltinCombinedModuleLateLintPass>
  34:     0x7f3a0cf6b8ed - rustc_lint[780114dc5e4bcf7b]::lint_mod
  35:     0x7f3a0c4edb1d - rustc_query_system[9c97b1ee9c99d518]::query::plumbing::try_execute_query::<rustc_query_impl[88efce5047ede0c6]::queries::lint_mod, rustc_query_impl[88efce5047ede0c6]::plumbing::QueryCtxt>
  36:     0x7f3a0c59eda0 - rustc_query_system[9c97b1ee9c99d518]::query::plumbing::get_query::<rustc_query_impl[88efce5047ede0c6]::queries::lint_mod, rustc_query_impl[88efce5047ede0c6]::plumbing::QueryCtxt, rustc_middle[8dad0eb348f48ec7]::dep_graph::dep_node::DepKind>
  37:     0x7f3a0c1c7bd0 - <rustc_query_impl[88efce5047ede0c6]::Queries as rustc_middle[8dad0eb348f48ec7]::ty::query::QueryEngine>::lint_mod
  38:     0x7f3a0a89f5f0 - <core[1afb7b33f9e88180]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f0947719a8019974]::sync::par_for_each_in<&[rustc_hir[abf5e5c0653d55df]::hir_id::OwnerId], <rustc_middle[8dad0eb348f48ec7]::hir::map::Map>::par_for_each_module<rustc_lint[780114dc5e4bcf7b]::late::check_crate<rustc_lint[780114dc5e4bcf7b]::BuiltinCombinedLateLintPass, rustc_interface[2575fbd8bca5080e]::passes::analysis::{closure#6}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}> as core[1afb7b33f9e88180]::ops::function::FnOnce<()>>::call_once
  39:     0x7f3a0a81a006 - std[c784e3df5bbe93e2]::panicking::try::<(), core[1afb7b33f9e88180]::panic::unwind_safe::AssertUnwindSafe<rustc_data_structures[f0947719a8019974]::sync::par_for_each_in<&[rustc_hir[abf5e5c0653d55df]::hir_id::OwnerId], <rustc_middle[8dad0eb348f48ec7]::hir::map::Map>::par_for_each_module<rustc_lint[780114dc5e4bcf7b]::late::check_crate<rustc_lint[780114dc5e4bcf7b]::BuiltinCombinedLateLintPass, rustc_interface[2575fbd8bca5080e]::passes::analysis::{closure#6}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}::{closure#0}>::{closure#0}>::{closure#0}::{closure#0}>>
  40:     0x7f3a0a7ccbfd - rustc_data_structures[f0947719a8019974]::sync::par_for_each_in::<&[rustc_hir[abf5e5c0653d55df]::hir_id::OwnerId], <rustc_middle[8dad0eb348f48ec7]::hir::map::Map>::par_for_each_module<rustc_lint[780114dc5e4bcf7b]::late::check_crate<rustc_lint[780114dc5e4bcf7b]::BuiltinCombinedLateLintPass, rustc_interface[2575fbd8bca5080e]::passes::analysis::{closure#6}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}::{closure#0}>::{closure#0}>
  41:     0x7f3a0a80038d - <rustc_session[3c45338a8d9e3773]::session::Session>::time::<(), rustc_lint[780114dc5e4bcf7b]::late::check_crate<rustc_lint[780114dc5e4bcf7b]::BuiltinCombinedLateLintPass, rustc_interface[2575fbd8bca5080e]::passes::analysis::{closure#6}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}>
  42:     0x7f3a0a8004b0 - <rustc_session[3c45338a8d9e3773]::session::Session>::time::<(), rustc_interface[2575fbd8bca5080e]::passes::analysis::{closure#6}::{closure#1}::{closure#2}::{closure#0}>
  43:     0x7f3a0a81a0b5 - std[c784e3df5bbe93e2]::panicking::try::<(), core[1afb7b33f9e88180]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[2575fbd8bca5080e]::passes::analysis::{closure#6}::{closure#1}::{closure#2}>>
  44:     0x7f3a0a8a04a8 - <core[1afb7b33f9e88180]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[2575fbd8bca5080e]::passes::analysis::{closure#6}::{closure#1}> as core[1afb7b33f9e88180]::ops::function::FnOnce<()>>::call_once
  45:     0x7f3a0a801c14 - <rustc_session[3c45338a8d9e3773]::session::Session>::time::<(), rustc_interface[2575fbd8bca5080e]::passes::analysis::{closure#6}>
  46:     0x7f3a0a7dfa6c - rustc_interface[2575fbd8bca5080e]::passes::analysis
  47:     0x7f3a0c4eaf8c - rustc_query_system[9c97b1ee9c99d518]::query::plumbing::try_execute_query::<rustc_query_impl[88efce5047ede0c6]::queries::analysis, rustc_query_impl[88efce5047ede0c6]::plumbing::QueryCtxt>
  48:     0x7f3a0c59e9f1 - rustc_query_system[9c97b1ee9c99d518]::query::plumbing::get_query::<rustc_query_impl[88efce5047ede0c6]::queries::analysis, rustc_query_impl[88efce5047ede0c6]::plumbing::QueryCtxt, rustc_middle[8dad0eb348f48ec7]::dep_graph::dep_node::DepKind>
  49:     0x7f3a0c1a524a - <rustc_query_impl[88efce5047ede0c6]::Queries as rustc_middle[8dad0eb348f48ec7]::ty::query::QueryEngine>::analysis
  50:     0x7f3a0a71e0a2 - <rustc_middle[8dad0eb348f48ec7]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[4e6f8daf397a3442]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[1afb7b33f9e88180]::result::Result<(), rustc_errors[91cc727be444362d]::ErrorGuaranteed>>
  51:     0x7f3a0a6fff25 - <rustc_interface[2575fbd8bca5080e]::interface::Compiler>::enter::<rustc_driver_impl[4e6f8daf397a3442]::run_compiler::{closure#1}::{closure#2}, core[1afb7b33f9e88180]::result::Result<core[1afb7b33f9e88180]::option::Option<rustc_interface[2575fbd8bca5080e]::queries::Linker>, rustc_errors[91cc727be444362d]::ErrorGuaranteed>>
  52:     0x7f3a0a692377 - rustc_span[7ffeea359ae9f249]::with_source_map::<core[1afb7b33f9e88180]::result::Result<(), rustc_errors[91cc727be444362d]::ErrorGuaranteed>, rustc_interface[2575fbd8bca5080e]::interface::run_compiler<core[1afb7b33f9e88180]::result::Result<(), rustc_errors[91cc727be444362d]::ErrorGuaranteed>, rustc_driver_impl[4e6f8daf397a3442]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  53:     0x7f3a0a700ba6 - <scoped_tls[34fa82c1b3c8f743]::ScopedKey<rustc_span[7ffeea359ae9f249]::SessionGlobals>>::set::<rustc_interface[2575fbd8bca5080e]::interface::run_compiler<core[1afb7b33f9e88180]::result::Result<(), rustc_errors[91cc727be444362d]::ErrorGuaranteed>, rustc_driver_impl[4e6f8daf397a3442]::run_compiler::{closure#1}>::{closure#0}, core[1afb7b33f9e88180]::result::Result<(), rustc_errors[91cc727be444362d]::ErrorGuaranteed>>
  54:     0x7f3a0a6a4a59 - std[c784e3df5bbe93e2]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[2575fbd8bca5080e]::util::run_in_thread_pool_with_globals<rustc_interface[2575fbd8bca5080e]::interface::run_compiler<core[1afb7b33f9e88180]::result::Result<(), rustc_errors[91cc727be444362d]::ErrorGuaranteed>, rustc_driver_impl[4e6f8daf397a3442]::run_compiler::{closure#1}>::{closure#0}, core[1afb7b33f9e88180]::result::Result<(), rustc_errors[91cc727be444362d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1afb7b33f9e88180]::result::Result<(), rustc_errors[91cc727be444362d]::ErrorGuaranteed>>
  55:     0x7f3a0a6f6d48 - std[c784e3df5bbe93e2]::panicking::try::<core[1afb7b33f9e88180]::result::Result<(), rustc_errors[91cc727be444362d]::ErrorGuaranteed>, core[1afb7b33f9e88180]::panic::unwind_safe::AssertUnwindSafe<<std[c784e3df5bbe93e2]::thread::Builder>::spawn_unchecked_<rustc_interface[2575fbd8bca5080e]::util::run_in_thread_pool_with_globals<rustc_interface[2575fbd8bca5080e]::interface::run_compiler<core[1afb7b33f9e88180]::result::Result<(), rustc_errors[91cc727be444362d]::ErrorGuaranteed>, rustc_driver_impl[4e6f8daf397a3442]::run_compiler::{closure#1}>::{closure#0}, core[1afb7b33f9e88180]::result::Result<(), rustc_errors[91cc727be444362d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1afb7b33f9e88180]::result::Result<(), rustc_errors[91cc727be444362d]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  56:     0x7f3a0a69ee37 - <<std[c784e3df5bbe93e2]::thread::Builder>::spawn_unchecked_<rustc_interface[2575fbd8bca5080e]::util::run_in_thread_pool_with_globals<rustc_interface[2575fbd8bca5080e]::interface::run_compiler<core[1afb7b33f9e88180]::result::Result<(), rustc_errors[91cc727be444362d]::ErrorGuaranteed>, rustc_driver_impl[4e6f8daf397a3442]::run_compiler::{closure#1}>::{closure#0}, core[1afb7b33f9e88180]::result::Result<(), rustc_errors[91cc727be444362d]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[1afb7b33f9e88180]::result::Result<(), rustc_errors[91cc727be444362d]::ErrorGuaranteed>>::{closure#1} as core[1afb7b33f9e88180]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  57:     0x7f3a09bf451e - std::sys::unix::thread::Thread::new::thread_start::h88c05699f861c7c1
  58:     0x7f3a09989b43 - <unknown>
  59:     0x7f3a09a1ba00 - <unknown>
  60:                0x0 - <unknown>
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.69.0-nightly (7ee75ca7f 2023-02-11) running on x86_64-unknown-linux-gnu


note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z simulate-remapped-rust-src-base=/rustc/FAKE_PREFIX -Z translate-remapped-path-to-local-path=no -Z deduplicate-diagnostics=no -C strip=debuginfo -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [eval_to_allocation_raw] const-evaluating + checking `STR_TOO_LONG`
#1 [eval_to_const_value_raw] simplifying constant for the type system `STR_TOO_LONG`
#2 [eval_to_const_value_raw] simplifying constant for the type system `STR_TOO_LONG`
#3 [lint_mod] linting top-level module
#4 [analysis] running analysis passes on this crate
error: aborting due to previous error
------------------------------------------



---- [ui] tests/ui/pattern/usefulness/issue-30240.rs stdout ----
diff of stderr:

4 LL |     match "world" {
5    |           ^^^^^^^ pattern `&_` not covered
- note: `str` defined here
-   --> $SRC_DIR/core/src/str/mod.rs:LL:COL
9    = note: the matched value is of type `&str`
9    = note: the matched value is of type `&str`
10 help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown


19 LL |     match "world" {
20    |           ^^^^^^^ pattern `&_` not covered
- note: `str` defined here
-   --> $SRC_DIR/core/src/str/mod.rs:LL:COL
24    = note: the matched value is of type `&str`
24    = note: the matched value is of type `&str`
25 help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/usefulness/issue-30240/issue-30240.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/usefulness/issue-30240/issue-30240.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args pattern/usefulness/issue-30240.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/pattern/usefulness/issue-30240.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/usefulness/issue-30240" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pattern/usefulness/issue-30240/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0004]: non-exhaustive patterns: `&_` not covered
  --> fake-test-src-base/pattern/usefulness/issue-30240.rs:2:11
   |
LL |     match "world" { //~ ERROR non-exhaustive patterns: `&_`
   |           ^^^^^^^ pattern `&_` not covered
   = note: the matched value is of type `&str`
   = note: the matched value is of type `&str`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
   |
LL ~         "hello" => {}
LL +         &_ => todo!()


error[E0004]: non-exhaustive patterns: `&_` not covered
  --> fake-test-src-base/pattern/usefulness/issue-30240.rs:6:11
   |
LL |     match "world" { //~ ERROR non-exhaustive patterns: `&_`
   |           ^^^^^^^ pattern `&_` not covered
   = note: the matched value is of type `&str`
   = note: the matched value is of type `&str`
help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
   |
LL ~         "hello" => {}
LL +         &_ => todo!()

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0004`.
For more information about this error, try `rustc --explain E0004`.
------------------------------------------


---- [ui] tests/ui/stdlib-unit-tests/issue-21058.rs stdout ----

error: test run failed!
status: exit status: 101
command: cd "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stdlib-unit-tests/issue-21058" && RUST_TEST_THREADS="16" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/stdlib-unit-tests/issue-21058/a"
stdout: none
--- stderr -------------------------------
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `"str"`,
 right: `"core::str::str"`', fake-test-src-base/stdlib-unit-tests/issue-21058.rs:21:5
------------------------------------------



