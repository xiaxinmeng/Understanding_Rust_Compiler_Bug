
SECTIONS
{
  .debug_gdb_scripts BLOCK(__section_alignment__) (NOLOAD) :
  {
    *(.debug_gdb_scripts)
  }
}
INSERT AFTER .debug_types;
