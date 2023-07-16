rust
                            if projected_ty.is_bool() {
                                unsafe {
                                    val = llvm::LLVMConstTrunc(val, Type::i1(self.ccx).to_ref());
                                }
                            }
