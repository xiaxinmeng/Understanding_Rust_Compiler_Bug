
$ rustc +1.31 -g --crate-type bin issue-36612.rs
$ readelf -S issue-36612
There are 44 section headers, starting at offset 0x3d5450:

Section Headers:
  [Nr] Name              Type             Address           Offset
       Size              EntSize          Flags  Link  Info  Align
  [17] .debug_gdb_script PROGBITS         00000000000597d9  000597d9
       0000000000000022  0000000000000000 AMS       0     0     1


$ rustc +1.32 -g --crate-type bin issue-36612.rs
$ readelf -S issue-36612
There are 44 section headers, starting at offset 0x248058:

Section Headers:
  [Nr] Name              Type             Address           Offset
       Size              EntSize          Flags  Link  Info  Align
  [17] .debug_gdb_script PROGBITS         00000000000281d0  000281d0
       0000000000000022  0000000000000001 AMS       0     0     1
