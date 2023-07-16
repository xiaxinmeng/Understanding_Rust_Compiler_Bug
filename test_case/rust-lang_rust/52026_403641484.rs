llvm
target datalayout = "p:32:32"

%S = type { [2 x i32] }

define i1 @foo([0 x %S]* %p, i32 %n) {
  %start.cast = bitcast [0 x %S]* %p to %S*
  %end = getelementptr inbounds [0 x %S], [0 x %S]* %p, i32 0, i32 %n, i32 0, i32 0
  %end.cast = bitcast i32* %end to %S*
  %last = getelementptr inbounds %S, %S* %end.cast, i32 -1
  %cmp = icmp eq %S* %last, %start.cast
  ret i1 %cmp
}
