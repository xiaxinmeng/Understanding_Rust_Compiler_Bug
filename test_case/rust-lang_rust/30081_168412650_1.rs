 llvm
%cmp = true
%thing = poison
%ptr2 = poison ; "Values other than phi nodes depend on their operands." :\
%cond = poison
br i1 %cond, label %none, label %some
