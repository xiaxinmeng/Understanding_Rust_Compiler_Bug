 llvm
%ptr = getelementptr inbounds i8, i8* %ptr, i64 0
%cond = icmp eq i8* %ptr, null
br i1 %cond, label %none, label %some
