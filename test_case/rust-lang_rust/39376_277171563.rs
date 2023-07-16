rust
    pub fn load(&self, ptr: ValueRef) -> ValueRef {
        self.count_insn("load");
        unsafe {
            let load = llvm::LLVMBuildLoad(self.llbuilder, ptr, noname())
            let ty = Type::from_ref(llvm::LLVMTypeOf(ptr));
            if ty.is_packed() {
                llvm::LLVMSetAlignment(load, 1 as c_uint);
            }    
            load 
        }    
    }  
