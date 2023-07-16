
$ cat test.c
#include <stdio.h>

int
main(int argc, char **argvp)
{
    asm volatile("movq $0, %%rax;"
          "movq $4, %%rcx;"
          "a_long_name_that_definitely_doesnt_get_autoinserted:"
          "addq $1, %%rax;"
          "cmpq %%rax, %%rcx;"
          "ja a_long_name_that_definitely_doesnt_get_autoinserted" :::);
    return 0;
}
$ clang --version
clang version 7.0.0 (tags/RELEASE_700/final)
Target: x86_64-pc-linux-gnu
Thread model: posix
InstalledDir: /usr/bin
$ clang test.c -o test
$ nm test | grep definitely
0000000000001140 t a_long_name_that_definitely_doesnt_get_autoinserted
$
