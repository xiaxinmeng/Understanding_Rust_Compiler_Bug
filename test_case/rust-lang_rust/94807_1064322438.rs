plain
running: "x86_64-fuchsia-clang++" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-I" "/checkout/src/llvm-project/libunwind/include" "-nostdinc++" "-fno-exceptions" "-fno-rtti" "-fstrict-aliasing" "-funwind-tables" "-fvisibility=hidden" "-D_LIBUNWIND_DISABLE_VISIBILITY_ANNOTATIONS" "-o" "/checkout/obj/build/x86_64-fuchsia/native/libunwind/Unwind-EHABI.o" "-c" "/checkout/src/llvm-project/libunwind/src/Unwind-EHABI.cpp"
exit status: 0
running: "x86_64-fuchsia-clang++" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-I" "/checkout/src/llvm-project/libunwind/include" "-nostdinc++" "-fno-exceptions" "-fno-rtti" "-fstrict-aliasing" "-funwind-tables" "-fvisibility=hidden" "-D_LIBUNWIND_DISABLE_VISIBILITY_ANNOTATIONS" "-o" "/checkout/obj/build/x86_64-fuchsia/native/libunwind/Unwind-seh.o" "-c" "/checkout/src/llvm-project/libunwind/src/Unwind-seh.cpp"
exit status: 0
running: "x86_64-fuchsia-clang++" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-I" "/checkout/src/llvm-project/libunwind/include" "-nostdinc++" "-fno-exceptions" "-fno-rtti" "-fstrict-aliasing" "-funwind-tables" "-fvisibility=hidden" "-D_LIBUNWIND_DISABLE_VISIBILITY_ANNOTATIONS" "-o" "/checkout/obj/build/x86_64-fuchsia/native/libunwind/libunwind.o" "-c" "/checkout/src/llvm-project/libunwind/src/libunwind.cpp"
cargo:warning=In file included from /checkout/src/llvm-project/libunwind/src/libunwind.cpp:30:
cargo:warning=In file included from /checkout/src/llvm-project/libunwind/src/AddressSpace.hpp:23:
cargo:warning=In file included from /checkout/src/llvm-project/libunwind/src/EHHeaderParser.hpp:17:
cargo:warning=In file included from /checkout/src/llvm-project/libunwind/src/DwarfParser.hpp:22:
cargo:warning=/checkout/src/llvm-project/libunwind/src/Registers.hpp:116:3: error: use of undeclared identifier 'static_assert'; did you mean 'static_cast'?
cargo:warning=  static_assert((check_fit<Registers_x86, unw_context_t>::does_fit),
cargo:warning=  ^
cargo:warning=/checkout/src/llvm-project/libunwind/src/Registers.hpp:342:3: error: use of undeclared identifier 'static_assert'; did you mean 'static_cast'?
cargo:warning=  static_assert((check_fit<Registers_x86_64, unw_context_t>::does_fit),
cargo:warning=  ^
cargo:warning=/checkout/src/llvm-project/libunwind/src/Registers.hpp:670:3: error: use of undeclared identifier 'static_assert'; did you mean 'static_cast'?
cargo:warning=  static_assert((check_fit<Registers_ppc, unw_context_t>::does_fit),
cargo:warning=  ^
cargo:warning=/checkout/src/llvm-project/libunwind/src/Registers.hpp:674:3: error: use of undeclared identifier 'static_assert'; did you mean 'static_cast'?
cargo:warning=  static_assert(sizeof(ppc_thread_state_t) == 160,
cargo:warning=  ^
cargo:warning=/checkout/src/llvm-project/libunwind/src/Registers.hpp:679:3: error: use of undeclared identifier 'static_assert'; did you mean 'static_cast'?
cargo:warning=  static_assert(sizeof(ppc_thread_state_t) + sizeof(ppc_float_state_t) == 424,
cargo:warning=  ^
cargo:warning=/checkout/src/llvm-project/libunwind/src/Registers.hpp:1237:3: error: use of undeclared identifier 'static_assert'; did you mean 'static_cast'?
cargo:warning=  static_assert((check_fit<Registers_ppc64, unw_context_t>::does_fit),
cargo:warning=  ^
cargo:warning=/checkout/src/llvm-project/libunwind/src/Registers.hpp:1241:3: error: use of undeclared identifier 'static_assert'; did you mean 'static_cast'?
cargo:warning=  static_assert(sizeof(_registers) == 312,
cargo:warning=  ^
cargo:warning=/checkout/src/llvm-project/libunwind/src/Registers.hpp:1246:3: error: use of undeclared identifier 'static_assert'; did you mean 'static_cast'?
cargo:warning=  static_assert(sizeof(_registers) +
cargo:warning=  ^
cargo:warning=/checkout/src/llvm-project/libunwind/src/Registers.hpp:1845:3: error: use of undeclared identifier 'static_assert'; did you mean 'static_cast'?
cargo:warning=  static_assert((check_fit<Registers_arm64, unw_context_t>::does_fit),
cargo:warning=  ^
cargo:warning=/checkout/src/llvm-project/libunwind/src/Registers.hpp:1848:3: error: use of undeclared identifier 'static_assert'; did you mean 'static_cast'?
cargo:warning=  static_assert(sizeof(GPRs) == 0x110,
cargo:warning=  ^
cargo:warning=/checkout/src/llvm-project/libunwind/src/Registers.hpp:2198:3: error: use of undeclared identifier 'static_assert'; did you mean 'static_cast'?
cargo:warning=  static_assert((check_fit<Registers_arm, unw_context_t>::does_fit),
cargo:warning=  ^
cargo:warning=/checkout/src/llvm-project/libunwind/src/Registers.hpp:2625:3: error: use of undeclared identifier 'static_assert'; did you mean 'static_cast'?
cargo:warning=  static_assert((check_fit<Registers_or1k, unw_context_t>::does_fit),
cargo:warning=  ^
cargo:warning=/checkout/src/llvm-project/libunwind/src/Registers.hpp:2831:3: error: use of undeclared identifier 'static_assert'; did you mean 'static_cast'?
cargo:warning=  static_assert((check_fit<Registers_mips_o32, unw_context_t>::does_fit),
cargo:warning=  ^
cargo:warning=/checkout/src/llvm-project/libunwind/src/Registers.hpp:3153:3: error: use of undeclared identifier 'static_assert'; did you mean 'static_cast'?
cargo:warning=  static_assert((check_fit<Registers_mips_newabi, unw_context_t>::does_fit),
cargo:warning=  ^
cargo:warning=/checkout/src/llvm-project/libunwind/src/Registers.hpp:3442:3: error: use of undeclared identifier 'static_assert'; did you mean 'static_cast'?
cargo:warning=  static_assert((check_fit<Registers_sparc, unw_context_t>::does_fit),
cargo:warning=  ^
cargo:warning=/checkout/src/llvm-project/libunwind/src/Registers.hpp:3595:25: warning: defaulted function definitions are a C++11 extension [-Wc++11-extensions]
cargo:warning=  Registers_sparc64() = default;
cargo:warning=                        ^
cargo:warning=/checkout/src/llvm-project/libunwind/src/Registers.hpp:3625:26: error: function definition does not declare parameters
cargo:warning=  sparc64_thread_state_t _registers{};
cargo:warning=                         ^
cargo:warning=/checkout/src/llvm-project/libunwind/src/Registers.hpp:3626:21: warning: in-class initialization of non-static data member is a C++11 extension [-Wc++11-extensions]
cargo:warning=  uint64_t _wcookie = 0;
cargo:warning=                    ^
cargo:warning=/checkout/src/llvm-project/libunwind/src/Registers.hpp:3614:35: error: use of undeclared identifier '_registers'
cargo:warning=  uint64_t getSP() const { return _registers.__regs[UNW_SPARC_O6] + 2047; }
cargo:warning=                                  ^
cargo:warning=/checkout/src/llvm-project/libunwind/src/Registers.hpp:3615:32: error: use of undeclared identifier '_registers'
cargo:warning=  void setSP(uint64_t value) { _registers.__regs[UNW_SPARC_O6] = value - 2047; }
cargo:warning=                               ^
cargo:warning=/checkout/src/llvm-project/libunwind/src/Registers.hpp:3616:35: error: use of undeclared identifier '_registers'
cargo:warning=  uint64_t getIP() const { return _registers.__regs[UNW_SPARC_O7]; }
cargo:warning=                                  ^
cargo:warning=fatal error: too many errors emitted, stopping now [-ferror-limit=]
cargo:warning=2 warnings and 20 errors generated.
exit status: 1


error occurred: Command "x86_64-fuchsia-clang++" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "--target=x86_64-fuchsia" "-I" "/checkout/src/llvm-project/libunwind/include" "-nostdinc++" "-fno-exceptions" "-fno-rtti" "-fstrict-aliasing" "-funwind-tables" "-fvisibility=hidden" "-D_LIBUNWIND_DISABLE_VISIBILITY_ANNOTATIONS" "-o" "/checkout/obj/build/x86_64-fuchsia/native/libunwind/libunwind.o" "-c" "/checkout/src/llvm-project/libunwind/src/libunwind.cpp" with args "x86_64-fuchsia-clang++" did not execute successfully (status code exit status: 1).

Build completed unsuccessfully in 0:09:25
