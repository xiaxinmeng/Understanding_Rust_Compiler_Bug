cpp
// foo.cpp
using fn_type = void(*)();
[[ noreturn ]] extern "C" void foo(fn_type x);
[[ noreturn ]] extern "C" void foo(fn_type x)  { x(); /* unreachable: */ throw 0; }
