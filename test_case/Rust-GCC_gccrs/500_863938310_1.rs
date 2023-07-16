
Reading specs from ./gcc/specs
COLLECT_GCC=./gcc/gccrs
COLLECT_LTO_WRAPPER=./gcc/lto-wrapper
Target: x86_64-apple-darwin20.1.0
Configured with: ../gccrs/configure --prefix=/Users/yangwenzhang/gccrs-install --disable-bootstrap --disable-multilib --enable-languages=rust --with-native-system-header-dir=/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include : (reconfigured) ../gccrs/configure --prefix=/Users/yangwenzhang/gccrs-install --disable-bootstrap --enable-multilib --enable-languages=rust --with-native-system-header-dir=/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include
Thread model: posix
Supported LTO compression algorithms: zlib zstd
gcc version 11.0.1 20210325 (experimental) (GCC) 
COLLECT_GCC_OPTIONS='-B' './gcc' '-v' '-mmacosx-version-min=11.0.0' '-asm_macosx_version_min=11.0' '-shared-libgcc' '-mtune=core2' '-dumpdir' 'a-'
 ./gcc/rust1 ../gccrs/gcc/testsuite/rust/execute/torture/block_expr1.rs -fPIC -quiet -dumpdir a- -dumpbase block_expr1.rs -dumpbase-ext .rs -mmacosx-version-min=11.0.0 -mtune=core2 -version -L./gcc -o /var/folders/m_/hsfwlg911nx39msyyw0tyvcc0000gn/T//ccwx78jP.s
GNU Rust (GCC) version 11.0.1 20210325 (experimental) (x86_64-apple-darwin20.1.0)
        compiled by GNU C version Apple LLVM 12.0.5 (clang-1205.0.22.9), GMP version 6.2.1, MPFR version 4.1.0, MPC version 1.2.1, isl version isl-0.24-GMP

GGC heuristics: --param ggc-min-expand=30 --param ggc-min-heapsize=4096
GNU Rust (GCC) version 11.0.1 20210325 (experimental) (x86_64-apple-darwin20.1.0)
        compiled by GNU C version Apple LLVM 12.0.5 (clang-1205.0.22.9), GMP version 6.2.1, MPFR version 4.1.0, MPC version 1.2.1, isl version isl-0.24-GMP

GGC heuristics: --param ggc-min-expand=30 --param ggc-min-heapsize=4096
COLLECT_GCC_OPTIONS='-B' './gcc' '-v' '-mmacosx-version-min=11.0.0'  '-shared-libgcc' '-mtune=core2' '-dumpdir' 'a-'
 ./gcc/as -arch x86_64 -v -force_cpusubtype_ALL -mmacosx-version-min=11.0 -o /var/folders/m_/hsfwlg911nx39msyyw0tyvcc0000gn/T//ccpHGSdi.o /var/folders/m_/hsfwlg911nx39msyyw0tyvcc0000gn/T//ccwx78jP.s
Apple clang version 12.0.5 (clang-1205.0.22.9)
Target: x86_64-apple-darwin20.1.0
Thread model: posix
InstalledDir: /Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin
 "/Applications/Xcode.app/Contents/Developer/Toolchains/XcodeDefault.xctoolchain/usr/bin/clang" -cc1as -triple x86_64-apple-macosx11.0.0 -filetype obj -main-file-name ccwx78jP.s -target-cpu penryn -fdebug-compilation-dir /Users/yangwenzhang/workspace/gsoc/macbuild -dwarf-debug-producer "Apple clang version 12.0.5 (clang-1205.0.22.9)" -dwarf-version=4 -mrelocation-model pic -mllvm -disable-aligned-alloc-awareness=1 -o /var/folders/m_/hsfwlg911nx39msyyw0tyvcc0000gn/T//ccpHGSdi.o /var/folders/m_/hsfwlg911nx39msyyw0tyvcc0000gn/T//ccwx78jP.s
COMPILER_PATH=./gcc/
LIBRARY_PATH=./gcc/

COLLECT_GCC_OPTIONS='-B' './gcc' '-v' '-mmacosx-version-min=11.0.0'  '-shared-libgcc' '-mtune=core2' '-dumpdir' 'a.'
 ./gcc/collect2 -dynamic -arch x86_64 -macosx_version_min 11.0.0 -weak_reference_mismatches non-weak -o a.out -L./gcc /var/folders/m_/hsfwlg911nx39msyyw0tyvcc0000gn/T//ccpHGSdi.o -lSystem -lgcc_ext.10.5 -lgcc -lSystem -no_compact_unwind -v

collect2 version 11.0.1 20210325 (experimental)
./gcc/collect-ld -dynamic -arch x86_64 -macosx_version_min 11.0.0 -weak_reference_mismatches non-weak -o a.out -L./gcc /var/folders/m_/hsfwlg911nx39msyyw0tyvcc0000gn/T//ccpHGSdi.o -lSystem -lgcc_ext.10.5 -lgcc -lSystem -no_compact_unwind -v
@(#)PROGRAM:ld  PROJECT:ld64-650.9
BUILD 00:19:30 Mar 17 2021
configured to support archs: armv6 armv7 armv7s arm64 arm64e arm64_32 i386 x86_64 x86_64h armv6m armv7k armv7m armv7em
Library search paths:
        ./gcc
        /usr/lib
        /usr/local/lib
Framework search paths:
        /Library/Frameworks/
        /System/Library/Frameworks/
ld: library not found for -lSystem
collect2: error: ld returned 1 exit status
