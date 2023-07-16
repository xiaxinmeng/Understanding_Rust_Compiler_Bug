plain
Updating files:  98% (32355/33015)
Updating files:  99% (32685/33015)
Updating files: 100% (33015/33015)
Updating files: 100% (33015/33015), done.
branch 'try' set up to track 'origin/try'.
Switched to a new branch 'try'
[command]"C:\Program Files\Git\bin\git.exe" log -1 --format='%H'
'031d87645f5dacc484cc4590a1aa9abc7409a80e'
##[group]Run src/ci/scripts/setup-environment.sh
src/ci/scripts/setup-environment.sh
---
  ARTIFACTS_AWS_ACCESS_KEY_ID: AKIA46X5W6CZN24CBO55
  CACHE_DOMAIN: ci-caches.rust-lang.org
  EXTRA_VARIABLES: {
 "RUST_CONFIGURE_ARGS": "--build=x86_64-pc-windows-msvc --host=x86_64-pc-windows-msvc --target=x86_64-pc-windows-msvc --enable-full-tools --enable-profiler",
 "SCRIPT": "python src/ci/pgo-windows.py",
}
##[endgroup]
adding extra environment variable DIST_REQUIRE_ALL_TOOLS
adding extra environment variable RUST_CONFIGURE_ARGS
---
file:.git/config remote.origin.url=https://github.com/rust-lang-ci/rust
file:.git/config remote.origin.fetch=+refs/heads/*:refs/remotes/origin/*
file:.git/config gc.auto=0
file:.git/config http.https://github.com/.extraheader=AUTHORIZATION: basic ***
file:.git/config branch.try.remote=origin
file:.git/config branch.try.merge=refs/heads/try
file:.git/config submodule.library/backtrace.url=https://github.com/rust-lang/backtrace-rs.git
file:.git/config submodule.library/stdarch.active=true
file:.git/config submodule.library/stdarch.url=https://github.com/rust-lang/stdarch.git
file:.git/config submodule.src/doc/edition-guide.active=true
---
[240/3006] Building CXX object lib\BinaryFormat\CMakeFiles\LLVMBinaryFormat.dir\Magic.cpp.obj
[241/3006] Building CXX object lib\BinaryFormat\CMakeFiles\LLVMBinaryFormat.dir\Minidump.cpp.obj
[242/3006] Linking CXX static library lib\LLVMBinaryFormat.lib
[243/3006] Linking CXX executable bin\llvm-tblgen.exe
FAILED: bin/llvm-tblgen.exe 
cmd.exe /C "cd . && "C:\Program Files\CMake\bin\cmake.exe" -E vs_link_exe --intdir=utils\TableGen\CMakeFiles\llvm-tblgen.dir --rc=C:\PROGRA~2\WI3CF2~1\10\bin\100220~1.0\x64\rc.exe --mt=C:\PROGRA~2\WI3CF2~1\10\bin\100220~1.0\x64\mt.exe --manifests  -- D:\a\rust\rust\citools\clang-rust\bin\lld-link.exe /nologo utils\TableGen\CMakeFiles\llvm-tblgen.dir\AsmMatcherEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\AsmWriterEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\AsmWriterInst.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\Attributes.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\CallingConvEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\CodeBeadsGen.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\CodeEmitterGen.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\CodeGenDAGPatterns.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\CodeGenHwModes.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\CodeGenInstruction.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\CodeGenMapTable.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\CodeGenRegisters.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\CodeGenSchedule.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\CodeGenTarget.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\DAGISelEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\DAGISelMatcherEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\DAGISelMatcherGen.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\DAGISelMatcherOpt.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\DAGISelMatcher.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\DFAEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\DFAPacketizerEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\DirectiveEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\DisassemblerEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\ExegesisEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\FastISelEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\FixedLenDecoderEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\GICombinerEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\GlobalISelEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\InfoByHwMode.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\InstrInfoEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\InstrDocsEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\IntrinsicEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\OptEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\OptParserEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\OptRSTEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\PredicateExpander.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\PseudoLoweringEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\CompressInstEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\RegisterBankEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\RegisterInfoEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\SDNodeProperties.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\SearchableTableEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\SubtargetEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\SubtargetFeatureInfo.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\TableGen.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\Types.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\X86DisassemblerTables.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\X86EVEX2VEXTablesEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\X86FoldTablesEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\X86ModRMFilters.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\X86RecognizableInstr.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\WebAssemblyDisassemblerEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\CTagsEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\__\__\resources\windows_version_resource.rc.res  /out:bin\llvm-tblgen.exe /implib:lib\llvm-tblgen.lib /pdb:bin\llvm-tblgen.pdb /version:0.0 /STACK:10000000 /libpath:D:/a/rust/rust/citools/clang-rust/lib/clang/14.0.1/lib/windows /INCREMENTAL:NO /subsystem:console  lib\LLVMSupport.lib  lib\LLVMTableGen.lib  lib\LLVMTableGenGlobalISel.lib  lib\LLVMTableGen.lib  lib\LLVMSupport.lib  psapi.lib  shell32.lib  ole32.lib  uuid.lib  advapi32.lib  delayimp.lib  -delayload:shell32.dll  -delayload:ole32.dll  lib\LLVMDemangle.lib  kernel32.lib user32.lib gdi32.lib winspool.lib shell32.lib ole32.lib oleaut32.lib uuid.lib comdlg32.lib advapi32.lib && cd ."
LINK: command "D:\a\rust\rust\citools\clang-rust\bin\lld-link.exe /nologo utils\TableGen\CMakeFiles\llvm-tblgen.dir\AsmMatcherEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\AsmWriterEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\AsmWriterInst.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\Attributes.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\CallingConvEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\CodeBeadsGen.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\CodeEmitterGen.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\CodeGenDAGPatterns.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\CodeGenHwModes.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\CodeGenInstruction.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\CodeGenMapTable.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\CodeGenRegisters.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\CodeGenSchedule.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\CodeGenTarget.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\DAGISelEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\DAGISelMatcherEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\DAGISelMatcherGen.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\DAGISelMatcherOpt.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\DAGISelMatcher.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\DFAEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\DFAPacketizerEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\DirectiveEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\DisassemblerEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\ExegesisEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\FastISelEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\FixedLenDecoderEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\GICombinerEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\GlobalISelEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\InfoByHwMode.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\InstrInfoEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\InstrDocsEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\IntrinsicEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\OptEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\OptParserEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\OptRSTEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\PredicateExpander.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\PseudoLoweringEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\CompressInstEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\RegisterBankEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\RegisterInfoEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\SDNodeProperties.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\SearchableTableEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\SubtargetEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\SubtargetFeatureInfo.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\TableGen.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\Types.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\X86DisassemblerTables.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\X86EVEX2VEXTablesEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\X86FoldTablesEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\X86ModRMFilters.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\X86RecognizableInstr.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\WebAssemblyDisassemblerEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\CTagsEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\__\__\resources\windows_version_resource.rc.res /out:bin\llvm-tblgen.exe /implib:lib\llvm-tblgen.lib /pdb:bin\llvm-tblgen.pdb /version:0.0 /STACK:10000000 /libpath:D:/a/rust/rust/citools/clang-rust/lib/clang/14.0.1/lib/windows /INCREMENTAL:NO /subsystem:console lib\LLVMSupport.lib lib\LLVMTableGen.lib lib\LLVMTableGenGlobalISel.lib lib\LLVMTableGen.lib lib\LLVMSupport.lib psapi.lib shell32.lib ole32.lib uuid.lib advapi32.lib delayimp.lib -delayload:shell32.dll -delayload:ole32.dll lib\LLVMDemangle.lib kernel32.lib user32.lib gdi32.lib winspool.lib shell32.lib ole32.lib oleaut32.lib uuid.lib comdlg32.lib advapi32.lib /MANIFEST /MANIFESTFILE:bin\llvm-tblgen.exe.manifest" failed (exit code 1) with the following output:
lld-link: error: undefined symbol: __llvm_profile_instrument_memop

>>> referenced by utils\TableGen\CMakeFiles\llvm-tblgen.dir\AsmMatcherEmitter.cpp.obj:(public: void __cdecl `anonymous namespace'::AsmMatcherEmitter::run(class llvm::raw_ostream &))

>>> referenced by utils\TableGen\CMakeFiles\llvm-tblgen.dir\AsmMatcherEmitter.cpp.obj:(public: void __cdecl `anonymous namespace'::AsmMatcherEmitter::run(class llvm::raw_ostream &))

>>> referenced by utils\TableGen\CMakeFiles\llvm-tblgen.dir\AsmMatcherEmitter.cpp.obj:(public: void __cdecl `anonymous namespace'::AsmMatcherEmitter::run(class llvm::raw_ostream &))

>>> referenced 10725 more times
[244/3006] Building CXX object lib\Extensions\CMakeFiles\LLVMExtensions.dir\Extensions.cpp.obj
[245/3006] Building CXX object lib\Bitstream\Reader\CMakeFiles\LLVMBitstreamReader.dir\BitstreamReader.cpp.obj
[246/3006] Building CXX object lib\MC\CMakeFiles\LLVMMC.dir\MCAsmInfoCOFF.cpp.obj
[247/3006] Building CXX object lib\MC\CMakeFiles\LLVMMC.dir\MCAsmInfoELF.cpp.obj
[247/3006] Building CXX object lib\MC\CMakeFiles\LLVMMC.dir\MCAsmInfoELF.cpp.obj
[248/3006] Building CXX object lib\MC\CMakeFiles\LLVMMC.dir\MCAsmInfoGOFF.cpp.obj
[249/3006] Building CXX object lib\MC\CMakeFiles\LLVMMC.dir\MCAsmInfoDarwin.cpp.obj
[250/3006] Building CXX object lib\MC\CMakeFiles\LLVMMC.dir\ConstantPools.cpp.obj
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit code: 1


build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\github.com-1ecc6299db9ec823\cmake-0.1.44\src\lib.rs:885:5
   0:     0x7ff6acef998f - std::backtrace_rs::backtrace::dbghelp::trace
   0:     0x7ff6acef998f - std::backtrace_rs::backtrace::dbghelp::trace
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\/library\std\src\..\..\backtrace\src\backtrace\dbghelp.rs:98
   1:     0x7ff6acef998f - std::backtrace_rs::backtrace::trace_unsynchronized
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\/library\std\src\..\..\backtrace\src\backtrace\mod.rs:66
   2:     0x7ff6acef998f - std::sys_common::backtrace::_print_fmt
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\/library\std\src\sys_common\backtrace.rs:66
   3:     0x7ff6acef998f - std::sys_common::backtrace::_print::impl$0::fmt
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\/library\std\src\sys_common\backtrace.rs:45
   4:     0x7ff6acf1a4ca - core::fmt::write
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\/library\core\src\fmt\mod.rs:1194
   5:     0x7ff6acef2ed9 - std::io::Write::write_fmt<std::sys::windows::stdio::Stderr>
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\/library\std\src\io\mod.rs:1655
   6:     0x7ff6acefc9fb - std::sys_common::backtrace::_print
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\/library\std\src\sys_common\backtrace.rs:48
   7:     0x7ff6acefc9fb - std::sys_common::backtrace::print
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\/library\std\src\sys_common\backtrace.rs:35
   8:     0x7ff6acefc9fb - std::panicking::default_hook::closure$1
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\/library\std\src\panicking.rs:295
   9:     0x7ff6acefc5ee - std::panicking::default_hook
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\/library\std\src\panicking.rs:314
  10:     0x7ff6acefcff1 - std::panicking::rust_panic_with_hook
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\/library\std\src\panicking.rs:698
  11:     0x7ff6acefcead - std::panicking::begin_panic_handler::closure$0
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\/library\std\src\panicking.rs:588
  12:     0x7ff6acefa297 - std::sys_common::backtrace::__rust_end_short_backtrace<std::panicking::begin_panic_handler::closure_env$0,never$>
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\/library\std\src\sys_common\backtrace.rs:138
  13:     0x7ff6acefcb89 - std::panicking::begin_panic_handler
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\/library\std\src\panicking.rs:584
  14:     0x7ff6acf504f5 - core::panicking::panic_fmt
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\/library\core\src\panicking.rs:143
  15:     0x7ff6acb84e22 - cmake::find_exe::{{closure}}::h18955f5ff0f2d640
  16:     0x7ff6acb847d8 - cmake::Config::maybe_clear::{{closure}}::h906c96b41c1f02ad
  17:     0x7ff6acb8168b - cmake::Config::build::hbed4a700ed8f5ede
  18:     0x7ff6acaf95b7 - bootstrap::native::impl$1::run
                               at D:\a\rust\rust\src\bootstrap\native.rs:659
  19:     0x7ff6ac9c88f0 - bootstrap::builder::Builder::ensure<bootstrap::native::Llvm>
                               at D:\a\rust\rust\src\bootstrap\builder.rs:1789
  20:     0x7ff6acab3a96 - bootstrap::compile::rustc_cargo_env
  21:     0x7ff6acab35b7 - bootstrap::compile::rustc_cargo
                               at D:\a\rust\rust\src\bootstrap\compile.rs:653
                               at D:\a\rust\rust\src\bootstrap\compile.rs:653
  22:     0x7ff6acab26a9 - bootstrap::compile::impl$3::run
                               at D:\a\rust\rust\src\bootstrap\compile.rs:589
  23:     0x7ff6ac988a9d - bootstrap::builder::Builder::ensure<bootstrap::compile::Rustc>
                               at D:\a\rust\rust\src\bootstrap\builder.rs:1789
  24:     0x7ff6acab793c - bootstrap::compile::impl$7::run
                               at D:\a\rust\rust\src\bootstrap\compile.rs:1102
  25:     0x7ff6ac97ff59 - bootstrap::builder::Builder::ensure<bootstrap::compile::Assemble>
                               at D:\a\rust\rust\src\bootstrap\builder.rs:1789
  26:     0x7ff6ac7dc7f4 - bootstrap::builder::Builder::compiler
                               at D:\a\rust\rust\src\bootstrap\builder.rs:766
  27:     0x7ff6acab7830 - bootstrap::compile::impl$7::run
                               at D:\a\rust\rust\src\bootstrap\compile.rs:1089
  28:     0x7ff6ac97ff59 - bootstrap::builder::Builder::ensure<bootstrap::compile::Assemble>
                               at D:\a\rust\rust\src\bootstrap\builder.rs:1789
  29:     0x7ff6ac7dc7f4 - bootstrap::builder::Builder::compiler
                               at D:\a\rust\rust\src\bootstrap\builder.rs:766
  30:     0x7ff6ac85191d - bootstrap::doc::impl$2::make_run
                               at D:\a\rust\rust\src\bootstrap\doc.rs:190
  31:     0x7ff6ac7d7aa3 - bootstrap::builder::StepDescription::maybe_run
                               at D:\a\rust\rust\src\bootstrap\builder.rs:224
  32:     0x7ff6ac7d812d - bootstrap::builder::StepDescription::run
                               at D:\a\rust\rust\src\bootstrap\builder.rs:262
  33:     0x7ff6ac7dc75d - bootstrap::builder::Builder::run_step_descriptions
                               at D:\a\rust\rust\src\bootstrap\builder.rs:758
  34:     0x7ff6ac7dc50b - bootstrap::builder::Builder::default_doc
                               at D:\a\rust\rust\src\bootstrap\builder.rs:742
  35:     0x7ff6acabde35 - bootstrap::dist::impl$0::run
                               at D:\a\rust\rust\src\bootstrap\dist.rs:74
  36:     0x7ff6aca28feb - bootstrap::builder::Builder::ensure<bootstrap::dist::Docs>
                               at D:\a\rust\rust\src\bootstrap\builder.rs:1789
  37:     0x7ff6acabdd8c - bootstrap::dist::impl$0::make_run
                               at D:\a\rust\rust\src\bootstrap\dist.rs:68
  38:     0x7ff6ac7d7aa3 - bootstrap::builder::StepDescription::maybe_run
                               at D:\a\rust\rust\src\bootstrap\builder.rs:224
  39:     0x7ff6ac7d812d - bootstrap::builder::StepDescription::run
                               at D:\a\rust\rust\src\bootstrap\builder.rs:262
  40:     0x7ff6ac7dc75d - bootstrap::builder::Builder::run_step_descriptions
                               at D:\a\rust\rust\src\bootstrap\builder.rs:758
  41:     0x7ff6ac7dc459 - bootstrap::builder::Builder::execute_cli
                               at D:\a\rust\rust\src\bootstrap\builder.rs:738
  42:     0x7ff6ac6b926b - bootstrap::Build::build
                               at D:\a\rust\rust\src\bootstrap\lib.rs:685
  43:     0x7ff6ac6a1b5f - bootstrap::main
                               at D:\a\rust\rust\src\bootstrap\bin\main.rs:34
  44:     0x7ff6ac6a1536 - core::ops::function::FnOnce::call_once<void (*)(),tuple$<> >
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\library\core\src\ops\function.rs:227
  45:     0x7ff6ac6a1017 - std::sys_common::backtrace::__rust_begin_short_backtrace<void (*)(),tuple$<> >
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\library\std\src\sys_common\backtrace.rs:122
  46:     0x7ff6ac6a26dc - std::rt::lang_start::closure$0<tuple$<> >
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\library\std\src\rt.rs:145
  47:     0x7ff6aceeb94e - core::ops::function::impls::impl$2::call_once
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\library\core\src\ops\function.rs:259
  48:     0x7ff6aceeb94e - std::panicking::try::do_call
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\/library\std\src\panicking.rs:492
  49:     0x7ff6aceeb94e - std::panicking::try
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\/library\std\src\panicking.rs:456
  50:     0x7ff6aceeb94e - std::panic::catch_unwind
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\/library\std\src\panic.rs:137
  51:     0x7ff6aceeb94e - std::rt::lang_start_internal::closure$2
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\/library\std\src\rt.rs:128
  52:     0x7ff6aceeb94e - std::panicking::try::do_call
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\/library\std\src\panicking.rs:492
  53:     0x7ff6aceeb94e - std::panicking::try
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\/library\std\src\panicking.rs:456
  54:     0x7ff6aceeb94e - std::panic::catch_unwind
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\/library\std\src\panic.rs:137
  55:     0x7ff6aceeb94e - std::rt::lang_start_internal
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\/library\std\src\rt.rs:128
  56:     0x7ff6ac6a26c0 - std::rt::lang_start<tuple$<> >
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\library\std\src\rt.rs:144
  57:     0x7ff6ac6a2246 - main
  58:     0x7ff6acf4e4a0 - invoke_main
                               at D:\a\_work\1\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:78
  59:     0x7ff6acf4e4a0 - __scrt_common_main_seh
                               at D:\a\_work\1\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:288
  60:     0x7ffb035b7974 - BaseThreadInitThunk
  61:     0x7ffb0505a2f1 - RtlUserThreadStart
Build completed unsuccessfully in 0:04:38
Traceback (most recent call last):
Traceback (most recent call last):
  File "D:\a\rust\rust\src\ci\pgo-windows.py", line 22, in <module>
    subprocess.run([
  File "C:\hostedtoolcache\windows\Python\3.10.4\x64\lib\subprocess.py", line 524, in run
    raise CalledProcessError(retcode, process.args,
subprocess.CalledProcessError: Command '['python', 'x.py', 'dist', '--llvm-profile-generate']' returned non-zero exit status 1.
