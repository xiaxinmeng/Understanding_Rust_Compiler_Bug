
stack backtrace:
   1:     0x7fb8c61c0203 - sys::backtrace::write::hea6a478ec87b730abes
   2:     0x7fb8c61c7e79 - panicking::on_panic::ha6b7c58caaf8841eZXw
   3:     0x7fb8c6188cba - rt::unwind::begin_unwind_inner::h2ae5e480a4c3513aJDw
   4:     0x7fb8c34efccd - rt::unwind::begin_unwind::h16240856428398531644
   5:     0x7fb8c34efc62 - diagnostic::SpanHandler::span_bug::h5e9014915f0079c69zA
   6:     0x7fb8c363670f - ext::base::ExtCtxt<'a>::span_bug::h1a6219100e4d81eaCC4
   7:     0x7fb8c3635d4d - ext::tt::macro_rules::compile::hc7dffa7078d20fd98Kf
   8:     0x7fb8c3634b8d - ext::base::ExtCtxt<'a>::insert_macro::h47b26e4b5ab6a1f5oA4
   9:     0x7fb8c36b6b87 - ext::expand::expand_item_mac::hc6309738c3ed2a51YOa
  10:     0x7fb8c36a3f10 - ext::expand::expand_annotatable::hc3f4137dd80acb21pjb
  11:     0x7fb8c36a0458 - ext::expand::expand_item::h7287419e7b5065abWLa
  12:     0x7fb8c36ac8e5 - iter::FlatMap<I, U, F>.Iterator::next::h14892992294534447661
  13:     0x7fb8c36ab8ef - vec::Vec<T>.FromIterator<T>::from_iter::h11190267192129314975
  14:     0x7fb8c36ab08c - fold::noop_fold_mod::h2443530358008925113
  15:     0x7fb8c36a7074 - ext::expand::expand_item_underscore::h40d03d36b76ff11fxMa
  16:     0x7fb8c3709b1d - fold::Folder::fold_item_simple::h1367980113330025814
  17:     0x7fb8c3709754 - ptr::P<T>::map::h11615510399762603660
  18:     0x7fb8c36a4ec6 - ext::expand::expand_annotatable::hc3f4137dd80acb21pjb
  19:     0x7fb8c36a0458 - ext::expand::expand_item::h7287419e7b5065abWLa
  20:     0x7fb8c3713329 - ext::expand::expand_crate::h3d56761e96f983b7XKb
  21:     0x7fb8c675e7fd - driver::phase_2_configure_and_expand::closure.21001
  22:     0x7fb8c6714346 - driver::phase_2_configure_and_expand::hebd0cc9db1be58340sa
  23:     0x7fb8c67031ca - driver::compile_input::h71b7b5b5231ef90bSba
  24:     0x7fb8c67bb441 - run_compiler::h6ed36dfceb83c86dp6b
  25:     0x7fb8c67b8cd2 - boxed::F.FnBox<A>::call_box::h13171859894887546764
  26:     0x7fb8c67b84d9 - rt::unwind::try::try_fn::h2615272205539064887
  27:     0x7fb8c6241728 - rust_try_inner
  28:     0x7fb8c6241715 - rust_try
  29:     0x7fb8c61b3337 - rt::unwind::try::inner_try::hd64652dbe064f51aCzw
  30:     0x7fb8c67b86f9 - boxed::F.FnBox<A>::call_box::h16246532808992170908
  31:     0x7fb8c61c6b61 - sys::thread::Thread::new::thread_start::h6c585e5414c1e6e0OJv
  32:     0x7fb8c01fc1d9 - start_thread
  33:     0x7fb8c5e19d0c - clone
  34:                0x0 - <unknown>
