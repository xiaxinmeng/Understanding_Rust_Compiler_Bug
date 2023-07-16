bash
% clang -std=c++11 test.cpp
ld: error: undefined symbol: __cxa_begin_catch
>>> referenced by test.cpp
>>>               /tmp/test-64b1e6.o:(__clang_call_terminate)

ld: error: undefined symbol: std::terminate()
>>> referenced by test.cpp
>>>               /tmp/test-64b1e6.o:(__clang_call_terminate)

ld: error: undefined symbol: __gxx_personality_v0
>>> referenced by test.cpp
>>>               /tmp/test-64b1e6.o:(.eh_frame+0x6B)
clang-9: error: linker command failed with exit code 1 (use -v to see invocation)
