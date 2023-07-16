llvm
define i64 @demo(i64 %x) {
entry:
    switch i64 %x, label %bb3 [
        i64 0, label %bb1
        i64 1, label %bb2
    ]
bb1:
    ret i64 0
bb2:
    %0 = icmp eq i64 %x, 100 ; this will necessarily be false because %x == 1
    br i1 %0, label %bb4, label %bb5
bb3:
    unreachable
bb4:
    ret i64 200
bb5:
    ret i64 10
}
