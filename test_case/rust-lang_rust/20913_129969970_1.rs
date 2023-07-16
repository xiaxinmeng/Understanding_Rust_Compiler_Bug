
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
thread 'rustc' panicked at 'index out of bounds: the len is 60 but the index is 66', ../src/libcollections/vec.rs:1263

stack backtrace:
   1:        0x10bb99b10 - sys::backtrace::write::hd55ab9ea9b602d10yws
   2:        0x10bba1d51 - panicking::on_panic::hd39d541ff0f0a91dzax
   3:        0x10bb65102 - rt::unwind::begin_unwind_inner::hfeb8eb2381958d15AFw
   4:        0x10bb65cf7 - rt::unwind::begin_unwind_fmt::hc97fa885aeb052d8GEw
   5:        0x10bba1947 - rust_begin_unwind
   6:        0x10bbed5f0 - panicking::panic_fmt::hed2a91b19970d30dqgC
   7:        0x10bbe89e0 - panicking::panic_bounds_check::h4b850c152f83bdddwfC
   8:        0x110f3be80 - parse::token::InternedString::new_from_name::h6ac50891a6d1538cqtT
   9:        0x110ffad9c - parse::parser::Parser<'a>::lit_from_token::h81e0338f54cfe402UVG
  10:        0x110fff3f4 - parse::parser::Parser<'a>::parse_lit::h2d3585ebb29b6419YZG
  11:        0x111005928 - parse::parser::Parser<'a>::parse_bottom_expr::h9eb11589d01302ca5uH
  12:        0x111009acd - parse::parser::Parser<'a>::parse_dot_or_call_expr::h7ea24d0d85e36381EYH
  13:        0x11100de7f - parse::parser::Parser<'a>::parse_prefix_expr::hb693898eab38ecadRrI
  14:        0x11100ee0d - parse::parser::Parser<'a>::parse_binops::h6468f59fda6455faICI
  15:        0x11100f80e - parse::parser::Parser<'a>::parse_assign_expr::h22b387af6ce7d486AII
  16:        0x110fdcbfc - parse::parser::Parser<'a>::parse_expr::ha1d1a094a9e8bc35ycF
  17:        0x10d4050b5 - parse::he2454624aa8950b9pfe
  18:        0x10d40487c - native::hf8fe1f594d820587Oaa
  19:        0x1109b8079 - ext::base::F.TTMacroExpander::expand::h1156222205761249190
  20:        0x1095bba8d - ext::expand::expand_expr::closure.67689
  21:        0x1095b5945 - ext::expand::expand_expr::he7f631e2a6756354Qsb
  22:        0x1095ce61d - ext::expand::expand_item_underscore::hf4ad7058627d3835mZb
  23:        0x109621cea - fold::noop_fold_item_simple::h12659161660526533640
  24:        0x109621879 - fold::noop_fold_item::h10847006839623481285
  25:        0x1095cbf77 - ext::expand::expand_annotatable::h5607d1f88b990a30Tvc
  26:        0x1095c78ec - ext::expand::expand_item::h055bf60ae8164737LYb
  27:        0x1095d4435 - iter::FlatMap<I, U, F>.Iterator::next::h12186547700541527931
  28:        0x1095d3f6c - vec::Vec<T>.FromIterator<T>::from_iter::h1052554477831759431
  29:        0x1095d3c9a - fold::noop_fold_mod::h6045941732705183073
  30:        0x1095cea76 - ext::expand::expand_item_underscore::hf4ad7058627d3835mZb
  31:        0x109621cea - fold::noop_fold_item_simple::h12659161660526533640
  32:        0x109621879 - fold::noop_fold_item::h10847006839623481285
  33:        0x1095ccbad - ext::expand::expand_annotatable::h5607d1f88b990a30Tvc
  34:        0x1095c78ec - ext::expand::expand_item::h055bf60ae8164737LYb
  35:        0x10962bdd0 - ext::expand::expand_crate::h79ff848875ed4ca0KZc
  36:        0x1080b60d3 - driver::phase_2_configure_and_expand::closure.26217
  37:        0x10806bda0 - driver::phase_2_configure_and_expand::h7db94f2f6299002cita
  38:        0x10805cafb - driver::compile_input::h2dcf2542220faeb3Tba
  39:        0x10813b09e - run_compiler::h4f62e6aa9c953a9fC7b
  40:        0x108138c8b - boxed::F.FnBox<A>::call_box::h16801768484572870575
  41:        0x1081385e2 - rt::unwind::try::try_fn::h15641500112529980395
  42:        0x10bba18f8 - __rust_try
  43:        0x10bb8da60 - rt::unwind::try::inner_try::h919d558e288762e9tBw
  44:        0x1081387a2 - boxed::F.FnBox<A>::call_box::h12543525262198032502
  45:        0x10bba0bad - sys::thread::Thread::new::thread_start::hb70673fe9975221bOZv
  46:     0x7fff8dd3a267 - _pthread_body
  47:     0x7fff8dd3a1e4 - _pthread_start

Could not compile `hello_world`.
