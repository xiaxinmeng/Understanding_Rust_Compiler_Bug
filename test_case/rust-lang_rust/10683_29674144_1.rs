 llvm
; Function Attrs: uwtable
define internal void @_ZN4main18hee8776918765f67an4v0.0E({ i64, %tydesc*, i8*, i8*, i8 }*) unnamed_addr #4 {
"function top level":
  %name = alloca { i64, i64, [0 x i8] }*
  %1 = alloca %str_slice
  %2 = alloca { i8*, i32 }
  %3 = alloca %str_slice
  %4 = call { i64, i64, [0 x i8] }* @"_ZN5ascii26StrAsciiExt$__extensions__14to_ascii_lower20h4985d7ef49cc8d0zCaY4v0.0E"({ i64, %tydesc*, i8*, i8*, i8 }* bitcast ({ i8*, i64 }* @_ZN4NAME18h2f2e2b20757b714ah4v0.0E to { i64, %tydesc*, i8*, i8*, i8 }*))
  store { i64, i64, [0 x i8] }* %4, { i64, i64, [0 x i8] }** %name
  %5 = bitcast { i64, i64, [0 x i8] }** %name to { i64, %tydesc*, i8*, i8*, i8 }*
  invoke void @"_ZN3str18Str$__extensions__8as_slice21h994e7dba4c3712c06vaL4v0.0E"(%str_slice* sret %1, { i64, %tydesc*, i8*, i8*, i8 }* %5)
          to label %"normal return" unwind label %unwind

"normal return":                                  ; preds = %"function top level"
  %6 = getelementptr inbounds %str_slice* %3, i32 0, i32 0
  store i8* getelementptr inbounds ([4 x i8]* @str1797, i32 0, i32 0), i8** %6
  %7 = getelementptr inbounds %str_slice* %3, i32 0, i32 1
  store i64 3, i64* %7
  %8 = invoke i8 @_ZN3str8eq_slice19h85916ee1bf3841d9a84v0.0E({ i64, %tydesc*, i8*, i8*, i8 }* undef, %str_slice* %1, %str_slice* %3)
          to label %"normal return2" unwind label %unwind

unwind:                                           ; preds = %"normal return", %"function top level"
  %9 = landingpad { i8*, i32 } personality i32 ()* @upcall_rust_personality
          cleanup
  call void @upcall_reset_stack_limit()
  store { i8*, i32 } %9, { i8*, i32 }* %2
  br label %cleanup

cleanup:                                          ; preds = %unwind
  call void @"_ZN8_$UP$str9glue_drop19h7187567e28f1bbb6aXE"({}* null, { i64, i64, [0 x i8] }** %name)
  %10 = load { i8*, i32 }* %2
  resume { i8*, i32 } %10

case_body:                                        ; preds = %match_case
  br label %join

case_body1:                                       ; preds = %match_else
  br label %join

match_else:                                       ; preds = %compare_next
  br label %case_body1

match_case:                                       ; preds = %"normal return2"
  br label %case_body

"normal return2":                                 ; preds = %"normal return"
  %11 = icmp ne i8 %8, 0
  br i1 %11, label %match_case, label %compare_next

compare_next:                                     ; preds = %"normal return2"
  br label %match_else

join:                                             ; preds = %case_body1, %case_body
  call void @"_ZN8_$UP$str9glue_drop19h7187567e28f1bbb6aXE"({}* null, { i64, i64, [0 x i8] }** %name)
  ret void
}
