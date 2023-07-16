
; ModuleID = 'align.c'
target datalayout = "e-p:64:64:64-i1:8:8-i8:8:8-i16:16:16-i32:32:32-i64:64:64-f32:32:32-f64:64:64-v64:64:64-v128:128:128-a0:0:64-s0:64:64-f80:128:128-n8:16:32:64"
target triple = "x86_64-apple-macosx10.7.3"

%struct.foo = type { %struct.registers_t }
%struct.registers_t = type { [22 x i8], [10 x i8] }

define void @bar(%struct.foo* %s) nounwind ssp {
  %1 = alloca %struct.foo*, align 8
  store %struct.foo* %s, %struct.foo** %1, align 8
  ret void
}

define i32 @main() nounwind ssp {
  ret i32 0
}
