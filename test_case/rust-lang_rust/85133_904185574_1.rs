llvm
define i64 @demo(i64 %x) {
entry:
    %switch = icmp ult i64 %x, 1
    %0 = icmp eq i64 %x, 100
    %. = select i1 %0, i64 200, i64 10
    %merge = select i1 %switch, i64 0, i64 %.
    ret i64 %merge
}
