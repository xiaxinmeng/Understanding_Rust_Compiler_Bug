bash
$ dlltool -d calling_conventions.def -l calling_conventions.dll.a

$ nm calling_conventions.dll.a | rg Sum
00000000 I __imp__stdcallSum@8@8
00000000 T _stdcallSum@8@8
00000000 I __imp__cdeclSum
00000000 T _cdeclSum
00000000 T @fastcallSum@8
00000000 I __imp_@fastcallSum@8
