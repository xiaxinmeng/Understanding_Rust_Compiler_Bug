
$ rustc basic-types-globals-metadata.rs -g -o abc
$ gdb ./abc
GNU gdb (GDB) 7.6.2 (Debian 7.6.2-1)
...
This GDB was configured as "x86_64-linux-gnu".
...

(gdb) whatis 'basic-types-globals-metadata::B'
No symbol "basic-types-globals-metadata::B" in current context.

(gdb) whatis 'abc::B'
type = bool
