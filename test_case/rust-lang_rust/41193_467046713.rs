none
error: need to add support for DW_TAG_base_type '()' encoded with DW_ATE = 0x7, bit_size = 0
error: tokio_mvce DWARF DIE at 0x0005b9e3 (class closure) has a member variable 0x0005b9ea (__0) whose type is a forward declaration, not a complete definition.
Try compiling the source file with -fstandalone-debug
Segmentation fault: 11

$ rustc --version
rustc 1.32.0 (9fda7c223 2019-01-16)

$ lldb --version
lldb-1000.11.38.2
  Swift-4.2
