
src/main.rs:3:37: 3:46 error: attempted access of field `Digit` on type `Character`, but no field with that name was found
src/main.rs:3     println!("The character is {}", ten.Digit);
                                                  ^~~~~~~~~
note: in expansion of format_args!
<std macros>:2:25: 2:58 note: expansion site
<std macros>:1:1: 2:62 note: in expansion of print!
<std macros>:3:1: 3:54 note: expansion site
<std macros>:1:1: 3:58 note: in expansion of println!
src/main.rs:3:5: 3:48 note: expansion site
error: internal compiler error: ID not mapped to struct fields: enum Character::Character (id=46)
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /Users/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-mac/build/src/libsyntax/diagnostic.rs:190

stack backtrace:
   1:        0x1070985fb - sys::backtrace::write::h93a3e34867af9d8946C
   2:        0x1070c42d8 - panicking::on_panic::hf4d2fc6c286c2686LWI
   3:        0x106fe01e9 - rt::unwind::begin_unwind_inner::h5fcff4a83d6257f8vFI
   4:        0x1067c6d3e - rt::unwind::begin_unwind::h17592283011135412256
   5:        0x1067c7569 - diagnostic::Handler::bug::h77b6b5d426a2844djgB
   6:        0x1041c0bfc - middle::ty::lookup_struct_fields::h445a114229c4fc0a4T5
   7:        0x10382adce - check::check_expr_with_unifier::suggest_field_names::h57c332ea88591095OGq
   8:        0x1037fd994 - check::check_expr_with_unifier::check_field::h7a7d3ac627d9e3eaIAq
   9:        0x10381906f - check::check_expr_with_unifier::h8136797732927255510
  10:        0x103814e38 - check::check_expr_with_unifier::h12757410945087278500
  11:        0x1037fbe93 - check::check_expr_with_unifier::closure.32755
  12:        0x10380525f - vec::Vec<T>.FromIterator<T>::from_iter::h7402513789693871901
  13:        0x103801209 - check::check_expr_with_unifier::h14283523106636836472
  14:        0x10371571c - check::_match::check_match::h163e8d573c8dc7f4Wbb
  15:        0x103818966 - check::check_expr_with_unifier::h8136797732927255510
  16:        0x1037ef4bc - check::check_expr_with_unifier::h7769900360174848749
  17:        0x1037c604c - check::check_argument_types::h53be59a031188521bAp
  18:        0x1037c26b5 - check::callee::confirm_builtin_call::hfbc3ca93d3eb2901rkm
  19:        0x1037c1052 - check::callee::check_call::h5815fd86705fa5aaTam
  20:        0x1037eca0d - check::check_expr_with_unifier::h7769900360174848749
  21:        0x1037c604c - check::check_argument_types::h53be59a031188521bAp
  22:        0x1037c26b5 - check::callee::confirm_builtin_call::hfbc3ca93d3eb2901rkm
  23:        0x1037c1052 - check::callee::check_call::h5815fd86705fa5aaTam
  24:        0x10381e597 - check::check_expr_with_unifier::h17674447472849350624
  25:        0x1037dba95 - check::check_block_with_expected::hf260d51b4579cdaar8r
  26:        0x1037bd555 - check::check_fn::hb963077cfe11dc0etsn
  27:        0x1037d83ef - check::check_bare_fn::h9573d4cb4fd54cc49hn
  28:        0x1037d0650 - check::check_item::h7a42ae765fa3c2fbNAn
  29:        0x1038a8962 - check_crate::closure.35967
  30:        0x1038a35ca - check_crate::h29f430cdae42ff74JhC
  31:        0x1035d3fe7 - driver::phase_3_run_analysis_passes::h1f59ceb3adbb8690sGa
  32:        0x1035ba896 - driver::compile_input::h559fc4baa5648f73Rba
  33:        0x10367b813 - run_compiler::hd536777f096765f9v2b
  34:        0x103679335 - thunk::F.Invoke<A, R>::invoke::h7317324818701278712
  35:        0x1036786f7 - rt::unwind::try::try_fn::h13709138315065660050
  36:        0x107148d28 - rust_try_inner
  37:        0x107148d15 - rust_try
  38:        0x103678a95 - thunk::F.Invoke<A, R>::invoke::h1207719554578523009
  39:        0x1070aed8d - sys::thread::create::thread_start::h0340cd55836306dfkDH
  40:     0x7fff951ed267 - _pthread_body
  41:     0x7fff951ed1e4 - _pthread_start

Could not compile `rust_beginner`.
