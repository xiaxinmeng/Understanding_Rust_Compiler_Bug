
$ cat foo.c                                                           ⏎
int foo(...) {}
$ gcc foo.c                                                           ⏎
foo.c:1:9: error: ISO C requires a named argument before '...'
 int foo(...) {
         ^
