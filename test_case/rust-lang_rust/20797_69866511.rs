
bash-3.2$ rustc -V
rustc 1.0.0-dev (170c4399e 2015-01-14 00:41:55 +0000)
bash-3.2$ RUST_BACKTRACE=1 cargo test
   Compiling subpaths v0.0.1 (file:///Users/samuel/Documents/Programming/filehash/src/subpaths)
error: internal compiler error: type_of with ty_projection
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /Users/samuel/Documents/rust/src/libsyntax/diagnostic.rs:182

stack backtrace:
   1:        0x110640dec - sys::backtrace::write::h724c84ec96e82337RUt
   2:        0x110662a8f - failure::on_fail::h6f2a6126417d36beIYz
   3:        0x1105cf74e - rt::unwind::begin_unwind_inner::hb2d99935a1d8bde1yGz
   4:        0x10e30ac4f - rt::unwind::begin_unwind::h6101901676675217320
   5:        0x10e30b5bb - diagnostic::Handler::bug::ha27a4766170cb062A1F
   6:        0x10d8e1670 - metadata::tydecode::parse_existential_bounds_::h10742984024508526701
   7:        0x10d022c88 - trans::type_of::type_of::h6ebeadb230102bb3Y1o
   8:        0x10d023225 - trans::type_of::type_of::h6ebeadb230102bb3Y1o
   9:        0x10d022190 - trans::type_of::type_of::h6ebeadb230102bb3Y1o
  10:        0x10d13929c - trans::debuginfo::StructMemberDescriptionFactory<'tcx>::create_member_descriptions::unboxed_closure.45482
  11:        0x10d138e82 - vec::Vec<T>.FromIterator<T>::from_iter::h1395047669026947608
  12:        0x10d1358bd - trans::debuginfo::StructMemberDescriptionFactory<'tcx>::create_member_descriptions::h153e84111a2448b60fF
  13:        0x10d137b0d - trans::debuginfo::RecursiveTypeDescription<'tcx>::finalize::h0a20b015282159ecJdF
  14:        0x10d1327a3 - trans::debuginfo::type_metadata::h594d4c508338d95dyZF
  15:        0x10d1392d4 - trans::debuginfo::StructMemberDescriptionFactory<'tcx>::create_member_descriptions::unboxed_closure.45482
  16:        0x10d138e82 - vec::Vec<T>.FromIterator<T>::from_iter::h1395047669026947608
  17:        0x10d1358bd - trans::debuginfo::StructMemberDescriptionFactory<'tcx>::create_member_descriptions::h153e84111a2448b60fF
  18:        0x10d137b0d - trans::debuginfo::RecursiveTypeDescription<'tcx>::finalize::h0a20b015282159ecJdF
  19:        0x10d1327a3 - trans::debuginfo::type_metadata::h594d4c508338d95dyZF
  20:        0x10d1392d4 - trans::debuginfo::StructMemberDescriptionFactory<'tcx>::create_member_descriptions::unboxed_closure.45482
  21:        0x10d138e82 - vec::Vec<T>.FromIterator<T>::from_iter::h1395047669026947608
  22:        0x10d1358bd - trans::debuginfo::StructMemberDescriptionFactory<'tcx>::create_member_descriptions::h153e84111a2448b60fF
  23:        0x10d137b0d - trans::debuginfo::RecursiveTypeDescription<'tcx>::finalize::h0a20b015282159ecJdF
  24:        0x10d1327a3 - trans::debuginfo::type_metadata::h594d4c508338d95dyZF
  25:        0x10d13bc32 - trans::debuginfo::VariantMemberDescriptionFactory<'tcx>::create_member_descriptions::unboxed_closure.45516
  26:        0x10d13b7f2 - vec::Vec<T>.FromIterator<T>::from_iter::h5500603777858505790
  27:        0x10d13a28b - trans::debuginfo::EnumMemberDescriptionFactory<'tcx>::create_member_descriptions::unboxed_closure.45490
  28:        0x10d139da2 - vec::Vec<T>.FromIterator<T>::from_iter::h14361925877225366780
  29:        0x10d135b83 - trans::debuginfo::EnumMemberDescriptionFactory<'tcx>::create_member_descriptions::hdd1b6f55c4c7b16eYmF
  30:        0x10d137b2b - trans::debuginfo::RecursiveTypeDescription<'tcx>::finalize::h0a20b015282159ecJdF
  31:        0x10d131bf9 - trans::debuginfo::type_metadata::h594d4c508338d95dyZF
  32:        0x10d13eca8 - trans::debuginfo::subroutine_type_metadata::h9bff0740736a5de7cVF
  33:        0x10d13262f - trans::debuginfo::type_metadata::h594d4c508338d95dyZF
  34:        0x10d0dba0b - trans::debuginfo::create_function_debug_context::h447bd3a847c41d0adzE
  35:        0x10d03fb3e - trans::base::new_fn_ctxt::h80d01085022bea25Tzt
  36:        0x10d0e04cd - trans::base::trans_closure::h32d8ee0946ac5386y0t
  37:        0x10d006197 - trans::base::trans_fn::ha19d856a27ef8fdadbu
  38:        0x10d007d13 - trans::monomorphize::monomorphic_fn::he3114699339f1503Fpd
  39:        0x10d0578e9 - trans::callee::trans_fn_ref_with_substs::h8ab7a417361225b5Ixg
  40:        0x10d0553e3 - trans::callee::trans_fn_ref::haab38ef68aad01e0Tlg
  41:        0x10d052edc - trans::callee::trans::h30cee5a1ce99c1e8Aag
  42:        0x10d05bbee - trans::callee::trans_call_inner::h17375577365636780878
  43:        0x10d062823 - trans::expr::trans_rvalue_dps_unadjusted::h698caf6d437867f53Si
  44:        0x10d06026a - trans::expr::trans_unadjusted::h527db00c397125ebQji
  45:        0x10d01af76 - trans::expr::trans::he91bdea9edc285d3gCh
  46:        0x10d02bda1 - trans::callee::trans_args::h5d1792d9a12db34czch
  47:        0x10d05e00e - trans::callee::trans_call_inner::h15873392703759300422
  48:        0x10d061296 - trans::expr::trans_rvalue_dps_unadjusted::h698caf6d437867f53Si
  49:        0x10d019f6b - trans::expr::trans_into::hd03a9a170d995fabMyh
  50:        0x10d115669 - trans::_match::mk_binding_alloca::h5771621100645690575
  51:        0x10d019564 - trans::base::init_local::ha40ddf9a1cd925e6w9s
  52:        0x10d01a826 - trans::controlflow::trans_block::h382e5b424a79fcedC3d
  53:        0x10d0e290c - trans::base::trans_closure::h32d8ee0946ac5386y0t
  54:        0x10d006197 - trans::base::trans_fn::ha19d856a27ef8fdadbu
  55:        0x10d00183b - trans::base::trans_item::h5af943f055925df9yyu
  56:        0x10d0e8a8c - trans::base::trans_crate::h1ae727bd77425dc1fuv
  57:        0x10ceb5d74 - driver::phase_4_translate_to_llvm::h5abdfc397f265750YMa
  58:        0x10ce97a36 - driver::compile_input::hb8b4e569f22b04afAba
  59:        0x10cf54c40 - run_compiler::hfcb743c274cb2b48e5b
  60:        0x10cf51fe3 - thunk::F.Invoke<A, R>::invoke::h5297055281723610762
  61:        0x10cf50e48 - rt::unwind::try::try_fn::h6169901515804371881
  62:        0x1106cf149 - rust_try_inner
  63:        0x1106cf136 - rust_try
  64:        0x10cf51486 - thunk::F.Invoke<A, R>::invoke::h14156849340732247684
  65:        0x110650162 - sys::thread::thread_start::hd3418b70b5ddcf22IJw
  66:     0x7fff8d7932fc - _pthread_body
  67:     0x7fff8d793279 - _pthread_body

Could not compile `subpaths`.
