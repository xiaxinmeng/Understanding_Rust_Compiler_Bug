bash
$ gcc -shared calling_conventions.c -o calling_conventions.dll
calling_conventions.c:16:1: warning: data definition has no type or storage class
   16 | __vectorcall __declspec(dllexport) int vectorcallSum(int a, int b)
      | ^~~~~~~~~~~~
calling_conventions.c:16:1: warning: type defaults to 'int' in declaration of '__vectorcall' [-Wimplicit-int]
calling_conventions.c:16:36: error: expected ',' or ';' before 'int'
   16 | __vectorcall __declspec(dllexport) int vectorcallSum(int a, int b)
      |                                    ^~~

# GCC didn't work, trying Clang
$ clang -shared calling_conventions.c -o calling_conventions.dll
D:/msys64/mingw32/bin/ld: cannot export vectorcallSum@@8: symbol not found
clang: error: linker command failed with exit code 1 (use -v to see invocation)

# Which failed because of BFD..., now LLD
$ clang -fuse-ld=lld -shared calling_conventions.c -o calling_conventions.dll

$ nm calling_conventions.dll | rg Sum
10001520 T @fastcallSum@8
100014e0 T _cdeclSum
10001500 T _stdcallSum@8
10001540 T vectorcallSum@@8
