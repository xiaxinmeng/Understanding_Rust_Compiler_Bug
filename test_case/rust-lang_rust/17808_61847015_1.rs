
.debug_info

COMPILE_UNIT<header overall offset = 0x00000000>:
< 0><0x0000000b>  DW_TAG_compile_unit
                    DW_AT_producer              rustc version 0.13.0-dev (0ecf914ce 2014-10-30 15:06:08 +0500)
                    DW_AT_language              <Unknown LANG value 0x9000>
                    DW_AT_name                  ./rust-version.rs
                    DW_AT_stmt_list             0x00000000
                    DW_AT_comp_dir              /home/mw/stuff/evec-issue
                    DW_AT_low_pc                0x00000000
                    DW_AT_high_pc               <offset-from-lowpc>71

LOCAL_SYMBOLS:
< 1><0x0000002a>    DW_TAG_namespace
                      DW_AT_name                  rust-version
< 2><0x0000002f>      DW_TAG_subprogram
                        DW_AT_low_pc                0x00000000
                        DW_AT_high_pc               <offset-from-lowpc>71
                        DW_AT_frame_base            len 0x0001: 56: DW_OP_reg6
                        DW_AT_MIPS_linkage_name     _ZN12rust-version4mainE
                        DW_AT_name                  main
                        DW_AT_decl_file             0x00000001 /home/mw/stuff/evec-issue/rust-version.rs
                        DW_AT_decl_line             0x00000003
< 3><0x00000048>        DW_TAG_lexical_block
                          DW_AT_low_pc                0x00000029
                          DW_AT_high_pc               <offset-from-lowpc>30
< 4><0x00000055>          DW_TAG_variable
                            DW_AT_location              len 0x0002: 9170: DW_OP_fbreg -16
                            DW_AT_name                  _foo
                            DW_AT_decl_file             0x00000001 /home/mw/stuff/evec-issue/rust-version.rs
                            DW_AT_decl_line             0x00000004
                            DW_AT_type                  <0x00000066>
< 1><0x00000066>    DW_TAG_array_type
                      DW_AT_type                  <0x0000007a>
< 2><0x0000006b>      DW_TAG_subrange_type
                        DW_AT_type                  <0x00000081>
                        DW_AT_lower_bound           0
                        DW_AT_count                 0x00000002
< 2><0x00000072>      DW_TAG_subrange_type
                        DW_AT_type                  <0x00000081>
                        DW_AT_lower_bound           0
                        DW_AT_count                 0x00000001
< 1><0x0000007a>    DW_TAG_base_type
                      DW_AT_name                  uint
                      DW_AT_encoding              DW_ATE_unsigned
                      DW_AT_byte_size             0x00000008
< 1><0x00000081>    DW_TAG_base_type
                      DW_AT_name                  sizetype
                      DW_AT_byte_size             0x00000008
                      DW_AT_encoding              DW_ATE_unsigned

.debug_line: line number info for a single cu
Source lines (from CU-DIE at .debug_info offset 0x0000000b):

            NS new statement, BB new basic block, ET end of text sequence
            PE prologue end, EB epilogue begin
            IA=val ISA number, DI=val discriminator value
<pc>        [row,col] NS BB ET PE EB IS= DI= uri: "filepath"
0x00000000  [   3, 0] NS uri: "/home/mw/stuff/evec-issue/rust-version.rs"
0x00000029  [   4, 0] NS PE
0x0000003d  [   7, 0] NS
0x00000047  [   7, 0] NS ET

.debug_pubnames
global die-in-sect 0x0000002a, cu-in-sect 0x0000000b, die-in-cu 0x0000002a, cu-header-in-sect 0x00000000 'rust-version'
global die-in-sect 0x0000002f, cu-in-sect 0x0000000b, die-in-cu 0x0000002f, cu-header-in-sect 0x00000000 'main'

.debug_macinfo

.debug_string
name at offset 0x00000000, length   62 is 'rustc version 0.13.0-dev (0ecf914ce 2014-10-30 15:06:08 +0500)'
name at offset 0x0000003f, length   17 is './rust-version.rs'
name at offset 0x00000051, length   25 is '/home/mw/stuff/evec-issue'
name at offset 0x0000006b, length   12 is 'rust-version'
name at offset 0x00000078, length   23 is '_ZN12rust-version4mainE'
name at offset 0x00000090, length    4 is 'main'
name at offset 0x00000095, length    4 is '_foo'
name at offset 0x0000009a, length    4 is 'uint'
name at offset 0x0000009f, length    8 is 'sizetype'
