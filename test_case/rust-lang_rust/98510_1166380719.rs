plain
failures:

---- [ui] src/test/ui/consts/const-eval/ub-ref-ptr.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-ref-ptr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-ref-ptr" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-ref-ptr/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: it is undefined behavior to use this value
   |
   |
LL | const UNALIGNED: &u16 = unsafe { mem::transmute(&[0u8; 4]) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered an unaligned reference (required 2 byte alignment but found 1)
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
           }

error: internal compiler error: compiler/rustc_const_eval/src/interpret/place.rs:290:13: dereferencing std::boxed::Box<u16>
error: internal compiler error: compiler/rustc_const_eval/src/interpret/place.rs:290:13: dereferencing std::boxed::Box<u16>

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1391:9
stack backtrace:
   0:     0x7f58de590a6c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8174aee7cbf2a759
   1:     0x7f58de5f7928 - core::fmt::write::h809849e5b526d26f
   2:     0x7f58de5807d1 - std::io::Write::write_fmt::h46e0d972eb5e1408
   3:     0x7f58de593a5e - std::panicking::default_hook::{{closure}}::h587c234b85cc1553
   4:     0x7f58de593749 - std::panicking::default_hook::h64e01dcad16aab76
   5:     0x7f58df0b1134 - rustc_driver[4bf3dc5bca8a0474]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f58de5941c2 - std::panicking::rust_panic_with_hook::h023635d05d31e773
   7:     0x7f58e19b3383 - std[8977bdadcc1f8408]::panicking::begin_panic::<rustc_errors[84accd696cbac90]::ExplicitBug>::{closure#0}
   8:     0x7f58e19b1896 - std[8977bdadcc1f8408]::sys_common::backtrace::__rust_end_short_backtrace::<std[8977bdadcc1f8408]::panicking::begin_panic<rustc_errors[84accd696cbac90]::ExplicitBug>::{closure#0}, !>
   9:     0x7f58df031316 - std[8977bdadcc1f8408]::panicking::begin_panic::<rustc_errors[84accd696cbac90]::ExplicitBug>
  10:     0x7f58e1a47bd6 - std[8977bdadcc1f8408]::panic::panic_any::<rustc_errors[84accd696cbac90]::ExplicitBug>
  11:     0x7f58e1a45516 - <rustc_errors[84accd696cbac90]::HandlerInner>::bug::<&alloc[2024e476fb4ac14a]::string::String>
  12:     0x7f58e1a451d0 - <rustc_errors[84accd696cbac90]::Handler>::bug::<&alloc[2024e476fb4ac14a]::string::String>
  13:     0x7f58e1b21e45 - rustc_middle[e9f05be9410ca4eb]::util::bug::opt_span_bug_fmt::<rustc_span[643b28f7be113f92]::span_encoding::Span>::{closure#0}
  14:     0x7f58e1b2147b - rustc_middle[e9f05be9410ca4eb]::ty::context::tls::with_opt::<rustc_middle[e9f05be9410ca4eb]::util::bug::opt_span_bug_fmt<rustc_span[643b28f7be113f92]::span_encoding::Span>::{closure#0}, ()>::{closure#0}
  15:     0x7f58e1b2142c - rustc_middle[e9f05be9410ca4eb]::ty::context::tls::with_opt::<rustc_middle[e9f05be9410ca4eb]::util::bug::opt_span_bug_fmt<rustc_span[643b28f7be113f92]::span_encoding::Span>::{closure#0}, ()>
  16:     0x7f58e1b21d89 - rustc_middle[e9f05be9410ca4eb]::util::bug::opt_span_bug_fmt::<rustc_span[643b28f7be113f92]::span_encoding::Span>
  17:     0x7f58df034425 - rustc_middle[e9f05be9410ca4eb]::util::bug::bug_fmt
  18:     0x7f58e05cb92d - <rustc_const_eval[c4585237aaf12952]::interpret::eval_context::InterpCx<rustc_const_eval[c4585237aaf12952]::const_eval::machine::CompileTimeInterpreter>>::ref_to_mplace
  19:     0x7f58e0688a66 - <rustc_const_eval[c4585237aaf12952]::interpret::validity::ValidityVisitor<rustc_const_eval[c4585237aaf12952]::const_eval::machine::CompileTimeInterpreter>>::check_safe_pointer
  20:     0x7f58e068b24b - <rustc_const_eval[c4585237aaf12952]::interpret::validity::ValidityVisitor<rustc_const_eval[c4585237aaf12952]::const_eval::machine::CompileTimeInterpreter>>::try_visit_primitive
  21:     0x7f58e05dbd36 - <rustc_const_eval[c4585237aaf12952]::interpret::eval_context::InterpCx<rustc_const_eval[c4585237aaf12952]::const_eval::machine::CompileTimeInterpreter>>::validate_operand_internal
  22:     0x7f58e05fc36b - rustc_const_eval[c4585237aaf12952]::const_eval::eval_queries::eval_to_allocation_raw_provider
  23:     0x7f58e0c6d3c7 - rustc_query_system[b071d76e80e8a73a]::query::plumbing::get_query::<rustc_query_impl[416187f9fb93b47]::queries::eval_to_allocation_raw, rustc_query_impl[416187f9fb93b47]::plumbing::QueryCtxt>
  24:     0x7f58e07d8f91 - <rustc_query_impl[416187f9fb93b47]::Queries as rustc_middle[e9f05be9410ca4eb]::ty::query::QueryEngine>::eval_to_allocation_raw
  25:     0x7f58e05f915e - rustc_const_eval[c4585237aaf12952]::const_eval::eval_queries::eval_to_const_value_raw_provider
  26:     0x7f58e0c74ada - rustc_query_system[b071d76e80e8a73a]::query::plumbing::get_query::<rustc_query_impl[416187f9fb93b47]::queries::eval_to_const_value_raw, rustc_query_impl[416187f9fb93b47]::plumbing::QueryCtxt>
  27:     0x7f58e07d9561 - <rustc_query_impl[416187f9fb93b47]::Queries as rustc_middle[e9f05be9410ca4eb]::ty::query::QueryEngine>::eval_to_const_value_raw
  28:     0x7f58e05f8ba3 - rustc_const_eval[c4585237aaf12952]::const_eval::eval_queries::eval_to_const_value_raw_provider
  29:     0x7f58e0c74ada - rustc_query_system[b071d76e80e8a73a]::query::plumbing::get_query::<rustc_query_impl[416187f9fb93b47]::queries::eval_to_const_value_raw, rustc_query_impl[416187f9fb93b47]::plumbing::QueryCtxt>
  30:     0x7f58e07d9561 - <rustc_query_impl[416187f9fb93b47]::Queries as rustc_middle[e9f05be9410ca4eb]::ty::query::QueryEngine>::eval_to_const_value_raw
  31:     0x7f58e1ad3186 - <rustc_middle[e9f05be9410ca4eb]::ty::query::TyCtxtEnsure>::const_eval_poly
  32:     0x7f58e14f0a78 - <rustc_lint[87a5528bae393346]::BuiltinCombinedModuleLateLintPass as rustc_lint[87a5528bae393346]::passes::LateLintPass>::check_item
  33:     0x7f58e1573b8c - <rustc_lint[87a5528bae393346]::late::LateContextAndPass<rustc_lint[87a5528bae393346]::BuiltinCombinedModuleLateLintPass> as rustc_hir[1c3d27f7e3423356]::intravisit::Visitor>::visit_nested_item
  34:     0x7f58e15a220c - rustc_hir[1c3d27f7e3423356]::intravisit::walk_mod::<rustc_lint[87a5528bae393346]::late::LateContextAndPass<rustc_lint[87a5528bae393346]::BuiltinCombinedModuleLateLintPass>>
  35:     0x7f58e157a12b - rustc_lint[87a5528bae393346]::late::late_lint_mod::<rustc_lint[87a5528bae393346]::BuiltinCombinedModuleLateLintPass>
  36:     0x7f58e14ec60d - rustc_lint[87a5528bae393346]::lint_mod
  37:     0x7f58e0b8319c - rustc_query_system[b071d76e80e8a73a]::query::plumbing::try_execute_query::<rustc_query_impl[416187f9fb93b47]::plumbing::QueryCtxt, rustc_query_system[b071d76e80e8a73a]::query::caches::DefaultCache<rustc_span[643b28f7be113f92]::def_id::LocalDefId, ()>>
  38:     0x7f58e0c9a024 - rustc_query_system[b071d76e80e8a73a]::query::plumbing::get_query::<rustc_query_impl[416187f9fb93b47]::queries::lint_mod, rustc_query_impl[416187f9fb93b47]::plumbing::QueryCtxt>
  39:     0x7f58e07cf904 - <rustc_query_impl[416187f9fb93b47]::Queries as rustc_middle[e9f05be9410ca4eb]::ty::query::QueryEngine>::lint_mod
  40:     0x7f58df2672a6 - <rustc_middle[e9f05be9410ca4eb]::hir::map::Map>::for_each_module::<rustc_lint[87a5528bae393346]::late::check_crate<rustc_lint[87a5528bae393346]::BuiltinCombinedLateLintPass, rustc_interface[ab64b9070d359980]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}::{closure#0}>
  41:     0x7f58df1f09e2 - <rustc_session[4bf863f84f086a92]::session::Session>::time::<(), rustc_lint[87a5528bae393346]::late::check_crate<rustc_lint[87a5528bae393346]::BuiltinCombinedLateLintPass, rustc_interface[ab64b9070d359980]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}>
  42:     0x7f58df1f0a90 - <rustc_session[4bf863f84f086a92]::session::Session>::time::<(), rustc_interface[ab64b9070d359980]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}>
  43:     0x7f58df1e1c95 - std[8977bdadcc1f8408]::panicking::try::<(), core[6ca84b1c24a80733]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[ab64b9070d359980]::passes::analysis::{closure#5}::{closure#1}::{closure#2}>>
  44:     0x7f58df26f8ac - <core[6ca84b1c24a80733]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[ab64b9070d359980]::passes::analysis::{closure#5}::{closure#1}> as core[6ca84b1c24a80733]::ops::function::FnOnce<()>>::call_once
  45:     0x7f58df1e1db6 - std[8977bdadcc1f8408]::panicking::try::<(), core[6ca84b1c24a80733]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[ab64b9070d359980]::passes::analysis::{closure#5}::{closure#1}>>
  46:     0x7f58df1f254f - <rustc_session[4bf863f84f086a92]::session::Session>::time::<(), rustc_interface[ab64b9070d359980]::passes::analysis::{closure#5}>
  47:     0x7f58df1dfbdc - rustc_interface[ab64b9070d359980]::passes::analysis
  48:     0x7f58e0bbb614 - rustc_query_system[b071d76e80e8a73a]::query::plumbing::try_execute_query::<rustc_query_impl[416187f9fb93b47]::plumbing::QueryCtxt, rustc_query_system[b071d76e80e8a73a]::query::caches::DefaultCache<(), core[6ca84b1c24a80733]::result::Result<(), rustc_errors[84accd696cbac90]::ErrorGuaranteed>>>
  49:     0x7f58e0c99ad2 - rustc_query_system[b071d76e80e8a73a]::query::plumbing::get_query::<rustc_query_impl[416187f9fb93b47]::queries::analysis, rustc_query_impl[416187f9fb93b47]::plumbing::QueryCtxt>
  50:     0x7f58e07b88ae - <rustc_query_impl[416187f9fb93b47]::Queries as rustc_middle[e9f05be9410ca4eb]::ty::query::QueryEngine>::analysis
  51:     0x7f58df118894 - <rustc_interface[ab64b9070d359980]::passes::QueryContext>::enter::<rustc_driver[4bf3dc5bca8a0474]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[6ca84b1c24a80733]::result::Result<(), rustc_errors[84accd696cbac90]::ErrorGuaranteed>>
  52:     0x7f58df0ceb85 - <rustc_interface[ab64b9070d359980]::interface::Compiler>::enter::<rustc_driver[4bf3dc5bca8a0474]::run_compiler::{closure#1}::{closure#2}, core[6ca84b1c24a80733]::result::Result<core[6ca84b1c24a80733]::option::Option<rustc_interface[ab64b9070d359980]::queries::Linker>, rustc_errors[84accd696cbac90]::ErrorGuaranteed>>
  53:     0x7f58df12613b - rustc_span[643b28f7be113f92]::with_source_map::<core[6ca84b1c24a80733]::result::Result<(), rustc_errors[84accd696cbac90]::ErrorGuaranteed>, rustc_interface[ab64b9070d359980]::interface::create_compiler_and_run<core[6ca84b1c24a80733]::result::Result<(), rustc_errors[84accd696cbac90]::ErrorGuaranteed>, rustc_driver[4bf3dc5bca8a0474]::run_compiler::{closure#1}>::{closure#1}>
  54:     0x7f58df0cfd21 - <scoped_tls[2eb65e05ba874bad]::ScopedKey<rustc_span[643b28f7be113f92]::SessionGlobals>>::set::<rustc_interface[ab64b9070d359980]::interface::run_compiler<core[6ca84b1c24a80733]::result::Result<(), rustc_errors[84accd696cbac90]::ErrorGuaranteed>, rustc_driver[4bf3dc5bca8a0474]::run_compiler::{closure#1}>::{closure#0}, core[6ca84b1c24a80733]::result::Result<(), rustc_errors[84accd696cbac90]::ErrorGuaranteed>>
  55:     0x7f58df11b9d9 - std[8977bdadcc1f8408]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ab64b9070d359980]::util::run_in_thread_pool_with_globals<rustc_interface[ab64b9070d359980]::interface::run_compiler<core[6ca84b1c24a80733]::result::Result<(), rustc_errors[84accd696cbac90]::ErrorGuaranteed>, rustc_driver[4bf3dc5bca8a0474]::run_compiler::{closure#1}>::{closure#0}, core[6ca84b1c24a80733]::result::Result<(), rustc_errors[84accd696cbac90]::ErrorGuaranteed>>::{closure#0}, core[6ca84b1c24a80733]::result::Result<(), rustc_errors[84accd696cbac90]::ErrorGuaranteed>>
  56:     0x7f58df11c579 - <<std[8977bdadcc1f8408]::thread::Builder>::spawn_unchecked_<rustc_interface[ab64b9070d359980]::util::run_in_thread_pool_with_globals<rustc_interface[ab64b9070d359980]::interface::run_compiler<core[6ca84b1c24a80733]::result::Result<(), rustc_errors[84accd696cbac90]::ErrorGuaranteed>, rustc_driver[4bf3dc5bca8a0474]::run_compiler::{closure#1}>::{closure#0}, core[6ca84b1c24a80733]::result::Result<(), rustc_errors[84accd696cbac90]::ErrorGuaranteed>>::{closure#0}, core[6ca84b1c24a80733]::result::Result<(), rustc_errors[84accd696cbac90]::ErrorGuaranteed>>::{closure#1} as core[6ca84b1c24a80733]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  57:     0x7f58de59f5a3 - std::sys::unix::thread::Thread::new::thread_start::h4ee95bea9fcf8a9f
  58:     0x7f58d8af2609 - start_thread
  59:     0x7f58de405133 - clone
  60:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.64.0-nightly (2ab373988 2022-06-25) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [eval_to_allocation_raw] const-evaluating + checking `UNALIGNED_BOX`
#1 [eval_to_const_value_raw] simplifying constant for the type system `UNALIGNED_BOX`
#2 [eval_to_const_value_raw] simplifying constant for the type system `UNALIGNED_BOX`
#3 [lint_mod] linting top-level module
#4 [analysis] running analysis passes on this crate
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0080`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/consts/const-eval/ub-wide-ptr.rs stdout ----

error: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-wide-ptr" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/ub-wide-ptr/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: it is undefined behavior to use this value
   |
   |
LL | const STR_TOO_LONG: &str = unsafe { mem::transmute((&42u8, 999usize)) };
   |
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────alloc8────────╼ e7 03 00 00 00 00 00 00 │ ╾──────╼........

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:39:1
   |
   |
LL | const NESTED_STR_MUCH_TOO_LONG: (&str,) = (unsafe { mem::transmute((&42, usize::MAX)) },);
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .0: encountered invalid reference metadata: slice is bigger than largest supported object
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────alloc14───────╼ ff ff ff ff ff ff ff ff │ ╾──────╼........

error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:42:1
   |
   |
LL | const STR_LENGTH_PTR: &str = unsafe { mem::transmute((&42u8, &3)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:46:1
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:46:1
   |
LL | const MY_STR_LENGTH_PTR: &MyStr = unsafe { mem::transmute((&42u8, &3)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error[E0080]: it is undefined behavior to use this value
error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:49:1
   |
LL | const MY_STR_MUCH_TOO_LONG: &MyStr = unsafe { mem::transmute((&42u8, usize::MAX)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered invalid reference metadata: slice is bigger than largest supported object
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────alloc34───────╼ ff ff ff ff ff ff ff ff │ ╾──────╼........

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:53:1
   |
   |
LL | const STR_NO_INIT: &str = unsafe { mem::transmute::<&[_], _>(&[MaybeUninit::<u8> { uninit: () }]) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .<deref>: encountered uninitialized data in `str`
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:56:1
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:56:1
   |
LL | const MYSTR_NO_INIT: &MyStr = unsafe { mem::transmute::<&[_], _>(&[MaybeUninit::<u8> { uninit: () }]) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .<deref>.0: encountered uninitialized data in `str`
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
           }

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:63:1
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:63:1
   |
LL | / const SLICE_LENGTH_UNINIT: &[u8] = unsafe {
LL | | //~^ ERROR it is undefined behavior to use this value
LL | |     let uninit_len = MaybeUninit::<usize> { uninit: () };
LL | |     mem::transmute((42, uninit_len))
LL | | };
   | |__^ type validation failed: encountered uninitialized reference
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               __ __ __ __ __ __ __ __ __ __ __ __ __ __ __ __ │ ░░░░░░░░░░░░░░░░

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:69:1
   |
   |
LL | const SLICE_TOO_LONG: &[u8] = unsafe { mem::transmute((&42u8, 999usize)) };
   |
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────alloc62───────╼ e7 03 00 00 00 00 00 00 │ ╾──────╼........

error[E0080]: it is undefined behavior to use this value
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:72:1
   |
   |
LL | const SLICE_TOO_LONG_OVERFLOW: &[u32] = unsafe { mem::transmute((&42u32, isize::MAX)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed: encountered invalid reference metadata: slice is bigger than largest supported object
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 16, align: 8) {
               ╾───────alloc70───────╼ ff ff ff ff ff ff ff 7f │ ╾──────╼........

error: any use of this value will cause an error
  --> /checkout/src/test/ui/consts/const-eval/ub-wide-ptr.rs:75:1
   |
   |
LL | const SLICE_LENGTH_PTR: &[u8] = unsafe { mem::transmute((&42u8, &3)) };
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: internal compiler error: compiler/rustc_const_eval/src/interpret/place.rs:290:13: dereferencing std::boxed::Box<[u8]>
error: internal compiler error: compiler/rustc_const_eval/src/interpret/place.rs:290:13: dereferencing std::boxed::Box<[u8]>

thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1391:9
stack backtrace:
   0:     0x7ff3cd2c1a6c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h8174aee7cbf2a759
   1:     0x7ff3cd328928 - core::fmt::write::h809849e5b526d26f
   2:     0x7ff3cd2b17d1 - std::io::Write::write_fmt::h46e0d972eb5e1408
   3:     0x7ff3cd2c4a5e - std::panicking::default_hook::{{closure}}::h587c234b85cc1553
   4:     0x7ff3cd2c4749 - std::panicking::default_hook::h64e01dcad16aab76
   5:     0x7ff3cdde2134 - rustc_driver[4bf3dc5bca8a0474]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7ff3cd2c51c2 - std::panicking::rust_panic_with_hook::h023635d05d31e773
   7:     0x7ff3d06e4383 - std[8977bdadcc1f8408]::panicking::begin_panic::<rustc_errors[84accd696cbac90]::ExplicitBug>::{closure#0}
   8:     0x7ff3d06e2896 - std[8977bdadcc1f8408]::sys_common::backtrace::__rust_end_short_backtrace::<std[8977bdadcc1f8408]::panicking::begin_panic<rustc_errors[84accd696cbac90]::ExplicitBug>::{closure#0}, !>
   9:     0x7ff3cdd62316 - std[8977bdadcc1f8408]::panicking::begin_panic::<rustc_errors[84accd696cbac90]::ExplicitBug>
  10:     0x7ff3d0778bd6 - std[8977bdadcc1f8408]::panic::panic_any::<rustc_errors[84accd696cbac90]::ExplicitBug>
  11:     0x7ff3d0776516 - <rustc_errors[84accd696cbac90]::HandlerInner>::bug::<&alloc[2024e476fb4ac14a]::string::String>
  12:     0x7ff3d07761d0 - <rustc_errors[84accd696cbac90]::Handler>::bug::<&alloc[2024e476fb4ac14a]::string::String>
  13:     0x7ff3d0852e45 - rustc_middle[e9f05be9410ca4eb]::util::bug::opt_span_bug_fmt::<rustc_span[643b28f7be113f92]::span_encoding::Span>::{closure#0}
  14:     0x7ff3d085247b - rustc_middle[e9f05be9410ca4eb]::ty::context::tls::with_opt::<rustc_middle[e9f05be9410ca4eb]::util::bug::opt_span_bug_fmt<rustc_span[643b28f7be113f92]::span_encoding::Span>::{closure#0}, ()>::{closure#0}
  15:     0x7ff3d085242c - rustc_middle[e9f05be9410ca4eb]::ty::context::tls::with_opt::<rustc_middle[e9f05be9410ca4eb]::util::bug::opt_span_bug_fmt<rustc_span[643b28f7be113f92]::span_encoding::Span>::{closure#0}, ()>
  16:     0x7ff3d0852d89 - rustc_middle[e9f05be9410ca4eb]::util::bug::opt_span_bug_fmt::<rustc_span[643b28f7be113f92]::span_encoding::Span>
  17:     0x7ff3cdd65425 - rustc_middle[e9f05be9410ca4eb]::util::bug::bug_fmt
  18:     0x7ff3cf2fc92d - <rustc_const_eval[c4585237aaf12952]::interpret::eval_context::InterpCx<rustc_const_eval[c4585237aaf12952]::const_eval::machine::CompileTimeInterpreter>>::ref_to_mplace
  19:     0x7ff3cf3b9a66 - <rustc_const_eval[c4585237aaf12952]::interpret::validity::ValidityVisitor<rustc_const_eval[c4585237aaf12952]::const_eval::machine::CompileTimeInterpreter>>::check_safe_pointer
  20:     0x7ff3cf3bc24b - <rustc_const_eval[c4585237aaf12952]::interpret::validity::ValidityVisitor<rustc_const_eval[c4585237aaf12952]::const_eval::machine::CompileTimeInterpreter>>::try_visit_primitive
  21:     0x7ff3cf30cd36 - <rustc_const_eval[c4585237aaf12952]::interpret::eval_context::InterpCx<rustc_const_eval[c4585237aaf12952]::const_eval::machine::CompileTimeInterpreter>>::validate_operand_internal
  22:     0x7ff3cf32d36b - rustc_const_eval[c4585237aaf12952]::const_eval::eval_queries::eval_to_allocation_raw_provider
  23:     0x7ff3cf99e3c7 - rustc_query_system[b071d76e80e8a73a]::query::plumbing::get_query::<rustc_query_impl[416187f9fb93b47]::queries::eval_to_allocation_raw, rustc_query_impl[416187f9fb93b47]::plumbing::QueryCtxt>
  24:     0x7ff3cf509f91 - <rustc_query_impl[416187f9fb93b47]::Queries as rustc_middle[e9f05be9410ca4eb]::ty::query::QueryEngine>::eval_to_allocation_raw
  25:     0x7ff3cf32a15e - rustc_const_eval[c4585237aaf12952]::const_eval::eval_queries::eval_to_const_value_raw_provider
  26:     0x7ff3cf9a5ada - rustc_query_system[b071d76e80e8a73a]::query::plumbing::get_query::<rustc_query_impl[416187f9fb93b47]::queries::eval_to_const_value_raw, rustc_query_impl[416187f9fb93b47]::plumbing::QueryCtxt>
  27:     0x7ff3cf50a561 - <rustc_query_impl[416187f9fb93b47]::Queries as rustc_middle[e9f05be9410ca4eb]::ty::query::QueryEngine>::eval_to_const_value_raw
  28:     0x7ff3cf329ba3 - rustc_const_eval[c4585237aaf12952]::const_eval::eval_queries::eval_to_const_value_raw_provider
  29:     0x7ff3cf9a5ada - rustc_query_system[b071d76e80e8a73a]::query::plumbing::get_query::<rustc_query_impl[416187f9fb93b47]::queries::eval_to_const_value_raw, rustc_query_impl[416187f9fb93b47]::plumbing::QueryCtxt>
  30:     0x7ff3cf50a561 - <rustc_query_impl[416187f9fb93b47]::Queries as rustc_middle[e9f05be9410ca4eb]::ty::query::QueryEngine>::eval_to_const_value_raw
  31:     0x7ff3d0804186 - <rustc_middle[e9f05be9410ca4eb]::ty::query::TyCtxtEnsure>::const_eval_poly
  32:     0x7ff3d0221a78 - <rustc_lint[87a5528bae393346]::BuiltinCombinedModuleLateLintPass as rustc_lint[87a5528bae393346]::passes::LateLintPass>::check_item
  33:     0x7ff3d02a4b8c - <rustc_lint[87a5528bae393346]::late::LateContextAndPass<rustc_lint[87a5528bae393346]::BuiltinCombinedModuleLateLintPass> as rustc_hir[1c3d27f7e3423356]::intravisit::Visitor>::visit_nested_item
  34:     0x7ff3d02d320c - rustc_hir[1c3d27f7e3423356]::intravisit::walk_mod::<rustc_lint[87a5528bae393346]::late::LateContextAndPass<rustc_lint[87a5528bae393346]::BuiltinCombinedModuleLateLintPass>>
  35:     0x7ff3d02ab12b - rustc_lint[87a5528bae393346]::late::late_lint_mod::<rustc_lint[87a5528bae393346]::BuiltinCombinedModuleLateLintPass>
  36:     0x7ff3d021d60d - rustc_lint[87a5528bae393346]::lint_mod
  37:     0x7ff3cf8b419c - rustc_query_system[b071d76e80e8a73a]::query::plumbing::try_execute_query::<rustc_query_impl[416187f9fb93b47]::plumbing::QueryCtxt, rustc_query_system[b071d76e80e8a73a]::query::caches::DefaultCache<rustc_span[643b28f7be113f92]::def_id::LocalDefId, ()>>
  38:     0x7ff3cf9cb024 - rustc_query_system[b071d76e80e8a73a]::query::plumbing::get_query::<rustc_query_impl[416187f9fb93b47]::queries::lint_mod, rustc_query_impl[416187f9fb93b47]::plumbing::QueryCtxt>
  39:     0x7ff3cf500904 - <rustc_query_impl[416187f9fb93b47]::Queries as rustc_middle[e9f05be9410ca4eb]::ty::query::QueryEngine>::lint_mod
  40:     0x7ff3cdf982a6 - <rustc_middle[e9f05be9410ca4eb]::hir::map::Map>::for_each_module::<rustc_lint[87a5528bae393346]::late::check_crate<rustc_lint[87a5528bae393346]::BuiltinCombinedLateLintPass, rustc_interface[ab64b9070d359980]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}::{closure#0}>
  41:     0x7ff3cdf219e2 - <rustc_session[4bf863f84f086a92]::session::Session>::time::<(), rustc_lint[87a5528bae393346]::late::check_crate<rustc_lint[87a5528bae393346]::BuiltinCombinedLateLintPass, rustc_interface[ab64b9070d359980]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}::{closure#0}>::{closure#1}::{closure#0}>
  42:     0x7ff3cdf21a90 - <rustc_session[4bf863f84f086a92]::session::Session>::time::<(), rustc_interface[ab64b9070d359980]::passes::analysis::{closure#5}::{closure#1}::{closure#2}::{closure#0}>
  43:     0x7ff3cdf12c95 - std[8977bdadcc1f8408]::panicking::try::<(), core[6ca84b1c24a80733]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[ab64b9070d359980]::passes::analysis::{closure#5}::{closure#1}::{closure#2}>>
  44:     0x7ff3cdfa08ac - <core[6ca84b1c24a80733]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[ab64b9070d359980]::passes::analysis::{closure#5}::{closure#1}> as core[6ca84b1c24a80733]::ops::function::FnOnce<()>>::call_once
  45:     0x7ff3cdf12db6 - std[8977bdadcc1f8408]::panicking::try::<(), core[6ca84b1c24a80733]::panic::unwind_safe::AssertUnwindSafe<rustc_interface[ab64b9070d359980]::passes::analysis::{closure#5}::{closure#1}>>
  46:     0x7ff3cdf2354f - <rustc_session[4bf863f84f086a92]::session::Session>::time::<(), rustc_interface[ab64b9070d359980]::passes::analysis::{closure#5}>
  47:     0x7ff3cdf10bdc - rustc_interface[ab64b9070d359980]::passes::analysis
  48:     0x7ff3cf8ec614 - rustc_query_system[b071d76e80e8a73a]::query::plumbing::try_execute_query::<rustc_query_impl[416187f9fb93b47]::plumbing::QueryCtxt, rustc_query_system[b071d76e80e8a73a]::query::caches::DefaultCache<(), core[6ca84b1c24a80733]::result::Result<(), rustc_errors[84accd696cbac90]::ErrorGuaranteed>>>
  49:     0x7ff3cf9caad2 - rustc_query_system[b071d76e80e8a73a]::query::plumbing::get_query::<rustc_query_impl[416187f9fb93b47]::queries::analysis, rustc_query_impl[416187f9fb93b47]::plumbing::QueryCtxt>
  50:     0x7ff3cf4e98ae - <rustc_query_impl[416187f9fb93b47]::Queries as rustc_middle[e9f05be9410ca4eb]::ty::query::QueryEngine>::analysis
  51:     0x7ff3cde49894 - <rustc_interface[ab64b9070d359980]::passes::QueryContext>::enter::<rustc_driver[4bf3dc5bca8a0474]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[6ca84b1c24a80733]::result::Result<(), rustc_errors[84accd696cbac90]::ErrorGuaranteed>>
  52:     0x7ff3cddffb85 - <rustc_interface[ab64b9070d359980]::interface::Compiler>::enter::<rustc_driver[4bf3dc5bca8a0474]::run_compiler::{closure#1}::{closure#2}, core[6ca84b1c24a80733]::result::Result<core[6ca84b1c24a80733]::option::Option<rustc_interface[ab64b9070d359980]::queries::Linker>, rustc_errors[84accd696cbac90]::ErrorGuaranteed>>
  53:     0x7ff3cde5713b - rustc_span[643b28f7be113f92]::with_source_map::<core[6ca84b1c24a80733]::result::Result<(), rustc_errors[84accd696cbac90]::ErrorGuaranteed>, rustc_interface[ab64b9070d359980]::interface::create_compiler_and_run<core[6ca84b1c24a80733]::result::Result<(), rustc_errors[84accd696cbac90]::ErrorGuaranteed>, rustc_driver[4bf3dc5bca8a0474]::run_compiler::{closure#1}>::{closure#1}>
  54:     0x7ff3cde00d21 - <scoped_tls[2eb65e05ba874bad]::ScopedKey<rustc_span[643b28f7be113f92]::SessionGlobals>>::set::<rustc_interface[ab64b9070d359980]::interface::run_compiler<core[6ca84b1c24a80733]::result::Result<(), rustc_errors[84accd696cbac90]::ErrorGuaranteed>, rustc_driver[4bf3dc5bca8a0474]::run_compiler::{closure#1}>::{closure#0}, core[6ca84b1c24a80733]::result::Result<(), rustc_errors[84accd696cbac90]::ErrorGuaranteed>>
  55:     0x7ff3cde4c9d9 - std[8977bdadcc1f8408]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[ab64b9070d359980]::util::run_in_thread_pool_with_globals<rustc_interface[ab64b9070d359980]::interface::run_compiler<core[6ca84b1c24a80733]::result::Result<(), rustc_errors[84accd696cbac90]::ErrorGuaranteed>, rustc_driver[4bf3dc5bca8a0474]::run_compiler::{closure#1}>::{closure#0}, core[6ca84b1c24a80733]::result::Result<(), rustc_errors[84accd696cbac90]::ErrorGuaranteed>>::{closure#0}, core[6ca84b1c24a80733]::result::Result<(), rustc_errors[84accd696cbac90]::ErrorGuaranteed>>
  56:     0x7ff3cde4d579 - <<std[8977bdadcc1f8408]::thread::Builder>::spawn_unchecked_<rustc_interface[ab64b9070d359980]::util::run_in_thread_pool_with_globals<rustc_interface[ab64b9070d359980]::interface::run_compiler<core[6ca84b1c24a80733]::result::Result<(), rustc_errors[84accd696cbac90]::ErrorGuaranteed>, rustc_driver[4bf3dc5bca8a0474]::run_compiler::{closure#1}>::{closure#0}, core[6ca84b1c24a80733]::result::Result<(), rustc_errors[84accd696cbac90]::ErrorGuaranteed>>::{closure#0}, core[6ca84b1c24a80733]::result::Result<(), rustc_errors[84accd696cbac90]::ErrorGuaranteed>>::{closure#1} as core[6ca84b1c24a80733]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  57:     0x7ff3cd2d05a3 - std::sys::unix::thread::Thread::new::thread_start::h4ee95bea9fcf8a9f
  58:     0x7ff3c7823609 - start_thread
  59:     0x7ff3cd136133 - clone
  60:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.64.0-nightly (2ab373988 2022-06-25) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -C codegen-units=1 -Z ui-testing -Z deduplicate-diagnostics=no -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [eval_to_allocation_raw] const-evaluating + checking `SLICE_TOO_LONG_BOX`
#1 [eval_to_const_value_raw] simplifying constant for the type system `SLICE_TOO_LONG_BOX`
#2 [eval_to_const_value_raw] simplifying constant for the type system `SLICE_TOO_LONG_BOX`
#3 [lint_mod] linting top-level module
#4 [analysis] running analysis passes on this crate
error: aborting due to 12 previous errors

For more information about this error, try `rustc --explain E0080`.
------------------------------------------
