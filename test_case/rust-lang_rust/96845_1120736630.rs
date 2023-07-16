
2022-05-09T07:12:59.4444743Z FAILED: ELF/CMakeFiles/lldELF.dir/InputSection.cpp.o 
2022-05-09T07:12:59.4451003Z sccache /usr/bin/c++  -DGTEST_HAS_RTTI=0 -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS -IELF -I/checkout/src/llvm-project/lld/ELF -I/checkout/src/llvm-project/lld/include -Iinclude -I/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/include -I/checkout/src/llvm-project/llvm/include -ffunction-sections -fdata-sections -fPIC -m64 -fPIC -fno-semantic-interposition -fvisibility-inlines-hidden -Werror=date-time -Wall -Wextra -Wno-unused-parameter -Wwrite-strings -Wcast-qual -Wno-missing-field-initializers -Wimplicit-fallthrough -Wno-class-memaccess -Wno-redundant-move -Wno-pessimizing-move -Wno-noexcept-type -Wdelete-non-virtual-dtor -Wsuggest-override -Wno-comment -Wmisleading-indentation -fdiagnostics-color -ffunction-sections -fdata-sections -O3 -DNDEBUG    -fno-exceptions -fno-rtti -std=gnu++14 -MD -MT ELF/CMakeFiles/lldELF.dir/InputSection.cpp.o -MF ELF/CMakeFiles/lldELF.dir/InputSection.cpp.o.d -o ELF/CMakeFiles/lldELF.dir/InputSection.cpp.o -c /checkout/src/llvm-project/lld/ELF/InputSection.cpp
2022-05-09T07:12:59.4460408Z /checkout/src/llvm-project/lld/ELF/InputSection.cpp: In member function ‘uint64_t lld::elf::SectionBase::getOffset(uint64_t) const’:
2022-05-09T07:12:59.4461212Z /checkout/src/llvm-project/lld/ELF/InputSection.cpp:176:14: error: ‘llvm::ArrayRef<unsigned char> lld::elf::InputSectionBase::rawData’ is protected within this context
2022-05-09T07:12:59.4461929Z   176 |     if (!es->rawData.empty())
2022-05-09T07:12:59.4462198Z       |              ^~~~~~~
2022-05-09T07:12:59.4462604Z In file included from /checkout/src/llvm-project/lld/ELF/InputSection.cpp:9:
2022-05-09T07:12:59.4463123Z /checkout/src/llvm-project/lld/ELF/InputSection.h:231:29: note: declared protected here
2022-05-09T07:12:59.4463497Z   231 |   mutable ArrayRef<uint8_t> rawData;
2022-05-09T07:12:59.4463769Z       |                             ^~~~~~~
