
$ cat c.c
#include <windows.h>
__stdcall int add(int a, int b) { return a + b; }
void call_stdcall(void) { MessageBoxW(0, 0, 0, 0); }

$ gcc -c c.c -o c.o
$ nm c.o
00000000 b .bss
00000000 d .data
00000000 r .eh_frame
00000000 r .rdata$zzz
00000000 t .text
         U __imp__MessageBoxW@16
00000000 T _add@8
0000000f T _call_stdcall
