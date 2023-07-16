llvm
  %8 = load i32, i32* %0, align 4, !tbaa !8, !alias.scope !10, !noalias !13
  store i32 %8, i32* %7, align 4, !tbaa !8, !alias.scope !13, !noalias !10
  store i32 7, i32* %0, align 4, !tbaa !8, !alias.scope !10, !noalias !13
