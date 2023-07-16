
define <4 x float> @_ZN3foo20h88e8b14ab8916c70faaE(<4 x float>) unnamed_addr #0 {
entry-block:
  %dropflag_hint_7 = alloca i8
  %x = alloca <4 x float>
  store i8 61, i8* %dropflag_hint_7
  store <4 x float> %0, <4 x float>* %x, align 16
  %1 = getelementptr inbounds <4 x float>, <4 x float>* %x, i32 0, i32 0
  %2 = load float, float* %1, align 4
  %3 = insertelement <4 x float> undef, float %2, i64 0
  %4 = getelementptr inbounds <4 x float>, <4 x float>* %x, i32 0, i32 2
  %5 = load float, float* %4, align 4
  %6 = insertelement <4 x float> %3, float %5, i64 1
  %7 = getelementptr inbounds <4 x float>, <4 x float>* %x, i32 0, i32 3
  %8 = load float, float* %7, align 4
  %9 = insertelement <4 x float> %6, float %8, i64 2
  %10 = getelementptr inbounds <4 x float>, <4 x float>* %x, i32 0, i32 1
  %11 = load float, float* %10, align 4
  %12 = insertelement <4 x float> %9, float %11, i64 3
  ret <4 x float> %12
}

