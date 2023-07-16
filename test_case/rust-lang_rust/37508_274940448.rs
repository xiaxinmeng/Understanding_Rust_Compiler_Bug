
; ModuleID = 'lib.bc'
source_filename = "lib.cgu-0.rs"
target datalayout = "e-m:e-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

@_ZN3lib3KEY17hf88c4e1e7cc4a068E = internal thread_local global { {}, i1 } zeroinitializer, align 1

; Function Attrs: norecurse nounwind readonly uwtable
define i8* @_ZN3lib3set17hd243fd6d5c29a0c6E() unnamed_addr #0 {
entry-block:
  %0 = load i8, i8* bitcast (i1* getelementptr inbounds ({ {}, i1 }, { {}, i1 }* @_ZN3lib3KEY17hf88c4e1e7cc4a068E, i64 0, i32 1) to i8*), align 1, !range !0
  %1 = icmp eq i8 %0, 0
  %. = select i1 %1, i8* bitcast ({ {}, i1 }* @_ZN3lib3KEY17hf88c4e1e7cc4a068E to i8*), i8* null
  ret i8* %.
}

attributes #0 = { norecurse nounwind readonly uwtable }

!0 = !{i8 0, i8 2}
