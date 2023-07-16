
$ cat test.c
int main() { }
$ clang -g test.c -o c_test
$ kcov --verify --debug=4 foo c_test
Cannot open linux-vdso.so.1
CRC mismatch for debug link /lib/x86_64-linux-gnu/libc-2.31.so. Should be 0xf854b801, is 0xef41b1a0!
No debug symbols in /lib/x86_64-linux-gnu/libc.so.6.
CRC mismatch for debug link /lib/x86_64-linux-gnu/libdl-2.31.so. Should be 0xf334bb32, is 0x2bb25318!
No debug symbols in /lib/x86_64-linux-gnu/libdl.so.2.
No debug symbols in /lib64/ld-linux-x86-64.so.2.
