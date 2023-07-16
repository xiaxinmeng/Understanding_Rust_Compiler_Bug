
/space/mine/rust/rust $ clang++ -Qunused-arguments -I/space/mine/rust/rust/src/llvm/include -I/space/mine/rust/rust/src/llvm/lib/Support -I/space/mine/rust/rust/x86_64-unknown-linux-gnu/llvm/include -I/space/mine/rust/rust/x86_64-unknown-linux-gnu/llvm/lib/Support  -D_DEBUG -D_GNU_SOURCE -D__STDC_CONSTANT_MACROS -D__STDC_FORMAT_MACROS -D__STDC_LIMIT_MACROS -O3 -fomit-frame-pointer -stdlib=libc++ -std=c++11 -fvisibility-inlines-hidden -fno-exceptions -fno-rtti -fPIC -ffunction-sections -fdata-sections -Wcast-qual    -Wall -W -Wno-unused-parameter -Wwrite-strings -Wcovered-switch-default -Wno-uninitialized -Wno-missing-field-initializers -c -MMD -MP -MF "/space/mine/rust/rust/x86_64-unknown-linux-gnu/llvm/lib/Support/Release+Asserts/APFloat.d.tmp" -MT "/space/mine/rust/rust/x86_64-unknown-linux-gnu/llvm/lib/Support/Release+Asserts/APFloat.o" -MT "/space/mine/rust/rust/x86_64-unknown-linux-gnu/llvm/lib/Support/Release+Asserts/APFloat.d" /space/mine/rust/rust/src/llvm/lib/Support/APFloat.cpp -o /space/mine/rust/rust/x86_64-unknown-linux-gnu/llvm/lib/Support/Release+Asserts/APFloat.o
In file included from /space/mine/rust/rust/src/llvm/lib/Support/APFloat.cpp:15:
In file included from /space/mine/rust/rust/src/llvm/include/llvm/ADT/APFloat.h:20:
In file included from /space/mine/rust/rust/src/llvm/include/llvm/ADT/APInt.h:19:
In file included from /space/mine/rust/rust/src/llvm/include/llvm/ADT/ArrayRef.h:14:
/space/mine/rust/rust/src/llvm/include/llvm/ADT/STLExtras.h:21:10: fatal error: 'cstddef' file not found
#include <cstddef> // for std::size_t
         ^
1 error generated.
/space/mine/rust/rust $ cat xx.cpp
#include <cstddef>
int main() {
        return 0;
}
/space/mine/rust/rust $ clang++ xx.cpp
/space/mine/rust/rust $ ./a.out && echo $?
0
