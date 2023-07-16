
cries@cries-desktop MINGW64 ~
$ rustc -g main.rs

cries@cries-desktop MINGW64 ~
$ ./main.exe
bash: ./main.exe: cannot execute binary file: Exec format error

c:\msys64\home\cries>dumpbin /summary main.exe
Microsoft (R) COFF/PE Dumper Version 14.00.22816.0
Copyright (C) Microsoft Corporation.  All rights reserved.


Dump of file main.exe

File Type: EXECUTABLE IMAGE

  Summary

        1000 .CRT
        2000 .bss
        2000 .data
        6000 .debug_abbrev
        3000 .debug_aranges
        9000 .debug_frame
        1000 .debug_gdb_scripts
       2F000 .debug_info
       15000 .debug_line
        8000 .debug_loc
        1000 .debug_pubnames
        1000 .debug_pubtypes
       20000 .debug_ranges
        1000 .debug_str
        1000 .edata
        2000 .idata
        F000 .pdata
       54000 .rdata
        5000 .reloc
       BD000 .text
        1000 .tls
       12000 .xdata

c:\msys64\home\cries>dumpbin /headers main.exe
Microsoft (R) COFF/PE Dumper Version 14.00.22816.0
Copyright (C) Microsoft Corporation.  All rights reserved.


Dump of file main.exe

PE signature found

File Type: EXECUTABLE IMAGE

FILE HEADER VALUES
            8664 machine (x64)
              16 number of sections
        55517ABA time date stamp Mon May 11 20:59:54 2015
          1B4600 file pointer to symbol table
            4B04 number of symbols
              F0 size of optional header
              26 characteristics
                   Executable
                   Line numbers stripped
                   Application can handle large (>2GB) addresses

OPTIONAL HEADER VALUES
             20B magic # (PE32+)
            2.25 linker version
           BC200 size of code
          137800 size of initialized data
            1A00 size of uninitialized data
            1500 entry point (0000000000401500) _ZN3f643f6410to_degrees20h16612ab8d9c3b55asKaE
            1000 base of code
          400000 image base (0000000000400000 to 00000000005C1FFF)
            1000 section alignment
             200 file alignment
            4.00 operating system version
            0.00 image version
            5.02 subsystem version
               0 Win32 version
          1C2000 size of image
             4F8 size of headers
          277478 checksum
               3 subsystem (Windows CUI)
             100 DLL characteristics
                   NX compatible
          200000 size of stack reserve
            1000 size of stack commit
          100000 size of heap reserve
            1000 size of heap commit
               0 loader flags
              10 number of directories
          137000 [      40] RVA [size] of Export Directory
          138000 [    1ABC] RVA [size] of Import Directory
               0 [       0] RVA [size] of Resource Directory
          114000 [    E8F8] RVA [size] of Exception Directory
               0 [       0] RVA [size] of Certificates Directory
          13C000 [    4148] RVA [size] of Base Relocation Directory
               0 [       0] RVA [size] of Debug Directory
               0 [       0] RVA [size] of Architecture Directory
               0 [       0] RVA [size] of Global Pointer Directory
          13B020 [      28] RVA [size] of Thread Storage Directory
               0 [       0] RVA [size] of Load Configuration Directory
               0 [       0] RVA [size] of Bound Import Directory
          138654 [     5C8] RVA [size] of Import Address Table Directory
               0 [       0] RVA [size] of Delay Import Directory
               0 [       0] RVA [size] of COM Descriptor Directory
               0 [       0] RVA [size] of Reserved Directory


SECTION HEADER #1
      /4 name (.debug_gdb_scripts)
      22 virtual size
FFC00000 virtual address (0000000100000000 to 0000000100000021)
     208 size of raw data
     4F8 file pointer to raw data (000004F8 to 000006FF)
       0 file pointer to relocation table
       0 file pointer to line numbers
       0 number of relocations
       0 number of line numbers
42100040 flags
         Initialized Data
         RESERVED - UNKNOWN
         Discardable
         Read Only

SECTION HEADER #2
   .text name
   BC080 virtual size
    1000 virtual address (0000000000401000 to 00000000004BD07F)
   BC200 size of raw data
     800 file pointer to raw data (00000800 to 000BC9FF)
       0 file pointer to relocation table
       0 file pointer to line numbers
       0 number of relocations
       0 number of line numbers
60500060 flags
         Code
         Initialized Data
         RESERVED - UNKNOWN
         RESERVED - UNKNOWN
         Execute Read

SECTION HEADER #3
   .data name
    1180 virtual size
   BE000 virtual address (00000000004BE000 to 00000000004BF17F)
    1200 size of raw data
   BCA00 file pointer to raw data (000BCA00 to 000BDBFF)
       0 file pointer to relocation table
       0 file pointer to line numbers
       0 number of relocations
       0 number of line numbers
C0700040 flags
         Initialized Data
         RESERVED - UNKNOWN
         RESERVED - UNKNOWN
         RESERVED - UNKNOWN
         Read Write

SECTION HEADER #4
  .rdata name
   53B10 virtual size
   C0000 virtual address (00000000004C0000 to 0000000000513B0F)
   53C00 size of raw data
   BDC00 file pointer to raw data (000BDC00 to 001117FF)
       0 file pointer to relocation table
       0 file pointer to line numbers
       0 number of relocations
       0 number of line numbers
40700040 flags
         Initialized Data
         RESERVED - UNKNOWN
         RESERVED - UNKNOWN
         RESERVED - UNKNOWN
         Read Only

SECTION HEADER #5
  .pdata name
    E8F8 virtual size
  114000 virtual address (0000000000514000 to 00000000005228F7)
    EA00 size of raw data
  111800 file pointer to raw data (00111800 to 001201FF)
       0 file pointer to relocation table
       0 file pointer to line numbers
       0 number of relocations
       0 number of line numbers
40300040 flags
         Initialized Data
         RESERVED - UNKNOWN
         RESERVED - UNKNOWN
         Read Only

SECTION HEADER #6
  .xdata name
   11908 virtual size
  123000 virtual address (0000000000523000 to 0000000000534907)
   11A00 size of raw data
  120200 file pointer to raw data (00120200 to 00131BFF)
       0 file pointer to relocation table
       0 file pointer to line numbers
       0 number of relocations
       0 number of line numbers
40300040 flags
         Initialized Data
         RESERVED - UNKNOWN
         RESERVED - UNKNOWN
         Read Only

SECTION HEADER #7
    .bss name
    18C0 virtual size
  135000 virtual address (0000000000535000 to 00000000005368BF)
       0 size of raw data
       0 file pointer to raw data
       0 file pointer to relocation table
       0 file pointer to line numbers
       0 number of relocations
       0 number of line numbers
C0700080 flags
         Uninitialized Data
         RESERVED - UNKNOWN
         RESERVED - UNKNOWN
         RESERVED - UNKNOWN
         Read Write

SECTION HEADER #8
  .edata name
      40 virtual size
  137000 virtual address (0000000000537000 to 000000000053703F)
     200 size of raw data
  131C00 file pointer to raw data (00131C00 to 00131DFF)
       0 file pointer to relocation table
       0 file pointer to line numbers
       0 number of relocations
       0 number of line numbers
40300040 flags
         Initialized Data
         RESERVED - UNKNOWN
         RESERVED - UNKNOWN
         Read Only

SECTION HEADER #9
  .idata name
    1ABC virtual size
  138000 virtual address (0000000000538000 to 0000000000539ABB)
    1C00 size of raw data
  131E00 file pointer to raw data (00131E00 to 001339FF)
       0 file pointer to relocation table
       0 file pointer to line numbers
       0 number of relocations
       0 number of line numbers
C0300040 flags
         Initialized Data
         RESERVED - UNKNOWN
         RESERVED - UNKNOWN
         Read Write

SECTION HEADER #A
    .CRT name
      78 virtual size
  13A000 virtual address (000000000053A000 to 000000000053A077)
     200 size of raw data
  133A00 file pointer to raw data (00133A00 to 00133BFF)
       0 file pointer to relocation table
       0 file pointer to line numbers
       0 number of relocations
       0 number of line numbers
C0400040 flags
         Initialized Data
         RESERVED - UNKNOWN
         Read Write

SECTION HEADER #B
    .tls name
      68 virtual size
  13B000 virtual address (000000000053B000 to 000000000053B067)
     200 size of raw data
  133C00 file pointer to raw data (00133C00 to 00133DFF)
       0 file pointer to relocation table
       0 file pointer to line numbers
       0 number of relocations
       0 number of line numbers
C0600040 flags
         Initialized Data
         RESERVED - UNKNOWN
         RESERVED - UNKNOWN
         Read Write

SECTION HEADER #C
  .reloc name
    4148 virtual size
  13C000 virtual address (000000000053C000 to 0000000000540147)
    4200 size of raw data
  133E00 file pointer to raw data (00133E00 to 00137FFF)
       0 file pointer to relocation table
       0 file pointer to line numbers
       0 number of relocations
       0 number of line numbers
42300040 flags
         Initialized Data
         RESERVED - UNKNOWN
         RESERVED - UNKNOWN
         Discardable
         Read Only

SECTION HEADER #D
     /23 name (.debug_aranges)
    2AF0 virtual size
  141000 virtual address (0000000000541000 to 0000000000543AEF)
    2C00 size of raw data
  138000 file pointer to raw data (00138000 to 0013ABFF)
       0 file pointer to relocation table
       0 file pointer to line numbers
       0 number of relocations
       0 number of line numbers
42500040 flags
         Initialized Data
         RESERVED - UNKNOWN
         RESERVED - UNKNOWN
         Discardable
         Read Only

SECTION HEADER #E
     /38 name (.debug_pubnames)
      96 virtual size
  144000 virtual address (0000000000544000 to 0000000000544095)
     200 size of raw data
  13AC00 file pointer to raw data (0013AC00 to 0013ADFF)
       0 file pointer to relocation table
       0 file pointer to line numbers
       0 number of relocations
       0 number of line numbers
42100040 flags
         Initialized Data
         RESERVED - UNKNOWN
         Discardable
         Read Only

SECTION HEADER #F
     /54 name (.debug_pubtypes)
     2D7 virtual size
  145000 virtual address (0000000000545000 to 00000000005452D6)
     400 size of raw data
  13AE00 file pointer to raw data (0013AE00 to 0013B1FF)
       0 file pointer to relocation table
       0 file pointer to line numbers
       0 number of relocations
       0 number of line numbers
42100040 flags
         Initialized Data
         RESERVED - UNKNOWN
         Discardable
         Read Only

SECTION HEADER #10
     /70 name (.debug_info)
   2E1F9 virtual size
  146000 virtual address (0000000000546000 to 00000000005741F8)
   2E200 size of raw data
  13B200 file pointer to raw data (0013B200 to 001693FF)
       0 file pointer to relocation table
       0 file pointer to line numbers
       0 number of relocations
       0 number of line numbers
42100040 flags
         Initialized Data
         RESERVED - UNKNOWN
         Discardable
         Read Only

SECTION HEADER #11
     /82 name (.debug_abbrev)
    5A79 virtual size
  175000 virtual address (0000000000575000 to 000000000057AA78)
    5C00 size of raw data
  169400 file pointer to raw data (00169400 to 0016EFFF)
       0 file pointer to relocation table
       0 file pointer to line numbers
       0 number of relocations
       0 number of line numbers
42100040 flags
         Initialized Data
         RESERVED - UNKNOWN
         Discardable
         Read Only

SECTION HEADER #12
     /96 name (.debug_line)
   14B8F virtual size
  17B000 virtual address (000000000057B000 to 000000000058FB8E)
   14C00 size of raw data
  16F000 file pointer to raw data (0016F000 to 00183BFF)
       0 file pointer to relocation table
       0 file pointer to line numbers
       0 number of relocations
       0 number of line numbers
42100040 flags
         Initialized Data
         RESERVED - UNKNOWN
         Discardable
         Read Only

SECTION HEADER #13
    /108 name (.debug_frame)
    8460 virtual size
  190000 virtual address (0000000000590000 to 000000000059845F)
    8600 size of raw data
  183C00 file pointer to raw data (00183C00 to 0018C1FF)
       0 file pointer to relocation table
       0 file pointer to line numbers
       0 number of relocations
       0 number of line numbers
42400040 flags
         Initialized Data
         RESERVED - UNKNOWN
         Discardable
         Read Only

SECTION HEADER #14
    /121 name (.debug_str)
     CF7 virtual size
  199000 virtual address (0000000000599000 to 0000000000599CF6)
     E00 size of raw data
  18C200 file pointer to raw data (0018C200 to 0018CFFF)
       0 file pointer to relocation table
       0 file pointer to line numbers
       0 number of relocations
       0 number of line numbers
42100040 flags
         Initialized Data
         RESERVED - UNKNOWN
         Discardable
         Read Only

SECTION HEADER #15
    /132 name (.debug_loc)
    7F51 virtual size
  19A000 virtual address (000000000059A000 to 00000000005A1F50)
    8000 size of raw data
  18D000 file pointer to raw data (0018D000 to 00194FFF)
       0 file pointer to relocation table
       0 file pointer to line numbers
       0 number of relocations
       0 number of line numbers
42100040 flags
         Initialized Data
         RESERVED - UNKNOWN
         Discardable
         Read Only

SECTION HEADER #16
    /143 name (.debug_ranges)
   1F470 virtual size
  1A2000 virtual address (00000000005A2000 to 00000000005C146F)
   1F600 size of raw data
  195000 file pointer to raw data (00195000 to 001B45FF)
       0 file pointer to relocation table
       0 file pointer to line numbers
       0 number of relocations
       0 number of line numbers
42100040 flags
         Initialized Data
         RESERVED - UNKNOWN
         Discardable
         Read Only

  Summary

        1000 .CRT
        2000 .bss
        2000 .data
        6000 .debug_abbrev
        3000 .debug_aranges
        9000 .debug_frame
        1000 .debug_gdb_scripts
       2F000 .debug_info
       15000 .debug_line
        8000 .debug_loc
        1000 .debug_pubnames
        1000 .debug_pubtypes
       20000 .debug_ranges
        1000 .debug_str
        1000 .edata
        2000 .idata
        F000 .pdata
       54000 .rdata
        5000 .reloc
       BD000 .text
        1000 .tls
       12000 .xdata

c:\msys64\home\cries>dumpbin /rawdata /section:.debug_gdb_scripts main.exe
Microsoft (R) COFF/PE Dumper Version 14.00.22816.0
Copyright (C) Microsoft Corporation.  All rights reserved.


Dump of file main.exe

File Type: EXECUTABLE IMAGE

SECTION HEADER #1
      /4 name (.debug_gdb_scripts)
      22 virtual size
FFC00000 virtual address (0000000100000000 to 0000000100000021)
     208 size of raw data
     4F8 file pointer to raw data (000004F8 to 000006FF)
       0 file pointer to relocation table
       0 file pointer to line numbers
       0 number of relocations
       0 number of line numbers
42100040 flags
         Initialized Data
         RESERVED - UNKNOWN
         Discardable
         Read Only

RAW DATA #1
  0000000100000000: 01 67 64 62 5F 6C 6F 61 64 5F 72 75 73 74 5F 70  .gdb_load_rust_p
  0000000100000010: 72 65 74 74 79 5F 70 72 69 6E 74 65 72 73 2E 70  retty_printers.p
  0000000100000020: 79 00                                            y.

  Summary

        1000 .debug_gdb_scripts
