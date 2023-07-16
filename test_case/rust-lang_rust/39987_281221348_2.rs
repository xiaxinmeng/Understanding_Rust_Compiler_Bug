
# USED does make it to the object file (which is the point of this feature)
$ clang -O3 -c foo.c

$ nm -C foo.o | grep USED
0000000000000000 r USED
