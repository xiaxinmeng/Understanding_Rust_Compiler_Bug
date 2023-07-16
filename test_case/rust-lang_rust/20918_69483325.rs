
  * frame #0: 0x00000001037d3c75 libstd-4e7c5e5c.dylib`rust_stack_exhausted + 37
    frame #1: 0x00000001002f2d5a librustc_trans-4e7c5e5c.dylib`lint::LintId.PartialEq::eq::h6538516d9011515alCr
    frame #2: 0x0000000100188f5c librustc_trans-4e7c5e5c.dylib`trans::common::erase_regions::RegionEraser$LT$$u{27}a$C$$u{20}$u{27}tcx$GT$.TypeFolder$LT$$u{27}tcx$GT$::fold_ty::hdb11fe61773fd770oKk + 44
    frame #3: 0x0000000100188ddd librustc_trans-4e7c5e5c.dylib`trans::common::erase_regions::h7703105742785118771 + 77
    frame #4: 0x000000010018815e librustc_trans-4e7c5e5c.dylib`trans::monomorphize::normalize_associated_type::h3997050791613523510 + 302
    frame #5: 0x0000000100254be5 librustc_trans-4e7c5e5c.dylib`trans::base::decl_rust_fn::h212c352cd5fc4cf8s1r + 325
    frame #6: 0x000000010026665e librustc_trans-4e7c5e5c.dylib`trans::base::register_fn::h4ce0eb41cba7c025dEu + 126
    frame #7: 0x0000000100183948 librustc_trans-4e7c5e5c.dylib`trans::base::get_item_val::hefbbd5389b60e7afc1u + 3752
    frame #8: 0x0000000100266d5d librustc_trans-4e7c5e5c.dylib`trans::base::create_entry_wrapper::create_entry_fn::h2d1d7fcb17403f8awSu + 973
    frame #9: 0x000000010026591a librustc_trans-4e7c5e5c.dylib`trans::base::finish_register_fn::ha1aeb097c10d306eOCu + 666
    frame #10: 0x00000001002666b5 librustc_trans-4e7c5e5c.dylib`trans::base::register_fn::h4ce0eb41cba7c025dEu + 213
    frame #11: 0x0000000100183948 librustc_trans-4e7c5e5c.dylib`trans::base::get_item_val::hefbbd5389b60e7afc1u + 3752
    frame #12: 0x0000000100266d5d librustc_trans-4e7c5e5c.dylib`trans::base::create_entry_wrapper::create_entry_fn::h2d1d7fcb17403f8awSu + 973

<snip>

    frame #11888: 0x0000000100266d1d librustc_trans-4e7c5e5c.dylib`trans::base::create_entry_wrapper::create_entry_fn::hbeda99ebf9512f9fwSu + 973
    frame #11889: 0x00000001002658da librustc_trans-4e7c5e5c.dylib`trans::base::finish_register_fn::hcfc60154d1285818OCu + 666
    frame #11890: 0x0000000100266675 librustc_trans-4e7c5e5c.dylib`trans::base::register_fn::h0ee69c538bdaf7b2dEu + 213
    frame #11891: 0x0000000100183908 librustc_trans-4e7c5e5c.dylib`trans::base::get_item_val::h697b171366de28ecc1u + 3752
    frame #11892: 0x0000000100266d1d librustc_trans-4e7c5e5c.dylib`trans::base::create_entry_wrapper::create_entry_fn::hbeda99ebf9512f9fwSu + 973
    frame #11893: 0x00000001002658da librustc_trans-4e7c5e5c.dylib`trans::base::finish_register_fn::hcfc60154d1285818OCu + 666
    frame #11894: 0x0000000100266675 librustc_trans-4e7c5e5c.dylib`trans::base::register_fn::h0ee69c538bdaf7b2dEu + 213
    frame #11895: 0x0000000100183908 librustc_trans-4e7c5e5c.dylib`trans::base::get_item_val::h697b171366de28ecc1u + 3752
    frame #11896: 0x0000000100180ce7 librustc_trans-4e7c5e5c.dylib`trans::base::trans_item::hdbddd5abd5742b0cEwu + 1063
    frame #11897: 0x000000010026a02c librustc_trans-4e7c5e5c.dylib`trans::base::trans_crate::h6077c32a4f7ba4e9lsv + 6044
    frame #11898: 0x000000010002ab3e librustc_driver-4e7c5e5c.dylib`driver::phase_4_translate_to_llvm::ha92297f2357a645fPFa + 1118
    frame #11899: 0x0000000100006f4b librustc_driver-4e7c5e5c.dylib`driver::compile_input::h31580cbd7ea87613xba + 8075
    frame #11900: 0x00000001000d1fda librustc_driver-4e7c5e5c.dylib`monitor::unboxed_closure.22557 + 6218
    frame #11901: 0x00000001000d0735 librustc_driver-4e7c5e5c.dylib`thunk::F.Invoke$LT$A$C$$u{20}R$GT$::invoke::h6367419564961841226 + 309
    frame #11902: 0x00000001000cf510 librustc_driver-4e7c5e5c.dylib`rt::unwind::try::try_fn::h7763956589852599824 + 160
    frame #11903: 0x000000010383d2a9 libstd-4e7c5e5c.dylib`rust_try_inner + 9
    frame #11904: 0x000000010383d296 libstd-4e7c5e5c.dylib`rust_try + 6
    frame #11905: 0x00000001000cfc0c librustc_driver-4e7c5e5c.dylib`thunk::F.Invoke$LT$A$C$$u{20}R$GT$::invoke::h16724184168577887652 + 1196
    frame #11906: 0x00000001037c4154 libstd-4e7c5e5c.dylib`sys::thread::thread_start::hbd8f2f8bdd3a3baadrw + 164
    frame #11907: 0x00007fff8e51d2fc libsystem_pthread.dylib`_pthread_body + 131
    frame #11908: 0x00007fff8e51d279 libsystem_pthread.dylib`_pthread_start + 176
    frame #11909: 0x00007fff8e51b4b1 libsystem_pthread.dylib`thread_start + 13
