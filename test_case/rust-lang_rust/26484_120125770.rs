
(lldb) 
frame #5: 0x00000001030bdeed librustc_llvm-7e44814b.dylib`isUnsignedDIType(DD=0x00000001c0d18400, Ty=0x0000000189547d00) + 349 at DwarfUnit.cpp:522
   519          T == dwarf::DW_TAG_structure_type ||
   520          T == dwarf::DW_TAG_union_type)
   521        return true;
-> 522      assert(T == dwarf::DW_TAG_typedef || T == dwarf::DW_TAG_const_type ||
   523             T == dwarf::DW_TAG_volatile_type ||
   524             T == dwarf::DW_TAG_restrict_type ||
   525             T == dwarf::DW_TAG_enumeration_type);
(lldb) p T
(llvm::dwarf::Tag) $0 = DW_TAG_subroutine_type
