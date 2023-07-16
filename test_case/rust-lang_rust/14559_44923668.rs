
$ cat c.c
__stdcall void std_func(void) {}

$ cat c.def
LIBRARY c
EXPORTS
  std_func=std_func@0

$ gcc c.c c.def -shared -o c.dll  # (c.def will be passed to ld)

\> dumpbin /exports c.dll
...
    ordinal hint RVA      name

          1    0 00001280 std_func
