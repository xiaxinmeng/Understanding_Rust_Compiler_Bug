plain
[2357/2920] Building CXX object lib/Target/X86/MCTargetDesc/CMakeFiles/LLVMX86Desc.dir/X86WinCOFFTargetStreamer.cpp.o
[2358/2920] Building CXX object lib/Target/AVR/CMakeFiles/LLVMAVRCodeGen.dir/AVRAsmPrinter.cpp.o
[2359/2920] Building CXX object lib/Target/AVR/CMakeFiles/LLVMAVRCodeGen.dir/AVRFrameLowering.cpp.o
[2360/2920] Building CXX object lib/Target/AVR/CMakeFiles/LLVMAVRCodeGen.dir/AVRExpandPseudoInsts.cpp.o
/checkout/src/llvm-project/llvm/lib/Target/AVR/AVRExpandPseudoInsts.cpp: In member function ‘bool {anonymous}::AVRExpandPseudo::expand({anonymous}::AVRExpandPseudo::Block&, {anonymous}::AVRExpandPseudo::BlockIt) [with unsigned int OP = 321; {anonymous}::AVRExpandPseudo::Block = llvm::MachineBasicBlock; {anonymous}::AVRExpandPseudo::BlockIt = llvm::MachineInstrBundleIterator<llvm::MachineInstr>]’:
/checkout/src/llvm-project/llvm/lib/Target/AVR/AVRExpandPseudoInsts.cpp:1287:23: warning: unused variable ‘STI’ [-Wunused-variable]
 1287 |   const AVRSubtarget &STI = MF.getSubtarget<AVRSubtarget>();
      |                       ^~~
/checkout/src/llvm-project/llvm/lib/Target/AVR/AVRExpandPseudoInsts.cpp: In member function ‘bool {anonymous}::AVRExpandPseudo::expand({anonymous}::AVRExpandPseudo::Block&, {anonymous}::AVRExpandPseudo::BlockIt) [with unsigned int OP = 323; {anonymous}::AVRExpandPseudo::Block = llvm::MachineBasicBlock; {anonymous}::AVRExpandPseudo::BlockIt = llvm::MachineInstrBundleIterator<llvm::MachineInstr>]’:
/checkout/src/llvm-project/llvm/lib/Target/AVR/AVRExpandPseudoInsts.cpp:1305:23: warning: unused variable ‘STI’ [-Wunused-variable]
 1305 |   const AVRSubtarget &STI = MF.getSubtarget<AVRSubtarget>();
[2361/2920] Building CXX object lib/Target/X86/AsmParser/CMakeFiles/LLVMX86AsmParser.dir/X86AsmParser.cpp.o
[2362/2920] Building CXX object lib/Target/AVR/CMakeFiles/LLVMAVRCodeGen.dir/AVRISelDAGToDAG.cpp.o
[2363/2920] Building CXX object lib/Target/AVR/CMakeFiles/LLVMAVRCodeGen.dir/AVRInstrInfo.cpp.o
[2364/2920] Building CXX object lib/Target/AVR/CMakeFiles/LLVMAVRCodeGen.dir/AVRISelLowering.cpp.o
---
/checkout/src/llvm-project/lld/ELF/ARMErrataFix.cpp:392:27: warning: ‘isecLimit’ may be used uninitialized in this function [-Wmaybe-uninitialized]
  392 |     (*patchIt)->outSecOff = isecLimit;
      |     ~~~~~~~~~~~~~~~~~~~~~~^~~~~~~~~~~
[60/124] Building CXX object ELF/CMakeFiles/lldELF.dir/InputSection.cpp.o
FAILED: ELF/CMakeFiles/lldELF.dir/InputSection.cpp.o 
sccache /usr/bin/c++  -DGTEST_HAS_RTTI=0 -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS -IELF -I/checkout/src/llvm-project/lld/ELF -I/checkout/src/llvm-project/lld/include -Iinclude -I/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/include -I/checkout/src/llvm-project/llvm/include -ffunction-sections -fdata-sections -fPIC -m64 -fPIC -fno-semantic-interposition -fvisibility-inlines-hidden -Werror=date-time -Wall -Wextra -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -Wimplicit-fallthrough -Wno-class-memaccess -Wno-redundant-move -Wno-pessimizing-move -Wno-noexcept-type -Wdelete-non-virtual-dtor -Wsuggest-override -Wno-comment -Wmisleading-indentation -fdiagnostics-color -ffunction-sections -fdata-sections -O3 -DNDEBUG    -fno-exceptions -fno-rtti -std=gnu++14 -MD -MT ELF/CMakeFiles/lldELF.dir/InputSection.cpp.o -MF ELF/CMakeFiles/lldELF.dir/InputSection.cpp.o.d -o ELF/CMakeFiles/lldELF.dir/InputSection.cpp.o -c /checkout/src/llvm-project/lld/ELF/InputSection.cpp
/checkout/src/llvm-project/lld/ELF/InputSection.cpp: In member function ‘uint64_t lld::elf::SectionBase::getOffset(uint64_t) const’:
/checkout/src/llvm-project/lld/ELF/InputSection.cpp:176:14: error: ‘llvm::ArrayRef<unsigned char> lld::elf::InputSectionBase::rawData’ is protected within this context
  176 |     if (!es->rawData.empty())
In file included from /checkout/src/llvm-project/lld/ELF/InputSection.cpp:9:
/checkout/src/llvm-project/lld/ELF/InputSection.h:231:29: note: declared protected here
  231 |   mutable ArrayRef<uint8_t> rawData;
      |                             ^~~~~~~
---
/checkout/src/llvm-project/llvm/include/llvm/ADT/DenseMap.h:1160:18: warning: ‘*((void*)(& __tmp)+24).llvm::SmallDenseMap<const lld::elf::OutputSection*, unsigned int, 16, llvm::DenseMapInfo<const lld::elf::OutputSection*, void>, llvm::detail::DenseMapPair<const lld::elf::OutputSection*, unsigned int> >::LargeRep::Buckets’ may be used uninitialized in this function [-Wmaybe-uninitialized]
thread 'main' panicked at '
command did not execute successfully, got: exit status: 1

build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.44/src/lib.rs:885:5
 1160 |     return Small ? getInlineBuckets() : getLargeRep()->Buckets;
      |            ~~~~~~^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/checkout/src/llvm-project/llvm/include/llvm/ADT/DenseMap.h:1169:18: warning: ‘*((void*)(& __tmp)+24).llvm::SmallDenseMap<const lld::elf::OutputSection*, unsigned int, 16, llvm::DenseMapInfo<const lld::elf::OutputSection*, void>, llvm::detail::DenseMapPair<const lld::elf::OutputSection*, unsigned int> >::LargeRep::NumBuckets’ may be used uninitialized in this function [-Wmaybe-uninitialized]
 1169 |     return Small ? InlineBuckets : getLargeRep()->NumBuckets;
 1169 |     return Small ? InlineBuckets : getLargeRep()->NumBuckets;
      |            ~~~~~~^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/checkout/src/llvm-project/llvm/include/llvm/ADT/DenseMap.h:1160:18: warning: ‘*((void*)(& __tmp)+24).llvm::SmallDenseMap<const lld::elf::OutputSection*, unsigned int, 16, llvm::DenseMapInfo<const lld::elf::OutputSection*, void>, llvm::detail::DenseMapPair<const lld::elf::OutputSection*, unsigned int> >::LargeRep::Buckets’ may be used uninitialized in this function [-Wmaybe-uninitialized]
 1160 |     return Small ? getInlineBuckets() : getLargeRep()->Buckets;
      |            ~~~~~~^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/checkout/src/llvm-project/llvm/include/llvm/ADT/DenseMap.h:1169:18: warning: ‘*((void*)(& __tmp)+24).llvm::SmallDenseMap<const lld::elf::OutputSection*, unsigned int, 16, llvm::DenseMapInfo<const lld::elf::OutputSection*, void>, llvm::detail::DenseMapPair<const lld::elf::OutputSection*, unsigned int> >::LargeRep::NumBuckets’ may be used uninitialized in this function [-Wmaybe-uninitialized]
 1169 |     return Small ? InlineBuckets : getLargeRep()->NumBuckets;
      |            ~~~~~~^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
ninja: build stopped: subcommand failed.
Build completed unsuccessfully in 0:05:12
