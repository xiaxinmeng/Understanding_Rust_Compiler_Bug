llvm
define void @slowness_base_1([3 x i64] * noalias nocapture dereferenceable(24), i1) {
start:
    br i1 %1, label %if_true, label %if_false
if_true:
    %x = getelementptr inbounds [3 x i64], [3 x i64]* %0, i32 0, i32 1
    store i64 1, i64* %x
    br label %ret
if_false:
    %y = getelementptr inbounds [3 x i64], [3 x i64]* %0, i32 0, i32 2
    store i64 1, i64* %y
    br label %ret
ret:
    ret void
}

define i64 @slowness_base_2([3 x i64] * noalias nocapture dereferenceable(24), i1) {
start:
    %allok = alloca [3 x i64]
    %allok_px = getelementptr inbounds [3 x i64], [3 x i64]* %allok, i32 0, i32 2
    store i64 2, i64* %allok_px
    call void @slowness_base_1([3 x i64] * %allok, i1 %1)
    %allok_p = getelementptr inbounds [3 x i64], [3 x i64]* %allok, i32 0, i32 2
    %allok_val = load i64, i64* %allok_p
    ret i64 %allok_val
}
