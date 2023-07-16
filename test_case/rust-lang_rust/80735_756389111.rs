bash
$ cat hello.c 
#include <stdio.h>

static __thread char buf[] = "Hello, world!";

int main() {
  puts(buf);
}
$ riscv64-linux-gnu-gcc hello.c -static -lpthread -o hello
$ qemu-riscv64 hello
Hello, world!
