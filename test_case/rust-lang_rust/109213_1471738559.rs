plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:24cb9080177205b6e8c946b17badbe402adc938f)
Download action repository 'rust-lang/simpleinfra@master' (SHA:3fb2b44a4eaebb9ed8086446bde46c27199ef5ed)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
Successfully built a233b7c428f6
Successfully tagged rust-ci:latest
Built container sha256:a233b7c428f69153763be294f3c199d5421cc09bbf465acb211fedf101d61ff5
Uploading finished image to https://ci-caches.rust-lang.org/docker/fa16b54262d00a378eee0b4414848c5f43b18d4834e4bed4c4343f19aa40bf2d28e70ed404889b52718c7269e7e82882fdc9e90fcf2e71af8c5d6ce646f6a1f8
upload failed: - to s3://rust-lang-ci-sccache2/docker/fa16b54262d00a378eee0b4414848c5f43b18d4834e4bed4c4343f19aa40bf2d28e70ed404889b52718c7269e7e82882fdc9e90fcf2e71af8c5d6ce646f6a1f8 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-14]
---
Check compiletest suite=run-make-fulldeps mode=run-make (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)

running 230 tests
............i.....ii.................................................................... 88/230
.....i...F.....................i...................iiiiiii.......i...................iii 176/230
Some tests failed in compiletest suite=run-make-fulldeps mode=run-make host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
failures:


---- [run-make] tests/run-make-fulldeps/issue-83045 stdout ----
error: make failed
status: exit status: 2
status: exit status: 2
command: cd "/checkout/tests/run-make-fulldeps/issue-83045" && AR="ar" CC="cc -ffunction-sections -fdata-sections -fPIC -m64" CXX="c++ -ffunction-sections -fdata-sections -fPIC -m64" HOST_RPATH_DIR="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" LD_LIB_PATH_ENVVAR="LD_LIBRARY_PATH" LLVM_BIN_DIR="/usr/lib/llvm-14/bin" LLVM_COMPONENTS="aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgputargetmca amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker dwp engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto m68k m68kasmparser m68kcodegen m68kdesc m68kdisassembler m68kinfo mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils ve veasmparser vecodegen vectorize vedesc vedisassembler veinfo webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo webassemblyutils windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86targetmca xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" LLVM_FILECHECK="/usr/lib/llvm-14/bin/FileCheck" NODE="/usr/bin/node" PYTHON="/usr/bin/python3" RUSTC="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" RUSTDOC="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" RUST_BUILD_STAGE="stage2-x86_64-unknown-linux-gnu" RUST_DEMANGLER="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools-bin/rust-demangler" S="/checkout" TARGET="x86_64-unknown-linux-gnu" TARGET_RPATH_DIR="/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" TMPDIR="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-83045/issue-83045" "make"
--- stdout -------------------------------
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-83045/issue-83045:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-83045/issue-83045 -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-83045/issue-83045  --crate-name=a --crate-type=rlib a.rs --verbose
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-83045/issue-83045:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-83045/issue-83045 -L /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-83045/issue-83045  --crate-name=b --crate-type=rlib --extern a=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-83045/issue-83045/liba.rlib b.rs --verbose
LD_LIBRARY_PATH="/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-83045/issue-83045:/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-bootstrap-tools/x86_64-unknown-linux-gnu/release/deps:/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/lib" '/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc' --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-83045/issue-83045 \
              --extern b=/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-83045/issue-83045/libb.rlib \
     --crate-type=rlib \
     --edition=2018 \
     c.rs 2>&1 | tee /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-83045/issue-83045/output.txt || exit 0
error[E0519]: the current crate is indistinguishable from one of its dependencies: it has the same crate-name `b` and was compiled with the same `-C metadata` arguments. This will result in symbol conflicts between the two.
 --> c.rs:1:5
1 | use b as _;
  |     ^

thread 'rustc' panicked at 'assertion failed: `(left == right)`
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `22`,
 right: `21`', compiler/rustc_metadata/src/creader.rs:147:9
stack backtrace:
   0:     0x7fc7b1a8baf5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hec7dfb0982cc4a7f
   1:     0x7fc7b1af8cc8 - core::fmt::write::h52b734109196c07f
   2:     0x7fc7b1a801d1 - std::io::Write::write_fmt::h9072b1b2145d5998
   3:     0x7fc7b1a8b901 - std::sys_common::backtrace::print::h76a9511ea61248f2
   4:     0x7fc7b1a8ead4 - std::panicking::default_hook::{{closure}}::h6fbd65b746146ef9
   5:     0x7fc7b1a8e7ba - std::panicking::default_hook::h13af2fcfb82b3ca0
   6:     0x7fc7b2591c85 - rustc_driver_impl[e2b5162d9f475000]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fc7b1a8f1f1 - std::panicking::rust_panic_with_hook::h86cedeb169deb504
   8:     0x7fc7b1a8ef69 - std::panicking::begin_panic_handler::{{closure}}::h4a6f0cf9e0dcbbcd
   9:     0x7fc7b1a8bfc6 - std::sys_common::backtrace::__rust_end_short_backtrace::hb7f042016c431532
  10:     0x7fc7b1a8ec47 - rust_begin_unwind
  11:     0x7fc7b1a452c3 - core::panicking::panic_fmt::h5ee90053ec21d7af
  12:     0x7fc7b1a4564c - core::panicking::assert_failed_inner::h4ef26b99052144f0
  13:     0x7fc7b24ad89b - core[80f943581d377b96]::panicking::assert_failed::<usize, usize>
  14:     0x7fc7b48ed111 - <rustc_metadata[ea61c72448fc15bb]::creader::CrateLoader>::maybe_resolve_crate
  15:     0x7fc7b48e86a6 - <rustc_metadata[ea61c72448fc15bb]::creader::CrateLoader>::resolve_crate
  16:     0x7fc7b48f0848 - <rustc_metadata[ea61c72448fc15bb]::creader::CrateLoader>::process_path_extern
  17:     0x7fc7b34dbf4d - <rustc_resolve[7f7365ee9d40719f]::Resolver>::extern_prelude_get
  18:     0x7fc7b34e1504 - <rustc_resolve[7f7365ee9d40719f]::Resolver>::early_resolve_ident_in_lexical_scope
  19:     0x7fc7b34e50e2 - <rustc_resolve[7f7365ee9d40719f]::Resolver>::resolve_ident_in_module_unadjusted_ext
  20:     0x7fc7b34e3fe4 - <rustc_resolve[7f7365ee9d40719f]::Resolver>::resolve_ident_in_module_ext
  21:     0x7fc7b34e37a0 - <rustc_resolve[7f7365ee9d40719f]::Resolver>::resolve_ident_in_module
  22:     0x7fc7b34c3f7c - <rustc_resolve[7f7365ee9d40719f]::Resolver>::finalize_import
  23:     0x7fc7b34c0b2a - <rustc_resolve[7f7365ee9d40719f]::Resolver>::finalize_imports
  24:     0x7fc7b343dc84 - <rustc_session[40725f0abc45a087]::session::Session>::time::<(), <rustc_resolve[7f7365ee9d40719f]::Resolver>::resolve_crate::{closure#0}>
  25:     0x7fc7b34d838d - <rustc_resolve[7f7365ee9d40719f]::Resolver>::resolve_crate
  26:     0x7fc7b264db70 - rustc_interface[7a30ece0d625b304]::passes::resolver_for_lowering
  27:     0x7fc7b4365f79 - rustc_query_system[cf45bf82d1cf2f0e]::query::plumbing::try_execute_query::<rustc_query_impl[ad12693d9d31ffb4]::queries::resolver_for_lowering, rustc_query_impl[ad12693d9d31ffb4]::plumbing::QueryCtxt>
  28:     0x7fc7b410f0a3 - <rustc_query_impl[ad12693d9d31ffb4]::Queries as rustc_middle[4b6408a24b96b77c]::ty::query::QueryEngine>::resolver_for_lowering
  29:     0x7fc7b2593f30 - <rustc_middle[4b6408a24b96b77c]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[e2b5162d9f475000]::run_compiler::{closure#1}::{closure#2}::{closure#2}, &rustc_data_structures[a31a310f6abccde1]::steal::Steal<(rustc_middle[4b6408a24b96b77c]::ty::ResolverAstLowering, alloc[451f9d15be6edd4a]::rc::Rc<rustc_ast[312290ca3f735160]::ast::Crate>)>>
  30:     0x7fc7b25d8ced - <rustc_interface[7a30ece0d625b304]::interface::Compiler>::enter::<rustc_driver_impl[e2b5162d9f475000]::run_compiler::{closure#1}::{closure#2}, core[80f943581d377b96]::result::Result<core[80f943581d377b96]::option::Option<rustc_interface[7a30ece0d625b304]::queries::Linker>, rustc_span[bdd53373a1ca0d3f]::ErrorGuaranteed>>
  31:     0x7fc7b2592f48 - rustc_span[bdd53373a1ca0d3f]::with_source_map::<core[80f943581d377b96]::result::Result<(), rustc_span[bdd53373a1ca0d3f]::ErrorGuaranteed>, rustc_interface[7a30ece0d625b304]::interface::run_compiler<core[80f943581d377b96]::result::Result<(), rustc_span[bdd53373a1ca0d3f]::ErrorGuaranteed>, rustc_driver_impl[e2b5162d9f475000]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  32:     0x7fc7b25cd617 - <scoped_tls[f08fc2af1499d2e8]::ScopedKey<rustc_span[bdd53373a1ca0d3f]::SessionGlobals>>::set::<rustc_interface[7a30ece0d625b304]::interface::run_compiler<core[80f943581d377b96]::result::Result<(), rustc_span[bdd53373a1ca0d3f]::ErrorGuaranteed>, rustc_driver_impl[e2b5162d9f475000]::run_compiler::{closure#1}>::{closure#0}, core[80f943581d377b96]::result::Result<(), rustc_span[bdd53373a1ca0d3f]::ErrorGuaranteed>>
  33:     0x7fc7b25abc36 - std[ca19ce65004288f1]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[7a30ece0d625b304]::util::run_in_thread_pool_with_globals<rustc_interface[7a30ece0d625b304]::interface::run_compiler<core[80f943581d377b96]::result::Result<(), rustc_span[bdd53373a1ca0d3f]::ErrorGuaranteed>, rustc_driver_impl[e2b5162d9f475000]::run_compiler::{closure#1}>::{closure#0}, core[80f943581d377b96]::result::Result<(), rustc_span[bdd53373a1ca0d3f]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[80f943581d377b96]::result::Result<(), rustc_span[bdd53373a1ca0d3f]::ErrorGuaranteed>>
  34:     0x7fc7b257b566 - std[ca19ce65004288f1]::panicking::try::<core[80f943581d377b96]::result::Result<(), rustc_span[bdd53373a1ca0d3f]::ErrorGuaranteed>, core[80f943581d377b96]::panic::unwind_safe::AssertUnwindSafe<<std[ca19ce65004288f1]::thread::Builder>::spawn_unchecked_<rustc_interface[7a30ece0d625b304]::util::run_in_thread_pool_with_globals<rustc_interface[7a30ece0d625b304]::interface::run_compiler<core[80f943581d377b96]::result::Result<(), rustc_span[bdd53373a1ca0d3f]::ErrorGuaranteed>, rustc_driver_impl[e2b5162d9f475000]::run_compiler::{closure#1}>::{closure#0}, core[80f943581d377b96]::result::Result<(), rustc_span[bdd53373a1ca0d3f]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[80f943581d377b96]::result::Result<(), rustc_span[bdd53373a1ca0d3f]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  35:     0x7fc7b25a1815 - <<std[ca19ce65004288f1]::thread::Builder>::spawn_unchecked_<rustc_interface[7a30ece0d625b304]::util::run_in_thread_pool_with_globals<rustc_interface[7a30ece0d625b304]::interface::run_compiler<core[80f943581d377b96]::result::Result<(), rustc_span[bdd53373a1ca0d3f]::ErrorGuaranteed>, rustc_driver_impl[e2b5162d9f475000]::run_compiler::{closure#1}>::{closure#0}, core[80f943581d377b96]::result::Result<(), rustc_span[bdd53373a1ca0d3f]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[80f943581d377b96]::result::Result<(), rustc_span[bdd53373a1ca0d3f]::ErrorGuaranteed>>::{closure#1} as core[80f943581d377b96]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  36:     0x7fc7b1a9b4de - std::sys::unix::thread::Thread::new::thread_start::h92ab59adecef6a01
  37:     0x7fc7b1835b43 - <unknown>
  38:     0x7fc7b18c7a00 - <unknown>
  39:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

---
end of query stack
error: aborting due to previous error

For more information about this error, try `rustc --explain E0519`.
"/checkout/src/etc/cat-and-grep.sh" E0463 < /checkout/obj/build/x86_64-unknown-linux-gnu/test/run-make-fulldeps/issue-83045/issue-83045/output.txt
[[[ begin stdout ]]]
error[E0519]: the current crate is indistinguishable from one of its dependencies: it has the same crate-name `b` and was compiled with the same `-C metadata` arguments. This will result in symbol conflicts between the two.
 --> c.rs:1:5
1 | use b as _;
  |     ^

thread 'rustc' panicked at 'assertion failed: `(left == right)`
thread 'rustc' panicked at 'assertion failed: `(left == right)`
  left: `22`,
 right: `21`', compiler/rustc_metadata/src/creader.rs:147:9
stack backtrace:
   0:     0x7fc7b1a8baf5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hec7dfb0982cc4a7f
   1:     0x7fc7b1af8cc8 - core::fmt::write::h52b734109196c07f
   2:     0x7fc7b1a801d1 - std::io::Write::write_fmt::h9072b1b2145d5998
   3:     0x7fc7b1a8b901 - std::sys_common::backtrace::print::h76a9511ea61248f2
   4:     0x7fc7b1a8ead4 - std::panicking::default_hook::{{closure}}::h6fbd65b746146ef9
   5:     0x7fc7b1a8e7ba - std::panicking::default_hook::h13af2fcfb82b3ca0
   6:     0x7fc7b2591c85 - rustc_driver_impl[e2b5162d9f475000]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7fc7b1a8f1f1 - std::panicking::rust_panic_with_hook::h86cedeb169deb504
   8:     0x7fc7b1a8ef69 - std::panicking::begin_panic_handler::{{closure}}::h4a6f0cf9e0dcbbcd
   9:     0x7fc7b1a8bfc6 - std::sys_common::backtrace::__rust_end_short_backtrace::hb7f042016c431532
  10:     0x7fc7b1a8ec47 - rust_begin_unwind
  11:     0x7fc7b1a452c3 - core::panicking::panic_fmt::h5ee90053ec21d7af
  12:     0x7fc7b1a4564c - core::panicking::assert_failed_inner::h4ef26b99052144f0
  13:     0x7fc7b24ad89b - core[80f943581d377b96]::panicking::assert_failed::<usize, usize>
  14:     0x7fc7b48ed111 - <rustc_metadata[ea61c72448fc15bb]::creader::CrateLoader>::maybe_resolve_crate
  15:     0x7fc7b48e86a6 - <rustc_metadata[ea61c72448fc15bb]::creader::CrateLoader>::resolve_crate
  16:     0x7fc7b48f0848 - <rustc_metadata[ea61c72448fc15bb]::creader::CrateLoader>::process_path_extern
  17:     0x7fc7b34dbf4d - <rustc_resolve[7f7365ee9d40719f]::Resolver>::extern_prelude_get
  18:     0x7fc7b34e1504 - <rustc_resolve[7f7365ee9d40719f]::Resolver>::early_resolve_ident_in_lexical_scope
  19:     0x7fc7b34e50e2 - <rustc_resolve[7f7365ee9d40719f]::Resolver>::resolve_ident_in_module_unadjusted_ext
  20:     0x7fc7b34e3fe4 - <rustc_resolve[7f7365ee9d40719f]::Resolver>::resolve_ident_in_module_ext
  21:     0x7fc7b34e37a0 - <rustc_resolve[7f7365ee9d40719f]::Resolver>::resolve_ident_in_module
  22:     0x7fc7b34c3f7c - <rustc_resolve[7f7365ee9d40719f]::Resolver>::finalize_import
  23:     0x7fc7b34c0b2a - <rustc_resolve[7f7365ee9d40719f]::Resolver>::finalize_imports
  24:     0x7fc7b343dc84 - <rustc_session[40725f0abc45a087]::session::Session>::time::<(), <rustc_resolve[7f7365ee9d40719f]::Resolver>::resolve_crate::{closure#0}>
  25:     0x7fc7b34d838d - <rustc_resolve[7f7365ee9d40719f]::Resolver>::resolve_crate
  26:     0x7fc7b264db70 - rustc_interface[7a30ece0d625b304]::passes::resolver_for_lowering
  27:     0x7fc7b4365f79 - rustc_query_system[cf45bf82d1cf2f0e]::query::plumbing::try_execute_query::<rustc_query_impl[ad12693d9d31ffb4]::queries::resolver_for_lowering, rustc_query_impl[ad12693d9d31ffb4]::plumbing::QueryCtxt>
  28:     0x7fc7b410f0a3 - <rustc_query_impl[ad12693d9d31ffb4]::Queries as rustc_middle[4b6408a24b96b77c]::ty::query::QueryEngine>::resolver_for_lowering
  29:     0x7fc7b2593f30 - <rustc_middle[4b6408a24b96b77c]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[e2b5162d9f475000]::run_compiler::{closure#1}::{closure#2}::{closure#2}, &rustc_data_structures[a31a310f6abccde1]::steal::Steal<(rustc_middle[4b6408a24b96b77c]::ty::ResolverAstLowering, alloc[451f9d15be6edd4a]::rc::Rc<rustc_ast[312290ca3f735160]::ast::Crate>)>>
  30:     0x7fc7b25d8ced - <rustc_interface[7a30ece0d625b304]::interface::Compiler>::enter::<rustc_driver_impl[e2b5162d9f475000]::run_compiler::{closure#1}::{closure#2}, core[80f943581d377b96]::result::Result<core[80f943581d377b96]::option::Option<rustc_interface[7a30ece0d625b304]::queries::Linker>, rustc_span[bdd53373a1ca0d3f]::ErrorGuaranteed>>
  31:     0x7fc7b2592f48 - rustc_span[bdd53373a1ca0d3f]::with_source_map::<core[80f943581d377b96]::result::Result<(), rustc_span[bdd53373a1ca0d3f]::ErrorGuaranteed>, rustc_interface[7a30ece0d625b304]::interface::run_compiler<core[80f943581d377b96]::result::Result<(), rustc_span[bdd53373a1ca0d3f]::ErrorGuaranteed>, rustc_driver_impl[e2b5162d9f475000]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  32:     0x7fc7b25cd617 - <scoped_tls[f08fc2af1499d2e8]::ScopedKey<rustc_span[bdd53373a1ca0d3f]::SessionGlobals>>::set::<rustc_interface[7a30ece0d625b304]::interface::run_compiler<core[80f943581d377b96]::result::Result<(), rustc_span[bdd53373a1ca0d3f]::ErrorGuaranteed>, rustc_driver_impl[e2b5162d9f475000]::run_compiler::{closure#1}>::{closure#0}, core[80f943581d377b96]::result::Result<(), rustc_span[bdd53373a1ca0d3f]::ErrorGuaranteed>>
  33:     0x7fc7b25abc36 - std[ca19ce65004288f1]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[7a30ece0d625b304]::util::run_in_thread_pool_with_globals<rustc_interface[7a30ece0d625b304]::interface::run_compiler<core[80f943581d377b96]::result::Result<(), rustc_span[bdd53373a1ca0d3f]::ErrorGuaranteed>, rustc_driver_impl[e2b5162d9f475000]::run_compiler::{closure#1}>::{closure#0}, core[80f943581d377b96]::result::Result<(), rustc_span[bdd53373a1ca0d3f]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[80f943581d377b96]::result::Result<(), rustc_span[bdd53373a1ca0d3f]::ErrorGuaranteed>>
  34:     0x7fc7b257b566 - std[ca19ce65004288f1]::panicking::try::<core[80f943581d377b96]::result::Result<(), rustc_span[bdd53373a1ca0d3f]::ErrorGuaranteed>, core[80f943581d377b96]::panic::unwind_safe::AssertUnwindSafe<<std[ca19ce65004288f1]::thread::Builder>::spawn_unchecked_<rustc_interface[7a30ece0d625b304]::util::run_in_thread_pool_with_globals<rustc_interface[7a30ece0d625b304]::interface::run_compiler<core[80f943581d377b96]::result::Result<(), rustc_span[bdd53373a1ca0d3f]::ErrorGuaranteed>, rustc_driver_impl[e2b5162d9f475000]::run_compiler::{closure#1}>::{closure#0}, core[80f943581d377b96]::result::Result<(), rustc_span[bdd53373a1ca0d3f]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[80f943581d377b96]::result::Result<(), rustc_span[bdd53373a1ca0d3f]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  35:     0x7fc7b25a1815 - <<std[ca19ce65004288f1]::thread::Builder>::spawn_unchecked_<rustc_interface[7a30ece0d625b304]::util::run_in_thread_pool_with_globals<rustc_interface[7a30ece0d625b304]::interface::run_compiler<core[80f943581d377b96]::result::Result<(), rustc_span[bdd53373a1ca0d3f]::ErrorGuaranteed>, rustc_driver_impl[e2b5162d9f475000]::run_compiler::{closure#1}>::{closure#0}, core[80f943581d377b96]::result::Result<(), rustc_span[bdd53373a1ca0d3f]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[80f943581d377b96]::result::Result<(), rustc_span[bdd53373a1ca0d3f]::ErrorGuaranteed>>::{closure#1} as core[80f943581d377b96]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  36:     0x7fc7b1a9b4de - std::sys::unix::thread::Thread::new::thread_start::h92ab59adecef6a01
  37:     0x7fc7b1835b43 - <unknown>
  38:     0x7fc7b18c7a00 - <unknown>
  39:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

---
error: aborting due to previous error

For more information about this error, try `rustc --explain E0519`.

[[[ end stdout ]]]
Error: cannot match: E0463
--- stderr -------------------------------
make: *** [Makefile:28: all] Error 1
------------------------------------------

