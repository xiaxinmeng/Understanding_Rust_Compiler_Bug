
$ llvm-dwarfdump -r 1 /my/binary
.debug_info contents:
0x00000000: Compile Unit: length = 0x000012fb version = 0x0004 abbr_offset = 0x0000 addr_size = 0x08 (next unit at 0x000012ff)

[... a bunch of other compilation units with version = 0x0004]

0x010a6be3: Compile Unit: length = 0x0000008f version = 0x0005 unit_type = DW_UT_compile abbr_offset = 0x83eff addr_size = 0x08 (next unit at 0x010a6c76)

0x010a6bef: DW_TAG_compile_unit
              DW_AT_producer	("clang version 14.0.1")
              DW_AT_language	(DW_LANG_C99)
              DW_AT_name	("./lib/builtins/popcountdi2.c")
              DW_AT_str_offsets_base	(0x00000008)
              DW_AT_stmt_list	(0x0065655b)
              DW_AT_comp_dir	("/cargo/registry/src/github.com-1ecc6299db9ec823/compiler_builtins-0.1.71")
              DW_AT_low_pc	(0x000000000147eb10)
              DW_AT_high_pc	(0x000000000147eb72)
              DW_AT_addr_base	(0x00000008)
              DW_AT_loclists_base	(0x0000000c)
