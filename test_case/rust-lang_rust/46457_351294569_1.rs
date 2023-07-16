
.debug_info contents:
0x00000000: Compile Unit: length = 0x0000008a version = 0x0004 abbr_offset = 0x0000 addr_size = 0x08 (next unit at 0x0000008e)

0x0000000b: DW_TAG_compile_unit [1] *
              DW_AT_producer [DW_FORM_strp]     ( .debug_str[0x00000000] = "clang LLVM (rustc version 1.24.0-dev)")
              DW_AT_language [DW_FORM_data2]    (0x001c)
              DW_AT_name [DW_FORM_strp] ( .debug_str[0x00000026] = "/home/m4b/tmp/bad_debug/nomangle.rs")
              DW_AT_stmt_list [DW_FORM_sec_offset]      (0x00000000)
              DW_AT_comp_dir [DW_FORM_strp]     ( .debug_str[0x0000004a] = "/tmp")
              DW_AT_low_pc [DW_FORM_addr]       (0x00000000000063b0)
              DW_AT_high_pc [DW_FORM_data4]     (0x00000092)

0x0000002a:   DW_TAG_variable [2]  
                DW_AT_name [DW_FORM_strp]       ( .debug_str[0x0000004f] = "TEST")
                DW_AT_type [DW_FORM_ref4]       (cu + 0x0040 => {0x00000040})
                DW_AT_external [DW_FORM_flag_present]   (true)
                DW_AT_decl_file [DW_FORM_data1] (0x01)
                DW_AT_decl_line [DW_FORM_data1] (0x02)
                DW_AT_Unknown_88 [DW_FORM_udata]        (1)
                DW_AT_location [DW_FORM_exprloc]        (<0x9> 03 40 49 06 00 00 00 00 00 )

0x00000040:   DW_TAG_base_type [3]  
                DW_AT_name [DW_FORM_strp]       ( .debug_str[0x0000007d] = "u64")
                DW_AT_encoding [DW_FORM_data1]  (0x07)
                DW_AT_byte_size [DW_FORM_data1] (0x08)

0x00000047:   DW_TAG_namespace [4] *
                DW_AT_name [DW_FORM_strp]       ( .debug_str[0x00000054] = "nomangle")
