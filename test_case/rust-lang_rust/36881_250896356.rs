
* thread #2: tid = 0xd16c56, 0x00000001007c05ef librustc_resolve-411f48d3.dylib`syntax::parse::token::intern::ha2fdc43b94e7e59d + 2927
  * frame #0: 0x00000001007c05ef librustc_resolve-411f48d3.dylib`syntax::parse::token::intern::ha2fdc43b94e7e59d + 2927
    frame #1: 0x00000001007f7a03 librustc_resolve-411f48d3.dylib`rustc_resolve::module_to_string::hfe434d26039d050b + 115
    frame #2: 0x00000001007e3e24 librustc_resolve-411f48d3.dylib`rustc_resolve::Resolver::resolve_module_path_from_root::hf44ade0b9208b1cf + 1012
    frame #3: 0x00000001007e4744 librustc_resolve-411f48d3.dylib`rustc_resolve::Resolver::resolve_module_path::heb8778a1018a3bb7 + 1156
    frame #4: 0x00000001007d6748 librustc_resolve-411f48d3.dylib`rustc_resolve::resolve_imports::ImportResolver::resolve_imports::hecc9b8b6425d6756 + 2008
    frame #5: 0x00000001007d3405 librustc_resolve-411f48d3.dylib`rustc_resolve::resolve_imports::_$LT$impl$u20$rustc_resolve..Resolver$LT$$u27$a$GT$$GT$::resolve_imports::hcb3093570e757745 + 21
    frame #6: 0x00000001000f1405 librustc_driver-411f48d3.dylib`rustc_driver::driver::phase_2_configure_and_expand::_$u7b$$u7b$closure$u7d$$u7d$::hab3b1f9b0576f59e + 69
    frame #7: 0x00000001000ab2ad librustc_driver-411f48d3.dylib`rustc_driver::driver::phase_2_configure_and_expand::h7891c7a88a497345 + 16285
    frame #8: 0x00000001000a3cd3 librustc_driver-411f48d3.dylib`rustc_driver::driver::compile_input::h8a5ca0f49da8c80a + 1555
    frame #9: 0x00000001000cd379 librustc_driver-411f48d3.dylib`rustc_driver::run_compiler::ha0d125e702c7be63 + 2041
    frame #10: 0x000000010000db21 librustc_driver-411f48d3.dylib`std::panicking::try::do_call::h25f69b2f74f3cecc + 161
    frame #11: 0x00000001041d9fbb libstd-411f48d3.dylib`__rust_maybe_catch_panic + 27
    frame #12: 0x000000010002d595 librustc_driver-411f48d3.dylib`_$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::h3f37dd793239cfe8 + 213
    frame #13: 0x00000001041d61b5 libstd-411f48d3.dylib`std::sys::thread::Thread::new::thread_start::h66211a1b34bcffec + 37
    frame #14: 0x00007fff9cd4b99d libsystem_pthread.dylib`_pthread_body + 131
    frame #15: 0x00007fff9cd4b91a libsystem_pthread.dylib`_pthread_start + 168
    frame #16: 0x00007fff9cd49351 libsystem_pthread.dylib`thread_start + 13
