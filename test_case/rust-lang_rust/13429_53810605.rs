 LLVM
; ModuleID = 'bad-float-cast.rs'
target datalayout = "e-p:32:32-i64:64-f80:32-n8:16:32"
target triple = "i686-pc-windows-gnu"

%str_slice = type { i8*, i32 }

@str147 = internal constant [4 x i8] c"%f\0A\00"

declare i32 @printf(i8*, double %f) unnamed_addr

; Function Attrs: uwtable
define internal void @print_double(double %f) unnamed_addr {
entry-block:
  %0 = getelementptr inbounds [4 x i8]* @str147, i32 0, i32 0
  call i32 @printf(i8* %0, double %f)
  ret void
}

define i32 @main(i32, i8**) unnamed_addr {
entry-block:
  %a = alloca double
  %b = alloca float
  store double 3.140000e+00, double* %a
  %2 = load double* %a
  %3 = fptrunc double %2 to float
  store float %3, float* %b
  %4 = load float* %b
  %5 = fpext float %4 to double
  call void @print_double(double %5)
  ret i32 0
}
