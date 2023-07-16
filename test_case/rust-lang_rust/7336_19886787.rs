 llvm
;*** IR Dump After Simplify the CFG ***
; Function Attrs: uwtable
define internal fastcc void @_ZN4main16_d7d397690bb61803_00E() #1 {
static_allocas:
%0 = alloca {}, align 8
%1 = bitcast {}* %0 to i8*
call void @llvm.lifetime.start(i64 0, i8* %1)
%2 = load i64** undef, align 8
store i64 1, i64* %2, align 8
call void @llvm.lifetime.end(i64 0, i8* %1)
ret void
}
;*** IR Dump After Combine redundant instructions ***
; Function Attrs: uwtable
define internal fastcc void @_ZN4main16_d7d397690bb61803_00E() #1 {
static_allocas:
store i64* undef, i64** null, align 536870912 ; (536870912 = 0x20000000)
store i64 1, i64* undef, align 8
ret void
}
;*** IR Dump After Tail Call Elimination ***
; Function Attrs: uwtable
define internal fastcc void @_ZN4main16_d7d397690bb61803_00E() #1 {
static_allocas:
store i64* undef, i64** null, align 536870912
store i64 1, i64* undef, align 8
ret void
}
;*** IR Dump After Simplify the CFG ***
; Function Attrs: uwtable
define internal fastcc void @_ZN4main16_d7d397690bb61803_00E() #1 {
static_allocas:
call void @llvm.trap()
unreachable
}
