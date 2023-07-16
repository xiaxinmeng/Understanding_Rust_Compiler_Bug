sh-session
$ ls
example.c
$ gcc -o - example.c
$ ls
-  example.c
$ ./-
Hello, world!
$ rm -- -
$ clang -o - example.c
$ ls
-  example.c
