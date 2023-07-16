
(lldb) bt
* thread #2: tid = 0x953094, 0x00000001000d7960 libstd-04ff901e-0.9-pre.dylib`rt::borrowck::fail_borrowed::anon::expr_fn::a3, stop reason = breakpoint 3.1
    frame #0: 0x00000001000d7960 libstd-04ff901e-0.9-pre.dylib`rt::borrowck::fail_borrowed::anon::expr_fn::a3
    frame #1: 0x00000001000b82d7 libstd-04ff901e-0.9-pre.dylib`c_str::with_c_str::he8257f3b1afbeab9e5c8dc0e0142c488c8e8acbbe2619d64866cb51b40422d0faH::v0.9.pre + 423
    frame #2: 0x00000001000b8130 libstd-04ff901e-0.9-pre.dylib`c_str::ToCStr$__extensions__::with_c_str::hd54d9b26a8fa5c8b83e29263c645910991b308f44cb73bb652a8f43ab7470e0cK2aF::v0.9.pre + 80
    frame #3: 0x00000001000d793a libstd-04ff901e-0.9-pre.dylib`rt::borrowck::fail_borrowed::hfff558ee34b47593514af774da5540530ada489b55c92bd151cf40579b07f43daw::v0.9.pre + 1706
    frame #4: 0x0000000101363bec librustc-5b94a16f-0.9-pre.dylib`middle::resolve::Module::all_imports_resolved::hfcce82677f72e4a2f53b95f5619e29d69ec169083c863dc12540742e3f5dc361Mkay::v0.9.pre + 188
    frame #5: 0x000000010138b582 librustc-5b94a16f-0.9-pre.dylib`middle::resolve::Resolver::resolve_imports_for_module::h097ed13bb78aeba56d42d53bd987dbada5164b206091da05b1b227c7979b83e2jDaU::v0.9.pre + 1986
    frame #6: 0x0000000101389dc7 librustc-5b94a16f-0.9-pre.dylib`middle::resolve::Resolver::resolve_imports_for_module_subtree::h63300ffa051f0fb20b814d2b9fe7f1063998edaa5b658e69ea833668ca924b6fjDaO::v0.9.pre + 199
    frame #7: 0x0000000101389fcb librustc-5b94a16f-0.9-pre.dylib`middle::resolve::Resolver::resolve_imports_for_module_subtree::h63300ffa051f0fb20b814d2b9fe7f1063998edaa5b658e69ea833668ca924b6fjDaO::v0.9.pre + 715
    frame #8: 0x00000001013b54c1 librustc-5b94a16f-0.9-pre.dylib`middle::resolve::resolve_crate::hc539f9cf139985abd335260c80f86976b2e097bff0bcb8ae91dff4af79c29416aG::v0.9.pre + 3937
    frame #9: 0x00000001017fc33f librustc-5b94a16f-0.9-pre.dylib`driver::driver::phase_3_run_analysis_passes::hb4d6d47a5c83ef210c5e3692eaf80af00a76638187539b10c4a65497866f2094aa::v0.9.pre + 1871
    frame #10: 0x0000000101802a49 librustc-5b94a16f-0.9-pre.dylib`driver::driver::compile_input::h483e091c326fc43ea6b2d40ff3221b822ca7a1f091f5ab31dcfc58862be987b4aD::v0.9.pre + 329
    frame #11: 0x0000000101826a0a librustc-5b94a16f-0.9-pre.dylib`run_compiler::ha19081b1ccfd91eabf64275fc65cf81db67cac74d88316690ea787f1e989176dar::v0.9.pre + 7370
    frame #12: 0x0000000101832f8d librustc-5b94a16f-0.9-pre.dylib`main_args::anon::expr_fn::aZ + 109
    frame #13: 0x000000010183169a librustc-5b94a16f-0.9-pre.dylib`monitor::anon::expr_fn::ag + 250
    frame #14: 0x000000010182f53c librustc-5b94a16f-0.9-pre.dylib`task::TaskBuilder::try::anon::expr_fn::Lya5aw + 76
    frame #15: 0x00000001000c4b18 libstd-04ff901e-0.9-pre.dylib`rt::task::__extensions__::build_start_wrapper::anon::anon::expr_fn::af + 104
    frame #16: 0x00000001000c3483 libstd-04ff901e-0.9-pre.dylib`rt::task::__extensions__::run::anon::expr_fn::a7 + 67
    frame #17: 0x00000001000dc983 libstd-04ff901e-0.9-pre.dylib`rust_try(f=<unavailable>, fptr=<unavailable>, env=<unavailable>) + 19 at rust_cxx_glue.cpp:20
    frame #18: 0x00000001000c3393 libstd-04ff901e-0.9-pre.dylib`rt::task::Unwinder::try::h12b0592b722d08688083f7c472e78b826ffa6e343fafd11d033694a7a6ca6e2foXa6::v0.9.pre + 67
    frame #19: 0x00000001000c2fdf libstd-04ff901e-0.9-pre.dylib`rt::task::Task::run::hfb90aa69cbd7bf1c7f0286a0be1c78842dcc4afa750be32b9fd05a0b442e3f176La4::v0.9.pre + 127
    frame #20: 0x00000001000c42d4 libstd-04ff901e-0.9-pre.dylib`rt::task::__extensions__::build_start_wrapper::anon::expr_fn::a1 + 500
