plain

actual output differed from expected
--- tests/pass/issues/issue-miri-2433.stderr
+++ <stderr output>
+error: internal compiler error: compiler/rustc_middle/src/ty/normalize_erasing_regions.rs:LL:CC: Failed to normalize <[closure@$DIR/issue-miri-2433.rs:LL:CC] as std::ops::FnOnce<(&S,)>>::Output, maybe try to call `try_normalize_erasing_regions` instead
+thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:LL:CC
+stack backtrace:
+stack backtrace:
+   0: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hbcdf23e750a13ab2
+   1: core::fmt::write::h4c4fdb10bc12d8db
+   2: std::io::Write::write_fmt::hdbc2d04cb7b53668
+   3: std::sys_common::backtrace::print::h1211d2eab0b0965e
+   4: std::panicking::default_hook::{{closure}}::h123cdef8a44d675d
+   5: std::panicking::default_hook::h07c6df43cd328b58
+   6: rustc_driver[c146c5b7b4634686]::DEFAULT_HOOK::{closure#0}::{closure#0}
+   7: std::panicking::rust_panic_with_hook::h1c8024fabba909da
+   8: std[82a801ad1f56fc1a]::panicking::begin_panic::{closure#0}
+   9: std[82a801ad1f56fc1a]::sys_common::backtrace::__rust_end_short_backtrace
+  10: std[82a801ad1f56fc1a]::panicking::begin_panic
+  11: std[82a801ad1f56fc1a]::panic::panic_any
+  12: <rustc_errors[946d7bf956b29fc9]::HandlerInner>::bug
+  13: <rustc_errors[946d7bf956b29fc9]::Handler>::bug
+  14: rustc_middle[4c33cc9cc145a931]::ty::context::tls::with_context_opt
+  15: rustc_middle[4c33cc9cc145a931]::util::bug::opt_span_bug_fmt
+  16: rustc_middle[4c33cc9cc145a931]::util::bug::bug_fmt
+  17: <rustc_middle[4c33cc9cc145a931]::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder>::normalize_generic_arg_after_erasing_regions
+  18: <rustc_middle[4c33cc9cc145a931]::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder as rustc_middle[4c33cc9cc145a931]::ty::fold::TypeFolder>::fold_ty
+  19: rustc_middle[4c33cc9cc145a931]::ty::util::fold_list
+  20: <rustc_middle[4c33cc9cc145a931]::ty::context::TyCtxt>::normalize_erasing_late_bound_regions
+  21: <rustc_const_eval[b08cec040e7b24a2]::interpret::eval_context::InterpCx<miri[f622776a5019f3f]::machine::MiriMachine>>::terminator
+  22: <core[a8f9135fb440a806]::panic::unwind_safe::AssertUnwindSafe<miri[f622776a5019f3f]::eval::eval_entry::{closure#0}> as core[a8f9135fb440a806]::ops::function::FnOnce<()>>::call_once
+  23: miri[f622776a5019f3f]::eval::eval_entry
+  24: <rustc_interface[c0f4784efb01abdd]::passes::QueryContext>::enter
+  25: <miri[6612b2aeeb6c0f52]::MiriCompilerCalls as rustc_driver[c146c5b7b4634686]::Callbacks>::after_analysis
+  26: <rustc_interface[c0f4784efb01abdd]::interface::Compiler>::enter
+  27: rustc_span[28bcdc41e509700b]::with_source_map
+  28: <scoped_tls[61b97c32a703749a]::ScopedKey<rustc_span[28bcdc41e509700b]::SessionGlobals>>::set
+  29: std[82a801ad1f56fc1a]::sys_common::backtrace::__rust_begin_short_backtrace
+  30: <<std[82a801ad1f56fc1a]::thread::Builder>::spawn_unchecked_<rustc_interface[c0f4784efb01abdd]::util::run_in_thread_pool_with_globals<rustc_interface[c0f4784efb01abdd]::interface::run_compiler<core[a8f9135fb440a806]::result::Result<(), rustc_errors[946d7bf956b29fc9]::ErrorGuaranteed>, rustc_driver[c146c5b7b4634686]::run_compiler::{closure#1}>::{closure#0}, core[a8f9135fb440a806]::result::Result<(), rustc_errors[946d7bf956b29fc9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[a8f9135fb440a806]::result::Result<(), rustc_errors[946d7bf956b29fc9]::ErrorGuaranteed>>::{closure#1} as core[a8f9135fb440a806]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
+  31: std::sys::PLATFORM::thread::Thread::new::thread_start::h1bd148d4d422b2b4
+  32: <unknown>
+  33: <unknown>
+  34: <unknown>
+note: the compiler unexpectedly panicked. this is a bug.
+
+note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md
+
+
+note: rustc 1.67.0-nightly (8cd322618 2022-11-15) running on x86_64-unknown-linux-gnu
+
+note: compiler flags: -Z ui-testing
+
+query stack during panic:
+end of query stack
+
+Miri caused an ICE during evaluation. Here's the interpreter backtrace at the time of the panic:
+note: the place in the program where the ICE was triggered
+   |
+LL |     fun(filter_positive());
+   |     ^^^^^^^^^^^^^^^^^^^^^^
+   |
+   |
+   = note: inside `with_positive::<[closure@$DIR/issue-miri-2433.rs:LL:CC]>` at $DIR/issue-miri-2433.rs:LL:CC
+  --> $DIR/issue-miri-2433.rs:LL:CC
+   |
+   |
+LL |     with_positive(|_| ());
+   = note: inside `<fn() as std::ops::FnOnce<()>>::call_once - shim(fn())` at RUSTLIB/core/src/ops/function.rs:LL:CC
+   = note: inside `std::sys_common::backtrace::__rust_begin_short_backtrace::<fn(), ()>` at RUSTLIB/std/src/sys_common/backtrace.rs:LL:CC
+   = note: inside closure at RUSTLIB/std/src/rt.rs:LL:CC
+   = note: inside `std::ops::function::impls::<impl std::ops::FnOnce<()> for &dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe>::call_once` at RUSTLIB/core/src/ops/function.rs:LL:CC
---
+


There were 1 unmatched diagnostics that occurred outside the testfile and had no pattern
    Ice: compiler/rustc_middle/src/ty/normalize_erasing_regions.rs:198:90: Failed to normalize <[closure@tests/pass/issues/issue-miri-2433.rs:23:19: 23:22] as std::ops::FnOnce<(&S,)>>::Output, maybe try to call `try_normalize_erasing_regions` instead
full stderr:
full stderr:
error: internal compiler error: compiler/rustc_middle/src/ty/normalize_erasing_regions.rs:198:90: Failed to normalize <[closure@tests/pass/issues/issue-miri-2433.rs:23:19: 23:22] as std::ops::FnOnce<(&S,)>>::Output, maybe try to call `try_normalize_erasing_regions` instead
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1555:9
stack backtrace:
stack backtrace:
   0:     0x7fd69fe4d4f2 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hbcdf23e750a13ab2
   1:     0x7fd69febb008 - core::fmt::write::h4c4fdb10bc12d8db
   2:     0x7fd69fe3e351 - std::io::Write::write_fmt::hdbc2d04cb7b53668
   3:     0x7fd69fe4d2b5 - std::sys_common::backtrace::print::h1211d2eab0b0965e
   4:     0x7fd69fe507f7 - std::panicking::default_hook::{{closure}}::h123cdef8a44d675d
   5:     0x7fd69fe50555 - std::panicking::default_hook::h07c6df43cd328b58
   6:     0x7fd6a07be4c2 - rustc_driver[c146c5b7b4634686]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fd69fe51103 - std::panicking::rust_panic_with_hook::h1c8024fabba909da
   8:     0x7fd6a39c1c43 - std[82a801ad1f56fc1a]::panicking::begin_panic::<rustc_errors[946d7bf956b29fc9]::ExplicitBug>::{closure#0}
   9:     0x7fd6a39ba246 - std[82a801ad1f56fc1a]::sys_common::backtrace::__rust_end_short_backtrace::<std[82a801ad1f56fc1a]::panicking::begin_panic<rustc_errors[946d7bf956b29fc9]::ExplicitBug>::{closure#0}, !>
  10:     0x7fd6a07615f6 - std[82a801ad1f56fc1a]::panicking::begin_panic::<rustc_errors[946d7bf956b29fc9]::ExplicitBug>
  11:     0x7fd6a3a89a16 - std[82a801ad1f56fc1a]::panic::panic_any::<rustc_errors[946d7bf956b29fc9]::ExplicitBug>
  12:     0x7fd6a3a82c07 - <rustc_errors[946d7bf956b29fc9]::HandlerInner>::bug::<&alloc[201d573aa8508c7d]::string::String>
  13:     0x7fd6a3a7ecd0 - <rustc_errors[946d7bf956b29fc9]::Handler>::bug::<&alloc[201d573aa8508c7d]::string::String>
  14:     0x7fd6a3b6cb70 - rustc_middle[4c33cc9cc145a931]::ty::context::tls::with_context_opt::<rustc_middle[4c33cc9cc145a931]::ty::context::tls::with_opt<rustc_middle[4c33cc9cc145a931]::util::bug::opt_span_bug_fmt<rustc_span[28bcdc41e509700b]::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
  15:     0x7fd6a3b6e129 - rustc_middle[4c33cc9cc145a931]::util::bug::opt_span_bug_fmt::<rustc_span[28bcdc41e509700b]::span_encoding::Span>
  16:     0x7fd6a0766ac5 - rustc_middle[4c33cc9cc145a931]::util::bug::bug_fmt
  17:     0x7fd6a3bfc403 - <rustc_middle[4c33cc9cc145a931]::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder>::normalize_generic_arg_after_erasing_regions
  18:     0x7fd6a3bfc476 - <rustc_middle[4c33cc9cc145a931]::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder as rustc_middle[4c33cc9cc145a931]::ty::fold::TypeFolder>::fold_ty
  19:     0x55df7b1673be - rustc_middle[4c33cc9cc145a931]::ty::util::fold_list::<rustc_middle[4c33cc9cc145a931]::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder, rustc_middle[4c33cc9cc145a931]::ty::Ty, <&rustc_middle[4c33cc9cc145a931]::ty::list::List<rustc_middle[4c33cc9cc145a931]::ty::Ty> as rustc_middle[4c33cc9cc145a931]::ty::fold::TypeFoldable>::try_fold_with<rustc_middle[4c33cc9cc145a931]::ty::normalize_erasing_regions::NormalizeAfterErasingRegionsFolder>::{closure#0}>
  20:     0x55df7b1290bf - <rustc_middle[4c33cc9cc145a931]::ty::context::TyCtxt>::normalize_erasing_late_bound_regions::<rustc_middle[4c33cc9cc145a931]::ty::sty::FnSig>
  21:     0x55df7b0269e7 - <rustc_const_eval[b08cec040e7b24a2]::interpret::eval_context::InterpCx<miri[f622776a5019f3f]::machine::MiriMachine>>::terminator
  22:     0x55df7b148b4e - <core[a8f9135fb440a806]::panic::unwind_safe::AssertUnwindSafe<miri[f622776a5019f3f]::eval::eval_entry::{closure#0}> as core[a8f9135fb440a806]::ops::function::FnOnce<()>>::call_once
  23:     0x55df7afbc1a5 - miri[f622776a5019f3f]::eval::eval_entry
  24:     0x55df7aee64e3 - <rustc_interface[c0f4784efb01abdd]::passes::QueryContext>::enter::<<miri[6612b2aeeb6c0f52]::MiriCompilerCalls as rustc_driver[c146c5b7b4634686]::Callbacks>::after_analysis::{closure#0}, ()>
  25:     0x55df7aeed09f - <miri[6612b2aeeb6c0f52]::MiriCompilerCalls as rustc_driver[c146c5b7b4634686]::Callbacks>::after_analysis
  26:     0x7fd6a08570cc - <rustc_interface[c0f4784efb01abdd]::interface::Compiler>::enter::<rustc_driver[c146c5b7b4634686]::run_compiler::{closure#1}::{closure#2}, core[a8f9135fb440a806]::result::Result<core[a8f9135fb440a806]::option::Option<rustc_interface[c0f4784efb01abdd]::queries::Linker>, rustc_errors[946d7bf956b29fc9]::ErrorGuaranteed>>
  27:     0x7fd6a07c00b2 - rustc_span[28bcdc41e509700b]::with_source_map::<core[a8f9135fb440a806]::result::Result<(), rustc_errors[946d7bf956b29fc9]::ErrorGuaranteed>, rustc_interface[c0f4784efb01abdd]::interface::run_compiler<core[a8f9135fb440a806]::result::Result<(), rustc_errors[946d7bf956b29fc9]::ErrorGuaranteed>, rustc_driver[c146c5b7b4634686]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  28:     0x7fd6a08445c1 - <scoped_tls[61b97c32a703749a]::ScopedKey<rustc_span[28bcdc41e509700b]::SessionGlobals>>::set::<rustc_interface[c0f4784efb01abdd]::interface::run_compiler<core[a8f9135fb440a806]::result::Result<(), rustc_errors[946d7bf956b29fc9]::ErrorGuaranteed>, rustc_driver[c146c5b7b4634686]::run_compiler::{closure#1}>::{closure#0}, core[a8f9135fb440a806]::result::Result<(), rustc_errors[946d7bf956b29fc9]::ErrorGuaranteed>>
  29:     0x7fd6a083f7c0 - std[82a801ad1f56fc1a]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[c0f4784efb01abdd]::util::run_in_thread_pool_with_globals<rustc_interface[c0f4784efb01abdd]::interface::run_compiler<core[a8f9135fb440a806]::result::Result<(), rustc_errors[946d7bf956b29fc9]::ErrorGuaranteed>, rustc_driver[c146c5b7b4634686]::run_compiler::{closure#1}>::{closure#0}, core[a8f9135fb440a806]::result::Result<(), rustc_errors[946d7bf956b29fc9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[a8f9135fb440a806]::result::Result<(), rustc_errors[946d7bf956b29fc9]::ErrorGuaranteed>>
  30:     0x7fd6a07d3fe9 - <<std[82a801ad1f56fc1a]::thread::Builder>::spawn_unchecked_<rustc_interface[c0f4784efb01abdd]::util::run_in_thread_pool_with_globals<rustc_interface[c0f4784efb01abdd]::interface::run_compiler<core[a8f9135fb440a806]::result::Result<(), rustc_errors[946d7bf956b29fc9]::ErrorGuaranteed>, rustc_driver[c146c5b7b4634686]::run_compiler::{closure#1}>::{closure#0}, core[a8f9135fb440a806]::result::Result<(), rustc_errors[946d7bf956b29fc9]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[a8f9135fb440a806]::result::Result<(), rustc_errors[946d7bf956b29fc9]::ErrorGuaranteed>>::{closure#1} as core[a8f9135fb440a806]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  31:     0x7fd69fe5df2e - std::sys::unix::thread::Thread::new::thread_start::h1bd148d4d422b2b4
  32:     0x7fd69faeeb43 - <unknown>
  33:     0x7fd69fb80a00 - <unknown>
  34:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.67.0-nightly (8cd322618 2022-11-15) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z ui-testing

query stack during panic:
end of query stack

Miri caused an ICE during evaluation. Here's the interpreter backtrace at the time of the panic:
note: the place in the program where the ICE was triggered
   |
LL |     fun(filter_positive());
   |     ^^^^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: inside `with_positive::<[closure@tests/pass/issues/issue-miri-2433.rs:23:19: 23:22]>` at tests/pass/issues/issue-miri-2433.rs:19:5
note: inside `main` at tests/pass/issues/issue-miri-2433.rs:23:5
  --> tests/pass/issues/issue-miri-2433.rs:23:5
   |
LL |     with_positive(|_| ());
   = note: inside `<fn() as std::ops::FnOnce<()>>::call_once - shim(fn())` at /checkout/library/core/src/ops/function.rs:507:5
   = note: inside `std::sys_common::backtrace::__rust_begin_short_backtrace::<fn(), ()>` at /checkout/library/std/src/sys_common/backtrace.rs:121:18
   = note: inside closure at /checkout/library/std/src/rt.rs:166:18
   = note: inside `std::ops::function::impls::<impl std::ops::FnOnce<()> for &dyn std::ops::Fn() -> i32 + std::marker::Sync + std::panic::RefUnwindSafe>::call_once` at /checkout/library/core/src/ops/function.rs:606:13
