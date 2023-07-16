
#  NOTE(--gc-sections) matches rustc default
$ clang -Wl,--gc-sections foo.c

$ nm -C a.out | grep USED
