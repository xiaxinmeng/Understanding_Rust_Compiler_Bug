llvm
define internal i32 @_ZN4core3num10NonZeroU323get17h3d02812f86f2e907E(i32) unnamed_addr #0 !dbg !4 {
  %self = alloca i32, align 4
  store i32 %0, i32* %self, align 4
  call void @llvm.dbg.declare(metadata i32* %self, metadata !19, metadata !DIExpression()), !dbg !21
  %1 = load i32, i32* %self, align 4, !dbg !22
  ret i32 %1, !dbg !23
}

define zeroext i1 @_ZN7example3foo17h7a9a6aa0879e3714E(i32) unnamed_addr #1 !dbg !24 {
  %x = alloca i32, align 4
  store i32 %0, i32* %x, align 4
  call void @llvm.dbg.declare(metadata i32* %x, metadata !29, metadata !DIExpression()), !dbg !30
  %1 = load i32, i32* %x, align 4, !dbg !31, !range !32
  %2 = call i32 @_ZN4core3num10NonZeroU323get17h3d02812f86f2e907E(i32 %1), !dbg !31
  br label %bb1, !dbg !31

bb1:                                              ; preds = %start
  %3 = icmp ne i32 %2, 0, !dbg !31
  ret i1 %3, !dbg !33
}

[...]
!32 = !{i32 1, i32 0}
