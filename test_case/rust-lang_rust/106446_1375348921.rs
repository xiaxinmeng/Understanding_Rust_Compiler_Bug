llvm
; ModuleID = 'bugpoint-reduced-simplified.bc'
source_filename = "getopts.b321dad9-cgu.8"
target datalayout = "E-m:a-i64:64-n32:64-S128-v256:256:256-v512:512:512"
target triple = "powerpc64-ibm-aix"

define hidden void @_RINvXs5_NtCs3Cu2L953qsK_5alloc6stringNtB6_6StringINtNtNtNtCs7eLrLyzl9su_4core4iter6traits7collect12FromIteratorReE9from_iterINtNtNtBS_8adapters4take4TakeINtNtNtBS_7sources6repeat6RepeatB1L_EEECsc4UU1XxBbkb_7getopts() unnamed_addr #0 personality ptr @rust_eh_personality {
  br i1 poison, label %8, label %1

1:                                                ; preds = %0
  br label %2

2:                                                ; preds = %5, %1
  br i1 poison, label %3, label %5

3:                                                ; preds = %2
  invoke void @_RINvNvMs_NtCs3Cu2L953qsK_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handlehNtNtB9_5alloc6GlobalECsc4UU1XxBbkb_7getopts()
          to label %4 unwind label %6

4:                                                ; preds = %3
  br label %5

5:                                                ; preds = %4, %2
  br i1 poison, label %8, label %2

6:                                                ; preds = %3
  %7 = landingpad { ptr, i32 }
          cleanup
  resume { ptr, i32 } %7

8:                                                ; preds = %5, %0
  ret void
}

declare i32 @rust_eh_personality(i32, i32, i64, ptr, ptr) unnamed_addr #0

declare hidden void @_RINvNvMs_NtCs3Cu2L953qsK_5alloc7raw_vecINtB7_6RawVecppE7reserve21do_reserve_and_handlehNtNtB9_5alloc6GlobalECsc4UU1XxBbkb_7getopts() unnamed_addr #0

attributes #0 = { "target-cpu"="pwr7" }
