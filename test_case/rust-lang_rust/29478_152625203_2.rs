
$ g++ foo.cpp -c
$ ar crus libfoo.a foo.o
$ rustc foo.rs -L. -lfoo -lstdc++
$ ./foo.exe
