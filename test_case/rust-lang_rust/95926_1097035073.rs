
> cat hello.c
#include<stdio.h>
int main() {
  printf("Hello, world!\n");
  return 0;
}
> musl-gcc hello.c -o hello -static-pie
ld -plugin /usr/lib/gcc/x86_64-linux-gnu/10/liblto_plugin.so -plugin-opt=/usr/lib/gcc/x86_64-linux-gnu/10/lto-wrapper -plugin-opt=-fresolution=/tmp/cciwsIZK.res -plugin-opt=-pass-through=/usr/lib/gcc/x86_64-linux-gnu/10/libgcc.a -plugin-opt=-pass-through=/usr/lib/gcc/x86_64-linux-gnu/10/libgcc_eh.a -plugin-opt=-pass-through=-lc -dynamic-linker /lib/ld-musl-x86_64.so.1 -nostdlib -pie -o hello /usr/lib/x86_64-linux-musl/Scrt1.o /usr/lib/x86_64-linux-musl/crti.o /usr/lib/gcc/x86_64-linux-gnu/10/crtbeginS.o -L/usr/lib/x86_64-linux-musl -L /usr/lib/gcc/x86_64-linux-gnu/10/. /tmp/ccNzdDMJ.o --start-group /usr/lib/gcc/x86_64-linux-gnu/10/libgcc.a /usr/lib/gcc/x86_64-linux-gnu/10/libgcc_eh.a -lc --end-group /usr/lib/gcc/x86_64-linux-gnu/10/crtendS.o /usr/lib/x86_64-linux-musl/crtn.o
> file hello
hello: ELF 64-bit LSB pie executable, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib/ld-musl-x86_64.so.1, with debug_info, not stripped
> musl-gcc hello.c -o hello -static
ld -plugin /usr/lib/gcc/x86_64-linux-gnu/10/liblto_plugin.so -plugin-opt=/usr/lib/gcc/x86_64-linux-gnu/10/lto-wrapper -plugin-opt=-fresolution=/tmp/cciPPY3o.res -plugin-opt=-pass-through=/usr/lib/gcc/x86_64-linux-gnu/10/libgcc.a -plugin-opt=-pass-through=/usr/lib/gcc/x86_64-linux-gnu/10/libgcc_eh.a -plugin-opt=-pass-through=-lc -dynamic-linker /lib/ld-musl-x86_64.so.1 -nostdlib -static -o hello /usr/lib/x86_64-linux-musl/Scrt1.o /usr/lib/x86_64-linux-musl/crti.o /usr/lib/gcc/x86_64-linux-gnu/10/crtbeginS.o -L/usr/lib/x86_64-linux-musl -L /usr/lib/gcc/x86_64-linux-gnu/10/. /tmp/cckhAQSp.o --start-group /usr/lib/gcc/x86_64-linux-gnu/10/libgcc.a /usr/lib/gcc/x86_64-linux-gnu/10/libgcc_eh.a -lc --end-group /usr/lib/gcc/x86_64-linux-gnu/10/crtendS.o /usr/lib/x86_64-linux-musl/crtn.o
> file hello
hello: ELF 64-bit LSB executable, x86-64, version 1 (SYSV), statically linked, with debug_info, not stripped
 > musl-gcc -fuse-ld=lld -static hello.c -o hello
ld.lld -plugin /usr/lib/gcc/x86_64-linux-gnu/10/liblto_plugin.so -plugin-opt=/usr/lib/gcc/x86_64-linux-gnu/10/lto-wrapper -plugin-opt=-fresolution=/tmp/cc788cAg.res -plugin-opt=-pass-through=/usr/lib/gcc/x86_64-linux-gnu/10/libgcc.a -plugin-opt=-pass-through=/usr/lib/gcc/x86_64-linux-gnu/10/libgcc_eh.a -plugin-opt=-pass-through=-lc -dynamic-linker /lib/ld-musl-x86_64.so.1 -nostdlib -static -o hello /usr/lib/x86_64-linux-musl/Scrt1.o /usr/lib/x86_64-linux-musl/crti.o /usr/lib/gcc/x86_64-linux-gnu/10/crtbeginS.o -L/usr/lib/x86_64-linux-musl -L /usr/lib/gcc/x86_64-linux-gnu/10/. /tmp/ccsEJC6i.o --start-group /usr/lib/gcc/x86_64-linux-gnu/10/libgcc.a /usr/lib/gcc/x86_64-linux-gnu/10/libgcc_eh.a -lc --end-group /usr/lib/gcc/x86_64-linux-gnu/10/crtendS.o /usr/lib/x86_64-linux-musl/crtn.o
> file hello
hello: ELF 64-bit LSB executable, x86-64, version 1 (SYSV), statically linked, interpreter /lib/ld-musl-x86_64.so.1, with debug_info, not stripped
> ./hello
zsh: segmentation fault  ./hello
