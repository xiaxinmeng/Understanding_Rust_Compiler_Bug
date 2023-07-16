
# on OSX
$ rustc foo.rs && nm -g libfoo.rlib
libfoo.rlib(foo.o):
                 U ___morestack
0000000000000000 T _foo

# on linux
$ rustc foo.rs && nm -g libfoo.rlib

foo.o:
0000000000000000 T foo
                 U __morestack

