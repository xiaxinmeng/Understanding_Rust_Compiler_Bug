
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: could not unify `fn(()) -> _` and `fn(u8) -> impl Future<Output = ()>`: ArgumentSorts(ExpectedFound { expected: (), found: u8 }, 0)
 --> src/lib.rs:8:25
  |
8 |     async fn uwu(x: u8) {}
  |                         ^
  |
  = note: delayed at compiler/rustc_hir_analysis/src/check/compare_method.rs:529:33

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler/rustc_trait_selection/src/traits/project.rs:2295:41

error: internal compiler error[[E0053]](https://doc.rust-lang.org/nightly/error-index.html#E0053): method `uwu` has an incompatible type for trait
 --> src/lib.rs:8:21
  |
8 |     async fn uwu(x: u8) {}
  |                     ^^ help: change the parameter type to match the trait: `()`
  |
note: while checking the return type of the `async fn`
 --> src/lib.rs:8:25
  |
8 |     async fn uwu(x: u8) {}
  |                         ^ checked the `Output` of this `async fn`, found opaque type

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1585:13
stack backtrace:
   0:     0x7f2929c17280 - std::backtrace_rs::backtrace::libunwind::trace::h90130e9f1ec9d9fc
                               at /rustc/85f4f41deb1557ca8ab228319d33003dd2f20f45/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f2929c17280 - std::backtrace_rs::backtrace::trace_unsynchronized::h40c4f6d986aff2e4
                               at /rustc/85f4f41deb1557ca8ab228319d33003dd2f20f45/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f2929c17280 - std::sys_common::backtrace::_print_fmt::h3822369be0084f83
                               at /rustc/85f4f41deb1557ca8ab228319d33003dd2f20f45/library/std/src/sys_common/backtrace.rs:65:5
   3:     0x7f2929c17280 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hdae4626daadc9f71
                               at /rustc/85f4f41deb1557ca8ab228319d33003dd2f20f45/library/std/src/sys_common/backtrace.rs:44:22
   4:     0x7f2925d5d4fe - core::fmt::write::h05cdd281d08b9d01
                               at /rustc/85f4f41deb1557ca8ab228319d33003dd2f20f45/library/core/src/fmt/mod.rs:1209:17
   5:     0x7f2929c0b365 - std::io::Write::write_fmt::h8e1559869183c8bb
                               at /rustc/85f4f41deb1557ca8ab228319d33003dd2f20f45/library/std/src/io/mod.rs:1682:15
   6:     0x7f2929c17045 - std::sys_common::backtrace::_print::h61ba1afc98d46ffd
                               at /rustc/85f4f41deb1557ca8ab228319d33003dd2f20f45/library/std/src/sys_common/backtrace.rs:47:5
   7:     0x7f2929c17045 - std::sys_common::backtrace::print::he9728762d292043e
                               at /rustc/85f4f41deb1557ca8ab228319d33003dd2f20f45/library/std/src/sys_common/backtrace.rs:34:9
   8:     0x7f2929c1935f - std::panicking::default_hook::{{closure}}::h726bafb7da990095
                               at /rustc/85f4f41deb1557ca8ab228319d33003dd2f20f45/library/std/src/panicking.rs:267:22
   9:     0x7f2929c1909a - std::panicking::default_hook::hdc60b442b1844a68
                               at /rustc/85f4f41deb1557ca8ab228319d33003dd2f20f45/library/std/src/panicking.rs:286:9
  10:     0x7f2928e98a14 - rustc_driver[b81881107b3cefb1]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7f2929c19b4d - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::h3db297037157cab4
                               at /rustc/85f4f41deb1557ca8ab228319d33003dd2f20f45/library/alloc/src/boxed.rs:2032:9
  12:     0x7f2929c19b4d - std::panicking::rust_panic_with_hook::hc98185590f941643
                               at /rustc/85f4f41deb1557ca8ab228319d33003dd2f20f45/library/std/src/panicking.rs:692:13
  13:     0x7f2928effbe1 - std[23e74d0f7912c00c]::panicking::begin_panic::<rustc_errors[43d68c63b698f7a8]::ExplicitBug>::{closure#0}
  14:     0x7f2928efe376 - std[23e74d0f7912c00c]::sys_common::backtrace::__rust_end_short_backtrace::<std[23e74d0f7912c00c]::panicking::begin_panic<rustc_errors[43d68c63b698f7a8]::ExplicitBug>::{closure#0}, !>
  15:     0x7f2928efcd16 - std[23e74d0f7912c00c]::panicking::begin_panic::<rustc_errors[43d68c63b698f7a8]::ExplicitBug>
  16:     0x7f2928efa1f6 - std[23e74d0f7912c00c]::panic::panic_any::<rustc_errors[43d68c63b698f7a8]::ExplicitBug>
  17:     0x7f2928424ed8 - <rustc_errors[43d68c63b698f7a8]::HandlerInner>::flush_delayed::<alloc[e7e939941d2090cd]::vec::Vec<rustc_errors[43d68c63b698f7a8]::diagnostic::Diagnostic>, &str>
  18:     0x7f29281ccc6e - <rustc_interface[74ce99d2ea85b1]::passes::QueryContext>::enter::<<rustc_interface[74ce99d2ea85b1]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[c2065954f2d40e37]::result::Result<alloc[e7e939941d2090cd]::boxed::Box<dyn core[c2065954f2d40e37]::any::Any>, rustc_errors[43d68c63b698f7a8]::ErrorGuaranteed>>
  19:     0x7f292818d523 - <rustc_interface[74ce99d2ea85b1]::queries::Queries>::ongoing_codegen
  20:     0x7f292818c632 - <rustc_interface[74ce99d2ea85b1]::interface::Compiler>::enter::<rustc_driver[b81881107b3cefb1]::run_compiler::{closure#1}::{closure#2}, core[c2065954f2d40e37]::result::Result<core[c2065954f2d40e37]::option::Option<rustc_interface[74ce99d2ea85b1]::queries::Linker>, rustc_errors[43d68c63b698f7a8]::ErrorGuaranteed>>
  21:     0x7f2928187672 - rustc_span[7a6d98bdad94076d]::with_source_map::<core[c2065954f2d40e37]::result::Result<(), rustc_errors[43d68c63b698f7a8]::ErrorGuaranteed>, rustc_interface[74ce99d2ea85b1]::interface::run_compiler<core[c2065954f2d40e37]::result::Result<(), rustc_errors[43d68c63b698f7a8]::ErrorGuaranteed>, rustc_driver[b81881107b3cefb1]::run_compiler::{closure#1}>::{closure#0}::{closure#1}>
  22:     0x7f2928187169 - <scoped_tls[866d789ca631dc2f]::ScopedKey<rustc_span[7a6d98bdad94076d]::SessionGlobals>>::set::<rustc_interface[74ce99d2ea85b1]::interface::run_compiler<core[c2065954f2d40e37]::result::Result<(), rustc_errors[43d68c63b698f7a8]::ErrorGuaranteed>, rustc_driver[b81881107b3cefb1]::run_compiler::{closure#1}>::{closure#0}, core[c2065954f2d40e37]::result::Result<(), rustc_errors[43d68c63b698f7a8]::ErrorGuaranteed>>
  23:     0x7f2928186778 - std[23e74d0f7912c00c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[74ce99d2ea85b1]::util::run_in_thread_pool_with_globals<rustc_interface[74ce99d2ea85b1]::interface::run_compiler<core[c2065954f2d40e37]::result::Result<(), rustc_errors[43d68c63b698f7a8]::ErrorGuaranteed>, rustc_driver[b81881107b3cefb1]::run_compiler::{closure#1}>::{closure#0}, core[c2065954f2d40e37]::result::Result<(), rustc_errors[43d68c63b698f7a8]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[c2065954f2d40e37]::result::Result<(), rustc_errors[43d68c63b698f7a8]::ErrorGuaranteed>>
  24:     0x7f292818649c - <<std[23e74d0f7912c00c]::thread::Builder>::spawn_unchecked_<rustc_interface[74ce99d2ea85b1]::util::run_in_thread_pool_with_globals<rustc_interface[74ce99d2ea85b1]::interface::run_compiler<core[c2065954f2d40e37]::result::Result<(), rustc_errors[43d68c63b698f7a8]::ErrorGuaranteed>, rustc_driver[b81881107b3cefb1]::run_compiler::{closure#1}>::{closure#0}, core[c2065954f2d40e37]::result::Result<(), rustc_errors[43d68c63b698f7a8]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[c2065954f2d40e37]::result::Result<(), rustc_errors[43d68c63b698f7a8]::ErrorGuaranteed>>::{closure#1} as core[c2065954f2d40e37]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  25:     0x7f2929c20a83 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::he8eec53b13ee98a2
                               at /rustc/85f4f41deb1557ca8ab228319d33003dd2f20f45/library/alloc/src/boxed.rs:2000:9
  26:     0x7f2929c20a83 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h04d7374ad262fa8b
                               at /rustc/85f4f41deb1557ca8ab228319d33003dd2f20f45/library/alloc/src/boxed.rs:2000:9
  27:     0x7f2929c20a83 - std::sys::unix::thread::Thread::new::thread_start::h0bc6cb3772aee1f7
                               at /rustc/85f4f41deb1557ca8ab228319d33003dd2f20f45/library/std/src/sys/unix/thread.rs:108:17
  28:     0x7f2925bd1609 - start_thread
  29:     0x7f2925af4133 - clone
  30:                0x0 - <unknown>

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (85f4f41de 2022-11-08) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C embed-bitcode=no -C codegen-units=1 -C debuginfo=2

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
