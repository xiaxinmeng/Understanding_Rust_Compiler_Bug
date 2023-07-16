
$ cat foo.ll
@foo = external hidden thread_local global i32

define i32 @bar() {
  %a = load i32, i32* @foo
  ret i32 %a
}
$ llc foo.ll -filetype=obj -o foo.o -mtriple=mipsel-unknown-linux-gnu
$ readelf -Ws foo.o

Symbol table '.symtab' contains 5 entries:
   Num:    Value  Size Type    Bind   Vis      Ndx Name
     0: 00000000     0 NOTYPE  LOCAL  DEFAULT  UND
     1: 00000000     0 FILE    LOCAL  DEFAULT  ABS foo.ll
     2: 00000000     0 SECTION LOCAL  DEFAULT    2
     3: 00000000    24 FUNC    GLOBAL DEFAULT    2 bar
     4: 00000000     0 TLS     GLOBAL HIDDEN   UND foo
$ llc foo.ll -filetype=obj -o foo.o -mtriple=mipsel-unknown-linux-gnu -relocation-model=pic
$ readelf -Ws foo.o

Symbol table '.symtab' contains 7 entries:
   Num:    Value  Size Type    Bind   Vis      Ndx Name
     0: 00000000     0 NOTYPE  LOCAL  DEFAULT  UND
     1: 00000000     0 FILE    LOCAL  DEFAULT  ABS foo.ll
     2: 00000000     0 SECTION LOCAL  DEFAULT    2
     3: 00000000     0 NOTYPE  GLOBAL DEFAULT  UND __tls_get_addr
     4: 00000000     0 NOTYPE  GLOBAL DEFAULT  UND _gp_disp
     5: 00000000    56 FUNC    GLOBAL DEFAULT    2 bar
     6: 00000000     0 NOTYPE  GLOBAL HIDDEN   UND foo
