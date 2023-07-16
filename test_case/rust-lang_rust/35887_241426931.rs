 sh
$ ls 
$ echo "int main() { return 0; }" > a.c
$ mkdir a.out
$ ls
a.c  a.out
$ gcc a.c
/usr/bin/ld: cannot open output file a.out: Is a directory
collect2: error: ld returned 1 exit status
$ 
