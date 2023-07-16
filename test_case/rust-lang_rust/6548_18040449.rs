 rust
pub unsafe fn LLVMBuildAtomicLoad(B: BuilderRef,
                                  PointerVal: ValueRef,
                                  Order: AtomicOrdering)
                               -> ValueRef;
