 llvm
*** IR Dump Before Module Verifier ***
; Function Attrs: uwtable
define internal i8* @_ZN8simplify20h85a1e2ef0fe110d4SaaE(i8*) unnamed_addr #4 {
entry-block:
  %sret_slot = alloca i8*
  %exp = alloca i8*
  %n = alloca i8***
  %1 = alloca i8**
  %2 = alloca { i8*, i32 }
  store i8* %0, i8** %exp, align 8
  %3 = load i8** %exp
  %4 = icmp ne i8* %3, null
  switch i1 %4, label %match_else [
    i1 true, label %match_case
  ]

case_body:                                        ; preds = %match_case
  %5 = load i8**** %n
  %6 = invoke noalias dereferenceable(8) i8** @"_ZN5boxed18Box$LT$T$GT$.Clone5clone21h17263907606676917116E"(i8*** noalias readonly dereferenceable(8) %5)
          to label %normal-return unwind label %unwind_custom_

case_body1:                                       ; preds = %match_else
  %7 = bitcast i8** %sret_slot to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %7, i8* bitcast (i8** @const861 to i8*), i64 8, i32 8, i1 false)
  br label %join

match_else:                                       ; preds = %entry-block
  br label %case_body1

match_case:                                       ; preds = %entry-block
  %8 = bitcast i8** %exp to i8***
  store i8*** %8, i8**** %n
  br label %case_body

normal-return:                                    ; preds = %case_body
  store i8** %6, i8*** %1, align 8
  %9 = load i8*** %1, align 8, !nonnull !0
  %10 = bitcast i8** %9 to i8*
  %11 = bitcast i8** %sret_slot to i8*
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* %11, i8* %10, i64 8, i32 8, i1 false)
  call void @"_ZN21Box$LT$Expression$GT$9drop.112917hd039fb3d1a2bcdceE"(i8*** %5)
  %12 = bitcast i8*** %5 to i8*
  call void @llvm.memset.p0i8.i64(i8* %12, i8 29, i64 8, i32 8, i1 false)
  br label %join

unwind_custom_:                                   ; preds = %case_body
  %13 = landingpad { i8*, i32 } personality i32 (i32, i32, i64, %"1.std::rt::libunwind::_Unwind_Exception"*, %"1.std::rt::libunwind::_Unwind_Context"*)* @rust_eh_personality
          cleanup
  store { i8*, i32 } %13, { i8*, i32 }* %2
  br label %clean_custom_2

resume:                                           ; preds = %clean_custom_
  %14 = load { i8*, i32 }* %2
  resume { i8*, i32 } %14

clean_custom_:                                    ; preds = %unwind_custom_4, %clean_custom_2
  call void @_ZN10Expression9drop.112617h851faf19442a2b98E(i8** %exp)
  br label %resume

clean_custom_2:                                   ; preds = %unwind_custom_
  call void @"_ZN21Box$LT$Expression$GT$9drop.112917hd039fb3d1a2bcdceE"(i8*** %5)
  %15 = bitcast i8*** %5 to i8*
  call void @llvm.memset.p0i8.i64(i8* %15, i8 29, i64 8, i32 8, i1 false)
  br label %clean_custom_

join:                                             ; preds = %case_body1, %normal-return
  %16 = bitcast i8** %9 to i8*
  invoke void @_ZN4heap13exchange_free20h10b00cb3f415f504kfaE(i8* %16, i64 8, i64 8)
          to label %normal-return3 unwind label %unwind_custom_4

normal-return3:                                   ; preds = %join
  call void @_ZN10Expression9drop.112617h851faf19442a2b98E(i8** %exp)
  %17 = load i8** %sret_slot, align 8
  ret i8* %17

unwind_custom_4:                                  ; preds = %join
  %18 = landingpad { i8*, i32 } personality i32 (i32, i32, i64, %"1.std::rt::libunwind::_Unwind_Exception"*, %"1.std::rt::libunwind::_Unwind_Context"*)* @rust_eh_personality
          cleanup
  store { i8*, i32 } %18, { i8*, i32 }* %2
  br label %clean_custom_
}
