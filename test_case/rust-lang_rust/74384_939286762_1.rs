llvm
%n = getelementptr inbounds [5 x [5 x [5 x { i8, i8 }]]], [5 x [5 x [5 x { i8, i8 }]]]* %0, i64 0, i64 4, i64 4, i64 3, i32 1
store i8 ..., i8* %n, align 1
