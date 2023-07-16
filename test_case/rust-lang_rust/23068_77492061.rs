
       Fresh bitflags v0.1.1
       Fresh gcc v0.3.1
       Fresh log v0.2.5
       Fresh rustc-serialize v0.2.15
       Fresh libc v0.1.2
       Fresh sdl2-sys v0.0.27 (https://github.com/quix00/rust-sdl2?branch=texture_lifetimes#fcea2ab7)
       Fresh rand v0.1.3
       Fresh sdl2 v0.0.28 (https://github.com/quix00/rust-sdl2?branch=texture_lifetimes#fcea2ab7)
       Fresh time v0.1.19
       Fresh sdl2_ttf v0.0.3 (https://github.com/quix00/rust-sdl2_ttf#5027f1ab)
   Compiling ecf v0.0.1 (file:///Users/eesponda/Documents/ecf)
     Running `rustc src/lib.rs --crate-name ecf --crate-type lib -g -C metadata=e844eaf30c8e1267 -C extra-filename=-e844eaf30c8e1267 --out-dir /Users/eesponda/Documents/ecf/target --emit=dep-info,link -L dependency=/Users/eesponda/Documents/ecf/target -L dependency=/Users/eesponda/Documents/ecf/target/deps --extern sdl2_ttf=/Users/eesponda/Documents/ecf/target/deps/libsdl2_ttf-2865844f4a4f80d2.rlib --extern time=/Users/eesponda/Documents/ecf/target/deps/libtime-f5c2f62bb1bdf976.rlib --extern rustc-serialize=/Users/eesponda/Documents/ecf/target/deps/librustc-serialize-ebc743271f9255aa.rlib --extern sdl2=/Users/eesponda/Documents/ecf/target/deps/libsdl2-b9ebff5a768e0711.rlib -L native=/Users/eesponda/Documents/ecf/target/build/time-f5c2f62bb1bdf976/out`
     Running `rustc examples/pong/main.rs --crate-name pong --crate-type bin -g --out-dir /Users/eesponda/Documents/ecf/target --emit=dep-info,link -L dependency=/Users/eesponda/Documents/ecf/target -L dependency=/Users/eesponda/Documents/ecf/target/deps --extern sdl2_ttf=/Users/eesponda/Documents/ecf/target/deps/libsdl2_ttf-2865844f4a4f80d2.rlib --extern time=/Users/eesponda/Documents/ecf/target/deps/libtime-f5c2f62bb1bdf976.rlib --extern rustc-serialize=/Users/eesponda/Documents/ecf/target/deps/librustc-serialize-ebc743271f9255aa.rlib --extern sdl2=/Users/eesponda/Documents/ecf/target/deps/libsdl2-b9ebff5a768e0711.rlib --extern ecf=/Users/eesponda/Documents/ecf/target/libecf-e844eaf30c8e1267.rlib -L native=/Users/eesponda/Documents/ecf/target/build/time-f5c2f62bb1bdf976/out`
thread 'rustc' panicked at 'Box<Any>', /Users/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-mac/build/src/libsyntax/diagnostic.rs:129

stack backtrace:
   1:        0x111f6db52 - sys::backtrace::write::h6b0889bc971c1ad3IDA
   2:        0x111f9c844 - panicking::on_panic::h0fcd1d74630df38dKsJ
   3:        0x111ebac67 - rt::unwind::begin_unwind_inner::h56b969a14fc61916gbJ
   4:        0x11164195e - rt::unwind::begin_unwind::h7097931347514503270
   5:        0x11164190b - diagnostic::SpanHandler::span_bug::hd809ae68b3d05ee6h0D
   6:        0x10ee19d7c - session::Session::span_bug::h5bc4aa3cff230a98ISp
   7:        0x10eb3b134 - trans::debuginfo::scope_metadata::hbc8249b53299cdbdSjE
   8:        0x10ea45fb9 - trans::debuginfo::set_source_location::h27a07f90a8aeec578MD
   9:        0x10e9f7925 - trans::expr::trans_into::h4a29f680db4b3526znh
  10:        0x10e9f7c45 - trans::expr::trans_into::h4a29f680db4b3526znh
  11:        0x10ea5e2c0 - trans::expr::trans_uniq_expr::h9953b7ba5686a321ukj
  12:        0x10ea5f053 - trans::expr::trans_unary::h112ae451f6aff215Jgj
  13:        0x10ea486fc - trans::expr::trans_unadjusted::h66feeb4ce59b408dz4h
  14:        0x10e9f7f79 - trans::expr::trans_into::h4a29f680db4b3526znh
  15:        0x10ea7d220 - trans::expr::trans_adt::h73a11f7497464a42O6i
  16:        0x10ea7fd52 - trans::expr::trans_struct::closure.42052
  17:        0x10ea693cb - trans::expr::trans_struct::h4294120fb3439252K2i
  18:        0x10ea4a6be - trans::expr::trans_rvalue_dps_unadjusted::hca2c07ae951484c14zi
  19:        0x10e9f7f59 - trans::expr::trans_into::h4a29f680db4b3526znh
  20:        0x10e9f8e29 - trans::controlflow::trans_block::h26a308528bb95051b5d
  21:        0x10ead7da9 - trans::base::trans_closure::h1e78b4450eb13a03Kkt
  22:        0x10e9e0645 - trans::base::trans_fn::h31c9353b85ff9dd9Dvt
  23:        0x10e9e21c3 - trans::monomorphize::monomorphic_fn::h8c4abbed98c102adusd
  24:        0x10ea2a58b - trans::callee::trans_fn_ref_with_substs::heb22ab8904f67bf64kg
  25:        0x10ea28b0e - trans::callee::trans_fn_ref::hb2be9d8aec49d9c4E9f
  26:        0x10ea261d0 - trans::callee::trans::ha12ccf10898e9b03VYf
  27:        0x10ea3d24d - trans::callee::trans_call_inner::h11221320451677754653
  28:        0x10ea4abec - trans::expr::trans_rvalue_dps_unadjusted::hca2c07ae951484c14zi
  29:        0x10e9f7f59 - trans::expr::trans_into::h4a29f680db4b3526znh
  30:        0x10ea7d220 - trans::expr::trans_adt::h73a11f7497464a42O6i
  31:        0x10ea7fd52 - trans::expr::trans_struct::closure.42052
  32:        0x10ea693cb - trans::expr::trans_struct::h4294120fb3439252K2i
  33:        0x10ea4a6be - trans::expr::trans_rvalue_dps_unadjusted::hca2c07ae951484c14zi
  34:        0x10e9f7f59 - trans::expr::trans_into::h4a29f680db4b3526znh
  35:        0x10eb11d26 - trans::_match::mk_binding_alloca::h3448676421097350985
  36:        0x10e9f7280 - trans::base::init_local::h08a9fcff749ad013czs
  37:        0x10e9f8b02 - trans::controlflow::trans_block::h26a308528bb95051b5d
  38:        0x10ead7da9 - trans::base::trans_closure::h1e78b4450eb13a03Kkt
  39:        0x10e9e0645 - trans::base::trans_fn::h31c9353b85ff9dd9Dvt
  40:        0x10e9e21c3 - trans::monomorphize::monomorphic_fn::h8c4abbed98c102adusd
  41:        0x10ea2a58b - trans::callee::trans_fn_ref_with_substs::heb22ab8904f67bf64kg
  42:        0x10ea28b0e - trans::callee::trans_fn_ref::hb2be9d8aec49d9c4E9f
  43:        0x10ea261d0 - trans::callee::trans::ha12ccf10898e9b03VYf
  44:        0x10ea3d24d - trans::callee::trans_call_inner::h11221320451677754653
  45:        0x10ea4abec - trans::expr::trans_rvalue_dps_unadjusted::hca2c07ae951484c14zi
  46:        0x10ea4851c - trans::expr::trans_unadjusted::h66feeb4ce59b408dz4h
  47:        0x10e9f987b - trans::expr::trans::h60a968a47a19defeHth
  48:        0x10ea387e1 - trans::callee::trans_args::h43fd254006ea96e2m1g
  49:        0x10ea3e34e - trans::callee::trans_call_inner::h11221320451677754653
  50:        0x10ea4abec - trans::expr::trans_rvalue_dps_unadjusted::hca2c07ae951484c14zi
  51:        0x10ea4851c - trans::expr::trans_unadjusted::h66feeb4ce59b408dz4h
  52:        0x10e9f987b - trans::expr::trans::h60a968a47a19defeHth
  53:        0x10ea387e1 - trans::callee::trans_args::h43fd254006ea96e2m1g
  54:        0x10ea3e34e - trans::callee::trans_call_inner::h11221320451677754653
  55:        0x10ea4abec - trans::expr::trans_rvalue_dps_unadjusted::hca2c07ae951484c14zi
  56:        0x10e9f7f59 - trans::expr::trans_into::h4a29f680db4b3526znh
  57:        0x10eb11d26 - trans::_match::mk_binding_alloca::h3448676421097350985
  58:        0x10e9f7280 - trans::base::init_local::h08a9fcff749ad013czs
  59:        0x10e9f8b02 - trans::controlflow::trans_block::h26a308528bb95051b5d
  60:        0x10ea4a050 - trans::expr::trans_rvalue_dps_unadjusted::hca2c07ae951484c14zi
  61:        0x10e9f7f59 - trans::expr::trans_into::h4a29f680db4b3526znh
  62:        0x10eaf94e0 - trans::_match::trans_match_inner::he768a1b2ccde6888CIw
  63:        0x10ea49fe3 - trans::expr::trans_rvalue_dps_unadjusted::hca2c07ae951484c14zi
  64:        0x10ea4851c - trans::expr::trans_unadjusted::h66feeb4ce59b408dz4h
  65:        0x10e9f987b - trans::expr::trans::h60a968a47a19defeHth
  66:        0x10e9f7352 - trans::base::init_local::h08a9fcff749ad013czs
  67:        0x10e9f8b02 - trans::controlflow::trans_block::h26a308528bb95051b5d
  68:        0x10ead7da9 - trans::base::trans_closure::h1e78b4450eb13a03Kkt
  69:        0x10e9e0645 - trans::base::trans_fn::h31c9353b85ff9dd9Dvt
  70:        0x10e9e21c3 - trans::monomorphize::monomorphic_fn::h8c4abbed98c102adusd
  71:        0x10ea2a58b - trans::callee::trans_fn_ref_with_substs::heb22ab8904f67bf64kg
  72:        0x10ea4389f - trans::meth::trans_method_callee::h92a644d0a2dc2a56N6x
  73:        0x10ea3edbb - trans::callee::trans_call_inner::h17268543188350918970
  74:        0x10ea4976a - trans::expr::trans_rvalue_dps_unadjusted::hca2c07ae951484c14zi
  75:        0x10e9f7f59 - trans::expr::trans_into::h4a29f680db4b3526znh
  76:        0x10e9f8e29 - trans::controlflow::trans_block::h26a308528bb95051b5d
  77:        0x10ead7da9 - trans::base::trans_closure::h1e78b4450eb13a03Kkt
  78:        0x10e9e0645 - trans::base::trans_fn::h31c9353b85ff9dd9Dvt
  79:        0x10e9dcaa6 - trans::base::trans_item::h8ce277cc13b040a3vTt
  80:        0x10e9dc8a7 - trans::base::trans_item::h8ce277cc13b040a3vTt
  81:        0x10eae0ecb - trans::base::trans_crate::hb69c466e59630a14GPu
  82:        0x10e417ec7 - driver::phase_4_translate_to_llvm::he4e045cdc3897b4crNa
  83:        0x10e3f37f2 - driver::compile_input::h25d96a14ec26932cIba
  84:        0x10e4c572e - run_compiler::h15c3ea085a111a6fH5b
  85:        0x10e4c2ac7 - thunk::F.Invoke<A, R>::invoke::h11905936354613679413
  86:        0x10e4c160f - rt::unwind::try::try_fn::h4025213131938170097
  87:        0x112019628 - rust_try_inner
  88:        0x112019615 - rust_try
  89:        0x10e4c1e48 - thunk::F.Invoke<A, R>::invoke::h13725635537516470714
  90:        0x111f850a2 - sys::thread::thread_start::h1120d6d2e105321012E
  91:     0x7fff902ed267 - _pthread_body
  92:     0x7fff902ed1e4 - _pthread_start
