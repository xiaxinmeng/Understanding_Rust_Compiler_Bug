plain
   Compiling unwind v0.0.0 (/checkout/library/unwind)
   Compiling std v0.0.0 (/checkout/library/std)
The following warnings were emitted during compilation:

warning: In file included from /checkout/src/llvm-project/libunwind/src/DwarfParser.hpp:22:0,
warning:                  from /checkout/src/llvm-project/libunwind/src/EHHeaderParser.hpp:17,
warning:                  from /checkout/src/llvm-project/libunwind/src/AddressSpace.hpp:23,
warning:                  from /checkout/src/llvm-project/libunwind/src/libunwind.cpp:21:
warning: /checkout/src/llvm-project/libunwind/src/Registers.hpp: In constructor 'libunwind::Registers_x86::Registers_x86(const void*)':
warning: /checkout/src/llvm-project/libunwind/src/Registers.hpp:108:62: error: 'static_assert' was not declared in this scope
warning:                  "x86 registers do not fit into unw_context_t");
warning:                                                               ^
warning: /checkout/src/llvm-project/libunwind/src/Registers.hpp: In constructor 'libunwind::Registers_x86_64::Registers_x86_64(const void*)':
warning: /checkout/src/llvm-project/libunwind/src/Registers.hpp:327:65: error: 'static_assert' was not declared in this scope
warning:                  "x86_64 registers do not fit into unw_context_t");
warning:                                                                  ^
warning: /checkout/src/llvm-project/libunwind/src/Registers.hpp: In constructor 'libunwind::Registers_ppc::Registers_ppc(const void*)':
warning: /checkout/src/llvm-project/libunwind/src/Registers.hpp:652:62: error: 'static_assert' was not declared in this scope
warning:                  "ppc registers do not fit into unw_context_t");
warning:                                                               ^
warning: /checkout/src/llvm-project/libunwind/src/Registers.hpp: In constructor 'libunwind::Registers_ppc64::Registers_ppc64(const void*)':
warning: /checkout/src/llvm-project/libunwind/src/Registers.hpp:1219:64: error: 'static_assert' was not declared in this scope
warning:                  "ppc64 registers do not fit into unw_context_t");
warning:                                                                 ^
warning: /checkout/src/llvm-project/libunwind/src/Registers.hpp: In constructor 'libunwind::Registers_arm64::Registers_arm64(const void*)':
warning: /checkout/src/llvm-project/libunwind/src/Registers.hpp:1827:64: error: 'static_assert' was not declared in this scope
warning:                  "arm64 registers do not fit into unw_context_t");
warning:                                                                 ^
warning: /checkout/src/llvm-project/libunwind/src/Registers.hpp: In constructor 'libunwind::Registers_arm::Registers_arm(const void*)':
warning: /checkout/src/llvm-project/libunwind/src/Registers.hpp:2165:62: error: 'static_assert' was not declared in this scope
warning:                  "arm registers do not fit into unw_context_t");
warning:                                                               ^
warning: /checkout/src/llvm-project/libunwind/src/Registers.hpp: In constructor 'libunwind::Registers_or1k::Registers_or1k(const void*)':
warning: /checkout/src/llvm-project/libunwind/src/Registers.hpp:2575:63: error: 'static_assert' was not declared in this scope
warning:                  "or1k registers do not fit into unw_context_t");
warning:                                                                ^
warning: /checkout/src/llvm-project/libunwind/src/Registers.hpp: In constructor 'libunwind::Registers_mips_o32::Registers_mips_o32(const void*)':
warning: /checkout/src/llvm-project/libunwind/src/Registers.hpp:2781:67: error: 'static_assert' was not declared in this scope
warning:                  "mips_o32 registers do not fit into unw_context_t");
warning:                                                                    ^
warning: /checkout/src/llvm-project/libunwind/src/Registers.hpp: In constructor 'libunwind::Registers_mips_newabi::Registers_mips_newabi(const void*)':
warning: /checkout/src/llvm-project/libunwind/src/Registers.hpp:3103:70: error: 'static_assert' was not declared in this scope
warning:                  "mips_newabi registers do not fit into unw_context_t");
warning:                                                                       ^
warning: /checkout/src/llvm-project/libunwind/src/Registers.hpp: In constructor 'libunwind::Registers_sparc::Registers_sparc(const void*)':
warning: /checkout/src/llvm-project/libunwind/src/Registers.hpp:3392:64: error: 'static_assert' was not declared in this scope
warning:                  "sparc registers do not fit into unw_context_t");
warning:                                                                 ^
warning: /checkout/src/llvm-project/libunwind/src/Registers.hpp: In constructor 'libunwind::Registers_hexagon::Registers_hexagon(const void*)':
warning: /checkout/src/llvm-project/libunwind/src/Registers.hpp:3576:66: error: 'static_assert' was not declared in this scope
warning:                  "hexagon registers do not fit into unw_context_t");
warning:                                                                   ^
warning: /checkout/src/llvm-project/libunwind/src/Registers.hpp: In constructor 'libunwind::Registers_riscv::Registers_riscv(const void*)':
warning: /checkout/src/llvm-project/libunwind/src/Registers.hpp:3755:64: error: 'static_assert' was not declared in this scope
warning:                  "riscv registers do not fit into unw_context_t");
warning:                                                                 ^
warning: /checkout/src/llvm-project/libunwind/src/Registers.hpp: In constructor 'libunwind::Registers_ve::Registers_ve(const void*)':
warning: /checkout/src/llvm-project/libunwind/src/Registers.hpp:4033:61: error: 'static_assert' was not declared in this scope
warning:                  "ve registers do not fit into unw_context_t");
warning:                                                              ^
warning: In file included from /checkout/src/llvm-project/libunwind/src/EHHeaderParser.hpp:17:0,
warning:                  from /checkout/src/llvm-project/libunwind/src/AddressSpace.hpp:23,
warning:                  from /checkout/src/llvm-project/libunwind/src/libunwind.cpp:21:
warning: /checkout/src/llvm-project/libunwind/src/DwarfParser.hpp: At global scope:
warning: /checkout/src/llvm-project/libunwind/src/DwarfParser.hpp:92:5: warning: scoped enums only available with -std=c++11 or -std=gnu++11
warning:      enum class InitializeTime { kLazy, kNormal };
warning:      ^
warning: /checkout/src/llvm-project/libunwind/src/DwarfParser.hpp:95:36: error: 'InitializeTime' is not a class or namespace
warning:      PrologInfo(InitializeTime IT = InitializeTime::kNormal) {
warning:                                     ^
warning: /checkout/src/llvm-project/libunwind/src/DwarfParser.hpp: In constructor 'libunwind::CFI_Parser<A>::PrologInfo::PrologInfo(libunwind::CFI_Parser<A>::PrologInfo::InitializeTime)':
warning: /checkout/src/llvm-project/libunwind/src/DwarfParser.hpp:96:17: error: 'InitializeTime' is not a class or namespace
warning:        if (IT == InitializeTime::kNormal)
warning:                  ^
warning: /checkout/src/llvm-project/libunwind/src/DwarfParser.hpp: In constructor 'libunwind::CFI_Parser<A>::RememberStack::RememberStack()':
warning: /checkout/src/llvm-project/libunwind/src/DwarfParser.hpp:137:29: error: 'nullptr' was not declared in this scope
warning:      RememberStack() : entry(nullptr) {}
warning:                              ^
warning: /checkout/src/llvm-project/libunwind/src/DwarfParser.hpp: In static member function 'static bool libunwind::CFI_Parser<A>::parseFDEInstructions(A&, const libunwind::CFI_Parser<A>::FDE_Info&, const libunwind::CFI_Parser<A>::CIE_Info&, libunwind::CFI_Parser<A>::pint_t, int, libunwind::CFI_Parser<A>::PrologInfo*)':
warning: /checkout/src/llvm-project/libunwind/src/DwarfParser.hpp:423:20: error: ISO C++ forbids declaration of 'info' with no type [-fpermissive]
warning:    for (const auto &info : parseInfoArray) {
warning:                     ^
warning: /checkout/src/llvm-project/libunwind/src/DwarfParser.hpp:423:27: warning: range-based 'for' loops only available with -std=c++11 or -std=gnu++11
warning:    for (const auto &info : parseInfoArray) {
warning:                            ^
warning: /checkout/src/llvm-project/libunwind/src/DwarfParser.hpp:424:21: error: request for member 'instructions' in 'info', which is of non-class type 'const int'
warning:      pint_t p = info.instructions;
warning:                      ^
warning: /checkout/src/llvm-project/libunwind/src/DwarfParser.hpp:425:35: error: request for member 'instructionsEnd' in 'info', which is of non-class type 'const int'
warning:      pint_t instructionsEnd = info.instructionsEnd;
warning:                                    ^
warning: /checkout/src/llvm-project/libunwind/src/DwarfParser.hpp:426:28: error: request for member 'pcoffset' in 'info', which is of non-class type 'const int'
warning:      pint_t pcoffset = info.pcoffset;
warning:                             ^
warning: /checkout/src/llvm-project/libunwind/src/DwarfParser.hpp:730:47: error: there are no arguments to 'static_assert' that depend on a template parameter, so a declaration of 'static_assert' must be available [-fpermissive]
warning:                        "uses the same constant");
warning:                                                ^
warning: /checkout/src/llvm-project/libunwind/src/DwarfParser.hpp:730:47: note: (if you use '-fpermissive', G++ will accept your code, but allowing the use of an undeclared name is deprecated)
warning: In file included from /checkout/src/llvm-project/libunwind/src/libunwind.cpp:21:0:
warning: /checkout/src/llvm-project/libunwind/src/AddressSpace.hpp: In function 'int libunwind::findUnwindSectionsByPhdr(dl_phdr_info*, size_t, void*)':
warning: /checkout/src/llvm-project/libunwind/src/AddressSpace.hpp:463:8: error: 'cbdata' does not name a type
warning:    auto cbdata = static_cast<dl_iterate_cb_data *>(data);
warning:         ^
warning: /checkout/src/llvm-project/libunwind/src/AddressSpace.hpp:464:33: error: 'cbdata' was not declared in this scope
warning:    if (pinfo->dlpi_phnum == 0 || cbdata->targetAddr < pinfo->dlpi_addr)
warning:                                  ^
warning: /checkout/src/llvm-project/libunwind/src/AddressSpace.hpp:481:62: error: 'cbdata' was not declared in this scope
warning:      if (checkAddrInSegment(&pinfo->dlpi_phdr[i], image_base, cbdata)) {
warning:                                                               ^
warning: /checkout/src/llvm-project/libunwind/src/AddressSpace.hpp:494:53: error: 'cbdata' was not declared in this scope
warning:      if (checkForUnwindInfoSegment(phdr, image_base, cbdata)) {
warning: In file included from /checkout/src/llvm-project/libunwind/src/libunwind.cpp:22:0:
warning: In file included from /checkout/src/llvm-project/libunwind/src/libunwind.cpp:22:0:
warning: /checkout/src/llvm-project/libunwind/src/UnwindCursor.hpp: At global scope:
warning: /checkout/src/llvm-project/libunwind/src/UnwindCursor.hpp:84:10: error: 'constexpr' does not name a type
warning:    static constexpr pint_t kSearchAll = static_cast<pint_t>(-1);
warning:           ^
warning: /checkout/src/llvm-project/libunwind/src/UnwindCursor.hpp:84:10: note: C++11 'constexpr' only available with -std=c++11 or -std=gnu++11
warning: /checkout/src/llvm-project/libunwind/src/UnwindCursor.hpp: In static member function 'static typename A::pint_t libunwind::DwarfFDECache<A>::findFDE(libunwind::DwarfFDECache<A>::pint_t, libunwind::DwarfFDECache<A>::pint_t)':
warning: /checkout/src/llvm-project/libunwind/src/UnwindCursor.hpp:142:33: error: 'kSearchAll' was not declared in this scope
warning:      if ((mh == p->mh) || (mh == kSearchAll)) {
warning:                                  ^
warning: /checkout/src/llvm-project/libunwind/src/UnwindCursor.hpp: In constructor 'libunwind::UnwindCursor<A, R>::UnwindCursor(unw_context_t*, A&)':
warning: /checkout/src/llvm-project/libunwind/src/UnwindCursor.hpp:1213:44: error: expected primary-expression before ')' token
warning:    static_assert((alignof(UnwindCursor<A, R>) <= alignof(unw_cursor_t)),
warning:                                             ^
warning: /checkout/src/llvm-project/libunwind/src/UnwindCursor.hpp:1213:44: error: there are no arguments to 'alignof' that depend on a template parameter, so a declaration of 'alignof' must be available [-fpermissive]
warning: /checkout/src/llvm-project/libunwind/src/UnwindCursor.hpp:1213:69: error: expected primary-expression before ')' token
warning:    static_assert((alignof(UnwindCursor<A, R>) <= alignof(unw_cursor_t)),
warning:                                                                      ^
warning: /checkout/src/llvm-project/libunwind/src/UnwindCursor.hpp:1213:69: error: there are no arguments to 'alignof' that depend on a template parameter, so a declaration of 'alignof' must be available [-fpermissive]
warning: /checkout/src/llvm-project/libunwind/src/UnwindCursor.hpp: In instantiation of 'libunwind::UnwindCursor<A, R>::UnwindCursor(unw_context_t*, A&) [with A = libunwind::LocalAddressSpace; R = libunwind::Registers_x86; unw_context_t = unw_context_t]':
warning: /checkout/src/llvm-project/libunwind/src/libunwind.cpp:73:56:   required from here
warning: /checkout/src/llvm-project/libunwind/src/UnwindCursor.hpp:1211:16: error: 'static_assert' was not declared in this scope
warning:    static_assert((check_fit<UnwindCursor<A, R>, unw_cursor_t>::does_fit),
warning:                 ^
warning: /checkout/src/llvm-project/libunwind/src/UnwindCursor.hpp:1213:25: error: 'alignof' was not declared in this scope
warning:    static_assert((alignof(UnwindCursor<A, R>) <= alignof(unw_cursor_t)),
warning:                          ^
warning: /checkout/src/llvm-project/libunwind/src/UnwindCursor.hpp:1213:56: error: 'alignof' was not declared in this scope, and no declarations were found by argument-dependent lookup at the point of instantiation [-fpermissive]
warning:    static_assert((alignof(UnwindCursor<A, R>) <= alignof(unw_cursor_t)),
warning:                                                         ^
warning: /checkout/src/llvm-project/libunwind/src/UnwindCursor.hpp:1213:25: note: 'alignof' declared here, later in the translation unit
warning:    static_assert((alignof(UnwindCursor<A, R>) <= alignof(unw_cursor_t)),
warning:                          ^
warning: /checkout/src/llvm-project/libunwind/src/UnwindCursor.hpp:1213:16: error: 'static_assert' was not declared in this scope, and no declarations were found by argument-dependent lookup at the point of instantiation [-fpermissive]
warning:    static_assert((alignof(UnwindCursor<A, R>) <= alignof(unw_cursor_t)),
warning:                 ^
warning: /checkout/src/llvm-project/libunwind/src/UnwindCursor.hpp:1211:16: note: 'static_assert' declared here, later in the translation unit
warning:    static_assert((check_fit<UnwindCursor<A, R>, unw_cursor_t>::does_fit),
warning:                 ^
warning: /checkout/src/llvm-project/libunwind/src/UnwindCursor.hpp: In instantiation of 'void libunwind::UnwindCursor<A, R>::setInfoBasedOnIPRegister(bool) [with A = libunwind::LocalAddressSpace; R = libunwind::Registers_x86]':
warning: /checkout/src/llvm-project/libunwind/src/libunwind.cpp:325:1:   required from here
warning: /checkout/src/llvm-project/libunwind/src/UnwindCursor.hpp:1975:47: error: 'kSearchAll' is not a member of 'libunwind::DwarfFDECache<libunwind::LocalAddressSpace>'
warning:    pint_t cachedFDE = DwarfFDECache<A>::findFDE(DwarfFDECache<A>::kSearchAll,
warning:                                                ^
warning: In file included from /checkout/src/llvm-project/libunwind/src/UnwindCursor.hpp:69:0,
warning:                  from /checkout/src/llvm-project/libunwind/src/libunwind.cpp:22:
warning: /checkout/src/llvm-project/libunwind/src/DwarfInstructions.hpp: In instantiation of 'static int libunwind::DwarfInstructions<A, R>::stepWithDwarf(A&, libunwind::DwarfInstructions<A, R>::pint_t, libunwind::DwarfInstructions<A, R>::pint_t, R&, bool&) [with A = libunwind::LocalAddressSpace; R = libunwind::Registers_x86; libunwind::DwarfInstructions<A, R>::pint_t = unsigned int]':
warning: /checkout/src/llvm-project/libunwind/src/UnwindCursor.hpp:954:50:   required from 'int libunwind::UnwindCursor<A, R>::stepWithDwarfFDE() [with A = libunwind::LocalAddressSpace; R = libunwind::Registers_x86]'
warning: /checkout/src/llvm-project/libunwind/src/UnwindCursor.hpp:2090:12:   required from 'int libunwind::UnwindCursor<A, R>::step() [with A = libunwind::LocalAddressSpace; R = libunwind::Registers_x86]'
warning: /checkout/src/llvm-project/libunwind/src/libunwind.cpp:325:1:   required from here
warning: /checkout/src/llvm-project/libunwind/src/DwarfInstructions.hpp:162:16: error: call to 'libunwind::CFI_Parser<A>::PrologInfo::PrologInfo(libunwind::CFI_Parser<A>::PrologInfo::InitializeTime) [with A = libunwind::LocalAddressSpace]' uses the default argument for parameter 1, which is not yet defined
warning:      PrologInfo prolog;


error: failed to run custom build command for `unwind v0.0.0 (/checkout/library/unwind)`
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/release/build/unwind-f75d62587c21abf6/build-script-build` (exit status: 1)
  --- stdout
  cargo:rerun-if-changed=build.rs
  OPT_LEVEL = Some("3")
  TARGET = Some("i686-unknown-linux-musl")
  HOST = Some("x86_64-unknown-linux-gnu")
  CXX_i686-unknown-linux-musl = Some("sccache musl-g++")
  CXXFLAGS_i686-unknown-linux-musl = Some("-ffunction-sections -fdata-sections -fPIC -m32 -march=i686 -Wl,-melf_i386 -static -Wa,-mrelax-relocations=no")
  CRATE_CC_NO_DEFAULTS = None
  DEBUG = Some("true")
  CARGO_CFG_TARGET_FEATURE = Some("fxsr,sse,sse2")
  CXX_i686-unknown-linux-musl = Some("sccache musl-g++")
  CXXFLAGS_i686-unknown-linux-musl = Some("-ffunction-sections -fdata-sections -fPIC -m32 -march=i686 -Wl,-melf_i386 -static -Wa,-mrelax-relocations=no")
  CRATE_CC_NO_DEFAULTS = None
  CARGO_CFG_TARGET_FEATURE = Some("fxsr,sse,sse2")
  cargo:rustc-link-search=native=/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/i686-unknown-linux-musl/release/build/unwind-238910bb1f3815ba/out
  running: "sccache" "/musl-i686/bin/musl-gcc" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-m32" "-march=i686" "-Wl,-melf_i386" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m32" "-march=i686" "-Wl,-melf_i386" "-static" "-Wa,-mrelax-relocations=no" "-I" "../../src/llvm-project/libunwind/include" "-nostdinc++" "-fno-exceptions" "-fno-rtti" "-fstrict-aliasing" "-funwind-tables" "-fvisibility=hidden" "-D_LIBUNWIND_DISABLE_VISIBILITY_ANNOTATIONS" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/i686-unknown-linux-musl/release/build/unwind-238910bb1f3815ba/out/Unwind-EHABI.o" "-c" "/checkout/src/llvm-project/libunwind/src/Unwind-EHABI.cpp"
  exit status: 0
  running: "sccache" "/musl-i686/bin/musl-gcc" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-m32" "-march=i686" "-Wl,-melf_i386" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m32" "-march=i686" "-Wl,-melf_i386" "-static" "-Wa,-mrelax-relocations=no" "-I" "../../src/llvm-project/libunwind/include" "-nostdinc++" "-fno-exceptions" "-fno-rtti" "-fstrict-aliasing" "-funwind-tables" "-fvisibility=hidden" "-D_LIBUNWIND_DISABLE_VISIBILITY_ANNOTATIONS" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/i686-unknown-linux-musl/release/build/unwind-238910bb1f3815ba/out/Unwind-seh.o" "-c" "/checkout/src/llvm-project/libunwind/src/Unwind-seh.cpp"
  exit status: 0
  running: "sccache" "/musl-i686/bin/musl-gcc" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-m32" "-march=i686" "-Wl,-melf_i386" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m32" "-march=i686" "-Wl,-melf_i386" "-static" "-Wa,-mrelax-relocations=no" "-I" "../../src/llvm-project/libunwind/include" "-nostdinc++" "-fno-exceptions" "-fno-rtti" "-fstrict-aliasing" "-funwind-tables" "-fvisibility=hidden" "-D_LIBUNWIND_DISABLE_VISIBILITY_ANNOTATIONS" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/i686-unknown-linux-musl/release/build/unwind-238910bb1f3815ba/out/libunwind.o" "-c" "/checkout/src/llvm-project/libunwind/src/libunwind.cpp"
  cargo:warning=In file included from /checkout/src/llvm-project/libunwind/src/DwarfParser.hpp:22:0,
  cargo:warning=                 from /checkout/src/llvm-project/libunwind/src/EHHeaderParser.hpp:17,
  cargo:warning=                 from /checkout/src/llvm-project/libunwind/src/AddressSpace.hpp:23,
  cargo:warning=                 from /checkout/src/llvm-project/libunwind/src/libunwind.cpp:21:
  cargo:warning=/checkout/src/llvm-project/libunwind/src/Registers.hpp: In constructor 'libunwind::Registers_x86::Registers_x86(const void*)':
  cargo:warning=/checkout/src/llvm-project/libunwind/src/Registers.hpp:108:62: error: 'static_assert' was not declared in this scope
  cargo:warning=                 "x86 registers do not fit into unw_context_t");
  cargo:warning=                                                              ^
  cargo:warning=/checkout/src/llvm-project/libunwind/src/Registers.hpp: In constructor 'libunwind::Registers_x86_64::Registers_x86_64(const void*)':
  cargo:warning=/checkout/src/llvm-project/libunwind/src/Registers.hpp:327:65: error: 'static_assert' was not declared in this scope
  cargo:warning=                 "x86_64 registers do not fit into unw_context_t");
  cargo:warning=                                                                 ^
  cargo:warning=/checkout/src/llvm-project/libunwind/src/Registers.hpp: In constructor 'libunwind::Registers_ppc::Registers_ppc(const void*)':
  cargo:warning=/checkout/src/llvm-project/libunwind/src/Registers.hpp:652:62: error: 'static_assert' was not declared in this scope
  cargo:warning=                 "ppc registers do not fit into unw_context_t");
  cargo:warning=                                                              ^
  cargo:warning=/checkout/src/llvm-project/libunwind/src/Registers.hpp: In constructor 'libunwind::Registers_ppc64::Registers_ppc64(const void*)':
  cargo:warning=/checkout/src/llvm-project/libunwind/src/Registers.hpp:1219:64: error: 'static_assert' was not declared in this scope
  cargo:warning=                 "ppc64 registers do not fit into unw_context_t");
  cargo:warning=                                                                ^
  cargo:warning=/checkout/src/llvm-project/libunwind/src/Registers.hpp: In constructor 'libunwind::Registers_arm64::Registers_arm64(const void*)':
  cargo:warning=/checkout/src/llvm-project/libunwind/src/Registers.hpp:1827:64: error: 'static_assert' was not declared in this scope
  cargo:warning=                 "arm64 registers do not fit into unw_context_t");
  cargo:warning=                                                                ^
  cargo:warning=/checkout/src/llvm-project/libunwind/src/Registers.hpp: In constructor 'libunwind::Registers_arm::Registers_arm(const void*)':
  cargo:warning=/checkout/src/llvm-project/libunwind/src/Registers.hpp:2165:62: error: 'static_assert' was not declared in this scope
  cargo:warning=                 "arm registers do not fit into unw_context_t");
  cargo:warning=                                                              ^
  cargo:warning=/checkout/src/llvm-project/libunwind/src/Registers.hpp: In constructor 'libunwind::Registers_or1k::Registers_or1k(const void*)':
  cargo:warning=/checkout/src/llvm-project/libunwind/src/Registers.hpp:2575:63: error: 'static_assert' was not declared in this scope
  cargo:warning=                 "or1k registers do not fit into unw_context_t");
  cargo:warning=                                                               ^
  cargo:warning=/checkout/src/llvm-project/libunwind/src/Registers.hpp: In constructor 'libunwind::Registers_mips_o32::Registers_mips_o32(const void*)':
  cargo:warning=/checkout/src/llvm-project/libunwind/src/Registers.hpp:2781:67: error: 'static_assert' was not declared in this scope
  cargo:warning=                 "mips_o32 registers do not fit into unw_context_t");
  cargo:warning=                                                                   ^
  cargo:warning=/checkout/src/llvm-project/libunwind/src/Registers.hpp: In constructor 'libunwind::Registers_mips_newabi::Registers_mips_newabi(const void*)':
  cargo:warning=/checkout/src/llvm-project/libunwind/src/Registers.hpp:3103:70: error: 'static_assert' was not declared in this scope
  cargo:warning=                 "mips_newabi registers do not fit into unw_context_t");
  cargo:warning=                                                                      ^
  cargo:warning=/checkout/src/llvm-project/libunwind/src/Registers.hpp: In constructor 'libunwind::Registers_sparc::Registers_sparc(const void*)':
  cargo:warning=/checkout/src/llvm-project/libunwind/src/Registers.hpp:3392:64: error: 'static_assert' was not declared in this scope
  cargo:warning=                 "sparc registers do not fit into unw_context_t");
  cargo:warning=                                                                ^
  cargo:warning=/checkout/src/llvm-project/libunwind/src/Registers.hpp: In constructor 'libunwind::Registers_hexagon::Registers_hexagon(const void*)':
  cargo:warning=/checkout/src/llvm-project/libunwind/src/Registers.hpp:3576:66: error: 'static_assert' was not declared in this scope
  cargo:warning=                 "hexagon registers do not fit into unw_context_t");
  cargo:warning=                                                                  ^
  cargo:warning=/checkout/src/llvm-project/libunwind/src/Registers.hpp: In constructor 'libunwind::Registers_riscv::Registers_riscv(const void*)':
  cargo:warning=/checkout/src/llvm-project/libunwind/src/Registers.hpp:3755:64: error: 'static_assert' was not declared in this scope
  cargo:warning=                 "riscv registers do not fit into unw_context_t");
  cargo:warning=                                                                ^
  cargo:warning=/checkout/src/llvm-project/libunwind/src/Registers.hpp: In constructor 'libunwind::Registers_ve::Registers_ve(const void*)':
  cargo:warning=/checkout/src/llvm-project/libunwind/src/Registers.hpp:4033:61: error: 'static_assert' was not declared in this scope
  cargo:warning=                 "ve registers do not fit into unw_context_t");
  cargo:warning=                                                             ^
  cargo:warning=In file included from /checkout/src/llvm-project/libunwind/src/EHHeaderParser.hpp:17:0,
  cargo:warning=                 from /checkout/src/llvm-project/libunwind/src/AddressSpace.hpp:23,
  cargo:warning=                 from /checkout/src/llvm-project/libunwind/src/libunwind.cpp:21:
  cargo:warning=/checkout/src/llvm-project/libunwind/src/DwarfParser.hpp: At global scope:
  cargo:warning=/checkout/src/llvm-project/libunwind/src/DwarfParser.hpp:92:5: warning: scoped enums only available with -std=c++11 or -std=gnu++11
  cargo:warning=     enum class InitializeTime { kLazy, kNormal };
  cargo:warning=     ^
  cargo:warning=/checkout/src/llvm-project/libunwind/src/DwarfParser.hpp:95:36: error: 'InitializeTime' is not a class or namespace
  cargo:warning=     PrologInfo(InitializeTime IT = InitializeTime::kNormal) {
  cargo:warning=                                    ^
  cargo:warning=/checkout/src/llvm-project/libunwind/src/DwarfParser.hpp: In constructor 'libunwind::CFI_Parser<A>::PrologInfo::PrologInfo(libunwind::CFI_Parser<A>::PrologInfo::InitializeTime)':
  cargo:warning=/checkout/src/llvm-project/libunwind/src/DwarfParser.hpp:96:17: error: 'InitializeTime' is not a class or namespace
  cargo:warning=       if (IT == InitializeTime::kNormal)
  cargo:warning=                 ^
  cargo:warning=/checkout/src/llvm-project/libunwind/src/DwarfParser.hpp: In constructor 'libunwind::CFI_Parser<A>::RememberStack::RememberStack()':
  cargo:warning=/checkout/src/llvm-project/libunwind/src/DwarfParser.hpp:137:29: error: 'nullptr' was not declared in this scope
  cargo:warning=     RememberStack() : entry(nullptr) {}
  cargo:warning=                             ^
  cargo:warning=/checkout/src/llvm-project/libunwind/src/DwarfParser.hpp: In static member function 'static bool libunwind::CFI_Parser<A>::parseFDEInstructions(A&, const libunwind::CFI_Parser<A>::FDE_Info&, const libunwind::CFI_Parser<A>::CIE_Info&, libunwind::CFI_Parser<A>::pint_t, int, libunwind::CFI_Parser<A>::PrologInfo*)':
  cargo:warning=/checkout/src/llvm-project/libunwind/src/DwarfParser.hpp:423:20: error: ISO C++ forbids declaration of 'info' with no type [-fpermissive]
  cargo:warning=   for (const auto &info : parseInfoArray) {
  cargo:warning=                    ^
  cargo:warning=/checkout/src/llvm-project/libunwind/src/DwarfParser.hpp:423:27: warning: range-based 'for' loops only available with -std=c++11 or -std=gnu++11
  cargo:warning=   for (const auto &info : parseInfoArray) {
  cargo:warning=                           ^
  cargo:warning=/checkout/src/llvm-project/libunwind/src/DwarfParser.hpp:424:21: error: request for member 'instructions' in 'info', which is of non-class type 'const int'
  cargo:warning=     pint_t p = info.instructions;
  cargo:warning=                     ^
  cargo:warning=/checkout/src/llvm-project/libunwind/src/DwarfParser.hpp:425:35: error: request for member 'instructionsEnd' in 'info', which is of non-class type 'const int'
  cargo:warning=     pint_t instructionsEnd = info.instructionsEnd;
  cargo:warning=                                   ^
  cargo:warning=/checkout/src/llvm-project/libunwind/src/DwarfParser.hpp:426:28: error: request for member 'pcoffset' in 'info', which is of non-class type 'const int'
  cargo:warning=     pint_t pcoffset = info.pcoffset;
  cargo:warning=                            ^
  cargo:warning=/checkout/src/llvm-project/libunwind/src/DwarfParser.hpp:730:47: error: there are no arguments to 'static_assert' that depend on a template parameter, so a declaration of 'static_assert' must be available [-fpermissive]
  cargo:warning=                       "uses the same constant");
  cargo:warning=                                               ^
  cargo:warning=/checkout/src/llvm-project/libunwind/src/DwarfParser.hpp:730:47: note: (if you use '-fpermissive', G++ will accept your code, but allowing the use of an undeclared name is deprecated)
  cargo:warning=In file included from /checkout/src/llvm-project/libunwind/src/libunwind.cpp:21:0:
  cargo:warning=/checkout/src/llvm-project/libunwind/src/AddressSpace.hpp: In function 'int libunwind::findUnwindSectionsByPhdr(dl_phdr_info*, size_t, void*)':
  cargo:warning=/checkout/src/llvm-project/libunwind/src/AddressSpace.hpp:463:8: error: 'cbdata' does not name a type
  cargo:warning=   auto cbdata = static_cast<dl_iterate_cb_data *>(data);
  cargo:warning=        ^
  cargo:warning=/checkout/src/llvm-project/libunwind/src/AddressSpace.hpp:464:33: error: 'cbdata' was not declared in this scope
  cargo:warning=   if (pinfo->dlpi_phnum == 0 || cbdata->targetAddr < pinfo->dlpi_addr)
  cargo:warning=                                 ^
  cargo:warning=/checkout/src/llvm-project/libunwind/src/AddressSpace.hpp:481:62: error: 'cbdata' was not declared in this scope
  cargo:warning=     if (checkAddrInSegment(&pinfo->dlpi_phdr[i], image_base, cbdata)) {
  cargo:warning=                                                              ^
  cargo:warning=/checkout/src/llvm-project/libunwind/src/AddressSpace.hpp:494:53: error: 'cbdata' was not declared in this scope
  cargo:warning=     if (checkForUnwindInfoSegment(phdr, image_base, cbdata)) {
  cargo:warning=                                                     ^
  cargo:warning=In file included from /checkout/src/llvm-project/libunwind/src/libunwind.cpp:22:0:
  cargo:warning=/checkout/src/llvm-project/libunwind/src/UnwindCursor.hpp: At global scope:
  cargo:warning=/checkout/src/llvm-project/libunwind/src/UnwindCursor.hpp:84:10: error: 'constexpr' does not name a type
  cargo:warning=   static constexpr pint_t kSearchAll = static_cast<pint_t>(-1);
  cargo:warning=          ^
  cargo:warning=/checkout/src/llvm-project/libunwind/src/UnwindCursor.hpp:84:10: note: C++11 'constexpr' only available with -std=c++11 or -std=gnu++11
  cargo:warning=/checkout/src/llvm-project/libunwind/src/UnwindCursor.hpp: In static member function 'static typename A::pint_t libunwind::DwarfFDECache<A>::findFDE(libunwind::DwarfFDECache<A>::pint_t, libunwind::DwarfFDECache<A>::pint_t)':
  cargo:warning=/checkout/src/llvm-project/libunwind/src/UnwindCursor.hpp:142:33: error: 'kSearchAll' was not declared in this scope
  cargo:warning=     if ((mh == p->mh) || (mh == kSearchAll)) {
  cargo:warning=                                 ^
  cargo:warning=/checkout/src/llvm-project/libunwind/src/UnwindCursor.hpp: In constructor 'libunwind::UnwindCursor<A, R>::UnwindCursor(unw_context_t*, A&)':
  cargo:warning=/checkout/src/llvm-project/libunwind/src/UnwindCursor.hpp:1213:44: error: expected primary-expression before ')' token
  cargo:warning=   static_assert((alignof(UnwindCursor<A, R>) <= alignof(unw_cursor_t)),
  cargo:warning=                                            ^
  cargo:warning=/checkout/src/llvm-project/libunwind/src/UnwindCursor.hpp:1213:44: error: there are no arguments to 'alignof' that depend on a template parameter, so a declaration of 'alignof' must be available [-fpermissive]
  cargo:warning=/checkout/src/llvm-project/libunwind/src/UnwindCursor.hpp:1213:69: error: expected primary-expression before ')' token
  cargo:warning=   static_assert((alignof(UnwindCursor<A, R>) <= alignof(unw_cursor_t)),
  cargo:warning=                                                                     ^
  cargo:warning=/checkout/src/llvm-project/libunwind/src/UnwindCursor.hpp:1213:69: error: there are no arguments to 'alignof' that depend on a template parameter, so a declaration of 'alignof' must be available [-fpermissive]
  cargo:warning=/checkout/src/llvm-project/libunwind/src/UnwindCursor.hpp: In instantiation of 'libunwind::UnwindCursor<A, R>::UnwindCursor(unw_context_t*, A&) [with A = libunwind::LocalAddressSpace; R = libunwind::Registers_x86; unw_context_t = unw_context_t]':
  cargo:warning=/checkout/src/llvm-project/libunwind/src/libunwind.cpp:73:56:   required from here
  cargo:warning=/checkout/src/llvm-project/libunwind/src/UnwindCursor.hpp:1211:16: error: 'static_assert' was not declared in this scope
  cargo:warning=   static_assert((check_fit<UnwindCursor<A, R>, unw_cursor_t>::does_fit),
  cargo:warning=                ^
  cargo:warning=/checkout/src/llvm-project/libunwind/src/UnwindCursor.hpp:1213:25: error: 'alignof' was not declared in this scope
  cargo:warning=   static_assert((alignof(UnwindCursor<A, R>) <= alignof(unw_cursor_t)),
  cargo:warning=                         ^
  cargo:warning=/checkout/src/llvm-project/libunwind/src/UnwindCursor.hpp:1213:56: error: 'alignof' was not declared in this scope, and no declarations were found by argument-dependent lookup at the point of instantiation [-fpermissive]
  cargo:warning=   static_assert((alignof(UnwindCursor<A, R>) <= alignof(unw_cursor_t)),
  cargo:warning=                                                        ^
  cargo:warning=/checkout/src/llvm-project/libunwind/src/UnwindCursor.hpp:1213:25: note: 'alignof' declared here, later in the translation unit
  cargo:warning=   static_assert((alignof(UnwindCursor<A, R>) <= alignof(unw_cursor_t)),
  cargo:warning=                         ^
  cargo:warning=/checkout/src/llvm-project/libunwind/src/UnwindCursor.hpp:1213:16: error: 'static_assert' was not declared in this scope, and no declarations were found by argument-dependent lookup at the point of instantiation [-fpermissive]
  cargo:warning=   static_assert((alignof(UnwindCursor<A, R>) <= alignof(unw_cursor_t)),
  cargo:warning=                ^
  cargo:warning=/checkout/src/llvm-project/libunwind/src/UnwindCursor.hpp:1211:16: note: 'static_assert' declared here, later in the translation unit
  cargo:warning=   static_assert((check_fit<UnwindCursor<A, R>, unw_cursor_t>::does_fit),
  cargo:warning=                ^
  cargo:warning=/checkout/src/llvm-project/libunwind/src/UnwindCursor.hpp: In instantiation of 'void libunwind::UnwindCursor<A, R>::setInfoBasedOnIPRegister(bool) [with A = libunwind::LocalAddressSpace; R = libunwind::Registers_x86]':
  cargo:warning=/checkout/src/llvm-project/libunwind/src/libunwind.cpp:325:1:   required from here
  cargo:warning=/checkout/src/llvm-project/libunwind/src/UnwindCursor.hpp:1975:47: error: 'kSearchAll' is not a member of 'libunwind::DwarfFDECache<libunwind::LocalAddressSpace>'
  cargo:warning=   pint_t cachedFDE = DwarfFDECache<A>::findFDE(DwarfFDECache<A>::kSearchAll,
  cargo:warning=                                               ^
  cargo:warning=In file included from /checkout/src/llvm-project/libunwind/src/UnwindCursor.hpp:69:0,
  cargo:warning=                 from /checkout/src/llvm-project/libunwind/src/libunwind.cpp:22:
  cargo:warning=/checkout/src/llvm-project/libunwind/src/DwarfInstructions.hpp: In instantiation of 'static int libunwind::DwarfInstructions<A, R>::stepWithDwarf(A&, libunwind::DwarfInstructions<A, R>::pint_t, libunwind::DwarfInstructions<A, R>::pint_t, R&, bool&) [with A = libunwind::LocalAddressSpace; R = libunwind::Registers_x86; libunwind::DwarfInstructions<A, R>::pint_t = unsigned int]':
  cargo:warning=/checkout/src/llvm-project/libunwind/src/UnwindCursor.hpp:954:50:   required from 'int libunwind::UnwindCursor<A, R>::stepWithDwarfFDE() [with A = libunwind::LocalAddressSpace; R = libunwind::Registers_x86]'
  cargo:warning=/checkout/src/llvm-project/libunwind/src/UnwindCursor.hpp:2090:12:   required from 'int libunwind::UnwindCursor<A, R>::step() [with A = libunwind::LocalAddressSpace; R = libunwind::Registers_x86]'
  cargo:warning=/checkout/src/llvm-project/libunwind/src/libunwind.cpp:325:1:   required from here
  cargo:warning=/checkout/src/llvm-project/libunwind/src/DwarfInstructions.hpp:162:16: error: call to 'libunwind::CFI_Parser<A>::PrologInfo::PrologInfo(libunwind::CFI_Parser<A>::PrologInfo::InitializeTime) [with A = libunwind::LocalAddressSpace]' uses the default argument for parameter 1, which is not yet defined
  cargo:warning=     PrologInfo prolog;
  cargo:warning=                ^

  --- stderr



  error occurred: Command "sccache" "/musl-i686/bin/musl-gcc" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-g" "-fno-omit-frame-pointer" "-m32" "-march=i686" "-Wl,-melf_i386" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m32" "-march=i686" "-Wl,-melf_i386" "-static" "-Wa,-mrelax-relocations=no" "-I" "../../src/llvm-project/libunwind/include" "-nostdinc++" "-fno-exceptions" "-fno-rtti" "-fstrict-aliasing" "-funwind-tables" "-fvisibility=hidden" "-D_LIBUNWIND_DISABLE_VISIBILITY_ANNOTATIONS" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-std/i686-unknown-linux-musl/release/build/unwind-238910bb1f3815ba/out/libunwind.o" "-c" "/checkout/src/llvm-project/libunwind/src/libunwind.cpp" with args "musl-gcc" did not execute successfully (status code exit status: 1).

warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] core test:false 20.626
error: build failed
