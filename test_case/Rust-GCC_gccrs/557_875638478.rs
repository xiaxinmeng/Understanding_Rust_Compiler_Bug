
Program received signal SIGSEGV, Segmentation fault.
Rust::HIR::LetStmt::as_string[abi:cxx11]() const (this=0x3d15fc0) at ../../gccrs/gcc/rust/hir/tree/rust-hir-full-test.cc:2705
2705      str += "\n" + indent_spaces (stay) + "let " + variables_pattern->as_string ();
(gdb) bt
#0  Rust::HIR::LetStmt::as_string[abi:cxx11]() const (this=0x3d15fc0) at ../../gccrs/gcc/rust/hir/tree/rust-hir-full-test.cc:2705
#1  0x0000000000fd4b7d in Rust::HIR::BlockExpr::as_string[abi:cxx11]() const (this=0x3cdd300) at ../../gccrs/gcc/rust/hir/tree/rust-hir-full-test.cc:985
#2  0x0000000000fd4498 in Rust::HIR::Function::as_string[abi:cxx11]() const (this=0x3ce15b0) at ../../gccrs/gcc/rust/hir/tree/rust-hir-full-test.cc:917
#3  0x0000000000fd09df in Rust::HIR::Crate::as_string[abi:cxx11]() const (this=0x7fffffffd520) at ../../gccrs/gcc/rust/hir/tree/rust-hir-full-test.cc:111
#4  0x0000000000eeac33 in Rust::Session::dump_hir (this=0x3b31820 <session>, hir=...) at ../../gccrs/gcc/rust/rust-session-manager.cc:891
#5  0x0000000000ee95ad in Rust::Session::parse_file (this=0x3b31820 <session>, filename=0x7fffffffe029 "test.rs") at ../../gccrs/gcc/rust/rust-session-manager.cc:562
#6  0x0000000000ee915d in Rust::Session::parse_files (this=0x3b31820 <session>, num_files=1, files=0x3cf1a20) at ../../gccrs/gcc/rust/rust-session-manager.cc:459
#7  0x0000000000e45265 in grs_langhook_parse_file () at ../../gccrs/gcc/rust/rust-lang.cc:171
#8  0x00000000018727b7 in compile_file () at ../../gccrs/gcc/toplev.c:457
#9  0x0000000001875a85 in do_compile () at ../../gccrs/gcc/toplev.c:2201
#10 0x0000000001875db4 in toplev::main (this=0x7fffffffdad6, argc=18, argv=0x7fffffffdbe8) at ../../gccrs/gcc/toplev.c:2340
#11 0x0000000002a7f1f9 in main (argc=18, argv=0x7fffffffdbe8) at ../../gccrs/gcc/main.c:39
