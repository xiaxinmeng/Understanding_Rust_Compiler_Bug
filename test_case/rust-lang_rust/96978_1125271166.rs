plain
Updating files:  98% (32355/33015)
Updating files:  99% (32685/33015)
Updating files: 100% (33015/33015)
Updating files: 100% (33015/33015), done.
branch 'try' set up to track 'origin/try'.
Switched to a new branch 'try'
[command]"C:\Program Files\Git\bin\git.exe" log -1 --format='%H'
'57b5bac113c6e1100592c1bf7d5a7fc61d74e332'
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
[246/3006] Building CXX object lib\FileCheck\CMakeFiles\LLVMFileCheck.dir\FileCheck.cpp.obj
[247/3006] Linking CXX static library lib\LLVMFileCheck.lib
[248/3006] Building CXX object lib\Debuginfod\CMakeFiles\LLVMDebuginfod.dir\Debuginfod.cpp.obj
[249/3006] Linking CXX executable bin\llvm-tblgen.exe
FAILED: bin/llvm-tblgen.exe 
cmd.exe /C "cd . && "C:\Program Files\CMake\bin\cmake.exe" -E vs_link_exe --intdir=utils\TableGen\CMakeFiles\llvm-tblgen.dir --rc=C:\PROGRA~2\WI3CF2~1\10\bin\100220~1.0\x64\rc.exe --mt=C:\PROGRA~2\WI3CF2~1\10\bin\100220~1.0\x64\mt.exe --manifests  -- C:\PROGRA~2\MICROS~1\2019\ENTERP~1\VC\Tools\MSVC\1429~1.301\bin\Hostx64\x64\link.exe /nologo utils\TableGen\CMakeFiles\llvm-tblgen.dir\AsmMatcherEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\AsmWriterEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\AsmWriterInst.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\Attributes.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\CallingConvEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\CodeBeadsGen.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\CodeEmitterGen.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\CodeGenDAGPatterns.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\CodeGenHwModes.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\CodeGenInstruction.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\CodeGenMapTable.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\CodeGenRegisters.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\CodeGenSchedule.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\CodeGenTarget.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\DAGISelEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\DAGISelMatcherEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\DAGISelMatcherGen.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\DAGISelMatcherOpt.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\DAGISelMatcher.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\DFAEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\DFAPacketizerEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\DirectiveEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\DisassemblerEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\ExegesisEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\FastISelEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\FixedLenDecoderEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\GICombinerEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\GlobalISelEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\InfoByHwMode.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\InstrInfoEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\InstrDocsEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\IntrinsicEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\OptEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\OptParserEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\OptRSTEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\PredicateExpander.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\PseudoLoweringEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\CompressInstEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\RegisterBankEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\RegisterInfoEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\SDNodeProperties.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\SearchableTableEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\SubtargetEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\SubtargetFeatureInfo.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\TableGen.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\Types.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\X86DisassemblerTables.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\X86EVEX2VEXTablesEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\X86FoldTablesEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\X86ModRMFilters.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\X86RecognizableInstr.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\WebAssemblyDisassemblerEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\CTagsEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\__\__\resources\windows_version_resource.rc.res  /out:bin\llvm-tblgen.exe /implib:lib\llvm-tblgen.lib /pdb:bin\llvm-tblgen.pdb /version:0.0 /STACK:10000000 -fprofile-generate="D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\profiles" /libpath:D:/a/rust/rust/citools/clang-rust/lib/clang/14.0.1/lib/windows /INCREMENTAL:NO /subsystem:console  lib\LLVMSupport.lib  lib\LLVMTableGen.lib  lib\LLVMTableGenGlobalISel.lib  lib\LLVMTableGen.lib  lib\LLVMSupport.lib  psapi.lib  shell32.lib  ole32.lib  uuid.lib  advapi32.lib  delayimp.lib  -delayload:shell32.dll  -delayload:ole32.dll  lib\LLVMDemangle.lib  kernel32.lib user32.lib gdi32.lib winspool.lib shell32.lib ole32.lib oleaut32.lib uuid.lib comdlg32.lib advapi32.lib && cd ."
LINK: command "C:\PROGRA~2\MICROS~1\2019\ENTERP~1\VC\Tools\MSVC\1429~1.301\bin\Hostx64\x64\link.exe /nologo utils\TableGen\CMakeFiles\llvm-tblgen.dir\AsmMatcherEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\AsmWriterEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\AsmWriterInst.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\Attributes.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\CallingConvEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\CodeBeadsGen.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\CodeEmitterGen.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\CodeGenDAGPatterns.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\CodeGenHwModes.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\CodeGenInstruction.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\CodeGenMapTable.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\CodeGenRegisters.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\CodeGenSchedule.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\CodeGenTarget.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\DAGISelEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\DAGISelMatcherEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\DAGISelMatcherGen.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\DAGISelMatcherOpt.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\DAGISelMatcher.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\DFAEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\DFAPacketizerEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\DirectiveEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\DisassemblerEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\ExegesisEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\FastISelEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\FixedLenDecoderEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\GICombinerEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\GlobalISelEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\InfoByHwMode.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\InstrInfoEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\InstrDocsEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\IntrinsicEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\OptEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\OptParserEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\OptRSTEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\PredicateExpander.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\PseudoLoweringEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\CompressInstEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\RegisterBankEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\RegisterInfoEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\SDNodeProperties.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\SearchableTableEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\SubtargetEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\SubtargetFeatureInfo.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\TableGen.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\Types.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\X86DisassemblerTables.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\X86EVEX2VEXTablesEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\X86FoldTablesEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\X86ModRMFilters.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\X86RecognizableInstr.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\WebAssemblyDisassemblerEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\CTagsEmitter.cpp.obj utils\TableGen\CMakeFiles\llvm-tblgen.dir\__\__\resources\windows_version_resource.rc.res /out:bin\llvm-tblgen.exe /implib:lib\llvm-tblgen.lib /pdb:bin\llvm-tblgen.pdb /version:0.0 /STACK:10000000 -fprofile-generate=D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\profiles /libpath:D:/a/rust/rust/citools/clang-rust/lib/clang/14.0.1/lib/windows /INCREMENTAL:NO /subsystem:console lib\LLVMSupport.lib lib\LLVMTableGen.lib lib\LLVMTableGenGlobalISel.lib lib\LLVMTableGen.lib lib\LLVMSupport.lib psapi.lib shell32.lib ole32.lib uuid.lib advapi32.lib delayimp.lib -delayload:shell32.dll -delayload:ole32.dll lib\LLVMDemangle.lib kernel32.lib user32.lib gdi32.lib winspool.lib shell32.lib ole32.lib oleaut32.lib uuid.lib comdlg32.lib advapi32.lib /MANIFEST /MANIFESTFILE:bin\llvm-tblgen.exe.manifest" failed (exit code 1120) with the following output:
LINK : warning LNK4044: unrecognized option '/fprofile-generate=D:\a\rust\rust\build\x86_64-pc-windows-msvc\llvm\build\profiles'; ignored

LLVMTableGenGlobalISel.lib(GIMatchDag.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMTableGenGlobalISel.lib(GIMatchDagPredicateDependencyEdge.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMSupport.lib(JSON.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMTableGenGlobalISel.lib(CodeExpander.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMTableGenGlobalISel.lib(GIMatchDagEdge.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMTableGenGlobalISel.lib(GIMatchDagInstr.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMTableGenGlobalISel.lib(GIMatchDagPredicate.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMTableGen.lib(TGLexer.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMTableGen.lib(TGParser.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMTableGenGlobalISel.lib(GIMatchDagOperands.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMTableGenGlobalISel.lib(GIMatchTree.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMTableGen.lib(SetTheory.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMTableGen.lib(Main.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMTableGen.lib(DetailedRecordsBackend.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMTableGen.lib(JSONBackend.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMTableGen.lib(TableGenBackend.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMTableGen.lib(Record.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMTableGen.lib(StringMatcher.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMTableGen.lib(Error.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMSupport.lib(MD5.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMSupport.lib(YAMLParser.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMSupport.lib(ARMTargetParser.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMSupport.lib(VersionTuple.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMSupport.lib(regexec.c.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMSupport.lib(Threading.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMSupport.lib(ToolOutputFile.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMSupport.lib(PrettyStackTrace.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMSupport.lib(RandomNumberGenerator.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMSupport.lib(Program.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMSupport.lib(APFloat.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMSupport.lib(regcomp.c.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMSupport.lib(Signals.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMSupport.lib(Statistic.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMSupport.lib(Timer.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMSupport.lib(WithColor.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMSupport.lib(Triple.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMSupport.lib(Process.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMSupport.lib(DebugCounter.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMSupport.lib(GraphWriter.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMSupport.lib(Path.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMSupport.lib(VirtualFileSystem.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMSupport.lib(ConvertUTFWrapper.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMSupport.lib(Host.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMSupport.lib(MemoryBuffer.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMSupport.lib(Error.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMSupport.lib(InitLLVM.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMSupport.lib(StringSaver.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMSupport.lib(SourceMgr.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMSupport.lib(FormattedStream.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMSupport.lib(FoldingSet.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMSupport.lib(CodeGenCoverage.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMSupport.lib(ErrorHandling.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMSupport.lib(StringExtras.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMSupport.lib(TypeSize.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMSupport.lib(Regex.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMSupport.lib(StringMap.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMSupport.lib(NativeFormatting.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMSupport.lib(APInt.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMSupport.lib(Debug.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMSupport.lib(raw_ostream.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMSupport.lib(StringRef.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMSupport.lib(Twine.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMSupport.lib(SmallPtrSet.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

WebAssemblyDisassemblerEmitter.cpp.obj : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

CTagsEmitter.cpp.obj : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMSupport.lib(CommandLine.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

LLVMSupport.lib(SmallVector.cpp.obj) : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

X86DisassemblerTables.cpp.obj : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

X86EVEX2VEXTablesEmitter.cpp.obj : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

X86FoldTablesEmitter.cpp.obj : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

X86RecognizableInstr.cpp.obj : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

SearchableTableEmitter.cpp.obj : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

SubtargetEmitter.cpp.obj : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

SubtargetFeatureInfo.cpp.obj : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

TableGen.cpp.obj : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

PseudoLoweringEmitter.cpp.obj : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

CompressInstEmitter.cpp.obj : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

RegisterBankEmitter.cpp.obj : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

RegisterInfoEmitter.cpp.obj : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

OptEmitter.cpp.obj : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

OptParserEmitter.cpp.obj : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

OptRSTEmitter.cpp.obj : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

PredicateExpander.cpp.obj : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

InfoByHwMode.cpp.obj : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

InstrInfoEmitter.cpp.obj : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

InstrDocsEmitter.cpp.obj : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

IntrinsicEmitter.cpp.obj : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

FastISelEmitter.cpp.obj : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

FixedLenDecoderEmitter.cpp.obj : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

GICombinerEmitter.cpp.obj : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

GlobalISelEmitter.cpp.obj : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

DFAPacketizerEmitter.cpp.obj : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

DirectiveEmitter.cpp.obj : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

DisassemblerEmitter.cpp.obj : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

ExegesisEmitter.cpp.obj : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

DAGISelMatcherGen.cpp.obj : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

DAGISelMatcherOpt.cpp.obj : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

DAGISelMatcher.cpp.obj : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

DFAEmitter.cpp.obj : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

CodeGenSchedule.cpp.obj : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

CodeGenTarget.cpp.obj : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

DAGISelEmitter.cpp.obj : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

DAGISelMatcherEmitter.cpp.obj : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

CodeGenHwModes.cpp.obj : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

CodeGenInstruction.cpp.obj : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

CodeGenMapTable.cpp.obj : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

CodeGenRegisters.cpp.obj : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

CallingConvEmitter.cpp.obj : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

CodeBeadsGen.cpp.obj : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

CodeEmitterGen.cpp.obj : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

CodeGenDAGPatterns.cpp.obj : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

AsmMatcherEmitter.cpp.obj : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

AsmWriterEmitter.cpp.obj : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

AsmWriterInst.cpp.obj : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

Attributes.cpp.obj : error LNK2001: unresolved external symbol __llvm_profile_instrument_memop

bin\llvm-tblgen.exe : fatal error LNK1120: 1 unresolved externals
[250/3006] Building CXX object lib\Debuginfod\CMakeFiles\LLVMDebuginfod.dir\DIFetcher.cpp.obj
[251/3006] Building CXX object lib\MC\CMakeFiles\LLVMMC.dir\MCAsmInfoCOFF.cpp.obj
[252/3006] Building CXX object lib\MC\CMakeFiles\LLVMMC.dir\MCAsmInfoGOFF.cpp.obj
[253/3006] Building CXX object lib\MC\CMakeFiles\LLVMMC.dir\MCAsmInfoWasm.cpp.obj
[253/3006] Building CXX object lib\MC\CMakeFiles\LLVMMC.dir\MCAsmInfoWasm.cpp.obj
[254/3006] Building CXX object lib\MC\CMakeFiles\LLVMMC.dir\MCAsmInfoELF.cpp.obj
[255/3006] Building CXX object lib\MC\CMakeFiles\LLVMMC.dir\MCAsmBackend.cpp.obj
[256/3006] Building CXX object lib\MC\CMakeFiles\LLVMMC.dir\ELFObjectWriter.cpp.obj
ninja: build stopped: subcommand failed.
command did not execute successfully, got: exit code: 1


build script failed, must exit now', C:\Users\runneradmin\.cargo\registry\src\github.com-1ecc6299db9ec823\cmake-0.1.44\src\lib.rs:885:5
   0:     0x7ff64227984f - std::backtrace_rs::backtrace::dbghelp::trace
   0:     0x7ff64227984f - std::backtrace_rs::backtrace::dbghelp::trace
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\/library\std\src\..\..\backtrace\src\backtrace\dbghelp.rs:98
   1:     0x7ff64227984f - std::backtrace_rs::backtrace::trace_unsynchronized
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\/library\std\src\..\..\backtrace\src\backtrace\mod.rs:66
   2:     0x7ff64227984f - std::sys_common::backtrace::_print_fmt
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\/library\std\src\sys_common\backtrace.rs:66
   3:     0x7ff64227984f - std::sys_common::backtrace::_print::impl$0::fmt
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\/library\std\src\sys_common\backtrace.rs:45
   4:     0x7ff64229a38a - core::fmt::write
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\/library\core\src\fmt\mod.rs:1194
   5:     0x7ff642272d99 - std::io::Write::write_fmt<std::sys::windows::stdio::Stderr>
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\/library\std\src\io\mod.rs:1655
   6:     0x7ff64227c8bb - std::sys_common::backtrace::_print
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\/library\std\src\sys_common\backtrace.rs:48
   7:     0x7ff64227c8bb - std::sys_common::backtrace::print
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\/library\std\src\sys_common\backtrace.rs:35
   8:     0x7ff64227c8bb - std::panicking::default_hook::closure$1
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\/library\std\src\panicking.rs:295
   9:     0x7ff64227c4ae - std::panicking::default_hook
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\/library\std\src\panicking.rs:314
  10:     0x7ff64227ceb1 - std::panicking::rust_panic_with_hook
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\/library\std\src\panicking.rs:698
  11:     0x7ff64227cd6d - std::panicking::begin_panic_handler::closure$0
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\/library\std\src\panicking.rs:588
  12:     0x7ff64227a157 - std::sys_common::backtrace::__rust_end_short_backtrace<std::panicking::begin_panic_handler::closure_env$0,never$>
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\/library\std\src\sys_common\backtrace.rs:138
  13:     0x7ff64227ca49 - std::panicking::begin_panic_handler
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\/library\std\src\panicking.rs:584
  14:     0x7ff6422d03b5 - core::panicking::panic_fmt
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\/library\core\src\panicking.rs:143
  15:     0x7ff641f04ce2 - cmake::find_exe::{{closure}}::h18955f5ff0f2d640
  16:     0x7ff641f04698 - cmake::Config::maybe_clear::{{closure}}::h906c96b41c1f02ad
  17:     0x7ff641f0154b - cmake::Config::build::hbed4a700ed8f5ede
  18:     0x7ff641e795b7 - bootstrap::native::impl$1::run
                               at D:\a\rust\rust\src\bootstrap\native.rs:659
  19:     0x7ff641d488f0 - bootstrap::builder::Builder::ensure<bootstrap::native::Llvm>
                               at D:\a\rust\rust\src\bootstrap\builder.rs:1789
  20:     0x7ff641e33a96 - bootstrap::compile::rustc_cargo_env
  21:     0x7ff641e335b7 - bootstrap::compile::rustc_cargo
                               at D:\a\rust\rust\src\bootstrap\compile.rs:653
                               at D:\a\rust\rust\src\bootstrap\compile.rs:653
  22:     0x7ff641e326a9 - bootstrap::compile::impl$3::run
                               at D:\a\rust\rust\src\bootstrap\compile.rs:589
  23:     0x7ff641d08a9d - bootstrap::builder::Builder::ensure<bootstrap::compile::Rustc>
                               at D:\a\rust\rust\src\bootstrap\builder.rs:1789
  24:     0x7ff641e3793c - bootstrap::compile::impl$7::run
                               at D:\a\rust\rust\src\bootstrap\compile.rs:1102
  25:     0x7ff641cfff59 - bootstrap::builder::Builder::ensure<bootstrap::compile::Assemble>
                               at D:\a\rust\rust\src\bootstrap\builder.rs:1789
  26:     0x7ff641b5c7f4 - bootstrap::builder::Builder::compiler
                               at D:\a\rust\rust\src\bootstrap\builder.rs:766
  27:     0x7ff641e37830 - bootstrap::compile::impl$7::run
                               at D:\a\rust\rust\src\bootstrap\compile.rs:1089
  28:     0x7ff641cfff59 - bootstrap::builder::Builder::ensure<bootstrap::compile::Assemble>
                               at D:\a\rust\rust\src\bootstrap\builder.rs:1789
  29:     0x7ff641b5c7f4 - bootstrap::builder::Builder::compiler
                               at D:\a\rust\rust\src\bootstrap\builder.rs:766
  30:     0x7ff641bd191d - bootstrap::doc::impl$2::make_run
                               at D:\a\rust\rust\src\bootstrap\doc.rs:190
  31:     0x7ff641b57aa3 - bootstrap::builder::StepDescription::maybe_run
                               at D:\a\rust\rust\src\bootstrap\builder.rs:224
  32:     0x7ff641b5812d - bootstrap::builder::StepDescription::run
                               at D:\a\rust\rust\src\bootstrap\builder.rs:262
  33:     0x7ff641b5c75d - bootstrap::builder::Builder::run_step_descriptions
                               at D:\a\rust\rust\src\bootstrap\builder.rs:758
  34:     0x7ff641b5c50b - bootstrap::builder::Builder::default_doc
                               at D:\a\rust\rust\src\bootstrap\builder.rs:742
  35:     0x7ff641e3de35 - bootstrap::dist::impl$0::run
                               at D:\a\rust\rust\src\bootstrap\dist.rs:74
  36:     0x7ff641da8feb - bootstrap::builder::Builder::ensure<bootstrap::dist::Docs>
                               at D:\a\rust\rust\src\bootstrap\builder.rs:1789
  37:     0x7ff641e3dd8c - bootstrap::dist::impl$0::make_run
                               at D:\a\rust\rust\src\bootstrap\dist.rs:68
  38:     0x7ff641b57aa3 - bootstrap::builder::StepDescription::maybe_run
                               at D:\a\rust\rust\src\bootstrap\builder.rs:224
  39:     0x7ff641b5812d - bootstrap::builder::StepDescription::run
                               at D:\a\rust\rust\src\bootstrap\builder.rs:262
  40:     0x7ff641b5c75d - bootstrap::builder::Builder::run_step_descriptions
                               at D:\a\rust\rust\src\bootstrap\builder.rs:758
  41:     0x7ff641b5c459 - bootstrap::builder::Builder::execute_cli
                               at D:\a\rust\rust\src\bootstrap\builder.rs:738
  42:     0x7ff641a3926b - bootstrap::Build::build
                               at D:\a\rust\rust\src\bootstrap\lib.rs:685
  43:     0x7ff641a21b5f - bootstrap::main
                               at D:\a\rust\rust\src\bootstrap\bin\main.rs:34
  44:     0x7ff641a21536 - core::ops::function::FnOnce::call_once<void (*)(),tuple$<> >
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\library\core\src\ops\function.rs:227
  45:     0x7ff641a21017 - std::sys_common::backtrace::__rust_begin_short_backtrace<void (*)(),tuple$<> >
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\library\std\src\sys_common\backtrace.rs:122
  46:     0x7ff641a226dc - std::rt::lang_start::closure$0<tuple$<> >
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\library\std\src\rt.rs:145
  47:     0x7ff64226b80e - core::ops::function::impls::impl$2::call_once
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\library\core\src\ops\function.rs:259
  48:     0x7ff64226b80e - std::panicking::try::do_call
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\/library\std\src\panicking.rs:492
  49:     0x7ff64226b80e - std::panicking::try
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\/library\std\src\panicking.rs:456
  50:     0x7ff64226b80e - std::panic::catch_unwind
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\/library\std\src\panic.rs:137
  51:     0x7ff64226b80e - std::rt::lang_start_internal::closure$2
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\/library\std\src\rt.rs:128
  52:     0x7ff64226b80e - std::panicking::try::do_call
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\/library\std\src\panicking.rs:492
  53:     0x7ff64226b80e - std::panicking::try
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\/library\std\src\panicking.rs:456
  54:     0x7ff64226b80e - std::panic::catch_unwind
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\/library\std\src\panic.rs:137
  55:     0x7ff64226b80e - std::rt::lang_start_internal
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\/library\std\src\rt.rs:128
  56:     0x7ff641a226c0 - std::rt::lang_start<tuple$<> >
                               at /rustc/0f231250ba7fe3a3f98a18aee9abfd65b104fb92\library\std\src\rt.rs:144
  57:     0x7ff641a22246 - main
  58:     0x7ff6422ce360 - invoke_main
                               at D:\a\_work\1\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:78
  59:     0x7ff6422ce360 - __scrt_common_main_seh
                               at D:\a\_work\1\s\src\vctools\crt\vcstartup\src\startup\exe_common.inl:288
  60:     0x7ffcb5b07974 - BaseThreadInitThunk
  61:     0x7ffcb68ea2f1 - RtlUserThreadStart
Build completed unsuccessfully in 0:04:52
Traceback (most recent call last):
Traceback (most recent call last):
  File "D:\a\rust\rust\src\ci\pgo-windows.py", line 22, in <module>
    subprocess.run([
  File "C:\hostedtoolcache\windows\Python\3.10.4\x64\lib\subprocess.py", line 524, in run
    raise CalledProcessError(retcode, process.args,
subprocess.CalledProcessError: Command '['python', 'x.py', 'dist', '--llvm-profile-generate']' returned non-zero exit status 1.
