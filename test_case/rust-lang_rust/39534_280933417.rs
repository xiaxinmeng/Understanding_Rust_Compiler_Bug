
error: linking with `cc` failed: exit code: 1
  |
  = note: "cc" "-m64" "-L" "/Users/eric/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib" "/Users/eric/Temp/rust/scratch-lto/target/debug/deps/scratch_lto-21b84faa2393c01d.core-any.volatile.o" "/Users/eric/Temp/rust/scratch-lto/target/debug/deps/scratch_lto-21b84faa2393c01d.scratch_lto.o" "/Users/eric/Temp/rust/scratch-lto/target/debug/deps/scratch_lto-21b84faa2393c01d.std-panicking.volatile.o" "-o" "/Users/eric/Temp/rust/scratch-lto/target/debug/deps/scratch_lto-21b84faa2393c01d" "-Wl,-dead_strip" "-nodefaultlibs" "-L" "/Users/eric/Temp/rust/scratch-lto/target/debug/deps" "-L" "/Users/eric/.rustup/toolchains/nightly-x86_64-apple-darwin/lib/rustlib/x86_64-apple-darwin/lib" "/var/folders/g1/jk6mlnz132g4wvw2g90hfzk40000gp/T/rustc.tj6lnwuKsdHN/liballoc_jemalloc-eeea41b1054fd303.rlib" "/var/folders/g1/jk6mlnz132g4wvw2g90hfzk40000gp/T/rustc.tj6lnwuKsdHN/libcompiler_builtins-a156453e4a4418dc.rlib" "-l" "System" "-l" "pthread" "-l" "c" "-l" "m"
  = note: Undefined symbols for architecture x86_64:
            "_rust_eh_personality", referenced from:
                Dwarf Exception Unwind Info (__eh_frame) in scratch_lto-21b84faa2393c01d.std-panicking.volatile.o
            "___rust_allocate", referenced from:
                alloc::heap::allocate::h4eb83c87d1ceebe0 in scratch_lto-21b84faa2393c01d.std-panicking.volatile.o
            "alloc::oom::oom::hafa494cd9e3b81ad", referenced from:
                alloc::heap::exchange_malloc::hde5cad0a985c0ad1 in scratch_lto-21b84faa2393c01d.std-panicking.volatile.o
            "std::panicking::rust_panic_with_hook::h8e6300d8e8aca457", referenced from:
                std::panicking::begin_panic::h9ec63481878b4a71 in scratch_lto-21b84faa2393c01d.std-panicking.volatile.o
            "std::rt::lang_start::he28fbf70e98a2b18", referenced from:
                _main in scratch_lto-21b84faa2393c01d.scratch_lto.o
          ld: symbol(s) not found for architecture x86_64
          clang: error: linker command failed with exit code 1 (use -v to see invocation)
