 llvm
; Function Attrs: uwtable
define i32 @_ZN3bar17h73f0a05a6ca431av4v0.0E({ i64, %tydesc*, i8*, i8*, i8 }* nocapture readnone, i32, i32) #3 {
"function top level":
  %3 = alloca %str_slice, align 8
  %4 = alloca %str_slice, align 8
  %5 = call { i32, i1 } @llvm.uadd.with.overflow.i32(i32 %1, i32 %2) #1
  %6 = extractvalue { i32, i1 } %5, 1
  br i1 %6, label %match_else.i, label %_ZN6option6Option6unwrap21h1283b974be2d27c1lBau4v0.0E.exit

match_else.i:                                     ; preds = %"function top level"
  %7 = bitcast %str_slice* %3 to i8*
  call void @llvm.lifetime.start(i64 16, i8* %7)
  %8 = bitcast %str_slice* %4 to i8*
  call void @llvm.lifetime.start(i64 16, i8* %8)
  %9 = getelementptr inbounds %str_slice* %3, i64 0, i32 0
  store i8* getelementptr inbounds ([44 x i8]* @str1262, i64 0, i64 0), i8** %9, align 8
  %10 = getelementptr inbounds %str_slice* %3, i64 0, i32 1
  store i64 43, i64* %10, align 8
  %11 = getelementptr inbounds %str_slice* %4, i64 0, i32 0
  store i8* getelementptr inbounds ([46 x i8]* @str1263, i64 0, i64 0), i8** %11, align 8
  %12 = getelementptr inbounds %str_slice* %4, i64 0, i32 1
  store i64 45, i64* %12, align 8
  call void @"_ZN3sys28FailWithCause$__extensions__9fail_with20hdb4c44d01ce41167qaF4v0.8E"({}* sret undef, { i64, %tydesc*, i8*, i8*, i8 }* undef, %str_slice* %3, %str_slice* %4, i64 323)
  unreachable

_ZN6option6Option6unwrap21h1283b974be2d27c1lBau4v0.0E.exit: ; preds = %"function top level"
  %13 = extractvalue { i32, i1 } %5, 0
  %14 = bitcast %str_slice* %3 to i8*
  call void @llvm.lifetime.start(i64 16, i8* %14)
  %15 = bitcast %str_slice* %4 to i8*
  call void @llvm.lifetime.start(i64 16, i8* %15)
  call void @llvm.lifetime.end(i64 16, i8* %14)
  call void @llvm.lifetime.end(i64 16, i8* %15)
  ret i32 %13
}
