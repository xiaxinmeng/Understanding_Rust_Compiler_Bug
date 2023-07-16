
$ gcc -std=c11 -o ./test test.c
In file included from test.c:2:
test.c: In function ‘add’:
test.c:10:25: warning: ‘float’ is promoted to ‘double’ when passed through ‘...’
   10 |         f += va_arg(ap, float);
      |                         ^
test.c:10:25: note: (so you should pass ‘double’ not ‘float’ to ‘va_arg’)
test.c:10:25: note: if this code is reached, the program will abort
$ ./test
Illegal instruction
