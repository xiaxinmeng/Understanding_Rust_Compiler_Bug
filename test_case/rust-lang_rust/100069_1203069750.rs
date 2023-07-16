plain
error: make failed
status: exit status: 2
command: "make"
--- stdout -------------------------------
cc -ffunction-sections -fdata-sections -fPIC -m64 -v -c -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-with-extern-types/extern-fn-with-extern-types/libctest.o ctest.c
ar crus /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-with-extern-types/extern-fn-with-extern-types/libctest.a /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-with-extern-types/extern-fn-with-extern-types/libctest.o
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-with-extern-types/extern-fn-with-extern-types:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-with-extern-types/extern-fn-with-extern-types -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-with-extern-types/extern-fn-with-extern-types  test.rs
rm /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-with-extern-types/extern-fn-with-extern-types/libctest.o
--- stderr -------------------------------
Using built-in specs.
Using built-in specs.
COLLECT_GCC=cc
OFFLOAD_TARGET_NAMES=nvptx-none:hsa
Target: x86_64-linux-gnu
Target: x86_64-linux-gnu
Configured with: ../src/configure -v --with-pkgversion='Ubuntu 9.4.0-1ubuntu1~20.04.1' --with-bugurl=file:///usr/share/doc/gcc-9/README.Bugs --enable-languages=c,ada,c++,go,brig,d,fortran,objc,obj-c++,gm2 --prefix=/usr --with-gcc-major-version-only --program-suffix=-9 --program-prefix=x86_64-linux-gnu- --enable-shared --enable-linker-build-id --libexecdir=/usr/lib --without-included-gettext --enable-threads=posix --libdir=/usr/lib --enable-nls --enable-clocale=gnu --enable-libstdcxx-debug --enable-libstdcxx-time=yes --with-default-libstdcxx-abi=new --enable-gnu-unique-object --disable-vtable-verify --enable-plugin --enable-default-pie --with-system-zlib --with-target-system-zlib=auto --enable-objc-gc=auto --enable-multiarch --disable-werror --with-arch-32=i686 --with-abi=m64 --with-multilib-list=m32,m64,mx32 --enable-multilib --with-tune=generic --enable-offload-targets=nvptx-none=/build/gcc-9-Av3uEd/gcc-9-9.4.0/debian/tmp-nvptx/usr,hsa --without-cuda-driver --enable-checking=release --build=x86_64-linux-gnu --host=x86_64-linux-gnu --target=x86_64-linux-gnu
Thread model: posix
gcc version 9.4.0 (Ubuntu 9.4.0-1ubuntu1~20.04.1) 
COLLECT_GCC_OPTIONS='-ffunction-sections' '-fdata-sections' '-fPIC' '-m64' '-v' '-c' '-o' '/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-with-extern-types/extern-fn-with-extern-types/libctest.o' '-mtune=generic' '-march=x86-64'
 /usr/lib/gcc/x86_64-linux-gnu/9/cc1 -quiet -v -imultiarch x86_64-linux-gnu ctest.c -quiet -dumpbase ctest.c -m64 -mtune=generic -march=x86-64 -auxbase-strip /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-with-extern-types/extern-fn-with-extern-types/libctest.o -version -ffunction-sections -fdata-sections -fPIC -fasynchronous-unwind-tables -fstack-protector-strong -Wformat -Wformat-security -fstack-clash-protection -fcf-protection -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-with-extern-types/extern-fn-with-extern-types/ccP3AGPD.s
GNU C17 (Ubuntu 9.4.0-1ubuntu1~20.04.1) version 9.4.0 (x86_64-linux-gnu)
 compiled by GNU C version 9.4.0, GMP version 6.2.0, MPFR version 4.0.2, MPC version 1.1.0, isl version isl-0.22.1-GMP

GGC heuristics: --param ggc-min-expand=100 --param ggc-min-heapsize=131072
ignoring nonexistent directory "/usr/local/include/x86_64-linux-gnu"
ignoring nonexistent directory "/usr/lib/gcc/x86_64-linux-gnu/9/include-fixed"
ignoring nonexistent directory "/usr/lib/gcc/x86_64-linux-gnu/9/../../../../x86_64-linux-gnu/include"
#include "..." search starts here:
#include <...> search starts here:
 /usr/lib/gcc/x86_64-linux-gnu/9/include
 /usr/include/x86_64-linux-gnu
 /usr/include
End of search list.
End of search list.
GNU C17 (Ubuntu 9.4.0-1ubuntu1~20.04.1) version 9.4.0 (x86_64-linux-gnu)
 compiled by GNU C version 9.4.0, GMP version 6.2.0, MPFR version 4.0.2, MPC version 1.1.0, isl version isl-0.22.1-GMP

GGC heuristics: --param ggc-min-expand=100 --param ggc-min-heapsize=131072
Compiler executable checksum: c0c95c0b4209efec1c1892d5ff24030b
COLLECT_GCC_OPTIONS='-ffunction-sections' '-fdata-sections' '-fPIC' '-m64' '-v' '-c' '-o' '/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-with-extern-types/extern-fn-with-extern-types/libctest.o' '-mtune=generic' '-march=x86-64'
 as -v --64 -o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-with-extern-types/extern-fn-with-extern-types/libctest.o /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-with-extern-types/extern-fn-with-extern-types/ccP3AGPD.s
GNU assembler version 2.34 (x86_64-linux-gnu) using BFD version (GNU Binutils for Ubuntu) 2.34
COMPILER_PATH=/usr/lib/gcc/x86_64-linux-gnu/9/:/usr/lib/gcc/x86_64-linux-gnu/9/:/usr/lib/gcc/x86_64-linux-gnu/:/usr/lib/gcc/x86_64-linux-gnu/9/:/usr/lib/gcc/x86_64-linux-gnu/
LIBRARY_PATH=/usr/lib/llvm-12/lib/../lib/:/usr/lib/gcc/x86_64-linux-gnu/9/:/usr/lib/gcc/x86_64-linux-gnu/9/../../../x86_64-linux-gnu/:/usr/lib/gcc/x86_64-linux-gnu/9/../../../../lib/:/lib/x86_64-linux-gnu/:/lib/../lib/:/usr/lib/x86_64-linux-gnu/:/usr/lib/../lib/:/usr/lib/llvm-12/lib/:/usr/lib/gcc/x86_64-linux-gnu/9/../../../:/lib/:/usr/lib/
COLLECT_GCC_OPTIONS='-ffunction-sections' '-fdata-sections' '-fPIC' '-m64' '-v' '-c' '-o' '/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/extern-fn-with-extern-types/extern-fn-with-extern-types/libctest.o' '-mtune=generic' '-march=x86-64'
ar: `u' modifier ignored since `D' is the default (see `U')
thread 'rustc' panicked at 'unexpected `def_kind` in `codegen_fn_attrs`: ForeignTy', compiler/rustc_typeck/src/collect.rs:2695:9
stack backtrace:
   0:     0x7facc1411f9c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hc9609f9a8a4ff2c9
   1:     0x7facc14789f8 - core::fmt::write::h9285f9374bf708c8
   2:     0x7facc14025b1 - std::io::Write::write_fmt::h5a230026a5932f80
   3:     0x7facc1414f5e - std::panicking::default_hook::{{closure}}::hf66fb347412d38a0
   4:     0x7facc1414c1f - std::panicking::default_hook::h639b116f195639ef
   5:     0x7facc1dc37c4 - rustc_driver[17ce2fe755a05f0d]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7facc1415712 - std::panicking::rust_panic_with_hook::h053adc160f72efa3
   7:     0x7facc1415537 - std::panicking::begin_panic_handler::{{closure}}::hc394c8189ce17840
   8:     0x7facc1412514 - std::sys_common::backtrace::__rust_end_short_backtrace::h92fffd1da67c276e
   9:     0x7facc1415202 - rust_begin_unwind
  10:     0x7facc13c4e13 - core::panicking::panic_fmt::h8be237caf11147fa
  11:     0x7facc2a8ab9d - rustc_typeck[9339b169f31e858b]::collect::codegen_fn_attrs
  12:     0x7facc398a276 - rustc_query_system[adfdf54f89ddeee2]::query::plumbing::try_execute_query::<rustc_query_impl[1d53cfea83f481dd]::plumbing::QueryCtxt, rustc_query_system[adfdf54f89ddeee2]::query::caches::ArenaCache<rustc_span[29ac8d5a826337dd]::def_id::DefId, rustc_middle[2e8cd5326dad2ad7]::middle::codegen_fn_attrs::CodegenFnAttrs>>
  13:     0x7facc3a92645 - rustc_query_system[adfdf54f89ddeee2]::query::plumbing::get_query::<rustc_query_impl[1d53cfea83f481dd]::queries::codegen_fn_attrs, rustc_query_impl[1d53cfea83f481dd]::plumbing::QueryCtxt>
  14:     0x7facc35d43d9 - <rustc_query_impl[1d53cfea83f481dd]::Queries as rustc_middle[2e8cd5326dad2ad7]::ty::query::QueryEngine>::codegen_fn_attrs
  15:     0x7facc3faf58c - rustc_metadata[1a73a97c86e1473c]::native_libs::collect
  16:     0x7facc3fb45d5 - <rustc_metadata[1a73a97c86e1473c]::rmeta::decoder::cstore_impl::provide::{closure#5} as core[ce9ef842f3876b34]::ops::function::FnOnce<(rustc_middle[2e8cd5326dad2ad7]::ty::context::TyCtxt, rustc_span[29ac8d5a826337dd]::def_id::CrateNum)>>::call_once
  17:     0x7facc398f18f - rustc_query_system[adfdf54f89ddeee2]::query::plumbing::try_execute_query::<rustc_query_impl[1d53cfea83f481dd]::plumbing::QueryCtxt, rustc_query_system[adfdf54f89ddeee2]::query::caches::ArenaCache<rustc_span[29ac8d5a826337dd]::def_id::CrateNum, alloc[e90f9170f64a15d9]::vec::Vec<rustc_session[e77cac0a1d3eacc4]::cstore::NativeLib>>>
  18:     0x7facc3a96f13 - rustc_query_system[adfdf54f89ddeee2]::query::plumbing::get_query::<rustc_query_impl[1d53cfea83f481dd]::queries::native_libraries, rustc_query_impl[1d53cfea83f481dd]::plumbing::QueryCtxt>
  19:     0x7facc35ac3e4 - <rustc_query_impl[1d53cfea83f481dd]::Queries as rustc_middle[2e8cd5326dad2ad7]::ty::query::QueryEngine>::native_libraries
  20:     0x7facc3c47c3c - <rustc_codegen_ssa[1d0e8f6f3e3547cf]::CrateInfo>::new
  21:     0x7facc20dee22 - rustc_codegen_ssa[1d0e8f6f3e3547cf]::back::write::start_async_codegen::<rustc_codegen_llvm[b6c4f0df82a326d7]::LlvmCodegenBackend>
  22:     0x7facc2012f15 - rustc_codegen_ssa[1d0e8f6f3e3547cf]::base::codegen_crate::<rustc_codegen_llvm[b6c4f0df82a326d7]::LlvmCodegenBackend>
  23:     0x7facc20c8aad - <rustc_codegen_llvm[b6c4f0df82a326d7]::LlvmCodegenBackend as rustc_codegen_ssa[1d0e8f6f3e3547cf]::traits::backend::CodegenBackend>::codegen_crate
  24:     0x7facc1f384cb - <rustc_session[e77cac0a1d3eacc4]::session::Session>::time::<alloc[e90f9170f64a15d9]::boxed::Box<dyn core[ce9ef842f3876b34]::any::Any>, rustc_interface[225fbc981f4ff74]::passes::start_codegen::{closure#0}>
  25:     0x7facc1efd72c - <rustc_interface[225fbc981f4ff74]::passes::QueryContext>::enter::<<rustc_interface[225fbc981f4ff74]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[ce9ef842f3876b34]::result::Result<alloc[e90f9170f64a15d9]::boxed::Box<dyn core[ce9ef842f3876b34]::any::Any>, rustc_errors[2e74e1233cec8f46]::ErrorGuaranteed>>
  26:     0x7facc1ee29ed - <rustc_interface[225fbc981f4ff74]::queries::Queries>::ongoing_codegen
  27:     0x7facc1dc544f - <rustc_interface[225fbc981f4ff74]::interface::Compiler>::enter::<rustc_driver[17ce2fe755a05f0d]::run_compiler::{closure#1}::{closure#2}, core[ce9ef842f3876b34]::result::Result<core[ce9ef842f3876b34]::option::Option<rustc_interface[225fbc981f4ff74]::queries::Linker>, rustc_errors[2e74e1233cec8f46]::ErrorGuaranteed>>
  28:     0x7facc1ddb698 - rustc_span[29ac8d5a826337dd]::with_source_map::<core[ce9ef842f3876b34]::result::Result<(), rustc_errors[2e74e1233cec8f46]::ErrorGuaranteed>, rustc_interface[225fbc981f4ff74]::interface::create_compiler_and_run<core[ce9ef842f3876b34]::result::Result<(), rustc_errors[2e74e1233cec8f46]::ErrorGuaranteed>, rustc_driver[17ce2fe755a05f0d]::run_compiler::{closure#1}>::{closure#1}>
  29:     0x7facc1dc7e7a - rustc_interface[225fbc981f4ff74]::interface::create_compiler_and_run::<core[ce9ef842f3876b34]::result::Result<(), rustc_errors[2e74e1233cec8f46]::ErrorGuaranteed>, rustc_driver[17ce2fe755a05f0d]::run_compiler::{closure#1}>
  30:     0x7facc1daeb22 - <scoped_tls[44bbf3a598eb9b4d]::ScopedKey<rustc_span[29ac8d5a826337dd]::SessionGlobals>>::set::<rustc_interface[225fbc981f4ff74]::interface::run_compiler<core[ce9ef842f3876b34]::result::Result<(), rustc_errors[2e74e1233cec8f46]::ErrorGuaranteed>, rustc_driver[17ce2fe755a05f0d]::run_compiler::{closure#1}>::{closure#0}, core[ce9ef842f3876b34]::result::Result<(), rustc_errors[2e74e1233cec8f46]::ErrorGuaranteed>>
  31:     0x7facc1db7109 - std[84edd46b21e6161c]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[225fbc981f4ff74]::util::run_in_thread_pool_with_globals<rustc_interface[225fbc981f4ff74]::interface::run_compiler<core[ce9ef842f3876b34]::result::Result<(), rustc_errors[2e74e1233cec8f46]::ErrorGuaranteed>, rustc_driver[17ce2fe755a05f0d]::run_compiler::{closure#1}>::{closure#0}, core[ce9ef842f3876b34]::result::Result<(), rustc_errors[2e74e1233cec8f46]::ErrorGuaranteed>>::{closure#0}, core[ce9ef842f3876b34]::result::Result<(), rustc_errors[2e74e1233cec8f46]::ErrorGuaranteed>>
  32:     0x7facc1e1e1ce - std[84edd46b21e6161c]::panicking::try::<core[ce9ef842f3876b34]::result::Result<(), rustc_errors[2e74e1233cec8f46]::ErrorGuaranteed>, core[ce9ef842f3876b34]::panic::unwind_safe::AssertUnwindSafe<<std[84edd46b21e6161c]::thread::Builder>::spawn_unchecked_<rustc_interface[225fbc981f4ff74]::util::run_in_thread_pool_with_globals<rustc_interface[225fbc981f4ff74]::interface::run_compiler<core[ce9ef842f3876b34]::result::Result<(), rustc_errors[2e74e1233cec8f46]::ErrorGuaranteed>, rustc_driver[17ce2fe755a05f0d]::run_compiler::{closure#1}>::{closure#0}, core[ce9ef842f3876b34]::result::Result<(), rustc_errors[2e74e1233cec8f46]::ErrorGuaranteed>>::{closure#0}, core[ce9ef842f3876b34]::result::Result<(), rustc_errors[2e74e1233cec8f46]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  33:     0x7facc1de95b2 - <<std[84edd46b21e6161c]::thread::Builder>::spawn_unchecked_<rustc_interface[225fbc981f4ff74]::util::run_in_thread_pool_with_globals<rustc_interface[225fbc981f4ff74]::interface::run_compiler<core[ce9ef842f3876b34]::result::Result<(), rustc_errors[2e74e1233cec8f46]::ErrorGuaranteed>, rustc_driver[17ce2fe755a05f0d]::run_compiler::{closure#1}>::{closure#0}, core[ce9ef842f3876b34]::result::Result<(), rustc_errors[2e74e1233cec8f46]::ErrorGuaranteed>>::{closure#0}, core[ce9ef842f3876b34]::result::Result<(), rustc_errors[2e74e1233cec8f46]::ErrorGuaranteed>>::{closure#1} as core[ce9ef842f3876b34]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  34:     0x7facc14209e5 - std::sys::unix::thread::Thread::new::thread_start::h421cd22368fee006
  35:     0x7facbb96b609 - start_thread
  36:     0x7facc127e133 - clone
  37:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.64.0-nightly (29d12297c 2022-08-02) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [codegen_fn_attrs] computing codegen attributes of `data`
#1 [native_libraries] looking up the native libraries of a linked crate
end of query stack
make: *** [Makefile:4: all] Error 101



failures:
