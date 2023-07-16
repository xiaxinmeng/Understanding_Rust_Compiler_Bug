
test.c:4:24: warning: initializing 'char *const' with an expression of type
      'const char [3]' discards qualifiers
      [-Wincompatible-pointer-types-discards-qualifiers]
        char *const cmd[] = { "ls", "-l", (char *)0 };
                              ^~~~
test.c:4:30: warning: initializing 'char *const' with an expression of type
      'const char [3]' discards qualifiers
      [-Wincompatible-pointer-types-discards-qualifiers]
        char *const cmd[] = { "ls", "-l", (char *)0 };
                                    ^~~~
