bash
$ gcc -shared calling_conventions.c -o calling_conventions.dll

$ nm calling_conventions.dll | rg Sum
64ec14fc T @fastcallSum@8
64ec14e0 T _cdeclSum
64ec14ed T _stdcallSum@8
