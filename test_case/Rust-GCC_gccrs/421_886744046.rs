
Program received signal SIGSEGV, Segmentation fault.                                                                                                                                                                                                   
Rust::AST::ExternalFunctionItem::as_string[abi:cxx11]() const (this=0x3d35f50) at ../../gccrs/gcc/rust/ast/rust-ast-full-test.cc:3598                                                                                                                  
3598      str += "\n (return) Type: " + return_type->as_string ();                                                                                                                                                                                     
(gdb) bt                                                                                                                   
#0  Rust::AST::ExternalFunctionItem::as_string[abi:cxx11]() const (this=0x3d35f50) at ../../gccrs/gcc/rust/ast/rust-ast-full-test.cc:3598
#1  0x0000000000e9e12a in Rust::AST::ExternBlock::as_string[abi:cxx11]() const (this=0x3cfc820) at ../../gccrs/gcc/rust/ast/rust-ast-full-test.cc:1316                                                                                                 
#2  0x0000000000e9875e in Rust::AST::Crate::as_string[abi:cxx11]() const (this=0x7fffffffd4e0) at ../../gccrs/gcc/rust/ast/rust-ast-full-test.cc:197                                                                                                   
#3  0x0000000000f2940e in Rust::Parser<Rust::Lexer>::debug_dump_ast_output (this=0x7fffffffd5d0, crate=..., out=...) at ../../gccrs/gcc/rust/parse/rust-parse-impl.h:14890                                                                             
#4  0x0000000000f0ea55 in Rust::Session::dump_ast (this=0x3b87820 <session>, parser=..., crate=...) at ../../gccrs/gcc/rust/rust-session-manager.cc:852                                                                                                
#5  0x0000000000f0d3fe in Rust::Session::parse_file (this=0x3b87820 <session>, filename=0x7fffffffe029 "test.rs") at ../../gccrs/gcc/rust/rust-session-manager.cc:494                                                                                  
#6  0x0000000000f0d1fb in Rust::Session::parse_files (this=0x3b87820 <session>, num_files=1, files=0x3d47a20) at ../../gccrs/gcc/rust/rust-session-manager.cc:458
#7  0x0000000000e68265 in grs_langhook_parse_file () at ../../gccrs/gcc/rust/rust-lang.cc:171
#8  0x00000000018bb23b in compile_file () at ../../gccrs/gcc/toplev.c:457
#9  0x00000000018be509 in do_compile () at ../../gccrs/gcc/toplev.c:2201
#10 0x00000000018be838 in toplev::main (this=0x7fffffffdad6, argc=18, argv=0x7fffffffdbe8) at ../../gccrs/gcc/toplev.c:2340
#11 0x0000000002ac7c7d in main (argc=18, argv=0x7fffffffdbe8) at ../../gccrs/gcc/main.c:39
