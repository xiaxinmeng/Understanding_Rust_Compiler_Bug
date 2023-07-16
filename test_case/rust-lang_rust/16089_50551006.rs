
$ cat foo.c
int main() { return 0; }
$ gcc foo.c -o foo -nostdlib
/usr/bin/ld: warning: cannot find entry symbol _start; defaulting to 0000000000400144
$ ./foo
zsh: segmentation fault (core dumped)  ./foo
