
stack backtrace:
   1:      0x39b62d1d5c9 - sys::backtrace::write::h350af8200010e467Z5r
   2:      0x39b62d25519 - panicking::on_panic::h4194b7c3f2c96e6a0Kw
   3:      0x39b62ce6212 - rt::unwind::begin_unwind_inner::hf945ded7ea7b4f57aqw
   4:      0x39b600c007d - rt::unwind::begin_unwind::h8796533312692526900
   5:      0x39b600c0012 - diagnostic::SpanHandler::span_bug::h7e9f54619e37e35df6B
   6:      0x39b601e2aef - ext::base::ExtCtxt<'a>::span_bug::h3dbcfa2d3363d87d315
   7:      0x39b601e256d - ext::tt::macro_rules::compile::hfcaa92a61bc2de27nah
   8:      0x39b601e19b7 - ext::base::ExtCtxt<'a>::insert_macro::ha3b4d3656c5b76a6PZ5
   9:      0x39b6025d912 - ext::expand::expand_item_mac::h1509ced45421c5b7cdc
  10:      0x39b6024f17b - ext::expand::expand_annotatable::h710e8d3534ffdf60EHc
  11:      0x39b6024bbee - ext::expand::expand_item::h74a5f04b07d79357cac
  12:      0x39b602564c3 - iter::FlatMap<I, U, F>.Iterator::next::h10849965593071494453
  13:      0x39b60256106 - vec::Vec<T>.FromIterator<T>::from_iter::h18311000090818088571
  14:      0x39b60251b72 - ext::expand::expand_item_underscore::he59e09d51a1a979cNac
  15:      0x39b602a4b95 - fold::noop_fold_item_simple::h6111943909848966401
  16:      0x39b602a499a - ptr::P<T>::map::h7073218183394239229
  17:      0x39b60250435 - ext::expand::expand_annotatable::h710e8d3534ffdf60EHc
  18:      0x39b6024bbee - ext::expand::expand_item::h74a5f04b07d79357cac
  19:      0x39b602ac27f - ext::expand::expand_crate::h020498b3ef52907ca9c
  20:      0x39b632a986d - driver::phase_2_configure_and_expand::closure.20415
  21:      0x39b632601d9 - driver::phase_2_configure_and_expand::h10040db46cc64371Ysa
  22:      0x39b6324ff1a - driver::compile_input::hd3dd730970eb45c2Qba
  23:      0x39b633071b1 - run_compiler::h4be6ca332f13312675b
  24:      0x39b63304a02 - boxed::F.FnBox<A>::call_box::h7925108573723512372
  25:      0x39b63303fc9 - rt::unwind::try::try_fn::h5378213576640735239
  26:      0x39b62d9dc28 - rust_try_inner
  27:      0x39b62d9dc15 - rust_try
  28:      0x39b63304264 - boxed::F.FnBox<A>::call_box::h12991688188475136491
  29:      0x39b62d242b1 - sys::thread::Thread::new::thread_start::h750ed153b2916909hxv
  30:      0x39b5cdc0373 - start_thread
  31:      0x39b6297627c - clone
  32:                0x0 - <unknown>
