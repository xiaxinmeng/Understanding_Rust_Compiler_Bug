
~ $ # static PIE, default
~ $ rustc -o hello hello.rs
~ $ ./hello
The program "+ + * - /" calculates the value 1
~ $ file hello
hello: ELF 64-bit LSB shared object, x86-64, version 1 (SYSV), dynamically linked, not stripped, with debug_info
~ $ readelf -d hello

Dynamic section at offset 0x2ee40 contains 17 entries:
  Tag        Type                         Name/Value
 0x0000000000000010 (SYMBOLIC)           0x0
 0x000000000000000c (INIT)               0x1890
 0x000000000000000d (FINI)               0x20d6c
 0x000000006ffffef5 (GNU_HASH)           0x1c8
 0x0000000000000005 (STRTAB)             0x278
 0x0000000000000006 (SYMTAB)             0x200
 0x000000000000000a (STRSZ)              60 (bytes)
 0x000000000000000b (SYMENT)             24 (bytes)
 0x0000000000000015 (DEBUG)              0x0
 0x0000000000000003 (PLTGOT)             0x22ef90
 0x0000000000000007 (RELA)               0x2b8
 0x0000000000000008 (RELASZ)             5592 (bytes)
 0x0000000000000009 (RELAENT)            24 (bytes)
 0x0000000000000018 (BIND_NOW)           
 0x000000006ffffffb (FLAGS_1)            Flags: NOW PIE
 0x000000006ffffff9 (RELACOUNT)          233
 0x0000000000000000 (NULL)               0x0
~ $ # dynamically linked libc
~ $ rustc -o hello.dyn hello.rs -C target-feature=-crt-static
~ $ ./hello.dyn
The program "+ + * - /" calculates the value 1
~ $ file hello.dyn
hello.dyn: ELF 64-bit LSB shared object, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib/ld-musl-x86_64.so.1, not stripped, with debug_info
~ $ readelf -d hello.dyn

Dynamic section at offset 0x23c00 contains 21 entries:
  Tag        Type                         Name/Value
 0x0000000000000001 (NEEDED)             Shared library: [libgcc_s.so.1]
 0x0000000000000001 (NEEDED)             Shared library: [libc.musl-x86_64.so.1]
 0x000000000000000c (INIT)               0x2ba8
 0x000000000000000d (FINI)               0x18510
 0x000000006ffffef5 (GNU_HASH)           0x258
 0x0000000000000005 (STRTAB)             0x9f8
 0x0000000000000006 (SYMTAB)             0x290
 0x000000000000000a (STRSZ)              1246 (bytes)
 0x000000000000000b (SYMENT)             24 (bytes)
 0x0000000000000015 (DEBUG)              0x0
 0x0000000000000003 (PLTGOT)             0x223d90
 0x0000000000000007 (RELA)               0xfb8
 0x0000000000000008 (RELASZ)             7152 (bytes)
 0x0000000000000009 (RELAENT)            24 (bytes)
 0x0000000000000018 (BIND_NOW)           
 0x000000006ffffffb (FLAGS_1)            Flags: NOW PIE
 0x000000006ffffffe (VERNEED)            0xf78
 0x000000006fffffff (VERNEEDNUM)         1
 0x000000006ffffff0 (VERSYM)             0xed6
 0x000000006ffffff9 (RELACOUNT)          225
 0x0000000000000000 (NULL)               0x0
~ $ # dynamically linked Rust libraries
~ $ rustc -o hello.fulldyn hello.rs -C target-feature=-crt-static -C prefer-dynamic
~ $ ./hello.fulldyn
The program "+ + * - /" calculates the value 1
~ $ file hello.fulldyn
hello.fulldyn: ELF 64-bit LSB shared object, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib/ld-musl-x86_64.so.1, not stripped, with debug_info
~ $ readelf -d hello.fulldyn 

Dynamic section at offset 0x2dd8 contains 22 entries:
  Tag        Type                         Name/Value
 0x0000000000000001 (NEEDED)             Shared library: [libstd-f27b85952e59013f.so]
 0x0000000000000001 (NEEDED)             Shared library: [libgcc_s.so.1]
 0x0000000000000001 (NEEDED)             Shared library: [libc.musl-x86_64.so.1]
 0x000000000000000c (INIT)               0x978
 0x000000000000000d (FINI)               0x18a4
 0x000000006ffffef5 (GNU_HASH)           0x220
 0x0000000000000005 (STRTAB)             0x438
 0x0000000000000006 (SYMTAB)             0x258
 0x000000000000000a (STRSZ)              595 (bytes)
 0x000000000000000b (SYMENT)             24 (bytes)
 0x0000000000000015 (DEBUG)              0x0
 0x0000000000000003 (PLTGOT)             0x202f78
 0x0000000000000007 (RELA)               0x6d8
 0x0000000000000008 (RELASZ)             672 (bytes)
 0x0000000000000009 (RELAENT)            24 (bytes)
 0x0000000000000018 (BIND_NOW)           
 0x000000006ffffffb (FLAGS_1)            Flags: NOW PIE
 0x000000006ffffffe (VERNEED)            0x6b8
 0x000000006fffffff (VERNEEDNUM)         1
 0x000000006ffffff0 (VERSYM)             0x68c
 0x000000006ffffff9 (RELACOUNT)          14
 0x0000000000000000 (NULL)               0x0
