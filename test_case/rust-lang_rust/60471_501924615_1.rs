
error: internal compiler error: constant in type had an ignored error: TooGeneric
 --> bug.rs:5:1
  |
5 | / pub unsafe fn size_of_units<T: Sized>() -> [(); std::mem::size_of::<T>()] {
6 | |     [(); std::mem::size_of::<T>()]
7 | | }
  | |_^

error: internal compiler error: cat_expr Errd
 --> bug.rs:5:75
  |
5 |   pub unsafe fn size_of_units<T: Sized>() -> [(); std::mem::size_of::<T>()] {
  |  ___________________________________________________________________________^
6 | |     [(); std::mem::size_of::<T>()]
7 | | }
  | |_^

error: internal compiler error: cat_expr Errd
 --> bug.rs:6:5
  |
6 |     [(); std::mem::size_of::<T>()]
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: internal compiler error: QualifyAndPromoteConstants: MIR had errors
 --> bug.rs:5:1
  |
5 | / pub unsafe fn size_of_units<T: Sized>() -> [(); std::mem::size_of::<T>()] {
6 | |     [(); std::mem::size_of::<T>()]
7 | | }
  | |_^

error: internal compiler error: broken MIR in DefId(0:12 ~ bug[8787]::size_of_units[0]) ("return type"): bad type [type error]
 --> bug.rs:5:1
  |
5 | / pub unsafe fn size_of_units<T: Sized>() -> [(); std::mem::size_of::<T>()] {
6 | |     [(); std::mem::size_of::<T>()]
7 | | }
  | |_^

error: internal compiler error: broken MIR in DefId(0:12 ~ bug[8787]::size_of_units[0]) (LocalDecl { mutability: Mut, is_user_variable: None, internal: false, is_block_tail: None, ty: [type error], user_ty: UserTypeProjections { contents: [] }, name: None, source_info: SourceInfo { span: bug.rs:5:1: 7:2, scope: scope[0] }, visibility_scope: scope[0] }): bad type [type error]
 --> bug.rs:5:1
  |
5 | / pub unsafe fn size_of_units<T: Sized>() -> [(); std::mem::size_of::<T>()] {
6 | |     [(); std::mem::size_of::<T>()]
7 | | }
  | |_^

thread 'rustc' panicked at 'no errors encountered even though `delay_span_bug` issued', src/librustc_errors/lib.rs:362:17
stack backtrace:
   0:        0x106aec7f5 - std::sys_common::backtrace::print::hec9d18c92704ea4e
   1:        0x106af9217 - std::panicking::default_hook::{{closure}}::h0f3d5aca7e2dedd3
   2:        0x106af8f9b - std::panicking::default_hook::h8b614fd8b5161649
   3:        0x1047eea33 - rustc::util::common::panic_hook::h4038cc825e59a289
   4:        0x106af9912 - std::panicking::rust_panic_with_hook::hc4db8272f23d21fe
   5:        0x106750335 - std::panicking::begin_panic::hfcd1d79b5affa3d2
   6:        0x1067459f1 - <rustc_errors::Handler as core::ops::drop::Drop>::drop::hfae51df6b481bd4c
   7:        0x10045ef46 - core::ptr::real_drop_in_place::h885b1a61bfd60f80
   8:        0x100464f35 - <alloc::rc::Rc<T> as core::ops::drop::Drop>::drop::h0ac0677665f36c1c
   9:        0x10049f492 - core::ptr::real_drop_in_place::h5ef9b819f6ef30a7
  10:        0x10049a549 - rustc_interface::interface::run_compiler_in_existing_thread_pool::had730590df397043
  11:        0x10043c3c7 - std::thread::local::LocalKey<T>::with::h6c21b0ad4820d31b
  12:        0x100439246 - scoped_tls::ScopedKey<T>::set::h8427d787086b9670
  13:        0x1004f1585 - syntax::with_globals::h05f0ddf32b715ba3
  14:        0x10049328a - std::sys_common::backtrace::__rust_begin_short_backtrace::h94ac2988de47efec
  15:        0x106b014df - __rust_maybe_catch_panic
  16:        0x10048a489 - std::panicking::try::hd7392db478333fd3
  17:        0x10048b43c - core::ops::function::FnOnce::call_once{{vtable.shim}}::hf40a9128315d3f3a
  18:        0x106adbd3e - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h8c046589a84a9004
  19:        0x106afa67e - std::sys_common::thread::start_thread::he35092619128de54
  20:        0x106ad9fd9 - std::sys::unix::thread::Thread::new::thread_start::ha5ad007dbd212143
  21:     0x7fff9241e93b - _pthread_body
  22:     0x7fff9241e887 - _pthread_body
query stack during panic:
end of query stack
