bash
$ cat hello.c 
#include <stdio.h>

int main() {
  puts("Hello, world!");
}
$ riscv64-linux-gnu-gcc -static hello.c -o hello
$ qemu-riscv64 hello
Hello, world!
