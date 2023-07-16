
    Compiling gl_generator v0.0.1 (https://github.com/bjz/gl-rs#678346eb)
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
task 'rustc' panicked at 'quote! with interpolated token', /home/rustbuild/src/rust-buildbot/slave/nightly-linux/build/src/libsyntax/ext/quote.rs:633

stack backtrace:
   1:     0x7f47ba80c860 - rt::backtrace::imp::write::h6962a9f035a3164aFaq
   2:     0x7f47ba80f8e0 - failure::on_fail::h88b5054949466854gwq
   3:     0x7f47bed55c70 - unwind::begin_unwind_inner::h956a67c80425b5677Rd
   4:     0x7f47bded1480 - unwind::begin_unwind::h9893951301925131742
   5:     0x7f47be128080 - ext::quote::mk_token::hc6ac50e71cba3aa4hfh
   6:     0x7f47be129970 - ext::quote::mk_tt::hb0bd798dda9f1f7ccqh
   7:     0x7f47be12b590 - ext::quote::mk_tt::closure.56433
   8:     0x7f47be12b2b0 - iter::FlatMap<'a, A, T, U>.Iterator<B>::next::h9285657917981752866
   9:     0x7f47be129970 - ext::quote::mk_tt::hb0bd798dda9f1f7ccqh
  10:     0x7f47be12b590 - ext::quote::mk_tt::closure.56433
  11:     0x7f47be12b2b0 - iter::FlatMap<'a, A, T, U>.Iterator<B>::next::h9285657917981752866
  12:     0x7f47be129970 - ext::quote::mk_tt::hb0bd798dda9f1f7ccqh
  13:     0x7f47be125440 - ext::quote::expand_tts::h26b9fb72d181c7a5Hvh
  14:     0x7f47be126b20 - ext::quote::expand_parse_call::hf87863a12802998agAh
  15:     0x7f47be0490e0 - ext::quote::expand_quote_item::h913b58f924867343m4g
  16:     0x7f47be0403c0 - ext::base::MacroExpanderFn.TTMacroExpander::expand::h2531c9a5a5c4a146VP6
  17:     0x7f47be09bf00 - ext::expand::expand_mac_invoc::h5070259855386140
  18:     0x7f47be098d20 - ext::expand::expand_expr::hd06339d50d4e3518pvd
  19:     0x7f47be09dd40 - fold::noop_fold_expr::h17508862488002781673
  20:     0x7f47be098d20 - ext::expand::expand_expr::hd06339d50d4e3518pvd
  21:     0x7f47be0a1cb0 - fold::noop_fold_expr::closure.54816
  22:     0x7f47bdf6ac80 - fold::Vec<T>.MoveMap<T>::move_map::h2103895945899500191
  23:     0x7f47be09dd40 - fold::noop_fold_expr::h17508862488002781673
  24:     0x7f47be098d20 - ext::expand::expand_expr::hd06339d50d4e3518pvd
  25:     0x7f47be0a1cb0 - fold::noop_fold_expr::closure.54816
  26:     0x7f47bdf6ac80 - fold::Vec<T>.MoveMap<T>::move_map::h2103895945899500191
  27:     0x7f47be09dd40 - fold::noop_fold_expr::h17508862488002781673
  28:     0x7f47be098d20 - ext::expand::expand_expr::hd06339d50d4e3518pvd
  29:     0x7f47be09dd40 - fold::noop_fold_expr::h17508862488002781673
  30:     0x7f47be098d20 - ext::expand::expand_expr::hd06339d50d4e3518pvd
  31:     0x7f47be0b8700 - ext::expand::expand_non_macro_stmt::closure.55042
  32:     0x7f47bdf592d0 - ptr::P<T>::map::h17045300039264601048
  33:     0x7f47be0b6310 - ext::expand::expand_stmt::h415289a49dd1653cK8d
  34:     0x7f47be0b81b0 - ext::expand::MacroExpander<'a, 'b>.Folder::fold_stmt::h5a37c5b35924fca9aBe
  35:     0x7f47be0d2c20 - ext::expand::expand_block_elts::closure.55281
  36:     0x7f47bdf64f50 - iter::Iterator::collect::h7500869249246266630
  37:     0x7f47be0d1e10 - ext::expand::expand_block_elts::closure.55274
  38:     0x7f47bdf641f0 - ptr::P<T>::map::h9226135620174701605
  39:     0x7f47be0d1cd0 - ext::expand::expand_block::hd4fa4bc2262a7bf2Eie
  40:     0x7f47be09dd40 - fold::noop_fold_expr::h17508862488002781673
  41:     0x7f47be098d20 - ext::expand::expand_expr::hd06339d50d4e3518pvd
  42:     0x7f47be098d20 - ext::expand::expand_expr::hd06339d50d4e3518pvd
  43:     0x7f47be0d1e10 - ext::expand::expand_block_elts::closure.55274
  44:     0x7f47bdf641f0 - ptr::P<T>::map::h9226135620174701605
  45:     0x7f47be0d1cd0 - ext::expand::expand_block::hd4fa4bc2262a7bf2Eie
  46:     0x7f47be0a5fb0 - ext::expand::expand_and_rename_fn_decl_and_block::h765a4b29f62668726xe
  47:     0x7f47be0ab8a0 - ext::expand::MacroExpander<'a, 'b>.Folder::fold_item_underscore::had677d0d3e6000edUAe
  48:     0x7f47be0ab3c0 - fold::noop_fold_item::closure.54914
  49:     0x7f47bdf55fa0 - ptr::P<T>::map::h379406011177471474
  50:     0x7f47be0a6570 - ext::expand::expand_item::hba76819e858c6d563Pd
  51:     0x7f47be0b19d0 - fold::noop_fold_mod::closure.54943
  52:     0x7f47bdff1ea0 - iter::Iterator::collect::h1493905177343541632
  53:     0x7f47be0b0850 - fold::Folder::fold_mod::h13213550518250326075
  54:     0x7f47be0ab8a0 - ext::expand::MacroExpander<'a, 'b>.Folder::fold_item_underscore::had677d0d3e6000edUAe
  55:     0x7f47be0ab3c0 - fold::noop_fold_item::closure.54914
  56:     0x7f47bdf55fa0 - ptr::P<T>::map::h379406011177471474
  57:     0x7f47be0a6570 - ext::expand::expand_item::hba76819e858c6d563Pd
  58:     0x7f47be0b19d0 - fold::noop_fold_mod::closure.54943
  59:     0x7f47bdff1ea0 - iter::Iterator::collect::h1493905177343541632
  60:     0x7f47be0b0850 - fold::Folder::fold_mod::h13213550518250326075
  61:     0x7f47be0ab8a0 - ext::expand::MacroExpander<'a, 'b>.Folder::fold_item_underscore::had677d0d3e6000edUAe
  62:     0x7f47be0ab3c0 - fold::noop_fold_item::closure.54914
  63:     0x7f47bdf55fa0 - ptr::P<T>::map::h379406011177471474
  64:     0x7f47be0a6570 - ext::expand::expand_item::hba76819e858c6d563Pd
  65:     0x7f47be0b19d0 - fold::noop_fold_mod::closure.54943
  66:     0x7f47bdff1ea0 - iter::Iterator::collect::h1493905177343541632
  67:     0x7f47be0b0850 - fold::Folder::fold_mod::h13213550518250326075
  68:     0x7f47be0ef220 - ext::expand::expand_crate::h848537a1a598e0e20De
  69:     0x7f47bfc527a0 - driver::driver::phase_2_configure_and_expand::closure.145021
  70:     0x7f47bf9be260 - driver::driver::phase_2_configure_and_expand::h022162699ec14c3aJcC
  71:     0x7f47bfc10f60 - driver::driver::compile_input::h9c830efbfaf34c7aP5B
  72:     0x7f47bfc93700 - driver::run_compiler::h4d729096c46ed8f76VF
  73:     0x7f47bfc935f0 - driver::run::closure.146431
  74:     0x7f47bf3cfb60 - task::TaskBuilder<S>::try_future::closure.104846
  75:     0x7f47bf3cf950 - task::TaskBuilder<S>::spawn_internal::closure.104817
  76:     0x7f47bf09c530 - task::NativeSpawner.Spawner::spawn::closure.8448
  77:     0x7f47bedaab70 - rust_try_inner
  78:     0x7f47bedaab60 - rust_try
  79:     0x7f47bed535f0 - unwind::try::h51d29a2dceb13095PGd
  80:     0x7f47bed53480 - task::Task::run::h8c90cb04437a131dFMc
  81:     0x7f47bf09c270 - task::NativeSpawner.Spawner::spawn::closure.8386
  82:     0x7f47bed54c90 - thread::thread_start::h7bbfc46247c32113U7c
  83:     0x7f47b9bbe250 - start_thread
  84:     0x7f47bea2f3b9 - clone
  85:                0x0 - <unknown>
