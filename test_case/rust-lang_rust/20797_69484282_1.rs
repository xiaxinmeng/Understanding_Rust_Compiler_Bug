
error: internal compiler error: get_unique_type_id_of_type() - unexpected type: <collections::vec::IntoIter<i32> as core::iter::Iterator>::Item, ty_projection(ProjectionTy { trait_ref: Rc(TraitRef { def_id: DefId { krate: 2u32, node: 42866u32 }, substs: Substs { types: VecPerParamSpace {TypeSpace: [], SelfSpace: [TyS { sty: ty_struct(DefId { krate: 3u32, node: 36733u32 }, Substs { types: VecPerParamSpace {TypeSpace: [TyS { sty: ty_int(i32), flags: 0, region_depth: 0u32 }], SelfSpace: [], FnSpace: [], }, regions: ErasedRegions }), flags: 0, region_depth: 0u32 }], FnSpace: [], }, regions: NonerasedRegions(VecPerParamSpace {TypeSpace: [], SelfSpace: [], FnSpace: [], }) } }), item_name: "Item"(67) })
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'Box<Any>', /Users/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-mac/build/src/libsyntax/diagnostic.rs:182

stack backtrace:
   1:        0x1113f24b5 - sys::backtrace::write::h64ca2fb259c4ae97lCt
   2:        0x11141494f - failure::on_fail::h64ff5ae6887860cc0Hz
   3:        0x1113805ca - rt::unwind::begin_unwind_inner::h1d4bc098fd350446Qpz
   4:        0x10f0880b7 - rt::unwind::begin_unwind::h1123425023578864118
   5:        0x10f088a28 - diagnostic::Handler::bug::h6d2bfe842975d6e0tWF
   6:        0x10e68c788 - session::Session::bug::h5da8d79b0a6f5276iRq
   7:        0x10deb5805 - trans::debuginfo::TypeMap<'tcx>::get_unique_type_id_of_type::hf00d365d13fe5f42rGD
   8:        0x10deb9557 - trans::debuginfo::TypeMap<'tcx>::get_unique_type_id_of_type::from_def_id_and_substs::h891ec143e99916f43PD
   9:        0x10deb5705 - trans::debuginfo::TypeMap<'tcx>::get_unique_type_id_of_type::hf00d365d13fe5f42rGD
  10:        0x10debb7cb - trans::debuginfo::type_metadata::h9edd372a193ff094RSF
  11:        0x10dec3cf5 - trans::debuginfo::StructMemberDescriptionFactory<'tcx>::create_member_descriptions::unboxed_closure.45659
  12:        0x10dec37b2 - vec::Vec<T>.FromIterator<T>::from_iter::h4078323428656498456
  13:        0x10debf54d - trans::debuginfo::StructMemberDescriptionFactory<'tcx>::create_member_descriptions::he532502aebbbff41j9E
  14:        0x10dec23e2 - trans::debuginfo::RecursiveTypeDescription<'tcx>::finalize::h4e94b4649bcaa46e26E
  15:        0x10debc539 - trans::debuginfo::type_metadata::h9edd372a193ff094RSF
  16:        0x10debdad2 - trans::debuginfo::declare_local::h82cfc8c7fa3e6298CPE
  17:        0x10debd32d - ast_util::walk_pat::walk_pat_::h14410195268450947789
  18:        0x10dda22a7 - trans::controlflow::trans_block::h1b3762e019b88de7B3d
  19:        0x10de6ba39 - trans::base::trans_closure::h83147acece2ec0c6EYt
  20:        0x10dd8d5f5 - trans::base::trans_fn::h99bd3c17ad60972dj9t
  21:        0x10dd88d4e - trans::base::trans_item::hdbddd5abd5742b0cEwu
  22:        0x10de7202c - trans::base::trans_crate::h6077c32a4f7ba4e9lsv
  23:        0x10dc2eb3e - driver::phase_4_translate_to_llvm::ha92297f2357a645fPFa
  24:        0x10dc0af4b - driver::compile_input::h31580cbd7ea87613xba
  25:        0x10dcd5fda - monitor::unboxed_closure.22557
  26:        0x10dcd4735 - thunk::F.Invoke<A, R>::invoke::h6367419564961841226
  27:        0x10dcd3510 - rt::unwind::try::try_fn::h7763956589852599824
  28:        0x11147b2a9 - rust_try_inner
  29:        0x11147b296 - rust_try
  30:        0x10dcd3c0c - thunk::F.Invoke<A, R>::invoke::h16724184168577887652
  31:        0x111402154 - sys::thread::thread_start::hbd8f2f8bdd3a3baadrw
  32:     0x7fff9358c2fc - _pthread_body
  33:     0x7fff9358c279 - _pthread_body
