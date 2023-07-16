
  %749955 = getelementptr inbounds [250000 x %str_slice], [250000 x %str_slice]* %words, i32 0, i32 249985
  %749956 = getelementptr inbounds %str_slice, %str_slice* %749955, i32 0, i32 0
  store i8* getelementptr inbounds ([6 x i8], [6 x i8]* @str.1321, i32 0, i32 0), i8** %749956
  %749957 = getelementptr inbounds %str_slice, %str_slice* %749955, i32 0, i32 1
  store i64 6, i64* %749957
