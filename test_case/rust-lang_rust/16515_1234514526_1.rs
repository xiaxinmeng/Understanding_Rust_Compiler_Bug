
define i32 @example::use_foo({ i32*, i32* }* noalias nocapture noundef readonly align 8 dereferenceable(16) %0) unnamed_addr #0 !dbg !5 {
  %2 = bitcast { i32*, i32* }* %0 to i32**, !dbg !10
  %3 = load i32*, i32** %2, align 8, !dbg !10, !nonnull !9, !align !11
  store i32 1, i32* %3, align 4, !dbg !10
  %4 = getelementptr inbounds { i32*, i32* }, { i32*, i32* }* %0, i64 0, i32 1, !dbg !12
  %5 = load i32*, i32** %4, align 8, !dbg !12, !nonnull !9, !align !11, !noundef !9
  store i32 2, i32* %5, align 4, !dbg !12
  %6 = load i32, i32* %3, align 4, !dbg !13
  ret i32 %6, !dbg !14
}
