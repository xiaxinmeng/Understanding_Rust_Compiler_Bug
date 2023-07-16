plain
+ gather_profiles Debug,Opt Full syn-1.0.89,cargo-0.60.0,serde-1.0.136,ripgrep-13.0.0,regex-1.5.5,clap-3.1.6,hyper-0.14.18
+ cd /checkout/obj
+ RUSTC_BOOTSTRAP=1
+ ./build/x86_64-unknown-linux-gnu/stage2/bin/rustc --edition=2021 --crate-type=lib ../library/core/src/lib.rs
error[E0391]: cycle detected when computing type of `future::from_generator::{opaque#0}`
  --> ../library/core/src/future/mod.rs:72:43
   |
72 | pub const fn from_generator<T>(gen: T) -> impl Future<Output = T::Return>
   |
   |
note: ...which requires borrow-checking `future::from_generator`...
  --> ../library/core/src/future/mod.rs:72:1
   |
72 | / pub const fn from_generator<T>(gen: T) -> impl Future<Output = T::Return>
73 | | where
74 | |     T: Generator<ResumeTy, Yield = ()>,
   | |_______________________________________^
note: ...which requires processing `future::from_generator`...
  --> ../library/core/src/future/mod.rs:72:1
   |
72 | / pub const fn from_generator<T>(gen: T) -> impl Future<Output = T::Return>
73 | | where
74 | |     T: Generator<ResumeTy, Yield = ()>,
   | |_______________________________________^
note: ...which requires const checking `future::from_generator`...
  --> ../library/core/src/future/mod.rs:72:1
   |
72 | / pub const fn from_generator<T>(gen: T) -> impl Future<Output = T::Return>
73 | | where
74 | |     T: Generator<ResumeTy, Yield = ()>,
   | |_______________________________________^
   = note: ...which requires computing whether `impl future::future::Future` is freeze...
   = note: ...which requires evaluating trait selection obligation `impl future::future::Future: marker::Freeze`...
   = note: ...which again requires computing type of `future::from_generator::{opaque#0}`, completing the cycle
   = note: cycle used when privacy access levels
error[E0075]: SIMD vector cannot be empty
  --> ../library/core/src/../../portable-simd/crates/core_simd/src/vector.rs:89:1
   |
   |
89 | / pub struct Simd<T, const LANES: usize>([T; LANES])
90 | | where
91 | |     T: SimdElement,
92 | |     LaneCount<LANES>: SupportedLaneCount;

error[E0075]: SIMD vector cannot be empty
  --> ../library/core/src/../../portable-simd/crates/core_simd/src/vector/ptr.rs:11:1
   |
   |
11 | pub(crate) struct SimdConstPtr<T, const LANES: usize>([*const T; LANES]);

error[E0075]: SIMD vector cannot be empty
  --> ../library/core/src/../../portable-simd/crates/core_simd/src/vector/ptr.rs:43:1
   |
   |
43 | pub(crate) struct SimdMutPtr<T, const LANES: usize>([*mut T; LANES]);


thread 'rustc' panicked at 'assertion failed: fcx.closure_kind(self.closure_substs).is_some()', compiler/rustc_typeck/src/check/callee.rs:602:9
stack backtrace:
   0:     0x7f32a5701a2d - std::backtrace_rs::backtrace::libunwind::trace::hbbf74e73902aff9f
                               at /rustc/72cca280044a72eb76a39cef9abc6f294209e98c/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f32a5701a2d - std::backtrace_rs::backtrace::trace_unsynchronized::h01591ee22efedaf6
                               at /rustc/72cca280044a72eb76a39cef9abc6f294209e98c/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f32a5701a2d - std::sys_common::backtrace::_print_fmt::h1795ebb22bbee2ee
                               at /rustc/72cca280044a72eb76a39cef9abc6f294209e98c/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7f32a5701a2d - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hb9cf7eabdd31a8d0
                               at /rustc/72cca280044a72eb76a39cef9abc6f294209e98c/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7f32a575d7ec - core::fmt::write::h3a15d6fd0971fadb
                               at /rustc/72cca280044a72eb76a39cef9abc6f294209e98c/library/core/src/fmt/mod.rs:1196:17
   5:     0x7f32a56f31a1 - std::io::Write::write_fmt::hfc806ece52775b4e
                               at /rustc/72cca280044a72eb76a39cef9abc6f294209e98c/library/std/src/io/mod.rs:1654:15
   6:     0x7f32a5704745 - std::sys_common::backtrace::_print::hdf0f7b833ef65a10
                               at /rustc/72cca280044a72eb76a39cef9abc6f294209e98c/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7f32a5704745 - std::sys_common::backtrace::print::hb24b244c52a1ca52
                               at /rustc/72cca280044a72eb76a39cef9abc6f294209e98c/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7f32a5704745 - std::panicking::default_hook::{{closure}}::hd13082ece438defa
                               at /rustc/72cca280044a72eb76a39cef9abc6f294209e98c/library/std/src/panicking.rs:295:22
   9:     0x7f32a57043b9 - std::panicking::default_hook::h633a9e809b0f2dd6
                               at /rustc/72cca280044a72eb76a39cef9abc6f294209e98c/library/std/src/panicking.rs:314:9
  10:     0x7f32a6072f89 - rustc_driver[a6693949afa2cc1b]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7f32a5704f16 - std::panicking::rust_panic_with_hook::ha1365abbc2299723
                               at /rustc/72cca280044a72eb76a39cef9abc6f294209e98c/library/std/src/panicking.rs:702:17
  12:     0x7f32a5704cd9 - std::panicking::begin_panic_handler::{{closure}}::h0f863655355827bf
                               at /rustc/72cca280044a72eb76a39cef9abc6f294209e98c/library/std/src/panicking.rs:586:13
  13:     0x7f32a5701ee4 - std::sys_common::backtrace::__rust_end_short_backtrace::h83d3f854a15649b9
                               at /rustc/72cca280044a72eb76a39cef9abc6f294209e98c/library/std/src/sys_common/backtrace.rs:138:18
  14:     0x7f32a5704a49 - rust_begin_unwind
                               at /rustc/72cca280044a72eb76a39cef9abc6f294209e98c/library/std/src/panicking.rs:584:5
  15:     0x7f32a56ca253 - core::panicking::panic_fmt::h44e1dbb495973038
                               at /rustc/72cca280044a72eb76a39cef9abc6f294209e98c/library/core/src/panicking.rs:142:14
  16:     0x7f32a56ca11d - core::panicking::panic::hd044b63e7b13b00e
                               at /rustc/72cca280044a72eb76a39cef9abc6f294209e98c/library/core/src/panicking.rs:48:5
  17:     0x7f32a6a4b0fe - <rustc_typeck[a50b9990c2c6e4a1]::check::callee::DeferredCallResolution>::resolve
  18:     0x7f32a6861dae - <rustc_typeck[a50b9990c2c6e4a1]::check::fn_ctxt::FnCtxt>::analyze_closure
  19:     0x7f32a6975f5b - <rustc_typeck[a50b9990c2c6e4a1]::check::upvar::InferBorrowKindVisitor as rustc_hir[a072e090e77602f7]::intravisit::Visitor>::visit_expr
  20:     0x7f32a687ddba - rustc_hir[a072e090e77602f7]::intravisit::walk_expr::<rustc_typeck[a50b9990c2c6e4a1]::check::upvar::InferBorrowKindVisitor>
  21:     0x7f32a698f1ac - <rustc_infer[e588600b3236623b]::infer::InferCtxtBuilder>::enter::<&rustc_middle[b60bdadf064a2b58]::ty::context::TypeckResults, <rustc_typeck[a50b9990c2c6e4a1]::check::inherited::InheritedBuilder>::enter<rustc_typeck[a50b9990c2c6e4a1]::check::typeck_with_fallback<rustc_typeck[a50b9990c2c6e4a1]::check::typeck::{closure#0}>::{closure#1}, &rustc_middle[b60bdadf064a2b58]::ty::context::TypeckResults>::{closure#0}>
  22:     0x7f32a690f90e - rustc_typeck[a50b9990c2c6e4a1]::check::typeck
  23:     0x7f32a77806b0 - rustc_query_system[379613fc975868b6]::query::plumbing::try_execute_query::<rustc_query_impl[64a726a2aa0b8a45]::plumbing::QueryCtxt, rustc_query_system[379613fc975868b6]::query::caches::DefaultCache<rustc_span[c05af3ffaa3530b0]::def_id::LocalDefId, &rustc_middle[b60bdadf064a2b58]::ty::context::TypeckResults>>
  24:     0x7f32a786ba98 - rustc_query_system[379613fc975868b6]::query::plumbing::get_query::<rustc_query_impl[64a726a2aa0b8a45]::queries::typeck, rustc_query_impl[64a726a2aa0b8a45]::plumbing::QueryCtxt>
  25:     0x7f32a690fbd1 - rustc_typeck[a50b9990c2c6e4a1]::check::typeck
  26:     0x7f32a77806b0 - rustc_query_system[379613fc975868b6]::query::plumbing::try_execute_query::<rustc_query_impl[64a726a2aa0b8a45]::plumbing::QueryCtxt, rustc_query_system[379613fc975868b6]::query::caches::DefaultCache<rustc_span[c05af3ffaa3530b0]::def_id::LocalDefId, &rustc_middle[b60bdadf064a2b58]::ty::context::TypeckResults>>
  27:     0x7f32a786ba98 - rustc_query_system[379613fc975868b6]::query::plumbing::get_query::<rustc_query_impl[64a726a2aa0b8a45]::queries::typeck, rustc_query_impl[64a726a2aa0b8a45]::plumbing::QueryCtxt>
  28:     0x7f32a6a31ef8 - <rustc_middle[b60bdadf064a2b58]::hir::map::Map>::par_body_owners::<rustc_typeck[a50b9990c2c6e4a1]::check::typeck_item_bodies::{closure#0}>
  29:     0x7f32a691423d - rustc_typeck[a50b9990c2c6e4a1]::check::typeck_item_bodies
  30:     0x7f32a77e1401 - rustc_query_system[379613fc975868b6]::query::plumbing::try_execute_query::<rustc_query_impl[64a726a2aa0b8a45]::plumbing::QueryCtxt, rustc_query_system[379613fc975868b6]::query::caches::DefaultCache<(), ()>>
  31:     0x7f32a784874a - rustc_query_system[379613fc975868b6]::query::plumbing::get_query::<rustc_query_impl[64a726a2aa0b8a45]::queries::typeck_item_bodies, rustc_query_impl[64a726a2aa0b8a45]::plumbing::QueryCtxt>
  32:     0x7f32a69806c6 - <rustc_session[17d0aa654587095]::session::Session>::time::<(), rustc_typeck[a50b9990c2c6e4a1]::check_crate::{closure#7}>
  33:     0x7f32a697edf1 - rustc_typeck[a50b9990c2c6e4a1]::check_crate
  34:     0x7f32a6144c71 - rustc_interface[4c32b335d677b8ca]::passes::analysis
  35:     0x7f32a77d4a9e - rustc_query_system[379613fc975868b6]::query::plumbing::try_execute_query::<rustc_query_impl[64a726a2aa0b8a45]::plumbing::QueryCtxt, rustc_query_system[379613fc975868b6]::query::caches::DefaultCache<(), core[2c1a827a97cee9f]::result::Result<(), rustc_errors[7d17f6f76bcb8864]::ErrorGuaranteed>>>
  36:     0x7f32a786be27 - rustc_query_system[379613fc975868b6]::query::plumbing::get_query::<rustc_query_impl[64a726a2aa0b8a45]::queries::analysis, rustc_query_impl[64a726a2aa0b8a45]::plumbing::QueryCtxt>
  37:     0x7f32a602b60b - <rustc_interface[4c32b335d677b8ca]::passes::QueryContext>::enter::<rustc_driver[a6693949afa2cc1b]::run_compiler::{closure#1}::{closure#2}::{closure#3}, core[2c1a827a97cee9f]::result::Result<(), rustc_errors[7d17f6f76bcb8864]::ErrorGuaranteed>>
  38:     0x7f32a600bff6 - <rustc_interface[4c32b335d677b8ca]::interface::Compiler>::enter::<rustc_driver[a6693949afa2cc1b]::run_compiler::{closure#1}::{closure#2}, core[2c1a827a97cee9f]::result::Result<core[2c1a827a97cee9f]::option::Option<rustc_interface[4c32b335d677b8ca]::queries::Linker>, rustc_errors[7d17f6f76bcb8864]::ErrorGuaranteed>>
  39:     0x7f32a607a163 - rustc_span[c05af3ffaa3530b0]::with_source_map::<core[2c1a827a97cee9f]::result::Result<(), rustc_errors[7d17f6f76bcb8864]::ErrorGuaranteed>, rustc_interface[4c32b335d677b8ca]::interface::create_compiler_and_run<core[2c1a827a97cee9f]::result::Result<(), rustc_errors[7d17f6f76bcb8864]::ErrorGuaranteed>, rustc_driver[a6693949afa2cc1b]::run_compiler::{closure#1}>::{closure#1}>
  40:     0x7f32a600d3a1 - rustc_interface[4c32b335d677b8ca]::interface::create_compiler_and_run::<core[2c1a827a97cee9f]::result::Result<(), rustc_errors[7d17f6f76bcb8864]::ErrorGuaranteed>, rustc_driver[a6693949afa2cc1b]::run_compiler::{closure#1}>
  41:     0x7f32a6005ba2 - <scoped_tls[3c386a548e3942db]::ScopedKey<rustc_span[c05af3ffaa3530b0]::SessionGlobals>>::set::<rustc_interface[4c32b335d677b8ca]::interface::run_compiler<core[2c1a827a97cee9f]::result::Result<(), rustc_errors[7d17f6f76bcb8864]::ErrorGuaranteed>, rustc_driver[a6693949afa2cc1b]::run_compiler::{closure#1}>::{closure#0}, core[2c1a827a97cee9f]::result::Result<(), rustc_errors[7d17f6f76bcb8864]::ErrorGuaranteed>>
  42:     0x7f32a602ea8f - std[153a13fe3f5bf892]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[4c32b335d677b8ca]::util::run_in_thread_pool_with_globals<rustc_interface[4c32b335d677b8ca]::interface::run_compiler<core[2c1a827a97cee9f]::result::Result<(), rustc_errors[7d17f6f76bcb8864]::ErrorGuaranteed>, rustc_driver[a6693949afa2cc1b]::run_compiler::{closure#1}>::{closure#0}, core[2c1a827a97cee9f]::result::Result<(), rustc_errors[7d17f6f76bcb8864]::ErrorGuaranteed>>::{closure#0}, core[2c1a827a97cee9f]::result::Result<(), rustc_errors[7d17f6f76bcb8864]::ErrorGuaranteed>>
  43:     0x7f32a602ff39 - <<std[153a13fe3f5bf892]::thread::Builder>::spawn_unchecked_<rustc_interface[4c32b335d677b8ca]::util::run_in_thread_pool_with_globals<rustc_interface[4c32b335d677b8ca]::interface::run_compiler<core[2c1a827a97cee9f]::result::Result<(), rustc_errors[7d17f6f76bcb8864]::ErrorGuaranteed>, rustc_driver[a6693949afa2cc1b]::run_compiler::{closure#1}>::{closure#0}, core[2c1a827a97cee9f]::result::Result<(), rustc_errors[7d17f6f76bcb8864]::ErrorGuaranteed>>::{closure#0}, core[2c1a827a97cee9f]::result::Result<(), rustc_errors[7d17f6f76bcb8864]::ErrorGuaranteed>>::{closure#1} as core[2c1a827a97cee9f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  44:     0x7f32a570ee33 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hc387514206527460
                               at /rustc/72cca280044a72eb76a39cef9abc6f294209e98c/library/alloc/src/boxed.rs:1872:9
  45:     0x7f32a570ee33 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h4debba4b31c23733
                               at /rustc/72cca280044a72eb76a39cef9abc6f294209e98c/library/alloc/src/boxed.rs:1872:9
  46:     0x7f32a570ee33 - std::sys::unix::thread::Thread::new::thread_start::hab04ab46ed2c40fe
                               at /rustc/72cca280044a72eb76a39cef9abc6f294209e98c/library/std/src/sys/unix/thread.rs:108:17
  47:     0x7f32a524a8ca - start_thread
  48:     0x7f32a4da8abd - clone
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.62.0-nightly (72cca2800 2022-05-14) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib

query stack during panic:
#0 [typeck] type-checking `num::dec2flt::slow::parse_long_mantissa`
#1 [typeck] type-checking `num::dec2flt::slow::parse_long_mantissa::{closure#0}`
#2 [typeck_item_bodies] type-checking all item bodies
#3 [analysis] running analysis passes on this crate
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0075, E0391.
For more information about an error, try `rustc --explain E0075`.
