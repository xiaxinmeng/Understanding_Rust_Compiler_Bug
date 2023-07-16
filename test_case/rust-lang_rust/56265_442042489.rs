
*** IR Dump Before Simplify the CFG ***
; Function Attrs: inlinehint nonlazybind uwtable
define internal fastcc void @"_ZN104_$LT$serde_json..de..MapKey$LT$$u27$a$C$$u20$R$GT$$u20$as$u20$serde..de..Deserializer$LT$$u27$de$GT$$GT$15deserialize_any17h1bbadfee38e7d479E"(%"core::result::Result<alloc::string::String, error::Error>"* noalias nocapture dereferenceable(32), i64* dereferenceable(56)) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality !dbg !3356 {
  %3 = alloca %"core::result::Result<read::Reference<str>, error::Error>", align 8
  %4 = bitcast i64* %1 to %"de::Deserializer<read::StrRead>"*, !dbg !3357
  invoke void @"_ZN46_$LT$serde_json..de..Deserializer$LT$R$GT$$GT$8eat_char17h1a9ac325db7807a1E"(%"de::Deserializer<read::StrRead>"* nonnull dereferenceable(56) %4)
          to label %10 unwind label %37, !dbg !3357

; <label>:5:                                      ; preds = %41, %37
  %6 = phi i8* [ %39, %37 ], [ %43, %41 ]
  %7 = phi i32 [ %40, %37 ], [ %44, %41 ]
  %8 = insertvalue { i8*, i32 } undef, i8* %6, 0, !dbg !3358
  %9 = insertvalue { i8*, i32 } %8, i32 %7, 1, !dbg !3358
  resume { i8*, i32 } %9, !dbg !3358

; <label>:10:                                     ; preds = %2
  %11 = getelementptr inbounds i64, i64* %1, i64 3, !dbg !3359
  %12 = bitcast i64* %11 to %"alloc::vec::Vec<u8>"*, !dbg !3359
  invoke void @"_ZN33_$LT$alloc..vec..Vec$LT$T$GT$$GT$8truncate17hf4079eba74f15922E"(%"alloc::vec::Vec<u8>"* nonnull dereferenceable(24) %12, i64 0)
          to label %13 unwind label %37, !dbg !3360

; <label>:13:                                     ; preds = %10
  %14 = bitcast %"core::result::Result<read::Reference<str>, error::Error>"* %3 to i8*, !dbg !3363
  call void @llvm.lifetime.start.p0i8(i64 32, i8* nonnull %14), !dbg !3363
  %15 = bitcast i64* %1 to %"read::StrRead"*, !dbg !3363
  invoke void @"_ZN96_$LT$serde_json..read..StrRead$LT$$u27$a$GT$$u20$as$u20$serde_json..read..Read$LT$$u27$a$GT$$GT$9parse_str17hce995223c32a683fE"(%"core::result::Result<read::Reference<str>, error::Error>"* noalias nocapture nonnull sret dereferenceable(32) %3, %"read::StrRead"* nonnull dereferenceable(24) %15, %"alloc::vec::Vec<u8>"* nonnull dereferenceable(24) %12)
          to label %16 unwind label %37, !dbg !3363

; <label>:16:                                     ; preds = %13
  %17 = getelementptr inbounds %"core::result::Result<read::Reference<str>, error::Error>", %"core::result::Result<read::Reference<str>, error::Error>"* %3, i64 0, i32 0, i64 0, !dbg !3364
  %18 = load i64, i64* %17, align 8, !dbg !3364, !range !455
  %19 = icmp eq i64 %18, 1, !dbg !3364
  %20 = getelementptr inbounds %"core::result::Result<read::Reference<str>, error::Error>", %"core::result::Result<read::Reference<str>, error::Error>"* %3, i64 0, i32 2, i64 0, !dbg !3364
  %21 = load i64, i64* %20, align 8, !dbg !3364
  br i1 %19, label %30, label %22, !dbg !3364

; <label>:22:                                     ; preds = %16
  %23 = getelementptr inbounds %"core::result::Result<read::Reference<str>, error::Error>", %"core::result::Result<read::Reference<str>, error::Error>"* %3, i64 0, i32 2, i64 1, !dbg !3364
  %24 = bitcast i64* %23 to [0 x i8]**, !dbg !3364
  %25 = load [0 x i8]*, [0 x i8]** %24, align 8, !dbg !3364
  %26 = getelementptr inbounds %"core::result::Result<read::Reference<str>, error::Error>", %"core::result::Result<read::Reference<str>, error::Error>"* %3, i64 0, i32 2, i64 2, !dbg !3364
  %27 = load i64, i64* %26, align 8, !dbg !3364
  %28 = icmp eq i64 %21, 0, !dbg !3365
  %29 = icmp ne [0 x i8]* %25, null
  call void @llvm.assume(i1 %29)
  br i1 %28, label %34, label %35, !dbg !3365

; <label>:30:                                     ; preds = %16
  %31 = getelementptr inbounds %"core::result::Result<alloc::string::String, error::Error>", %"core::result::Result<alloc::string::String, error::Error>"* %0, i64 0, i32 2, i64 0, !dbg !3366
  store i64 %21, i64* %31, align 8, !dbg !3366
  %32 = getelementptr inbounds %"core::result::Result<alloc::string::String, error::Error>", %"core::result::Result<alloc::string::String, error::Error>"* %0, i64 0, i32 0, i64 0, !dbg !3366
  store i64 1, i64* %32, align 8, !dbg !3366
  call void @llvm.lifetime.end.p0i8(i64 32, i8* nonnull %14), !dbg !3369
  br label %33, !dbg !3369

; <label>:33:                                     ; preds = %36, %30
  ret void, !dbg !3370

; <label>:34:                                     ; preds = %22
  invoke void @"_ZN85_$LT$serde..de..impls..StringVisitor$u20$as$u20$serde..de..Visitor$LT$$u27$de$GT$$GT$9visit_str17h49a37b04e6e36535E"(%"core::result::Result<alloc::string::String, error::Error>"* noalias nocapture nonnull sret dereferenceable(32) %0, [0 x i8]* noalias nonnull readonly %25, i64 %27)
          to label %36 unwind label %41, !dbg !3371

; <label>:35:                                     ; preds = %22
  invoke void @"_ZN85_$LT$serde..de..impls..StringVisitor$u20$as$u20$serde..de..Visitor$LT$$u27$de$GT$$GT$9visit_str17h49a37b04e6e36535E"(%"core::result::Result<alloc::string::String, error::Error>"* noalias nocapture nonnull sret dereferenceable(32) %0, [0 x i8]* noalias nonnull readonly %25, i64 %27)
          to label %36 unwind label %41, !dbg !3377

; <label>:36:                                     ; preds = %35, %34
  call void @llvm.lifetime.end.p0i8(i64 32, i8* nonnull %14), !dbg !3369
  br label %33, !dbg !3369

; <label>:37:                                     ; preds = %10, %13, %2
  %38 = landingpad { i8*, i32 }
          cleanup
  %39 = extractvalue { i8*, i32 } %38, 0
  %40 = extractvalue { i8*, i32 } %38, 1
  br label %5

; <label>:41:                                     ; preds = %34, %35
  %42 = landingpad { i8*, i32 }
          cleanup
  %43 = extractvalue { i8*, i32 } %42, 0
  %44 = extractvalue { i8*, i32 } %42, 1
  br label %5
}
BISECT: running pass (8747) Simplify the CFG on function (_ZN104_$LT$serde_json..de..MapKey$LT$$u27$a$C$$u20$R$GT$$u20$as$u20$serde..de..Deserializer$LT$$u27$de$GT$$GT$15deserialize_any17h1bbadfee38e7d479E)
*** IR Dump After Simplify the CFG ***
; Function Attrs: inlinehint nonlazybind uwtable
define internal fastcc void @"_ZN104_$LT$serde_json..de..MapKey$LT$$u27$a$C$$u20$R$GT$$u20$as$u20$serde..de..Deserializer$LT$$u27$de$GT$$GT$15deserialize_any17h1bbadfee38e7d479E"(%"core::result::Result<alloc::string::String, error::Error>"* noalias nocapture dereferenceable(32), i64* dereferenceable(56)) unnamed_addr #0 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality !dbg !3356 {
  %3 = alloca %"core::result::Result<read::Reference<str>, error::Error>", align 8
  %4 = bitcast i64* %1 to %"de::Deserializer<read::StrRead>"*, !dbg !3357
  invoke void @"_ZN46_$LT$serde_json..de..Deserializer$LT$R$GT$$GT$8eat_char17h1a9ac325db7807a1E"(%"de::Deserializer<read::StrRead>"* nonnull dereferenceable(56) %4)
          to label %10 unwind label %34, !dbg !3357

; <label>:5:                                      ; preds = %38, %34
  %6 = phi i8* [ %36, %34 ], [ %40, %38 ]
  %7 = phi i32 [ %37, %34 ], [ %41, %38 ]
  %8 = insertvalue { i8*, i32 } undef, i8* %6, 0, !dbg !3358
  %9 = insertvalue { i8*, i32 } %8, i32 %7, 1, !dbg !3358
  resume { i8*, i32 } %9, !dbg !3358

; <label>:10:                                     ; preds = %2
  %11 = getelementptr inbounds i64, i64* %1, i64 3, !dbg !3359
  %12 = bitcast i64* %11 to %"alloc::vec::Vec<u8>"*, !dbg !3359
  invoke void @"_ZN33_$LT$alloc..vec..Vec$LT$T$GT$$GT$8truncate17hf4079eba74f15922E"(%"alloc::vec::Vec<u8>"* nonnull dereferenceable(24) %12, i64 0)
          to label %13 unwind label %34, !dbg !3360

; <label>:13:                                     ; preds = %10
  %14 = bitcast %"core::result::Result<read::Reference<str>, error::Error>"* %3 to i8*, !dbg !3363
  call void @llvm.lifetime.start.p0i8(i64 32, i8* nonnull %14), !dbg !3363
  %15 = bitcast i64* %1 to %"read::StrRead"*, !dbg !3363
  invoke void @"_ZN96_$LT$serde_json..read..StrRead$LT$$u27$a$GT$$u20$as$u20$serde_json..read..Read$LT$$u27$a$GT$$GT$9parse_str17hce995223c32a683fE"(%"core::result::Result<read::Reference<str>, error::Error>"* noalias nocapture nonnull sret dereferenceable(32) %3, %"read::StrRead"* nonnull dereferenceable(24) %15, %"alloc::vec::Vec<u8>"* nonnull dereferenceable(24) %12)
          to label %16 unwind label %34, !dbg !3363

; <label>:16:                                     ; preds = %13
  %17 = getelementptr inbounds %"core::result::Result<read::Reference<str>, error::Error>", %"core::result::Result<read::Reference<str>, error::Error>"* %3, i64 0, i32 0, i64 0, !dbg !3364
  %18 = load i64, i64* %17, align 8, !dbg !3364, !range !455
  %19 = icmp eq i64 %18, 1, !dbg !3364
  %20 = getelementptr inbounds %"core::result::Result<read::Reference<str>, error::Error>", %"core::result::Result<read::Reference<str>, error::Error>"* %3, i64 0, i32 2, i64 0, !dbg !3364
  %21 = load i64, i64* %20, align 8, !dbg !3364
  br i1 %19, label %29, label %22, !dbg !3364

; <label>:22:                                     ; preds = %16
  %23 = getelementptr inbounds %"core::result::Result<read::Reference<str>, error::Error>", %"core::result::Result<read::Reference<str>, error::Error>"* %3, i64 0, i32 2, i64 1, !dbg !3364
  %24 = bitcast i64* %23 to [0 x i8]**, !dbg !3364
  %25 = load [0 x i8]*, [0 x i8]** %24, align 8, !dbg !3364
  %26 = getelementptr inbounds %"core::result::Result<read::Reference<str>, error::Error>", %"core::result::Result<read::Reference<str>, error::Error>"* %3, i64 0, i32 2, i64 2, !dbg !3364
  %27 = load i64, i64* %26, align 8, !dbg !3364
  %28 = icmp ne [0 x i8]* %25, null
  call void @llvm.assume(i1 %28)
  invoke void @"_ZN85_$LT$serde..de..impls..StringVisitor$u20$as$u20$serde..de..Visitor$LT$$u27$de$GT$$GT$9visit_str17h49a37b04e6e36535E"(%"core::result::Result<alloc::string::String, error::Error>"* noalias nocapture nonnull sret dereferenceable(32) %0, [0 x i8]* noalias nonnull readonly %25, i64 %27)
          to label %33 unwind label %38

; <label>:29:                                     ; preds = %16
  %30 = getelementptr inbounds %"core::result::Result<alloc::string::String, error::Error>", %"core::result::Result<alloc::string::String, error::Error>"* %0, i64 0, i32 2, i64 0, !dbg !3365
  store i64 %21, i64* %30, align 8, !dbg !3365
  %31 = getelementptr inbounds %"core::result::Result<alloc::string::String, error::Error>", %"core::result::Result<alloc::string::String, error::Error>"* %0, i64 0, i32 0, i64 0, !dbg !3365
  store i64 1, i64* %31, align 8, !dbg !3365
  call void @llvm.lifetime.end.p0i8(i64 32, i8* nonnull %14), !dbg !3368
  br label %32, !dbg !3368

; <label>:32:                                     ; preds = %33, %29
  ret void, !dbg !3369

; <label>:33:                                     ; preds = %22
  call void @llvm.lifetime.end.p0i8(i64 32, i8* nonnull %14), !dbg !3368
  br label %32, !dbg !3368

; <label>:34:                                     ; preds = %10, %13, %2
  %35 = landingpad { i8*, i32 }
          cleanup
  %36 = extractvalue { i8*, i32 } %35, 0
  %37 = extractvalue { i8*, i32 } %35, 1
  br label %5

; <label>:38:                                     ; preds = %22
  %39 = landingpad { i8*, i32 }
          cleanup
  %40 = extractvalue { i8*, i32 } %39, 0
  %41 = extractvalue { i8*, i32 } %39, 1
  br label %5
}
