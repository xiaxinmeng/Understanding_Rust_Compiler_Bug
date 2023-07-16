
* thread #2: tid = 0x13e07bf, 0x0000000103d8a47a libsyntax-6eb85298.dylib`syntax::parse::parser::Parser::parse_item_::hd45ad4c7b92b0207 + 7418
  * frame #0: 0x0000000103d8a47a libsyntax-6eb85298.dylib`syntax::parse::parser::Parser::parse_item_::hd45ad4c7b92b0207 + 7418
    frame #1: 0x0000000103d72690 libsyntax-6eb85298.dylib`syntax::parse::parser::Parser::parse_stmt_without_recovery::he0ba5443013bdaf6 + 2320
    frame #2: 0x0000000103d71b7a libsyntax-6eb85298.dylib`syntax::parse::parser::Parser::parse_stmt_::hdf5f69e1800c32ad + 42
    frame #3: 0x0000000103d75c56 libsyntax-6eb85298.dylib`syntax::parse::parser::Parser::parse_full_stmt::hd11716e3359a7818 + 38
    frame #4: 0x0000000103df72ad libsyntax-6eb85298.dylib`syntax::ext::expand::_$LT$impl$u20$syntax..parse..parser..Parser$LT$$u27$a$GT$$GT$::parse_expansion::h8c1627ae52debed0 + 669
    frame #5: 0x0000000103e1e5aa libsyntax-6eb85298.dylib`syntax::ext::tt::macro_rules::ParserAnyMacro::make::hfe157ca05d8b40d4 + 74
    frame #6: 0x0000000103e4de35 libsyntax-6eb85298.dylib`syntax::ext::expand::_$LT$impl$u20$syntax..ext..base..MacResult$u20$for$u20$syntax..ext..tt..macro_rules..ParserAnyMacro$LT$$u27$a$GT$$GT$::make_stmts::h790644120b41cd96 + 37
    frame #7: 0x0000000103e3d6f1 libsyntax-6eb85298.dylib`syntax::ext::expand::ExpansionKind::make_from::hc049eb22fc5e17d1 + 209
    frame #8: 0x0000000103df500b libsyntax-6eb85298.dylib`syntax::ext::expand::MacroExpander::expand::h737d7f1635448d5d + 7179
    frame #9: 0x0000000103df2fe6 libsyntax-6eb85298.dylib`syntax::ext::expand::MacroExpander::expand_crate::hd89d06b7a6333976 + 502
    frame #10: 0x0000000103dfe09e libsyntax-6eb85298.dylib`syntax::ext::expand::expand_crate::h80012253cfa7f1f5 + 142
    frame #11: 0x00000001000f3e6c librustc_driver-6eb85298.dylib`rustc_driver::driver::phase_2_configure_and_expand::_$u7b$$u7b$closure$u7d$$u7d$::hc795707fb140075e + 428
    frame #12: 0x00000001000ac336 librustc_driver-6eb85298.dylib`rustc_driver::driver::phase_2_configure_and_expand::h04526112c643cb41 + 8166
    frame #13: 0x00000001000a6d13 librustc_driver-6eb85298.dylib`rustc_driver::driver::compile_input::h5b63ccd49eeeb98b + 1555
    frame #14: 0x00000001000cf1aa librustc_driver-6eb85298.dylib`rustc_driver::run_compiler::h98c7274e7cb1d11d + 3098
    frame #15: 0x000000010000dff9 librustc_driver-6eb85298.dylib`std::panicking::try::do_call::h99ed0da044e497c3 + 873
    frame #16: 0x00000001041beb3b libstd-6eb85298.dylib`__rust_maybe_catch_panic + 27
    frame #17: 0x000000010002d000 librustc_driver-6eb85298.dylib`_$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::hbdd5a14cd8e33b97 + 144
    frame #18: 0x00000001041bac75 libstd-6eb85298.dylib`std::sys::thread::Thread::new::thread_start::h50b05608a499d2b2 + 37
    frame #19: 0x00007fff9cd4b99d libsystem_pthread.dylib`_pthread_body + 131
    frame #20: 0x00007fff9cd4b91a libsystem_pthread.dylib`_pthread_start + 168
    frame #21: 0x00007fff9cd49351 libsystem_pthread.dylib`thread_start + 13
