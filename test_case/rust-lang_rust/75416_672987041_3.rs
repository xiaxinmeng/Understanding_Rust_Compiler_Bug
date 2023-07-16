
        // 2. Also, to inject the new BasicBlock requires adding the block to the end of the vector
        //    of BasicBlocks, and then, since the new block has to be inserted BEFORE another block,
        //    the "next" BasicBlockData has to be swapped with the new intrinsic Call terminator
        //    BasicBlockData, which results in LLVM IR that jumps around more than it would
        //    otherwise. (What was the first block in a Function moves to the last block in the
        //    function, and shows up last in LLVM IR, with a `br`anch to that last block after
        //    incrementing the intrinsic. A custom `Statement` would avoid all of this.
        // 3. A custom statement should allow us to embed metadata in the new Statement variant(s)
        //    without requiring me to encode them as `Operand`s, for which the translations can be
        //    somewhat obscure. Currently, the `Operand` types require very specific `Ty`pe
        //    metadata that was really hard to get right, just to make the intrinsics look like
        //    normal Rust function calls, even though they are never actually used that way.
        // 4. The filename `str` generates an `Allocation` for the string content, and even though
        //    this const operand is _only_ used pre-codegen, this unused alloc still gets injected
        //    into LLVM IR. This entire issue can be avoided with a custom `Statement`.
        // 5. The temporary objects and corresponding `StorageLive` and `StorageDead` `Statements`,
        //    required by the `Call` as a return result (though empty), can be removed.
        // 6. All of the essentially "fake" coverage intrinsic function declarations, and the
        //    associated "lang items" can be removed.
        // 7. The special handling of these calls during code generation can be migrated to more
        //    isolated and coverage-specific handlers of the coverage `Statement`(s), which would
        //    make the rest of the intrinsic codegen handling less confusing.
        //
        // All of these changes will probably result in a simpler and smaller overall
        // implementation.
