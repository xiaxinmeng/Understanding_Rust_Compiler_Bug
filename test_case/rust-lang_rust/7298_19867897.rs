 llvm
define void @_ZN3foo17_87e6ad83abde57f23_00E([8 x i64]* nocapture, { i64, %tydesc*, i8*, i8*, i8 } addrspace(1)* nocapture) #1 {
static_allocas:
   %2 = bitcast [8 x i64]* %0 to i8*
   call void @llvm.memset.p0i8.i64(i8* %2, i8 0, i64 64, i32 8, i1 false)
   ret void
}

define void @_ZN3bar17_87e6ad83abde57f23_00E([8 x i64]* nocapture, { i64, %tydesc*, i8*, i8*, i8 } addrspace(1)* nocapture) #1 {
static_allocas:
   %2 = alloca [8 x i64], align 8
   3 = bitcast [8 x i64]* %2 to i8*
   call void @llvm.memset.p0i8.i64(i8* %3, i8 0, i64 64, i32 8, i1 false)
   %4 = bitcast [8 x i64]* %0 to i8*
   call void @llvm.memcpy.p0i8.p0i8.i64(i8* %4, i8* %3, i64 64, i32 8, i1 false)
   ret void
}
