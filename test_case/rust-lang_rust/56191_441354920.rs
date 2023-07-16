llvm
%LayoutPrimitiveInfo2 = type { [0 x i32], [7 x float], [0 x i16], i16, [1 x i16] }

; in a function
; loop to initialize all floats to 0
; then...
%9 = getelementptr inbounds %LayoutPrimitiveInfo2, %LayoutPrimitiveInfo2* %0, i32 0, i32 3
store i16 0, i16* %9, align 2
; and implicitly
; %10 = getelementptr inbounds %LayoutPrimitiveInfo2, %LayoutPrimitiveInfo2* %0, i32 0, i32 4
; store [1 x i16] undef, [1 x i16]* %10, align 2
