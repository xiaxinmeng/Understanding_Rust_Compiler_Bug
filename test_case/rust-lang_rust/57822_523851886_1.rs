
< 0><0x0000000b>  DW_TAG_compile_unit
                    DW_AT_producer              GNU C17 8.3.0 -mtune=generic -march=x86-64 -g
                    DW_AT_language              DW_LANG_C99
                    DW_AT_name                  test.c
                    DW_AT_comp_dir              F:\rust
                    DW_AT_low_pc                0x00401560
                    DW_AT_high_pc               <offset-from-lowpc>29
                    DW_AT_stmt_list             0x00000729
< 1><0x0000005e>    DW_TAG_subprogram
                      DW_AT_external              yes(1)
                      DW_AT_name                  main
                      DW_AT_decl_file             0x00000001 F:\rust/F:\rust/test.c
                      DW_AT_decl_line             0x00000001
                      DW_AT_decl_column           0x00000005
                      DW_AT_type                  0x000000b0<.debug_info+0x000078f5>
                      DW_AT_low_pc                0x00401560
                      DW_AT_high_pc               <offset-from-lowpc>29
                      DW_AT_frame_base            len 0x0001: 9c: DW_OP_call_frame_cfa
                      DW_AT_GNU_all_tail_call_sites yes(1)
                      DW_AT_sibling               0x000000b0<.debug_info+0x000078f5>
< 2><0x00000081>      DW_TAG_structure_type
                        DW_AT_name                  Foobar
                        DW_AT_byte_size             0x00000004
                        DW_AT_decl_file             0x00000001 F:\rust/F:\rust/test.c
                        DW_AT_decl_line             0x00000002
                        DW_AT_decl_column           0x00000009
                        DW_AT_sibling               0x0000009d<.debug_info+0x000078e2>
< 3><0x00000091>        DW_TAG_member
                          DW_AT_name                  i
                          DW_AT_decl_file             0x00000001 F:\rust/F:\rust/test.c
                          DW_AT_decl_line             0x00000003
                          DW_AT_decl_column           0x00000007
                          DW_AT_type                  0x000000b0<.debug_info+0x000078f5>
                          DW_AT_data_member_location  0
< 2><0x0000009d>      DW_TAG_variable
                        DW_AT_name                  foobar
                        DW_AT_decl_file             0x00000001 F:\rust/F:\rust/test.c
                        DW_AT_decl_line             0x00000005
                        DW_AT_decl_column           0x00000010
                        DW_AT_type                  0x00000081<.debug_info+0x000078c6>
                        DW_AT_location              len 0x0002: 916c: DW_OP_fbreg -20
< 1><0x000000b0>    DW_TAG_base_type
                      DW_AT_byte_size             0x00000004
                      DW_AT_encoding              DW_ATE_signed
                      DW_AT_name                  int
