
$ rustc +nightly -Z thinlto -C codegen-units=16 -O foo.rs  --crate-type rlib --emit obj
$ readelf -Ws *.o
File: foo.foo0-8787f43e282added376259c1adb08b80.rs.rust-cgu.o

Symbol table '.symtab' contains 6 entries:
   Num:    Value          Size Type    Bind   Vis      Ndx Name
     0: 0000000000000000     0 NOTYPE  LOCAL  DEFAULT  UND 
     1: 0000000000000000     0 FILE    LOCAL  DEFAULT  ABS foo0-8787f43e282added376259c1adb08b80.rs
     2: 0000000000000000     0 SECTION LOCAL  DEFAULT    3 
     3: 0000000000000000    21 FUNC    GLOBAL DEFAULT    3 _ZN3foo1a3bar17hef56ac03a5da7c66E
     4: 0000000000000000     0 TLS     GLOBAL HIDDEN   UND _ZN3foo3FOO17h556917dcc656cbe0E.llvm.54FD3A5C
     5: 0000000000000000     0 NOTYPE  GLOBAL DEFAULT  UND __tls_get_addr

File: foo.foo1-8787f43e282added376259c1adb08b80.rs.rust-cgu.o

Symbol table '.symtab' contains 7 entries:
   Num:    Value          Size Type    Bind   Vis      Ndx Name
     0: 0000000000000000     0 NOTYPE  LOCAL  DEFAULT  UND 
     1: 0000000000000000     0 FILE    LOCAL  DEFAULT  ABS foo1-8787f43e282added376259c1adb08b80.rs
     2: 0000000000000000     0 TLS     LOCAL  DEFAULT    5 .tdata._ZN3foo3FOO17h556917dcc656cbe0E.llvm.54FD3A5C
     3: 0000000000000000     0 SECTION LOCAL  DEFAULT    3 
     4: 0000000000000000     4 TLS     GLOBAL HIDDEN     5 _ZN3foo3FOO17h556917dcc656cbe0E.llvm.54FD3A5C
     5: 0000000000000000    22 FUNC    GLOBAL HIDDEN     3 _ZN3foo3foo17hac5efa801639546aE
6: 0000000000000000 0 NOTYPE GLOBAL DEFAULT UND __tls_get_addr
