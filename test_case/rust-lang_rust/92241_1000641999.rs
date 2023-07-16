console
$ echo '#![feature(thread_local)] #[thread_local] #[no_mangle] pub static FOO: u64 = 0;' \
  | rustc --crate-type lib - --emit=obj -o foo.o
$ readelf -S foo.o
There are 6 section headers, starting at offset 0xe0:

Section Headers:
  [Nr] Name              Type             Address           Offset
       Size              EntSize          Flags  Link  Info  Align
  [ 0]                   NULL             0000000000000000  00000000
       0000000000000000  0000000000000000           0     0     0
  [ 1] .strtab           STRTAB           0000000000000000  00000090
       000000000000004a  0000000000000000           0     0     1
  [ 2] .text             PROGBITS         0000000000000000  00000040
       0000000000000000  0000000000000000  AX       0     0     4
  [ 3] .tdata.FOO        PROGBITS         0000000000000000  00000040
       0000000000000008  0000000000000000 WAT       0     0     8
  [ 4] .note.GNU-stack   PROGBITS         0000000000000000  00000048
       0000000000000000  0000000000000000           0     0     1
  [ 5] .symtab           SYMTAB           0000000000000000  00000048
       0000000000000048  0000000000000018           1     2     8
Key to Flags:
  W (write), A (alloc), X (execute), M (merge), S (strings), I (info),
  L (link order), O (extra OS processing required), G (group), T (TLS),
  C (compressed), x (unknown), o (OS specific), E (exclude),
  l (large), p (processor specific)
