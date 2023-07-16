
% cat test.c
int has_underscore;
int x asm("no_underscore");
% clang -c test.c
% nm test.o
0000000000000004 C _has_underscore
0000000000000004 C no_underscore
