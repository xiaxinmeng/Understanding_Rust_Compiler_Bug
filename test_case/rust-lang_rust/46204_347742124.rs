
netbsd# cat hello.c
#include <stdio.h>

char *response();

int main() {
    printf("%s\n", response());
}
netbsd# cat libfoo.c
char *response() {
        return "Hello world!";
}
netbsd# cc -fPIC -shared -o libfoo.so libfoo.c
netbsd# cc -L. -o hello hello.c -lfoo
netbsd# ./hello
Shared object "libfoo.so" not found
netbsd# cc -L. -o hello hello.c -lfoo -Wl,--enable-new-dtags -Wl,-rpath,/root
netbsd# ./hello
Hello world!
netbsd# readelf -d hello | grep /root
 0x000000000000000f (RPATH)              Library rpath: [/root]
 0x000000000000001d (RUNPATH)            Library runpath: [/root]
netbsd# ld --version
GNU ld (NetBSD Binutils nb1) 2.23.2
Copyright 2012 Free Software Foundation, Inc.
This program is free software; you may redistribute it under the terms of
the GNU General Public License version 3 or (at your option) a later version.
This program has absolutely no warranty.

