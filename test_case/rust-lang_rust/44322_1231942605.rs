sh
$ git clone https://github.com/CryZe/multi-lib-lto-rust-bug-repro.git
$ cd multi-lib-lto-rust-bug-repro/
$ make
gcc main.c -Llib1/target/release/ -Llib2/target/release/ -llib1 -llib2 -lpthread -ldl
lib2/target/release//liblib2.a(lib2-f2e7454a5f282da7.lib2.2398f8e6-cgu.1.rcgu.o): In function `rust_eh_personality':
/rustc/bc4b39c271bbd36736cbf1c0a1ac23d5df38d365/library/std/src/personality/gcc.rs:244: multiple definition of `rust_eh_personality'
lib1/target/release//liblib1.a(lib1-bfa64102e7d38c18.lib1.c0e19170-cgu.1.rcgu.o):/rustc/bc4b39c271bbd36736cbf1c0a1ac23d5df38d365/library/std/src/personality/gcc.rs:244: first defined here
lib2/target/release//liblib2.a(lib2-f2e7454a5f282da7.lib2.2398f8e6-cgu.1.rcgu.o):(.init_array.00099+0x0): multiple definition of `std::sys::unix::args::imp::ARGV_INIT_ARRAY'
lib1/target/release//liblib1.a(lib1-bfa64102e7d38c18.lib1.c0e19170-cgu.1.rcgu.o):(.init_array.00099+0x0): first defined here
