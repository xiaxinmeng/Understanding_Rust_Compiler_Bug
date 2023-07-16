llvm
@A = thread_local constant i32 1, align 4
@B = local_unnamed_addr constant i32* @A, align 8
