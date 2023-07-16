
@items_data = private constant { i64, i64, [3 x i32] } { i64 12 /* len */, i64 12 /* allocated */, [3 x i32] [i32 10, i32 20, i32 30] }
@items = internal constant { i64, i64, [0 x i32] }* bitcast( ... @items_data)
