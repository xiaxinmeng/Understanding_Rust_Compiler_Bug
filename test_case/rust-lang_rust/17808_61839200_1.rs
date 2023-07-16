

.debug_info

COMPILE_UNIT<header overall offset = 0x00000000>:
< 0><0x0000000b>  DW_TAG_compile_unit
                    DW_AT_producer              GNU C 4.8.3 20140911 (Red Hat 4.8.3-7) -mtune=generic -march=x86-64 -g
                    DW_AT_language              DW_LANG_C89
                    DW_AT_name                  ./cversion.c
                    DW_AT_comp_dir              /home/mw/stuff/evec-issue
                    DW_AT_low_pc                0x004004f0
                    DW_AT_high_pc               <offset-from-lowpc>25
                    DW_AT_stmt_list             0x00000000

LOCAL_SYMBOLS:
< 1><0x0000002d>    DW_TAG_subprogram
                      DW_AT_external              yes(1)
                      DW_AT_name                  main
                      DW_AT_decl_file             0x00000001 ./cversion.c
                      DW_AT_decl_line             0x00000002
                      DW_AT_type                  <0x0000005d>
                      DW_AT_low_pc                0x004004f0
                      DW_AT_high_pc               <offset-from-lowpc>25
                      DW_AT_frame_base            len 0x0001: 9c: DW_OP_call_frame_cfa
                      DW_AT_GNU_all_call_sites    yes(1)
                      DW_AT_sibling               <0x0000005d>
< 2><0x0000004e>      DW_TAG_variable
                        DW_AT_name                  _foo
                        DW_AT_decl_file             0x00000001 ./cversion.c
                        DW_AT_decl_line             0x00000003
                        DW_AT_type                  <0x00000064>
                        DW_AT_location              len 0x0002: 9160: DW_OP_fbreg -32
< 1><0x0000005d>    DW_TAG_base_type
                      DW_AT_byte_size             0x00000004
                      DW_AT_encoding              DW_ATE_signed
                      DW_AT_name                  int
< 1><0x00000064>    DW_TAG_array_type
                      DW_AT_type                  <0x0000005d>
                      DW_AT_sibling               <0x0000007a>
< 2><0x0000006d>      DW_TAG_subrange_type
                        DW_AT_type                  <0x0000007a>
                        DW_AT_upper_bound           1
< 2><0x00000073>      DW_TAG_subrange_type
                        DW_AT_type                  <0x0000007a>
                        DW_AT_upper_bound           0
< 1><0x0000007a>    DW_TAG_base_type
                      DW_AT_byte_size             0x00000008
                      DW_AT_encoding              DW_ATE_unsigned
                      DW_AT_name                  sizetype

.debug_line: line number info for a single cu
Source lines (from CU-DIE at .debug_info offset 0x0000000b):

            NS new statement, BB new basic block, ET end of text sequence
            PE prologue end, EB epilogue begin
            IA=val ISA number, DI=val discriminator value
<pc>        [row,col] NS BB ET PE EB IS= DI= uri: "filepath"
0x004004f0  [   2, 0] NS uri: "/home/mw/stuff/evec-issue/./cversion.c"
0x004004f4  [   3, 0] NS
0x00400502  [   5, 0] NS
0x00400507  [   6, 0] NS
0x00400509  [   6, 0] NS ET

.debug_pubnames

.debug_macinfo

.debug_string
name at offset 0x00000000, length   25 is '/home/mw/stuff/evec-issue'
name at offset 0x0000001a, length    8 is 'sizetype'
name at offset 0x00000023, length   12 is './cversion.c'
name at offset 0x00000030, length   70 is 'GNU C 4.8.3 20140911 (Red Hat 4.8.3-7) -mtune=generic -march=x86-64 -g'
name at offset 0x00000077, length    4 is '_foo'
name at offset 0x0000007c, length    4 is 'main'

.debug_aranges

COMPILE_UNIT<header overall offset = 0x00000000>:
< 0><0x0000000b>  DW_TAG_compile_unit
                    DW_AT_producer              GNU C 4.8.3 20140911 (Red Hat 4.8.3-7) -mtune=generic -march=x86-64 -g
                    DW_AT_language              DW_LANG_C89
                    DW_AT_name                  ./cversion.c
                    DW_AT_comp_dir              /home/mw/stuff/evec-issue
                    DW_AT_low_pc                0x004004f0
                    DW_AT_high_pc               <offset-from-lowpc>25
                    DW_AT_stmt_list             0x00000000


arange starts at 0x004004f0, length of 0x00000019, cu_die_offset = 0x0000000b
arange end
