
error: internal compiler error: no errors encountered even though `delay_span_bug` issued

error: internal compiler error: unexpected const parent in type_of(): TypeBinding(TypeBinding { hir_id: HirId { owner: OwnerId { def_id: DefId(0:4 ~ playground[38d5]::T::A) }, local_id: 3 }, ident: X#0, gen_args: GenericArgs { args: [], bindings: [], parenthesized: false, span_ext: no-location (#0) }, kind: Equality { term: Const(AnonConst { hir_id: HirId { owner: OwnerId { def_id: DefId(0:4 ~ playground[38d5]::T::A) }, local_id: 1 }, body: BodyId { hir_id: HirId { owner: OwnerId { def_id: DefId(0:4 ~ playground[38d5]::T::A) }, local_id: 2 } } }) }, span: src/lib.rs:5:17: 5:25 (#0) })
  |
  = note: delayed at compiler/rustc_typeck/src/collect/type_of.rs:456:23

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler/rustc_typeck/src/check/coercion.rs:176:49

error: internal compiler error: `InferCtxt` incorrectly tainted by errors
  |
  = note: delayed at compiler/rustc_infer/src/infer/mod.rs:1282:27

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler/rustc_mir_build/src/build/mod.rs:627:18

error: internal compiler error: mir_const_qualif: MIR had errors
 --> src/lib.rs:5:21
  |
5 |     type A: S<C<X = 0i32> = 34>;
  |                     ^^^^
  |
  = note: delayed at compiler/rustc_mir_transform/src/lib.rs:252:18

error: internal compiler error: PromoteTemps: MIR had errors
 --> src/lib.rs:5:21
  |
5 |     type A: S<C<X = 0i32> = 34>;
  |                     ^^^^
  |
  = note: delayed at compiler/rustc_const_eval/src/transform/promote_consts.rs:53:22

error: internal compiler error: broken MIR in DefId(0:5 ~ playground[38d5]::T::A::{constant#0}) ("return type"): bad type [type error]
 --> src/lib.rs:5:21
  |
5 |     type A: S<C<X = 0i32> = 34>;
  |                     ^^^^
  |
  = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:523:13

error: internal compiler error: TyKind::Error constructed but no error reported
  |
  = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:791:20

error: internal compiler error: broken MIR in DefId(0:5 ~ playground[38d5]::T::A::{constant#0}) (LocalDecl { mutability: Mut, local_info: None, internal: false, is_block_tail: None, ty: [type error], user_ty: None, source_info: SourceInfo { span: src/lib.rs:5:21: 5:25 (#0), scope: scope[0] } }): bad type [type error]
 --> src/lib.rs:5:21
  |
5 |     type A: S<C<X = 0i32> = 34>;
  |                     ^^^^
  |
  = note: delayed at compiler/rustc_borrowck/src/type_check/mod.rs:523:13

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1530:13
stack backtrace:
   0:     0x7f54d15e9170 - std::backtrace_rs::backtrace::libunwind::trace::heafe4e76b4fd4cac
                               at /rustc/f5193a9fcc73dc09e41a90c5a2c97fc9acc16032/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   1:     0x7f54d15e9170 - std::backtrace_rs::backtrace::trace_unsynchronized::h19321be3e640ae38
                               at /rustc/f5193a9fcc73dc09e41a90c5a2c97fc9acc16032/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f54d15e9170 - std::sys_common::backtrace::_print_fmt::h2fcd792e338f1cc5
                               at /rustc/f5193a9fcc73dc09e41a90c5a2c97fc9acc16032/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7f54d15e9170 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::he5b28f0056e90af1
                               at /rustc/f5193a9fcc73dc09e41a90c5a2c97fc9acc16032/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7f54d1643e2e - core::fmt::write::h8cee4989ef91009c
                               at /rustc/f5193a9fcc73dc09e41a90c5a2c97fc9acc16032/library/core/src/fmt/mod.rs:1202:17
   5:     0x7f54d15d9db5 - std::io::Write::write_fmt::h3fb743e9e09d846d
                               at /rustc/f5193a9fcc73dc09e41a90c5a2c97fc9acc16032/library/std/src/io/mod.rs:1679:15
   6:     0x7f54d15ebef3 - std::sys_common::backtrace::_print::h751a1d130f5db246
                               at /rustc/f5193a9fcc73dc09e41a90c5a2c97fc9acc16032/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7f54d15ebef3 - std::sys_common::backtrace::print::h42eee5f5e3400273
                               at /rustc/f5193a9fcc73dc09e41a90c5a2c97fc9acc16032/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7f54d15ebef3 - std::panicking::default_hook::{{closure}}::hced954fcffb6800d
                               at /rustc/f5193a9fcc73dc09e41a90c5a2c97fc9acc16032/library/std/src/panicking.rs:267:22
   9:     0x7f54d15ebbdf - std::panicking::default_hook::hbc950bafaf526371
                               at /rustc/f5193a9fcc73dc09e41a90c5a2c97fc9acc16032/library/std/src/panicking.rs:286:9
  10:     0x7f54d3e04a21 - <rustc_driver[bfc58f2d3d03ea26]::DEFAULT_HOOK::{closure#0}::{closure#0} as core[7246776d9d7acd71]::ops::function::FnOnce<(&core[7246776d9d7acd71]::panic::panic_info::PanicInfo,)>>::call_once::{shim:vtable#0}
  11:     0x7f54d15ec72b - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::hc100ed610fe9f17d
                               at /rustc/f5193a9fcc73dc09e41a90c5a2c97fc9acc16032/library/alloc/src/boxed.rs:1954:9
  12:     0x7f54d15ec72b - std::panicking::rust_panic_with_hook::ha913975239a26952
                               at /rustc/f5193a9fcc73dc09e41a90c5a2c97fc9acc16032/library/std/src/panicking.rs:673:13
  13:     0x7f54d4f14911 - std[651c5132bc9a8dcc]::panicking::begin_panic::<rustc_errors[d0cc166540f61a28]::ExplicitBug>::{closure#0}
  14:     0x7f54d4f142e6 - std[651c5132bc9a8dcc]::sys_common::backtrace::__rust_end_short_backtrace::<std[651c5132bc9a8dcc]::panicking::begin_panic<rustc_errors[d0cc166540f61a28]::ExplicitBug>::{closure#0}, !>
  15:     0x7f54d4f13f86 - std[651c5132bc9a8dcc]::panicking::begin_panic::<rustc_errors[d0cc166540f61a28]::ExplicitBug>
  16:     0x7f54d4f11ce6 - std[651c5132bc9a8dcc]::panic::panic_any::<rustc_errors[d0cc166540f61a28]::ExplicitBug>
  17:     0x7f54d395f7d8 - <rustc_errors[d0cc166540f61a28]::HandlerInner as core[7246776d9d7acd71]::ops::drop::Drop>::drop
  18:     0x7f54d2c0c948 - core[7246776d9d7acd71]::ptr::drop_in_place::<rustc_session[24bfe84e1f2672f9]::parse::ParseSess>
  19:     0x7f54d2bca1d8 - <alloc[f7d3967a3dbd7407]::rc::Rc<rustc_session[24bfe84e1f2672f9]::session::Session> as core[7246776d9d7acd71]::ops::drop::Drop>::drop
  20:     0x7f54d2bc725d - core[7246776d9d7acd71]::ptr::drop_in_place::<rustc_interface[8f9ce55010e218f1]::interface::Compiler>
  21:     0x7f54d2bc49dc - rustc_interface[8f9ce55010e218f1]::interface::create_compiler_and_run::<core[7246776d9d7acd71]::result::Result<(), rustc_errors[d0cc166540f61a28]::ErrorGuaranteed>, rustc_driver[bfc58f2d3d03ea26]::run_compiler::{closure#1}>
  22:     0x7f54d2bc2ee1 - <scoped_tls[d826b3ad62b639a0]::ScopedKey<rustc_span[761cb8bfe4b7bf0c]::SessionGlobals>>::set::<rustc_interface[8f9ce55010e218f1]::interface::run_compiler<core[7246776d9d7acd71]::result::Result<(), rustc_errors[d0cc166540f61a28]::ErrorGuaranteed>, rustc_driver[bfc58f2d3d03ea26]::run_compiler::{closure#1}>::{closure#0}, core[7246776d9d7acd71]::result::Result<(), rustc_errors[d0cc166540f61a28]::ErrorGuaranteed>>
  23:     0x7f54d2bc2bcf - std[651c5132bc9a8dcc]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[8f9ce55010e218f1]::util::run_in_thread_pool_with_globals<rustc_interface[8f9ce55010e218f1]::interface::run_compiler<core[7246776d9d7acd71]::result::Result<(), rustc_errors[d0cc166540f61a28]::ErrorGuaranteed>, rustc_driver[bfc58f2d3d03ea26]::run_compiler::{closure#1}>::{closure#0}, core[7246776d9d7acd71]::result::Result<(), rustc_errors[d0cc166540f61a28]::ErrorGuaranteed>>::{closure#0}, core[7246776d9d7acd71]::result::Result<(), rustc_errors[d0cc166540f61a28]::ErrorGuaranteed>>
  24:     0x7f54d3c64790 - <<std[651c5132bc9a8dcc]::thread::Builder>::spawn_unchecked_<rustc_interface[8f9ce55010e218f1]::util::run_in_thread_pool_with_globals<rustc_interface[8f9ce55010e218f1]::interface::run_compiler<core[7246776d9d7acd71]::result::Result<(), rustc_errors[d0cc166540f61a28]::ErrorGuaranteed>, rustc_driver[bfc58f2d3d03ea26]::run_compiler::{closure#1}>::{closure#0}, core[7246776d9d7acd71]::result::Result<(), rustc_errors[d0cc166540f61a28]::ErrorGuaranteed>>::{closure#0}, core[7246776d9d7acd71]::result::Result<(), rustc_errors[d0cc166540f61a28]::ErrorGuaranteed>>::{closure#1} as core[7246776d9d7acd71]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  25:     0x7f54d15f6393 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hbba34111c230ef2f
                               at /rustc/f5193a9fcc73dc09e41a90c5a2c97fc9acc16032/library/alloc/src/boxed.rs:1940:9
  26:     0x7f54d15f6393 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h6b157af76f91c602
                               at /rustc/f5193a9fcc73dc09e41a90c5a2c97fc9acc16032/library/alloc/src/boxed.rs:1940:9
  27:     0x7f54d15f6393 - std::sys::unix::thread::Thread::new::thread_start::h8b871f8c8d25fce2
                               at /rustc/f5193a9fcc73dc09e41a90c5a2c97fc9acc16032/library/std/src/sys/unix/thread.rs:108:17
  28:     0x7f54d14c9609 - start_thread
  29:     0x7f54d13ec133 - clone
  30:                0x0 - <unknown>

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.66.0-nightly (f5193a9fc 2022-09-25) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C embed-bitcode=no -C codegen-units=1 -C debuginfo=2

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
error: could not compile `playground`
