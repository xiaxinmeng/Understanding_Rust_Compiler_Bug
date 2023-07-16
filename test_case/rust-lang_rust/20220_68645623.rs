 console
thread 'rustc' panicked at 'assertion failed: did.krate != ast::LOCAL_CRATE', /Users/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-mac/build/src/librustc/middle/ty.rs:5429

stack backtrace:
   1:        0x112a5ea85 - sys::backtrace::write::h99bbbbef588b9184Klt
   2:        0x112a83f1f - failure::on_fail::h8b1c6fd57e4213d95nz
   3:        0x1129e895a - rt::unwind::begin_unwind_inner::hfd8d974c1cdeb74f35y
   4:        0x10fb41dfc - rt::unwind::begin_unwind::h9670423602147760625
   5:        0x10fec0a48 - middle::ty::lookup_trait_def::h0d278c058dac7da0bJ8
   6:        0x10f71a8bf - astconv::trait_defines_associated_type_named::h6e1f2adb924a7a3ed8r
   7:        0x10f729f14 - astconv::associated_path_def_to_ty::he58c9dd43523b1ady3r
   8:        0x10f7f5b58 - astconv::ast_ty_to_ty::unboxed_closure.36572
   9:        0x10f7f4217 - astconv::ast_ty_to_ty::h9692175568744562300
  10:        0x10f7f3889 - vec::Vec<T>.FromIterator<T>::from_iter::h2513327853763018693
  11:        0x10f81ca4b - astconv::instantiate_poly_trait_ref::h13560246163170871033
  12:        0x10f81c340 - vec::Vec<T>.FromIterator<T>::from_iter::h14680717809933419516
  13:        0x10f86855e - collect::conv_param_bounds::h18412785619248994413
  14:        0x10f897bbb - collect::ty_generics_for_trait::predicates_for_associated_types::unboxed_closure.38347
  15:        0x10f8975f1 - vec::Vec<T>.FromIterator<T>::from_iter::h5667342797167702362
  16:        0x10f86764e - collect::ty_generics_for_trait::h86232192f12b9d3cL6t
  17:        0x10f7d313a - collect::trait_def_of_item::h38b7e2843be79580FSt
  18:        0x10f7d0441 - collect::CollectTraitDefVisitor<'a, 'tcx>.visit..Visitor<'v>::visit_item::he1dd377f8daa9f9cm6s
  19:        0x10f8d00ff - check_crate::unboxed_closure.40064
  20:        0x10f8ce0eb - check_crate::h8fb6181edbba0f5bjsx
  21:        0x10f12d592 - driver::phase_3_run_analysis_passes::h80f5730bb84ad4dbTva
  22:        0x10f119f1c - driver::compile_input::h3940b6305f304e80wba
  23:        0x10f1e1853 - thunk::F.Invoke<A, R>::invoke::h10598202800189765697
  24:        0x10f1de9b0 - rt::unwind::try::try_fn::h2354011666368721707
  25:        0x112aec409 - rust_try_inner
  26:        0x112aec3f6 - rust_try
  27:        0x10f1df0f6 - thunk::F.Invoke<A, R>::invoke::h10531483554991743553
  28:        0x112a6fed4 - sys::thread::thread_start::h156b750c645e6d54M8v
  29:     0x7fff8115e899 - _pthread_body
  30:     0x7fff8115e72a - _pthread_struct_init
