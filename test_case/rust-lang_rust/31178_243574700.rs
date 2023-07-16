
#0  0x67c1e320 in rust_panic ()
   from C:\Program Files\Rust stable GNU 1.11\bin\std-39b92f95.dll
#1  0x67c04c84 in std::panicking::rust_panic_with_hook::hd7b83626099d3416 ()
   from C:\Program Files\Rust stable GNU 1.11\bin\std-39b92f95.dll
#2  0x67c3fa88 in std::panicking::begin_panic::h941ea76fc945d925 ()
   from C:\Program Files\Rust stable GNU 1.11\bin\std-39b92f95.dll
#3  0x67c06d6d in std::panicking::begin_panic_fmt::h30280d4dd3f149f5 ()
   from C:\Program Files\Rust stable GNU 1.11\bin\std-39b92f95.dll
#4  0x67c3f82f in rust_begin_unwind ()
   from C:\Program Files\Rust stable GNU 1.11\bin\std-39b92f95.dll
#5  0x67c51fa8 in core::panicking::panic_fmt::h2d3cc8234dde51b4 ()
   from C:\Program Files\Rust stable GNU 1.11\bin\std-39b92f95.dll
#6  0x6ca3b2ab in _fu226___ZN4core6result13unwrap_failed10_FILE_LINE17h730ed9164be33e67E ()
   from C:\Program Files\Rust stable GNU 1.11\bin\rustc_metadata-39b92f95.dll
#7  0x6cae3e47 in _fu343___ZN38_$LT$core..option..Option$LT$T$GT$$GT$6unwrap14_MSG_FILE_LINE17hc89d807017dd066bE ()
   from C:\Program Files\Rust stable GNU 1.11\bin\rustc_metadata-39b92f95.dll
#8  0x6cac6160 in rustc_metadata::creader::CrateReader::load::hb86defd12ffcdae5 () from C:\Program Files\Rust stable GNU 1.11\bin\rustc_metadata-39b92f95.dll
#9  0x6cac9b7b in rustc_metadata::creader::CrateReader::read_extension_crate::h6101a5aa9052a0fc ()
   from C:\Program Files\Rust stable GNU 1.11\bin\rustc_metadata-39b92f95.dll
#10 0x6cacb3ad in rustc_metadata::creader::CrateReader::read_exported_macros::hd179a7d05e3b7c05 ()
   from C:\Program Files\Rust stable GNU 1.11\bin\rustc_metadata-39b92f95.dll
#11 0x6caf6133 in _fu888___ZN3std11collections4hash3map11RandomState3new4KEYS17hb806d9bac9a41080E ()
   from C:\Program Files\Rust stable GNU 1.11\bin\rustc_metadata-39b92f95.dll
#12 0x674f8d57 in _$LT$syntax..ext..expand..MacroExpander..load_macros..MacroLoadingVisitor$LT$$u27 $a$C$$u20$$u27$b$GT$$u20$as$u20$syntax..visit..Visitor$GT$::visit_item::h8628ee4498e5def8 ()
   from C:\Program Files\Rust stable GNU 1.11\bin\syntax-39b92f95.dll
#13 0x6753bb2e in syntax::ext::expand::expand_crate::h943e1e51f06dd940 ()
   from C:\Program Files\Rust stable GNU 1.11\bin\syntax-39b92f95.dll
#14 0x7044acac in _fu108___ZN5alloc7raw_vec11alloc_guard14_MSG_FILE_LINE17h0fd6f822bd86c07aE ()
   from C:\Program Files\Rust stable GNU 1.11\bin\rustc_driver-39b92f95.dll
#15 0x704439b5 in _fu478___ZN5rustc4util6common4time5DEPTH17h6eb268ad83e9cdadE ()
   from C:\Program Files\Rust stable GNU 1.11\bin\rustc_driver-39b92f95.dll
#16 0x704158db in rustc_driver::driver::compile_input::hdfe4405d66704c31 ()
   from C:\Program Files\Rust stable GNU 1.11\bin\rustc_driver-39b92f95.dll
#17 0x70406c2e in rustc_driver::run_compiler::h581448fb74257353 ()
   from C:\Program Files\Rust stable GNU 1.11\bin\rustc_driver-39b92f95.dll
#18 0x70403c0f in _fu2___ZN38_$LT$core..option..Option$LT$T$GT$$GT$6unwrap14_MSG_FILE_LINE17hc89d807017dd066bE ()
   from C:\Program Files\Rust stable GNU 1.11\bin\rustc_driver-39b92f95.dll
#19 0x67c4c50b in __rust_try ()
   from C:\Program Files\Rust stable GNU 1.11\bin\std-39b92f95.dll
#20 0x67c4c4a1 in __rust_maybe_catch_panic ()
   from C:\Program Files\Rust stable GNU 1.11\bin\std-39b92f95.dll
#21 0x70404795 in _fu46___ZN3std9panicking11PANIC_COUNT17h747ab620a51161d7E ()
   from C:\Program Files\Rust stable GNU 1.11\bin\rustc_driver-39b92f95.dll
#22 0x67c3bfa2 in std::sys::thread::Thread::new::thread_start::h7c59ebf0bbb78261 () from C:\Program Files\Rust stable GNU 1.11\bin\std-39b92f95.dll
#23 0x75404198 in KERNEL32!BaseThreadInitThunk ()
   from C:\Windows\system32\kernel32.dll
#24 0x7720445d in ?? ()
#25 0x7720442b in ?? ()
#26 0x00000000 in ?? ()
(gdb)
