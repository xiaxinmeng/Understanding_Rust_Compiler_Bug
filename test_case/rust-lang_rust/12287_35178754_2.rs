
$ gcc  -L. c.o -la -lb
./libb.a(b.o): In function `b':
b.c:(.text+0xa): undefined reference to `a'
collect2: error: ld returned 1 exit status
