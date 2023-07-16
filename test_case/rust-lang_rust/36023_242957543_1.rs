 llvm
bb3:
  %a = alloca %"2.std::result::Result<std::string::String, std::option::Option<std::string::String>>", align 8
  %a1 = alloca %"2.std::result::Result<std::string::String, std::option::Option<std::string::String>>", align 8
  %0 = bitcast %"2.std::result::Result<std::string::String, std::option::Option<std::string::String>>"* %a to i8*
  call void @llvm.lifetime.start(i64 32, i8* %0)
  call fastcc void @_ZN4test8err_none17h78cf98032df7f4e6E(%"2.std::result::Result<std::string::String, std::option::Option<std::string::String>>"* noalias nocapture nonnull dereferenceable(32) %a)
  %1 = getelementptr inbounds %"2.std::result::Result<std::string::String, std::option::Option<std::string::String>>", %"2.std::result::Result<std::string::String, std::option::Option<std::string::String>>"* %a, i64 0, i32 0
  %2 = load i64, i64* %1, align 8, !range !1
  %switch = icmp eq i64 %2, 1
  %3 = getelementptr inbounds %"2.std::result::Result<std::string::String, std::option::Option<std::string::String>>", %"2.std::result::Result<std::string::String, std::option::Option<std::string::String>>"* %a, i64 0, i32 2
  br i1 %switch, label %bb3.i, label %bb2.i

bb2.i:                                            ; preds = %bb3
  %4 = getelementptr inbounds [3 x i64], [3 x i64]* %3, i64 0, i64 0
  %5 = load i64, i64* %4, align 8, !range !2, !alias.scope !3
  %6 = getelementptr inbounds %"2.std::result::Result<std::string::String, std::option::Option<std::string::String>>", %"2.std::result::Result<std::string::String, std::option::Option<std::string::String>>"* %a, i64 0, i32 2, i64 2
  %7 = load i64, i64* %6, align 8, !alias.scope !10
  br label %bb8

bb3.i:                                            ; preds = %bb3
  br label %bb8
