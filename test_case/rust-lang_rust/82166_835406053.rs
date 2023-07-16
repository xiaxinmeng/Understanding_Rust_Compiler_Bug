
running: "s390x-unknown-linux-musl-gcc" "-O0" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-I" "../../src/llvm-project/libunwind/include" "-std=c99" "-std=c++11" "-nostdinc++" "-fno-exceptions" "-fno-rtti" "-fstrict-aliasing" "-funwind-tables" "-fvisibility=hidden" "-D_LIBUNWIND_DISABLE_VISIBILITY_ANNOTATIONS" "-o" "/io/tests/target/s390x-unknown-linux-musl/debug/build/unwind-fe58df189b7622b8/out/../../src/llvm-project/libunwind/src/libunwind.o" "-c" "../../src/llvm-project/libunwind/src/libunwind.cpp"
  cargo:warning=cc1plus: warning: command line option '-std=c99' is valid for C/ObjC but not for C++
  cargo:warning=../../src/llvm-project/libunwind/src/libunwind.cpp:68:3: error: #error Architecture not supported
  cargo:warning=   68 | # error Architecture not supported
  cargo:warning=      |   ^~~~~
  cargo:warning=../../src/llvm-project/libunwind/src/libunwind.cpp: In function 'int __unw_init_local(unw_cursor_t*, unw_context_t*)':
  cargo:warning=../../src/llvm-project/libunwind/src/libunwind.cpp:71:57: error: 'REGISTER_KIND' was not declared in this scope
  cargo:warning=   71 |   new (reinterpret_cast<UnwindCursor<LocalAddressSpace, REGISTER_KIND> *>(cursor))
  cargo:warning=      |                                                         ^~~~~~~~~~~~~
  cargo:warning=../../src/llvm-project/libunwind/src/libunwind.cpp:71:70: error: template argument 2 is invalid
  cargo:warning=   71 |   new (reinterpret_cast<UnwindCursor<LocalAddressSpace, REGISTER_KIND> *>(cursor))
  cargo:warning=      |                                                                      ^
  cargo:warning=../../src/llvm-project/libunwind/src/libunwind.cpp:71:8: error: expected type-specifier before 'reinterpret_cast'
  cargo:warning=   71 |   new (reinterpret_cast<UnwindCursor<LocalAddressSpace, REGISTER_KIND> *>(cursor))
  cargo:warning=      |        ^~~~~~~~~~~~~~~~
  cargo:warning=../../src/llvm-project/libunwind/src/libunwind.cpp:71:8: error: expected ')' before 'reinterpret_cast'
  cargo:warning=   71 |   new (reinterpret_cast<UnwindCursor<LocalAddressSpace, REGISTER_KIND> *>(cursor))
  cargo:warning=      |       ~^~~~~~~~~~~~~~~~
  cargo:warning=      |        )
  exit status: 1

  --- stderr


  error occurred: Command "s390x-unknown-linux-musl-gcc" "-O0" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-I" "../../src/llvm-project/libunwind/include" "-std=c99" "-std=c++11" "-nostdinc++" "-fno-exceptions" "-fno-rtti" "-fstrict-aliasing" "-funwind-tables" "-fvisibility=hidden" "-D_LIBUNWIND_DISABLE_VISIBILITY_ANNOTATIONS" "-o" "/io/tests/target/s390x-unknown-linux-musl/debug/build/unwind-fe58df189b7622b8/out/../../src/llvm-project/libunwind/src/libunwind.o" "-c" "../../src/llvm-project/libunwind/src/libunwind.cpp" with args "s390x-unknown-linux-musl-gcc" did not execute successfully (status code exit status: 1).
