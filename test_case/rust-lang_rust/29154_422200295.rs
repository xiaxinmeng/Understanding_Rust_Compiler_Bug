
(lldb) frame variable
error: bin DWARF DIE at 0x0001302f (class Option<char>) has a member variable 0x00013036 (RUST$ENCODED$ENUM$0$None) whose type is a forward declaration, not a complete definition.
Try compiling the source file with -fno-limit-debug-info
error: need to add support for DW_TAG_base_type 'char' encoded with DW_ATE = 0x8, bit_size = 32
Illegal instruction: 4
