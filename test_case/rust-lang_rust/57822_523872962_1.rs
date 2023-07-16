
< 1><0x0000002a>    DW_TAG_namespace
                      DW_AT_name                  debuginfo_nesting_test
< 2><0x0000002f>      DW_TAG_subprogram
                        DW_AT_low_pc                0x000044f0
                        DW_AT_high_pc               <offset-from-lowpc>141
                        DW_AT_frame_base            len 0x0001: 57: DW_OP_reg7
                        DW_AT_linkage_name          _ZN22debuginfo_nesting_test4main17hbbfb22ccb8684996E
                        DW_AT_name                  main
                        DW_AT_decl_file             0x00000001 /xoxo/stuff/debuginfo_nesting_test/src/main.rs
                        DW_AT_decl_line             0x00000001
                        DW_AT_main_subprogram       yes(1)
< 3><0x00000048>        DW_TAG_lexical_block
                          DW_AT_low_pc                0x000044fc
                          DW_AT_high_pc               <offset-from-lowpc>124
< 4><0x00000055>          DW_TAG_variable
                            DW_AT_location              len 0x0002: 9124: DW_OP_fbreg 36
                            DW_AT_name                  b
                            DW_AT_decl_file             0x00000001 /xoxo/stuff/debuginfo_nesting_test/src/main.rs
                            DW_AT_decl_line             0x00000006
                            DW_AT_type                  <0x00000088>
< 4><0x00000063>          DW_TAG_lexical_block
                            DW_AT_low_pc                0x00004510
                            DW_AT_high_pc               <offset-from-lowpc>54
< 5><0x00000070>            DW_TAG_variable
                              DW_AT_location              len 0x0003: 91f000: DW_OP_fbreg 112
                              DW_AT_name                  arg0
                              DW_AT_alignment             0x00000001
                              DW_AT_decl_file             0x00000001 /xoxo/stuff/debuginfo_nesting_test/src/main.rs
                              DW_AT_decl_line             0x00000007
                              DW_AT_type                  <0x000000a4>
< 2><0x00000083>      DW_TAG_namespace
                        DW_AT_name                  main
< 3><0x00000088>        DW_TAG_structure_type
                          DW_AT_name                  XYZ
                          DW_AT_byte_size             0x00000004
                          DW_AT_alignment             0x00000004
< 4><0x0000008f>          DW_TAG_member
                            DW_AT_name                  a
                            DW_AT_type                  <0x0000009d>
                            DW_AT_alignment             0x00000004
                            DW_AT_data_member_location  0
< 1><0x0000009d>    DW_TAG_base_type
                      DW_AT_name                  u32
                      DW_AT_encoding              DW_ATE_unsigned
                      DW_AT_byte_size             0x00000004
< 1><0x000000a4>    DW_TAG_pointer_type
                      DW_AT_type                  <0x0000009d>
                      DW_AT_name                  &u32
