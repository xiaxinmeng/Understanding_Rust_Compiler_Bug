 llvm
%cmp = icmp eq i8* %ptr, null
%thing = getelementptr i8, i8* %ptr, i64 0
%ptr = select i1 %cmp, i8* null, i8* %thing
%cond = icmp eq i8* %ptr, null
br i1 %cond, label %none, label %some
