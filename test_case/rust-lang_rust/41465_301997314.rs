
markus@x4 bin % touch foo.rs && valgrind --track-origins=yes --trace-children=yes ./rustc --crate-type staticlib foo.rs                                                   [98/7992]
==13005== Memcheck, a memory error detector                                                                                                                                        
==13005== Copyright (C) 2002-2015, and GNU GPL'd, by Julian Seward et al.                                                                                                          
==13005== Using Valgrind-3.12.0 and LibVEX; rerun with -h for copyright info                                                                                                       
==13005== Command: ./rustc --crate-type staticlib foo.rs                                                                                                                           
==13005==                                                                                                                                                
==13005== Thread 2 rustc:                                                                                                                                                          
==13005== Conditional jump or move depends on uninitialised value(s)                                                                                                            
==13005==    at 0x8A358B4: syntax::parse::parser::Parser::new::h456d13989cd69ec4 (in /home/markus/tmp/tmp/rust/usr/local/lib/libsyntax-93b867817c21b413.so)                        
==13005==    by 0x8A9D1E8: syntax::parse::filemap_to_parser::h89ca72f64f73f0ec (in /home/markus/tmp/tmp/rust/usr/local/lib/libsyntax-93b867817c21b413.so)
==13005==    by 0x8A9C984: syntax::parse::parse_crate_from_file::h52d201d2d6148edf (in /home/markus/tmp/tmp/rust/usr/local/lib/libsyntax-93b867817c21b413.so)                      
==13005==    by 0x4B094E8: rustc_driver::driver::phase_1_parse_input::_$u7b$$u7b$closure$u7d$$u7d$::hea178be44144e2b6 (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_driver-8
2fa3cb513b2150e.so)                                                                                                                                                                
==13005==    by 0x4B08B65: rustc_driver::driver::phase_1_parse_input::h9f089fd5e52bada6 (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_driver-82fa3cb513b2150e.so)
==13005==    by 0x4B04306: rustc_driver::driver::compile_input::h3941889c5f739fd2 (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_driver-82fa3cb513b2150e.so)                 
==13005==    by 0x4B4D1E1: rustc_driver::run_compiler::hd92623a1326f823c (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_driver-82fa3cb513b2150e.so)
==13005==    by 0x4A5746B: std::sys_common::backtrace::__rust_begin_short_backtrace::h445445fc67a4011d (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_driver-82fa3cb513b2150e
.so)                                                                  
==13005==    by 0x4E60ECA: __rust_maybe_catch_panic (lib.rs:98)                                                                                                                    
==13005==    by 0x4A8A0C0: _$LT$F$u20$as$u20$alloc..boxed..FnBox$LT$A$GT$$GT$::call_box::hf4118457ff3d19dc (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_driver-82fa3cb513b2
150e.so)                                                                                                                                                                           
==13005==    by 0x4E56575: call_once<(),()> (boxed.rs:658)                                                
==13005==    by 0x4E56575: start_thread (thread.rs:21)                                                                                                                             
==13005==    by 0x4E56575: std::sys::imp::thread::Thread::new::thread_start::h4049bf47f364e0f8 (thread.rs:84)                                                                      
==13005==    by 0xD85237B: start_thread (pthread_create.c:455)                                                                                                                     
==13005==  Uninitialised value was created by a stack allocation                                                                                                                   
==13005==    at 0x8A9D184: syntax::parse::filemap_to_parser::h89ca72f64f73f0ec (in /home/markus/tmp/tmp/rust/usr/local/lib/libsyntax-93b867817c21b413.so)                          
==13005==                                                                                                                                                                          
==13005== Conditional jump or move depends on uninitialised value(s)                                                                                                               
==13005==    at 0x4E6AA20: je_arena_salloc (arena.h:1388)                                                                                                                          
==13005==    by 0x4E6AA20: je_isalloc (jemalloc_internal.h:1054)                                                                                                                   
==13005==    by 0x4E6AA20: rallocx (jemalloc.c:2419)                                                                                                                               
==13005==    by 0x923E13E: _$LT$alloc..raw_vec..RawVec$LT$T$GT$$GT$::double::h746fadf638d3a65f (in /home/markus/tmp/tmp/rust/usr/local/lib/libsyntax_pos-364f7f3b054e6796.so)      
==13005==    by 0x9241895: syntax_pos::symbol::Interner::intern::hf484e790b0aa4b15 (in /home/markus/tmp/tmp/rust/usr/local/lib/libsyntax_pos-364f7f3b054e6796.so)                  
==13005==    by 0x92410F5: syntax_pos::symbol::Symbol::intern::h50fea05cde2cbf92 (in /home/markus/tmp/tmp/rust/usr/local/lib/libsyntax_pos-364f7f3b054e6796.so)                    
==13005==    by 0x7351810: serialize::serialize::Decoder::read_enum_variant_arg::h81db90fbf83c26b7 (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_metadata-d3323a2104c2ad27.s
o)                                                                                                                                                                                 
==13005==    by 0x732FB3C: _$LT$collections..vec..Vec$LT$T$GT$$u20$as$u20$serialize..serialize..Decodable$GT$::decode::h8566cacadf292514 (in /home/markus/tmp/tmp/rust/usr/local/li
b/librustc_metadata-d3323a2104c2ad27.so)                                                                                                                                           
==13005==    by 0x73A5DFB: rustc_metadata::creader::CrateLoader::register_crate::h463e5f5a7d0801c5 (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_metadata-d3323a2104c2ad27.s
o)                                                                                                                                                                                 
==13005==    by 0x73A958C: rustc_metadata::creader::CrateLoader::resolve_crate::h5893dde60755de0d (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_metadata-d3323a2104c2ad27.so
)                                                                                                                                                                                  
==13005==    by 0x73AA39B: rustc_metadata::creader::CrateLoader::resolve_crate_deps::_$u7b$$u7b$closure$u7d$$u7d$::h316dcdec17504bb5 (in /home/markus/tmp/tmp/rust/usr/local/lib/li
brustc_metadata-d3323a2104c2ad27.so)                                                                                                                                               
==13005==    by 0x7343F0F: _$LT$collections..vec..Vec$LT$T$GT$$u20$as$u20$collections..vec..SpecExtend$LT$T$C$$u20$I$GT$$GT$::spec_extend::hdacb47417427d83f (in /home/markus/tmp/t
mp/rust/usr/local/lib/librustc_metadata-d3323a2104c2ad27.so)                                                                                                                       
==13005==    by 0x73A5D11: rustc_metadata::creader::CrateLoader::register_crate::h463e5f5a7d0801c5 (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_metadata-d3323a2104c2ad27.s
o)                                                                                                                                                                                 
==13005==    by 0x73A958C: rustc_metadata::creader::CrateLoader::resolve_crate::h5893dde60755de0d (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_metadata-d3323a2104c2ad27.so
)                                                                                                                                                                                  
==13005==  Uninitialised value was created by a heap allocation                                                                                                                    
==13005==    at 0x402F26F: operator new(unsigned long) (vg_replace_malloc.c:334)                                                                                                
==13005==    by 0xB5A3FCF: void* llvm::object_creator<(anonymous namespace)::CommandLineParser>() (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_llvm-570aa8bfbfd7daff.so)   
==13005==    by 0xB5C42C1: llvm::ManagedStaticBase::RegisterManagedStatic(void* (*)(), void (*)(void*)) const (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_llvm-570aa8bfbfd
7daff.so)                                                                                                                                                                          
==13005==    by 0xB599F95: llvm::ManagedStatic<(anonymous namespace)::CommandLineParser>::operator*() [clone .constprop.394] (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_l
lvm-570aa8bfbfd7daff.so)                                                                                                                                                           
==13005==    by 0xB5A378F: llvm::cl::Option::addArgument() (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_llvm-570aa8bfbfd7daff.so)                                          
==13005==    by 0x9CBCA1D: _GLOBAL__sub_I_X86AsmInstrumentation.cpp (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_llvm-570aa8bfbfd7daff.so)                                 
==13005==    by 0x4010EEF: call_init (dl-init.c:72)                                                                                                                                
==13005==    by 0x4010EEF: _dl_init (dl-init.c:120)                                                                                                                                
==13005==    by 0x4000C59: ??? (in /lib64/ld-2.25.90.so)                                                                                                                           
==13005==    by 0x3: ???                                                                                                                                                           
==13005==    by 0xFFF000806: ???                                                                                                                                                   
==13005==    by 0xFFF00080E: ???                                                                                                                                                   
==13005==    by 0xFFF00081B: ???                                                                                                                                                   
==13005==                                                                                                                                                                          
==13005== Use of uninitialised value of size 8                                                                                                                                     
==13005==    at 0x4E6AA2D: je_index2size_lookup (jemalloc_internal.h:753)                                                                                                          
==13005==    by 0x4E6AA2D: je_index2size (jemalloc_internal.h:763)                                                                                                                 
==13005==    by 0x4E6AA2D: je_arena_salloc (arena.h:1414)                                                                                                                          
==13005==    by 0x4E6AA2D: je_isalloc (jemalloc_internal.h:1054)                    
==13005==    by 0x4E6AA2D: rallocx (jemalloc.c:2419)                                                                                                                            
==13005==    by 0x923E13E: _$LT$alloc..raw_vec..RawVec$LT$T$GT$$GT$::double::h746fadf638d3a65f (in /home/markus/tmp/tmp/rust/usr/local/lib/libsyntax_pos-364f7f3b054e6796.so)   
==13005==    by 0x9241895: syntax_pos::symbol::Interner::intern::hf484e790b0aa4b15 (in /home/markus/tmp/tmp/rust/usr/local/lib/libsyntax_pos-364f7f3b054e6796.so)                  
==13005==    by 0x92410F5: syntax_pos::symbol::Symbol::intern::h50fea05cde2cbf92 (in /home/markus/tmp/tmp/rust/usr/local/lib/libsyntax_pos-364f7f3b054e6796.so)
==13005==    by 0x7351810: serialize::serialize::Decoder::read_enum_variant_arg::h81db90fbf83c26b7 (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_metadata-d3323a2104c2ad27.s
o)                                                                                                                                                                                
==13005==    by 0x732FB3C: _$LT$collections..vec..Vec$LT$T$GT$$u20$as$u20$serialize..serialize..Decodable$GT$::decode::h8566cacadf292514 (in /home/markus/tmp/tmp/rust/usr/local/li
b/librustc_metadata-d3323a2104c2ad27.so)                                                                                                          
==13005==    by 0x73A5DFB: rustc_metadata::creader::CrateLoader::register_crate::h463e5f5a7d0801c5 (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_metadata-d3323a2104c2ad27.s
o)                                                                           
==13005==    by 0x73A958C: rustc_metadata::creader::CrateLoader::resolve_crate::h5893dde60755de0d (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_metadata-d3323a2104c2ad27.so
)                       
==13005==    by 0x73AA39B: rustc_metadata::creader::CrateLoader::resolve_crate_deps::_$u7b$$u7b$closure$u7d$$u7d$::h316dcdec17504bb5 (in /home/markus/tmp/tmp/rust/usr/local/lib/li
brustc_metadata-d3323a2104c2ad27.so)                  
==13005==    by 0x7343F0F: _$LT$collections..vec..Vec$LT$T$GT$$u20$as$u20$collections..vec..SpecExtend$LT$T$C$$u20$I$GT$$GT$::spec_extend::hdacb47417427d83f (in /home/markus/tmp/t
mp/rust/usr/local/lib/librustc_metadata-d3323a2104c2ad27.so)
==13005==    by 0x73A5D11: rustc_metadata::creader::CrateLoader::register_crate::h463e5f5a7d0801c5 (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_metadata-d3323a2104c2ad27.s
o)                                                                          
==13005==    by 0x73A958C: rustc_metadata::creader::CrateLoader::resolve_crate::h5893dde60755de0d (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_metadata-d3323a2104c2ad27.so
)                                                                     
==13005==  Uninitialised value was created by a heap allocation                                                                                                              
==13005==    at 0x402F26F: operator new(unsigned long) (vg_replace_malloc.c:334)                                                                                 
==13005==    by 0xB5A3FCF: void* llvm::object_creator<(anonymous namespace)::CommandLineParser>() (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_llvm-570aa8bfbfd7daff.so)
==13005==    by 0xB5C42C1: llvm::ManagedStaticBase::RegisterManagedStatic(void* (*)(), void (*)(void*)) const (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_llvm-570aa8bfbfd
7daff.so)       
==13005==    by 0xB599F95: llvm::ManagedStatic<(anonymous namespace)::CommandLineParser>::operator*() [clone .constprop.394] (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_l
lvm-570aa8bfbfd7daff.so)                                                                                                                                                          
==13005==    by 0xB5A378F: llvm::cl::Option::addArgument() (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_llvm-570aa8bfbfd7daff.so)
==13005==    by 0x9CBCA1D: _GLOBAL__sub_I_X86AsmInstrumentation.cpp (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_llvm-570aa8bfbfd7daff.so)
==13005==    by 0x4010EEF: call_init (dl-init.c:72)
==13005==    by 0x4010EEF: _dl_init (dl-init.c:120)
==13005==    by 0x4000C59: ??? (in /lib64/ld-2.25.90.so)
==13005==    by 0x3: ???
==13005==    by 0xFFF000806: ???
==13005==    by 0xFFF00080E: ???
==13005==    by 0xFFF00081B: ???
==13005==
==13005== Conditional jump or move depends on uninitialised value(s)
==13005==    at 0x4E6A973: je_arena_salloc (arena.h:1388)
==13005==    by 0x4E6A973: je_isalloc (jemalloc_internal.h:1054)
==13005==    by 0x4E6A973: rallocx (jemalloc.c:2401)
==13005==    by 0x923E13E: _$LT$alloc..raw_vec..RawVec$LT$T$GT$$GT$::double::h746fadf638d3a65f (in /home/markus/tmp/tmp/rust/usr/local/lib/libsyntax_pos-364f7f3b054e6796.so)
==13005==    by 0x9241895: syntax_pos::symbol::Interner::intern::hf484e790b0aa4b15 (in /home/markus/tmp/tmp/rust/usr/local/lib/libsyntax_pos-364f7f3b054e6796.so)
==13005==    by 0x92410F5: syntax_pos::symbol::Symbol::intern::h50fea05cde2cbf92 (in /home/markus/tmp/tmp/rust/usr/local/lib/libsyntax_pos-364f7f3b054e6796.so)
==13005==    by 0x7351810: serialize::serialize::Decoder::read_enum_variant_arg::h81db90fbf83c26b7 (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_metadata-d3323a2104c2ad27.s
o)
==13005==    by 0x732FB3C: _$LT$collections..vec..Vec$LT$T$GT$$u20$as$u20$serialize..serialize..Decodable$GT$::decode::h8566cacadf292514 (in /home/markus/tmp/tmp/rust/usr/local/li
b/librustc_metadata-d3323a2104c2ad27.so)
==13005==    by 0x73A5DFB: rustc_metadata::creader::CrateLoader::register_crate::h463e5f5a7d0801c5 (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_metadata-d3323a2104c2ad27.s
o)
==13005==    by 0x73A958C: rustc_metadata::creader::CrateLoader::resolve_crate::h5893dde60755de0d (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_metadata-d3323a2104c2ad27.so
)
==13005==    by 0x73AA39B: rustc_metadata::creader::CrateLoader::resolve_crate_deps::_$u7b$$u7b$closure$u7d$$u7d$::h316dcdec17504bb5 (in /home/markus/tmp/tmp/rust/usr/local/lib/li
brustc_metadata-d3323a2104c2ad27.so)
==13005==    by 0x7343F0F: _$LT$collections..vec..Vec$LT$T$GT$$u20$as$u20$collections..vec..SpecExtend$LT$T$C$$u20$I$GT$$GT$::spec_extend::hdacb47417427d83f (in /home/markus/tmp/t
mp/rust/usr/local/lib/librustc_metadata-d3323a2104c2ad27.so)
==13005==    by 0x73A5D11: rustc_metadata::creader::CrateLoader::register_crate::h463e5f5a7d0801c5 (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_metadata-d3323a2104c2ad27.s
o)
==13005==    by 0x73A958C: rustc_metadata::creader::CrateLoader::resolve_crate::h5893dde60755de0d (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_metadata-d3323a2104c2ad27.so
)
==13005==  Uninitialised value was created by a heap allocation
==13005==    at 0x402F26F: operator new(unsigned long) (vg_replace_malloc.c:334)
==13005==    by 0xB5A3FCF: void* llvm::object_creator<(anonymous namespace)::CommandLineParser>() (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_llvm-570aa8bfbfd7daff.so)
==13005==    by 0xB5C42C1: llvm::ManagedStaticBase::RegisterManagedStatic(void* (*)(), void (*)(void*)) const (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_llvm-570aa8bfbfd
7daff.so)
==13005==    by 0xB599F95: llvm::ManagedStatic<(anonymous namespace)::CommandLineParser>::operator*() [clone .constprop.394] (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_l
lvm-570aa8bfbfd7daff.so)
==13005==    by 0xB5A378F: llvm::cl::Option::addArgument() (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_llvm-570aa8bfbfd7daff.so)
==13005==    by 0x9CBCA1D: _GLOBAL__sub_I_X86AsmInstrumentation.cpp (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_llvm-570aa8bfbfd7daff.so)
==13005==    by 0x4010EEF: call_init (dl-init.c:72)
==13005==    by 0x4010EEF: _dl_init (dl-init.c:120)
==13005==    by 0x4000C59: ??? (in /lib64/ld-2.25.90.so)
==13005==    by 0x3: ???
==13005==    by 0xFFF000806: ???
==13005==    by 0xFFF00080E: ???
==13005==    by 0xFFF00081B: ???            
==13005==                                   
==13005== Use of uninitialised value of size 8                                           
==13005==    at 0x4E6A980: je_index2size_lookup (jemalloc_internal.h:753)                
==13005==    by 0x4E6A980: je_index2size (jemalloc_internal.h:763)                       
==13005==    by 0x4E6A980: je_arena_salloc (arena.h:1414)                                
==13005==    by 0x4E6A980: je_isalloc (jemalloc_internal.h:1054)                         
==13005==    by 0x4E6A980: rallocx (jemalloc.c:2401)                                     
==13005==    by 0x923E13E: _$LT$alloc..raw_vec..RawVec$LT$T$GT$$GT$::double::h746fadf638d3a65f (in /home/markus/tmp/tmp/rust/usr/local/lib/libsyntax_pos-364f7f3b054e6796.so)      
==13005==    by 0x9241895: syntax_pos::symbol::Interner::intern::hf484e790b0aa4b15 (in /home/markus/tmp/tmp/rust/usr/local/lib/libsyntax_pos-364f7f3b054e6796.so)                  
==13005==    by 0x92410F5: syntax_pos::symbol::Symbol::intern::h50fea05cde2cbf92 (in /home/markus/tmp/tmp/rust/usr/local/lib/libsyntax_pos-364f7f3b054e6796.so)                    
==13005==    by 0x7351810: serialize::serialize::Decoder::read_enum_variant_arg::h81db90fbf83c26b7 (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_metadata-d3323a2104c2ad27.so)                                          
==13005==    by 0x732FB3C: _$LT$collections..vec..Vec$LT$T$GT$$u20$as$u20$serialize..serialize..Decodable$GT$::decode::h8566cacadf292514 (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_metadata-d3323a2104c2ad27.so)    
==13005==    by 0x73A5DFB: rustc_metadata::creader::CrateLoader::register_crate::h463e5f5a7d0801c5 (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_metadata-d3323a2104c2ad27.so)                                          
==13005==    by 0x73A958C: rustc_metadata::creader::CrateLoader::resolve_crate::h5893dde60755de0d (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_metadata-d3323a2104c2ad27.so)                                           
==13005==    by 0x73AA39B: rustc_metadata::creader::CrateLoader::resolve_crate_deps::_$u7b$$u7b$closure$u7d$$u7d$::h316dcdec17504bb5 (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_metadata-d3323a2104c2ad27.so)        
==13005==    by 0x7343F0F: _$LT$collections..vec..Vec$LT$T$GT$$u20$as$u20$collections..vec..SpecExtend$LT$T$C$$u20$I$GT$$GT$::spec_extend::hdacb47417427d83f (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_metadata-d3323a2104c2ad27.so)                             
==13005==    by 0x73A5D11: rustc_metadata::creader::CrateLoader::register_crate::h463e5f5a7d0801c5 (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_metadata-d3323a2104c2ad27.so)                                          
==13005==    by 0x73A958C: rustc_metadata::creader::CrateLoader::resolve_crate::h5893dde60755de0d (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_metadata-d3323a2104c2ad27.so)                                           
==13005==  Uninitialised value was created by a heap allocation                          
==13005==    at 0x402F26F: operator new(unsigned long) (vg_replace_malloc.c:334)         
==13005==    by 0xB5A3FCF: void* llvm::object_creator<(anonymous namespace)::CommandLineParser>() (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_llvm-570aa8bfbfd7daff.so)   
==13005==    by 0xB5C42C1: llvm::ManagedStaticBase::RegisterManagedStatic(void* (*)(), void (*)(void*)) const (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_llvm-570aa8bfbfd7daff.so)                                   
==13005==    by 0xB599F95: llvm::ManagedStatic<(anonymous namespace)::CommandLineParser>::operator*() [clone .constprop.394] (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_llvm-570aa8bfbfd7daff.so)                    
==13005==    by 0xB5A378F: llvm::cl::Option::addArgument() (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_llvm-570aa8bfbfd7daff.so)                                          
==13005==    by 0x9CBCA1D: _GLOBAL__sub_I_X86AsmInstrumentation.cpp (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_llvm-570aa8bfbfd7daff.so)                                 
==13005==    by 0x4010EEF: call_init (dl-init.c:72)                                      
==13005==    by 0x4010EEF: _dl_init (dl-init.c:120)                                      
==13005==    by 0x4000C59: ??? (in /lib64/ld-2.25.90.so)                                 
==13005==    by 0x3: ???                    
==13005==    by 0xFFF000806: ???            
==13005==    by 0xFFF00080E: ???            
==13005==    by 0xFFF00081B: ???            
==13005==                                   
==13005== Invalid read of size 1            
==13005==    at 0x73A65BC: rustc_metadata::creader::CrateLoader::register_crate::h463e5f5a7d0801c5 (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_metadata-d3323a2104c2ad27.so)                                          
==13005==    by 0x73A958C: rustc_metadata::creader::CrateLoader::resolve_crate::h5893dde60755de0d (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_metadata-d3323a2104c2ad27.so)                                           
==13005==    by 0x73AA39B: rustc_metadata::creader::CrateLoader::resolve_crate_deps::_$u7b$$u7b$closure$u7d$$u7d$::h316dcdec17504bb5 (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_metadata-d3323a2104c2ad27.so)        
==13005==    by 0x7343F0F: _$LT$collections..vec..Vec$LT$T$GT$$u20$as$u20$collections..vec..SpecExtend$LT$T$C$$u20$I$GT$$GT$::spec_extend::hdacb47417427d83f (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_metadata-d3323a2104c2ad27.so)                             
==13005==    by 0x73A5D11: rustc_metadata::creader::CrateLoader::register_crate::h463e5f5a7d0801c5 (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_metadata-d3323a2104c2ad27.so)                                          
==13005==    by 0x73A958C: rustc_metadata::creader::CrateLoader::resolve_crate::h5893dde60755de0d (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_metadata-d3323a2104c2ad27.so)                                           
==13005==    by 0x73AF6A5: _$LT$rustc_metadata..creader..CrateLoader$LT$$u27$a$GT$$u20$as$u20$rustc..middle..cstore..CrateLoader$GT$::process_item::h1dc2d8cb217054ac (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_metadata-d3323a2104c2ad27.so)                    
==13005==    by 0x6842591: rustc_resolve::build_reduced_graph::_$LT$impl$u20$rustc_resolve..Resolver$LT$$u27$a$GT$$GT$::build_reduced_graph_for_item::hfda070be51758377 (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_resolve-97b73742dfc67e0f.so)                   
==13005==    by 0x684847A: _$LT$rustc_resolve..build_reduced_graph..BuildReducedGraphVisitor$LT$$u27$a$C$$u20$$u27$b$GT$$u20$as$u20$syntax..visit..Visitor$LT$$u27$a$GT$$GT$::visit_item::hd7ab9eb4e32f5ff3 (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_resolve-97b73742dfc67e0f.so)                                                                         
==13005==    by 0x684887F: _$LT$rustc_resolve..build_reduced_graph..BuildReducedGraphVisitor$LT$$u27$a$C$$u20$$u27$b$GT$$u20$as$u20$syntax..visit..Visitor$LT$$u27$a$GT$$GT$::visit_item::hd7ab9eb4e32f5ff3 (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_resolve-97b73742dfc67e0f.so)                                                                         
==13005==    by 0x683A18F: rustc_resolve::macros::_$LT$impl$u20$syntax..ext..base..Resolver$u20$for$u20$rustc_resolve..Resolver$LT$$u27$a$GT$$GT$::visit_expansion::h834650b395834280 (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_resolve-97b73742dfc67e0f.so)     
==13005==    by 0x8AFD7E8: syntax::ext::expand::MacroExpander::collect_invocations::hfa379968c93659ec (in /home/markus/tmp/tmp/rust/usr/local/lib/libsyntax-93b867817c21b413.so)   
==13005==  Address 0xb is not stack'd, malloc'd or (recently) free'd                     
==13005==                                   
==13005==                                   
==13005== Process terminating with default action of signal 11 (SIGSEGV)                 
==13005==  Access not within mapped region at address 0xB                                
==13005==    at 0x73A65BC: rustc_metadata::creader::CrateLoader::register_crate::h463e5f5a7d0801c5 (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_metadata-d3323a2104c2ad27.so)                                          
==13005==    by 0x73A958C: rustc_metadata::creader::CrateLoader::resolve_crate::h5893dde60755de0d (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_metadata-d3323a2104c2ad27.so)                                           
==13005==    by 0x73AA39B: rustc_metadata::creader::CrateLoader::resolve_crate_deps::_$u7b$$u7b$closure$u7d$$u7d$::h316dcdec17504bb5 (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_metadata-d3323a2104c2ad27.so)        
==13005==    by 0x7343F0F: _$LT$collections..vec..Vec$LT$T$GT$$u20$as$u20$collections..vec..SpecExtend$LT$T$C$$u20$I$GT$$GT$::spec_extend::hdacb47417427d83f (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_metadata-d3323a2104c2ad27.so)                             
==13005==    by 0x73A5D11: rustc_metadata::creader::CrateLoader::register_crate::h463e5f5a7d0801c5 (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_metadata-d3323a2104c2ad27.so)                                          
==13005==    by 0x73A958C: rustc_metadata::creader::CrateLoader::resolve_crate::h5893dde60755de0d (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_metadata-d3323a2104c2ad27.so)                                           
==13005==    by 0x73AF6A5: _$LT$rustc_metadata..creader..CrateLoader$LT$$u27$a$GT$$u20$as$u20$rustc..middle..cstore..CrateLoader$GT$::process_item::h1dc2d8cb217054ac (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_metadata-d3323a2104c2ad27.so)                    
==13005==    by 0x6842591: rustc_resolve::build_reduced_graph::_$LT$impl$u20$rustc_resolve..Resolver$LT$$u27$a$GT$$GT$::build_reduced_graph_for_item::hfda070be51758377 (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_resolve-97b73742dfc67e0f.so)                   
==13005==    by 0x684847A: _$LT$rustc_resolve..build_reduced_graph..BuildReducedGraphVisitor$LT$$u27$a$C$$u20$$u27$b$GT$$u20$as$u20$syntax..visit..Visitor$LT$$u27$a$GT$$GT$::visit_item::hd7ab9eb4e32f5ff3 (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_resolve-97b73742dfc67e0f.so)                                                                         
==13005==    by 0x684887F: _$LT$rustc_resolve..build_reduced_graph..BuildReducedGraphVisitor$LT$$u27$a$C$$u20$$u27$b$GT$$u20$as$u20$syntax..visit..Visitor$LT$$u27$a$GT$$GT$::visit_item::hd7ab9eb4e32f5ff3 (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_resolve-97b73742dfc67e0f.so)                                                                         
==13005==    by 0x683A18F: rustc_resolve::macros::_$LT$impl$u20$syntax..ext..base..Resolver$u20$for$u20$rustc_resolve..Resolver$LT$$u27$a$GT$$GT$::visit_expansion::h834650b395834280 (in /home/markus/tmp/tmp/rust/usr/local/lib/librustc_resolve-97b73742dfc67e0f.so)     
==13005==    by 0x8AFD7E8: syntax::ext::expand::MacroExpander::collect_invocations::hfa379968c93659ec (in /home/markus/tmp/tmp/rust/usr/local/lib/libsyntax-93b867817c21b413.so)   
==13005==  If you believe this happened as a result of a stack                           
==13005==  overflow in your program's main thread (unlikely but                          
==13005==  possible), you can try to increase the size of the                            
==13005==  main thread stack using the --main-stacksize= flag.                           
==13005==  The main thread stack size used in this run was 8388608.                      
==13005==                                   
==13005== HEAP SUMMARY:                     
==13005==     in use at exit: 226,671 bytes in 1,670 blocks                              
==13005==   total heap usage: 1,769 allocs, 99 frees, 419,933 bytes allocated            
==13005==                                   
==13005== LEAK SUMMARY:                     
==13005==    definitely lost: 744 bytes in 1 blocks                                      
==13005==    indirectly lost: 7,976 bytes in 14 blocks                                   
==13005==      possibly lost: 384 bytes in 1 blocks                                      
==13005==    still reachable: 217,567 bytes in 1,654 blocks                              
==13005==                       of which reachable via heuristic:                        
==13005==                         stdstring          : 163 bytes in 5 blocks             
==13005==         suppressed: 0 bytes in 0 blocks                                        
==13005== Rerun with --leak-check=full to see details of leaked memory                   
==13005==                                   
==13005== For counts of detected and suppressed errors, rerun with: -v                   
==13005== ERROR SUMMARY: 7 errors from 6 contexts (suppressed: 0 from 0) 
