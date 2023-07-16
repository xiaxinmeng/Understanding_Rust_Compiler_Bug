
  %95 = getelementptr inbounds %str_slice, %str_slice* %temp37, i32 0, i32 0
  store i8* getelementptr inbounds ([13 x i8], [13 x i8]* @str63338, i32 0, i32 0), i8** %95
  %96 = getelementptr inbounds %str_slice, %str_slice* %temp37, i32 0, i32 1
  store i64 13, i64* %96
  %97 = getelementptr inbounds %str_slice, %str_slice* %temp37, i32 0, i32 0
  %98 = load i8*, i8** %97
  %99 = getelementptr inbounds %str_slice, %str_slice* %temp37, i32 0, i32 1
  %100 = load i64, i64* %99
  store [0 x %str_slice]* @ref63335, [0 x %str_slice]** %temp40
  %101 = load [0 x %str_slice]*, [0 x %str_slice]** %temp40, !nonnull !0
  %102 = bitcast [0 x %str_slice]* %101 to %str_slice*
  %103 = getelementptr inbounds { %str_slice, { %str_slice*, i64 } }, { %str_slice, { %str_slice*, i64 } }* %temp35, i32 0, i32 0
  %104 = getelementptr inbounds %str_slice, %str_slice* %103, i32 0, i32 0
  store i8* %98, i8** %104
  %105 = getelementptr inbounds %str_slice, %str_slice* %103, i32 0, i32 1
  store i64 %100, i64* %105
  %106 = getelementptr inbounds { %str_slice, { %str_slice*, i64 } }, { %str_slice, { %str_slice*, i64 } }* %temp35, i32 0, i32 1
  %107 = getelementptr inbounds { %str_slice*, i64 }, { %str_slice*, i64 }* %106, i32 0, i32 0
  store %str_slice* %102, %str_slice** %107
  %108 = getelementptr inbounds { %str_slice*, i64 }, { %str_slice*, i64 }* %106, i32 0, i32 1
  store i64 0, i64* %108
  %109 = getelementptr inbounds { %str_slice, { %str_slice*, i64 } }, { %str_slice, { %str_slice*, i64 } }* %temp35, i32 0, i32 0
  %110 = getelementptr inbounds %str_slice, %str_slice* %109, i32 0, i32 0
  %111 = load i8*, i8** %110
  %112 = getelementptr inbounds %str_slice, %str_slice* %109, i32 0, i32 1
  %113 = load i64, i64* %112
  %114 = getelementptr inbounds { %str_slice, { %str_slice*, i64 } }, { %str_slice, { %str_slice*, i64 } }* %temp35, i32 0, i32 1
  %115 = getelementptr inbounds { %str_slice*, i64 }, { %str_slice*, i64 }* %114, i32 0, i32 0
  %116 = load %str_slice*, %str_slice** %115
  %117 = getelementptr inbounds { %str_slice*, i64 }, { %str_slice*, i64 }* %114, i32 0, i32 1
  %118 = load i64, i64* %117
  %119 = invoke i8* @"_ZN5glium2gl2Gl9load_with28_$u7b$$u7b$closure$u7d$$u7d$17h26487c07ccd16bb4E"(%closure.4* dereferenceable(8) %metaloadfn, i8* noalias nonnull readonly %111, i64 %113, %str_slice* noalias nonnull readonly %116, i64 %118)
          to label %bb9 unwind label %cleanup

bb9:                                              ; preds = %bb8
  invoke void @_ZN5glium2gl5FnPtr3new17h33e17555c3c378b0E(%"gl::FnPtr"* noalias nocapture sret dereferenceable(16) %tmp_ret3, i8* %119)
          to label %bb10 unwind label %cleanup

bb10:                                             ; preds = %bb9
  %120 = getelementptr inbounds %"gl::FnPtr", %"gl::FnPtr"* %tmp_ret3, i32 0, i32 0
  %121 = getelementptr inbounds %"gl::FnPtr", %"gl::FnPtr"* %tmp_ret3, i32 0, i32 1
  %122 = load i8*, i8** %120
  %123 = load i8, i8* %121, !range !14
  %124 = trunc i8 %123 to i1
