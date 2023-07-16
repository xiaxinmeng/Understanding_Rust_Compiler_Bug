
$ clang -gsplit-dwarf /tmp/banana/test.c -c -o /tmp/outdir/foo.o
$ clang outdir/foo.o -o outdir/hm
$ llvm-dwarfdump outdir/hm | grep -C5 foo.dwo

0x00002114: DW_TAG_compile_unit
              DW_AT_stmt_list	(0x000005ae)
              DW_AT_comp_dir	("/tmp")
              DW_AT_GNU_pubnames	(true)
              DW_AT_GNU_dwo_name	("/tmp/outdir/foo.dwo")
              DW_AT_GNU_dwo_id	(0xde4d396f3bf0e257)
              DW_AT_low_pc	(0x0000000000401100)
              DW_AT_high_pc	(0x0000000000401103)
              DW_AT_GNU_addr_base	(0x00000000)
0x00002139: Compile Unit: length = 0x0000001e, format = DWARF32, version = 0x0002, abbr_offset = 0x03c0, addr_size = 0x08 (next unit at 0x0000215b)
$ strace -o trace llvm-dwp -e outdir/hm -o outdir/hm.dwp
error: No such file or directory
$ cat trace | grep foo.dwo
openat(AT_FDCWD, "/tmp/tmp/outdir/foo.dwo", O_RDONLY|O_CLOEXEC) = -1 ENOENT (No such file or directory)
