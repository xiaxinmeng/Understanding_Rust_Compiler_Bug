
$ clang -std=c++20 -xc++ - -o /dev/null -c <<<'extern "C" { struct EmptyStruct {}; union EmptyUnion {}; }'
<stdin>:1:14: warning: empty struct has size 0 in C, size 1 in C++ [-Wextern-c-compat]
extern "C" { struct EmptyStruct {}; union EmptyUnion {}; }
             ^
<stdin>:1:37: warning: empty union has size 0 in C, size 1 in C++ [-Wextern-c-compat]
extern "C" { struct EmptyStruct {}; union EmptyUnion {}; }
                                    ^
2 warnings generated.
