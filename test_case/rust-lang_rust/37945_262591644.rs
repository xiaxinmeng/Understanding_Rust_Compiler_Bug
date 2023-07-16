llvm
define zeroext i1 @sroa_fail(float**) {
entry-block:
  %buf0 = alloca float*
  %buf1 = alloca i8*

  %load0 = load float*, float** %0, align 8
  store float* %load0, float** %buf0

  %load1 = load float*, float** %buf0, align 8, !nonnull !0

  %bc = bitcast float* %load1 to i8*
  store i8* %bc, i8** %buf1, align 8

  %load2 = load i8*, i8** %buf1, align 8
  %ret = icmp eq i8* %load2, null
  ret i1 %ret
}

!0 = !{}
