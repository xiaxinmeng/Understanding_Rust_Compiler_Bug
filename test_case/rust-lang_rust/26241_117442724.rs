
../../projects/rust/src/libstd/lib.rs:1:1: 1:1 note: expansion site
rustc: ../../../../../../../projects/rust/src/llvm/lib/CodeGen/AsmPrinter/DwarfUnit.cpp:525: bool isUnsignedDIType(llvm::DwarfDebug*, const llvm::DIType*): Assertion `T == dwarf::DW_TAG_typedef || T == dwarf::DW_TAG_const_type || T == dwarf::DW_TAG_volatile_type || T == dwarf::DW_TAG_restrict_type || T == dwarf::DW_TAG_enumeration_type' failed.
/home/simon/projects/rust/mk/tests.mk:394: recipe for target 'x86_64-unknown-linux-gnu/stage1/test/stdtest-x86_64-unknown-linux-gnu' failed
make: *** [x86_64-unknown-linux-gnu/stage1/test/stdtest-x86_64-unknown-linux-gnu] Aborted (core dumped)
