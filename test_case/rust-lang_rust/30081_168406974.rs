 llvm
%cmp = icmp eq i8* %ptr, null
%thing = getelementptr inbounds i8, i8* %ptr, i64 0
%ptr2 = select i1 %cmp, i8* null, i8* %thing
%cond = icmp eq i8* %ptr2, null
br i1 %cond, label %none, label %some
