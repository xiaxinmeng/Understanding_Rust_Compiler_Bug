
.debug_info contents:
0x00000000: Compile Unit: length = 0x00000679 version = 0x0004 abbr_offset = 0x0000 addr_size = 0x08 (next unit at 0x0000067d)

0x0000000b: DW_TAG_compile_unit [1] *
              DW_AT_producer [DW_FORM_strp]     ( .debug_str[0x00000000] = "clang version 3.9.1 (tags/RELEASE_391/final)")
              DW_AT_language [DW_FORM_data2]    (0x0004)
              DW_AT_name [DW_FORM_strp] ( .debug_str[0x0000002d] = "r.cpp")
              DW_AT_stmt_list [DW_FORM_sec_offset]      (0x00000000)
              DW_AT_comp_dir [DW_FORM_strp]     ( .debug_str[0x00000033] = "/home/m4b/tmp/bad_debug")
              DW_AT_low_pc [DW_FORM_addr]       (0x0000000000000670)
              DW_AT_high_pc [DW_FORM_data4]     (0x00000030)

0x0000002a:   DW_TAG_namespace [2] *
                DW_AT_name [DW_FORM_strp]       ( .debug_str[0x0000017b] = "x")
                DW_AT_decl_file [DW_FORM_data1] (0x01)
                DW_AT_decl_line [DW_FORM_data1] (0x03)

0x00000031:     DW_TAG_variable [3]  
                  DW_AT_name [DW_FORM_strp]     ( .debug_str[0x0000004b] = "ZZQ")
                  DW_AT_type [DW_FORM_ref4]     (cu + 0x0047 => {0x00000047})
                  DW_AT_external [DW_FORM_flag_present] (true)
                  DW_AT_decl_file [DW_FORM_data1]       (0x01)
                  DW_AT_decl_line [DW_FORM_data1]       (0x07)
                  DW_AT_location [DW_FORM_exprloc]      (<0x9> 03 34 10 20 00 00 00 00 00 )
