
/space/mine/rust/rust $ ./configure --prefix=$(pwd)/../dist --enable-clang && make
[...]
make: llvm
make[1]: Entering directory `/space/mine/rust/rust/x86_64-unknown-linux-gnu/llvm'
llvm[1]: Constructing LLVMBuild project information.
make[1]: Leaving directory `/space/mine/rust/rust/x86_64-unknown-linux-gnu/llvm'
make[1]: Entering directory `/space/mine/rust/rust/x86_64-unknown-linux-gnu/llvm'
make[2]: Entering directory `/space/mine/rust/rust/x86_64-unknown-linux-gnu/llvm/lib/Support'
llvm[2]: Compiling APFloat.cpp for Release+Asserts build
In file included from /space/mine/rust/rust/src/llvm/lib/Support/APFloat.cpp:15:
In file included from /space/mine/rust/rust/src/llvm/include/llvm/ADT/APFloat.h:20:
In file included from /space/mine/rust/rust/src/llvm/include/llvm/ADT/APInt.h:19:
In file included from /space/mine/rust/rust/src/llvm/include/llvm/ADT/ArrayRef.h:14:
/space/mine/rust/rust/src/llvm/include/llvm/ADT/STLExtras.h:21:10: fatal error: 'cstddef' file not found
#include <cstddef> // for std::size_t
         ^
1 error generated.
[...]
/space/mine/rust/rust $ locate cstddef
/usr/include/c++/4.6/cstddef
/space/mine/rust/rust $ clang --version
Ubuntu clang version 3.4-1ubuntu3~precise1 (tags/RELEASE_34/final) (based on LLVM 3.4)
Target: x86_64-pc-linux-gnu
Thread model: posix
