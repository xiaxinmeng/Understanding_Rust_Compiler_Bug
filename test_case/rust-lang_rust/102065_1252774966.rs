
$ readelf -Ws librustc_llvm-*.rlib | grep LLVMRustContextCreate
    38: 0000000000000000     0 SECTION LOCAL  DEFAULT   61 .text.LLVMRustContextCreate
   259: 0000000000000000    51 FUNC    GLOBAL DEFAULT   61 LLVMRustContextCreate
